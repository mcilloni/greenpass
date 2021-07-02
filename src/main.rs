use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fmt,
    fs::read,
    io::{self, prelude::*, stdin},
    process::exit,
};

use chrono::prelude::*;
use clap::{AppSettings, Clap};
use flate2::read::ZlibDecoder;
use serde_cbor::Value;
use serde_derive::Deserialize;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Clap)]
#[clap(version = "0.0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// File to process. Omit or specify `-` to read from stdin
    #[clap(default_value = "-")]
    file: String,
}

fn read_stdin() -> Result<Vec<u8>> {
    let mut buf = Vec::new();

    stdin().read_to_end(&mut buf)?;

    Ok(buf)
}

#[derive(Deserialize)]
struct Cwt(pub Vec<Value>);

impl fmt::Display for Cwt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cwt {{ values: [")?;

        let mut iter = self.0.iter();
        if let Some(v1) = iter.next() {
            write!(f, "{:?}", v1)?;

            for v in iter {
                write!(f, ", {:?}", v)?;
            }
        }

        write!(f, "] }}")
    }
}

#[derive(Deserialize)]
struct RawCert(pub BTreeMap<isize, Value>);

impl fmt::Display for RawCert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cert {{\n")?;

        for (k, v) in &self.0 {
            write!(f, "  {}: {:?}\n", k, v)?;
        }

        write!(f, "}}")
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error("invalid base45 in input")]
    InvalidBase45(#[from] base45::DecodeError),

    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("invalid key in document: {0}")]
    InvalidKey(String),

    #[error("invalid format for `{key}`")]
    InvalidFormatFor { key: String },

    #[error("failed to parse a payload as CBOR")]
    MalformedCBOR(#[from] serde_cbor::Error),

    #[error("the root structure for the certificate is malformed")]
    MalformedCWT,

    #[error("malformed date: {0}")]
    MalformedDate(String),

    #[error("found unexpected non-string keys in map")]
    MalformedStringMap,

    #[error("missing initial HC string from input")]
    MissingHCID,

    #[error("invalid key in document: {0}")]
    MissingKey(String),

    #[error("spurious leftover data detected: {0:?}")]
    SpuriousData(BTreeMap<String, Value>),
}

macro_rules! map_empty {
    ($m:expr) => {
        if !$m.is_empty() {
            return Err(Error::SpuriousData($m));
        }
    }
}

// does not work for Tag, which is not needed
macro_rules! gen_extract {
    ($name:ident, $variant:path, $for_type:ty) => {
        fn $name(m: &mut BTreeMap<String, Value>, k: &str) -> Result<$for_type> {
            extract_key(m, k).and_then(|v| match v {
                $variant(r) => Ok(r),
                _ => Err(Error::InvalidFormatFor { key: k.into() }),
            })
        }
    };
}

gen_extract!(extract_array, Value::Array, Vec<Value>);

fn extract_date(m: &mut BTreeMap<String, Value>, k: &str) -> Result<NaiveDate> {
    extract_string(m, k)
        .and_then(|ds| NaiveDate::parse_from_str(&ds, "%F").map_err(|_| Error::MalformedDate(ds)))
}

fn extract_isodatetime(m: &mut BTreeMap<String, Value>, k: &str) -> Result<DateTime<FixedOffset>> {
    extract_string(m, k).and_then(|ds| {
        DateTime::parse_from_str(&ds, "%+")
            .or_else(|_| DateTime::parse_from_str(&ds, "%Y-%m-%dT%H:%M:%S%.f%#z"))
            .or_else(|_| DateTime::parse_from_str(&ds, "%Y-%m-%dT%H:%M:%S%.f%z"))
            .map_err(|_| Error::MalformedDate(ds))
    })
}

gen_extract!(extract_int, Value::Integer, i128);

fn extract_key(m: &mut BTreeMap<String, Value>, k: &str) -> Result<Value> {
    m.remove(k).ok_or_else(|| Error::MissingKey(k.into()))
}

gen_extract!(extract_map, Value::Map, BTreeMap<Value, Value>);

gen_extract!(extract_string, Value::Text, String);

fn extract_string_map(m: &mut BTreeMap<String, Value>, k: &str) -> Result<BTreeMap<String, Value>> {
    to_strmap(k, extract_key(m, k)?)
}

#[derive(Debug)]
enum CertInfo {
    Recovery(Recovery),
    Test(Test),
    Vaccine(Vaccine),
}

#[derive(Debug)]
struct GreenPass {
    date_of_birth: String,  // dob can have weird formats
    surname: String,        // nam/fn
    givenname: String,      // nam/gn
    std_surname: String,    // nam/fnt
    std_givenname: String,  // nam/gnt
    ver: String,            // ver
    entries: Vec<CertInfo>, // [v | t | r]
}

impl TryFrom<BTreeMap<String, Value>> for GreenPass {
    type Error = Error;

