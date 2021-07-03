use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fmt,
    io::{self, Read},
};

use chrono::prelude::*;
use flate2::read::ZlibDecoder;
use serde_cbor::Value;
use serde_derive::Deserialize;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize)]
struct Cwt(Vec<Value>);

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
struct RawCert(BTreeMap<isize, Value>);

/// Error type that represents every possible error condition encountered while loading a certificate
#[derive(Debug, Error)]
pub enum Error {
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
    };
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

gen_extract!(extract_string, Value::Text, String);

fn extract_string_map(m: &mut BTreeMap<String, Value>, k: &str) -> Result<BTreeMap<String, Value>> {
    to_strmap(k, extract_key(m, k)?)
}

#[derive(Debug, PartialEq)]
pub enum CertInfo {
    Recovery(Recovery),
    Test(Test),
    Vaccine(Vaccine),
}

/// Structure that represents a Green Pass entry.
#[derive(Debug, PartialEq)]
pub struct GreenPass {
    /// Date of birth
    pub date_of_birth: String, // dob can have weird formats

    /// Family name
    pub surname: String, // nam/fn

    /// First name
    pub givenname: String, // nam/gn

    /// Family name in standardized form (see docs)
    pub std_surname: String, // nam/fnt

    /// First name in standardized form
    pub std_givenname: String, // nam/gnt

    /// Document version
    pub ver: String, // ver

    /// Attestation of immunity from an illness due to vaccination, recovery or a negative test
    pub entries: Vec<CertInfo>, // [v | t | r]
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

/// Represents the whole certificate blob (excluding metadata and signature, which are unsupported at the moment)
#[derive(Debug, PartialEq)]
pub struct HealthCert {
    // Member country that issued the bundle (might be missing)
    pub some_issuer: Option<String>,

    /// Bundle creation timestamp
    pub created: DateTime<Utc>,

    /// Bundle expiration timestamp
    pub expires: DateTime<Utc>,

    /// List of passes contained in this bundle
    pub passes: Vec<GreenPass>,
}

/// Attests the full recovery from a given disease
#[derive(Debug, PartialEq)]
pub struct Recovery {
    /// Certificate ID
    pub cert_id: String, // ci

    /// Member State where the test was performed
    pub country: String, // co

    /// Date of diagnosis
    pub diagnosed: NaiveDate, // fr

    /// String that identifies the contracted disease
    pub disease: String, // tg

    /// Issuing entity
    pub issuer: String, // is

    /// Recovery attestation validity start date
    pub valid_from: NaiveDate, // df

    /// Recovery attestation validity expire date
    pub valid_until: NaiveDate, // du
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

/// Identifies the recognized test types
#[derive(Debug, PartialEq)]
pub enum TestName {
    /// A Nucleic Acid Amplification Test, with the name of the specific test
    NAAT { name: String }, // nm

    /// A Rapid Antigen Test, with a string identifying the device from the JRC database
    RAT { device_id: String }, // ma
}

/// Attests that a test for a given disease has been conducted.
#[derive(Debug, PartialEq)]
pub struct Test {
    /// Certificate ID
    pub cert_id: String, // ci

    /// Date and time when samples where collected
    pub collect_ts: DateTime<FixedOffset>, // sc

    /// Member State where the test was performed
    pub country: String, // co

    /// Target disease
    pub disease: String, // tg

    /// Issuing entity
    pub issuer: String, // is

    /// Name and identifier of the used testing technology
    pub name: TestName, // nm | ma

    /// Test result, as defined in  SNOMED CT GPS
    pub result: String, // tr

    /// Coded string value identifying the testing method
    pub test_type: String, // tt

    /// Name of the centre that conducted the test
    pub testing_centre: String, // tc
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

/// Attests that an individual has been vaccinated for a given disease.
#[derive(Debug, PartialEq)]
pub struct Vaccine {
    /// Certificate ID
    pub cert_id: String, // ci

    /// Vaccination country
    pub country: String, // co

    /// Vaccination date
    pub date: NaiveDate, // dt

    /// Targeted disease
    pub disease: String, // tg

    /// Number of administered doses
    pub dose_number: usize, // dn

    /// Total number of doses required by the administered vaccine
    pub dose_total: usize, // sd

    /// Issuing entity
    pub issuer: String, // is

    /// EUDCC Gateway market authorization identifier
    pub market_auth: String, // ma

    /// Product identifier as defined in EUDCC Gateway
    pub product: String, // mp

    /// Type of vaccine or prophylaxis used as defined in EUDCC Gateway
    pub prophylaxis_kind: String, // vp
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
        let prophylaxis_kind = extract_string(&mut values, "vp")?;

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
            prophylaxis_kind,
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

impl TryFrom<&str> for HealthCert {
    type Error = Error;

    fn try_from(data: &str) -> std::result::Result<Self, Self::Error> {
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
}

/// Parses a Base45 CBOR Web Token containing a EU Health Certificate. No signature validation is currently performed by
/// this crate.
///
/// ```no_run
/// use std::{error::Error, fs::read_to_string};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     // Read a Base45 payload extracted from a QR code
///     let buf_str = read_to_string("base45_file.txt")?;
///
///     let health_cert = greenpass::parse(&buf_str)?;
///
///     println!("{:#?}", health_cert);
///     
///     Ok(())
/// }
/// ```
pub fn parse(data: &str) -> Result<HealthCert> {
    HealthCert::try_from(data)
}