    fn try_from(mut values: BTreeMap<String, Value>) -> std::result::Result<Self, Self::Error> {
        let date_of_birth = extract_string(&mut values, "dob")?;
        let ver = extract_string(&mut values, "ver")?;

        let entries = if let Ok(rs) = extract_array(&mut values, "r") {
            rs.into_iter()
                .map(|v| {
                    to_strmap("recovery entry", v)
                        .and_then(Recovery::try_from)
                        .map(CertInfo::Recovery)
                })
                .collect::<Result<_>>()?
        } else if let Ok(ts) = extract_array(&mut values, "t") {
            ts.into_iter()
                .map(|v| {
                    to_strmap("test entry", v)
                        .and_then(Test::try_from)
                        .map(CertInfo::Test)
                })
                .collect::<Result<_>>()?
        } else if let Ok(vs) = extract_array(&mut values, "v") {
            vs.into_iter()
                .map(|v| {
                    to_strmap("vaccine entry", v)
                        .and_then(Vaccine::try_from)
                        .map(CertInfo::Vaccine)
                })
                .collect::<Result<_>>()?
        } else {
            return Err(Error::MissingKey("r, t or v (the actual data)".into()));
        };

        let mut nam = extract_string_map(&mut values, "nam")?;

        let surname = extract_string(&mut nam, "fn")?;
        let givenname = extract_string(&mut nam, "gn")?;
        let std_surname = extract_string(&mut nam, "fnt")?;
        let std_givenname = extract_string(&mut nam, "gnt")?;

        let gp = GreenPass {
            date_of_birth,
            surname,
            givenname,
            std_surname,
            std_givenname,
            ver,
            entries,
        };

        map_empty!(values);

        Ok(gp)
    }
}

#[derive(Debug)]
struct HealthCert {
    some_issuer: Option<String>,
    created: DateTime<Utc>,
    expires: DateTime<Utc>,
    passes: Vec<GreenPass>,
}

#[derive(Debug)]
struct Recovery {
    cert_id: String,        // ci
    country: String,        // co
    diagnosed: NaiveDate,   // fr
    disease: String,        // tg
    issuer: String,         // is
    valid_from: NaiveDate,  // df
    valid_until: NaiveDate, // du
}

impl TryFrom<BTreeMap<String, Value>> for Recovery {
    type Error = Error;

    fn try_from(mut values: BTreeMap<String, Value>) -> std::result::Result<Self, Self::Error> {
        let cert_id = extract_string(&mut values, "ci")?;
        let country = extract_string(&mut values, "co")?;
        let diagnosed = extract_date(&mut values, "fr")?;
        let disease = extract_string(&mut values, "tg")?;
        let issuer = extract_string(&mut values, "is")?;
        let valid_from = extract_date(&mut values, "df")?;
        let valid_until = extract_date(&mut values, "du")?;

        let gp = Recovery {
            cert_id,
            country,
            diagnosed,
            disease,
            issuer,
            valid_from,
            valid_until,
        };

        map_empty!(values);

        Ok(gp)
    }
}

#[derive(Debug)]
enum TestName {
    NAAT { name: String },     // nm
    RAT { device_id: String }, // ma
}

#[derive(Debug)]
struct Test {
    cert_id: String,                   // ci
    collect_ts: DateTime<FixedOffset>, // sc
    country: String,                   // co
    disease: String,                   // tg
    issuer: String,                    // is
    name: TestName,                    // nm | ma
    result: String,                    // tr
    test_type: String,                 // tt
    testing_centre: String,            // tc
}

impl TryFrom<BTreeMap<String, Value>> for Test {
    type Error = Error;

    fn try_from(mut values: BTreeMap<String, Value>) -> std::result::Result<Self, Self::Error> {
        let cert_id = extract_string(&mut values, "ci")?;
        let collect_ts = extract_isodatetime(&mut values, "sc")?;
        let country = extract_string(&mut values, "co")?;
        let disease = extract_string(&mut values, "tg")?;
        let issuer = extract_string(&mut values, "is")?;

        let name = if let Ok(nm) = extract_string(&mut values, "nm") {
            TestName::NAAT { name: nm }
        } else if let Ok(ma) = extract_string(&mut values, "ma") {
            TestName::RAT { device_id: ma }
        } else {
            return Err(Error::MissingKey("ma or nm in test".into()));
        };

        let result = extract_string(&mut values, "tr")?;
        let test_type = extract_string(&mut values, "tt")?;
        let testing_centre = extract_string(&mut values, "tc")?;

        let ts = Test {
            cert_id,
            collect_ts,
            country,
            disease,
            issuer,
            name,
            result,
            test_type,
            testing_centre,
        };

        map_empty!(values);

        Ok(ts)
    }
}

#[derive(Debug)]
struct Vaccine {
    cert_id: String,          // ci
    country: String,          // co
    date: NaiveDate,          // dt
    disease: String,          // tg
    dose_total: usize,        // sd
    dose_number: usize,       // dn
    issuer: String,           // is
    market_auth: String,      // ma
    product: String,          // mp
    prophilaxis_kind: String, // vp
}

impl TryFrom<BTreeMap<String, Value>> for Vaccine {
    type Error = Error;

    fn try_from(mut values: BTreeMap<String, Value>) -> std::result::Result<Self, Self::Error> {
        let cert_id = extract_string(&mut values, "ci")?;
        let country = extract_string(&mut values, "co")?;
        let date = extract_date(&mut values, "dt")?;
        let disease = extract_string(&mut values, "tg")?;
        let dose_number = extract_int(&mut values, "dn")? as usize;
        let dose_total = extract_int(&mut values, "sd")? as usize;
        let issuer = extract_string(&mut values, "is")?;
        let market_auth = extract_string(&mut values, "ma")?;
        let product = extract_string(&mut values, "mp")?;
        let prophilaxis_kind = extract_string(&mut values, "vp")?;

        let gp = Vaccine {
            cert_id,
            country,
            date,
            disease,
            dose_number,
            dose_total,
            issuer,
            market_auth,
            product,
            prophilaxis_kind,
        };

        map_empty!(values);

        Ok(gp)
    }
}

fn to_strmap(desc: &str, v: Value) -> Result<BTreeMap<String, Value>> {
    match v {
        Value::Map(m) => m
            .into_iter()
            .map(|(k, v)| match k {
                Value::Text(s) => Ok((s, v)),
                _ => Err(Error::MalformedStringMap),
            })
            .collect(),
        _ => Err(Error::InvalidFormatFor { key: desc.into() }),
    }
}

fn process_cert(data: String) -> Result<HealthCert> {
    const HCID: &str = "HC1:";

    if !data.starts_with(HCID) {
        return Err(Error::MissingHCID);
    }

    let defl = base45::decode(data[HCID.len()..].trim())?;

    let mut dec = ZlibDecoder::new(&defl as &[u8]);

    let mut data = Vec::new();
    dec.read_to_end(&mut data)?;

    let Cwt(cwt_arr) = serde_cbor::from_slice(&data)?;

    if cwt_arr.len() != 4 {
        return Err(Error::MalformedCWT);
    }

    let RawCert(mut cert_map) = match &cwt_arr[2] {
        Value::Bytes(bys) => serde_cbor::from_slice(bys)?,
        _ => {
            return Err(Error::InvalidFormatFor {
                key: "root cert".into(),
            })
        }
    };

    let some_issuer = if let Some(iss_v) = cert_map.remove(&1) {
        match iss_v {
            Value::Text(iss) => Some(iss),
            _ => {
                return Err(Error::InvalidFormatFor {
                    key: "issuing country".into(),
                })
            }
        }
    } else {
        None
    };

    let expires = match cert_map
        .remove(&4)
        .ok_or_else(|| Error::MissingKey("expiration timestamp".into()))?
    {
        Value::Integer(ts) => Utc.timestamp(ts as i64, 0),
        _ => {
            return Err(Error::InvalidFormatFor {
                key: "expiration timestamp".into(),
            })
        }
    };

    let created = match cert_map
        .remove(&6)
        .ok_or_else(|| Error::MissingKey("issue timestamp".into()))?
    {
        Value::Integer(ts) => Utc.timestamp(ts as i64, 0),
        _ => {
            return Err(Error::InvalidFormatFor {
                key: "issue timestamp".into(),
            })
        }
    };

    let hcerts = match cert_map
        .remove(&-260)
        .ok_or_else(|| Error::MissingKey("hcert".into()))?
    {
        Value::Map(hcmap) => hcmap
            .into_iter()
            .map(|(_, v)| to_strmap("hcert", v))
            .collect::<Result<Vec<_>>>()?,
        _ => {
            return Err(Error::InvalidFormatFor {
                key: "hcert".into(),
            })
        }
    };

    let passes = hcerts
        .into_iter()
        .map(GreenPass::try_from)
        .collect::<Result<Vec<_>>>()?;

    Ok(HealthCert {
        some_issuer,
        created,
        expires,
        passes,
    })
}

fn main_do() -> std::result::Result<(), anyhow::Error> {
    let Opts { file } = Opts::parse();

    let buf = if file == "-" {
        read_stdin()?
    } else {
        read(file)?
    };

    let buf_str = String::from_utf8(buf)?;

    println!("{:#?}", process_cert(buf_str)?);

    Ok(())
}

fn main() {
    if let Err(e) = main_do() {
        eprintln!("error: {}", e);
        exit(-1);
    }
}
