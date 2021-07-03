use std::{
    cell::RefCell,
    fs::read,
    io::{self, prelude::*, stdin},
    iter::repeat,
    process::exit,
};

use clap::{AppSettings, Clap};
use greenpass::{CertInfo, GreenPass, HealthCert, Recovery, Test, TestName, Vaccine};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Utility to quickly inspect EU Digital Green Certificates. Does not support validation yet.
#[derive(Clap)]
#[clap(version = VERSION)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// File containing a Base45 QR code payload.
    /// Omit or specify `-` to read from stdin
    #[clap(default_value = "-")]
    file: String,
}

fn read_stdin() -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();

    stdin().read_to_end(&mut buf)?;

    Ok(buf)
}

thread_local! {
    static WS: RefCell<String> = RefCell::default();
}

fn pad(n: usize) {
    WS.with(|wsbuf| {
        if wsbuf.borrow().len() != n {
            *wsbuf.borrow_mut() = repeat(" ").take(n).collect();
        }

        print!("{}", wsbuf.borrow());
    });
}

macro_rules! padn {
    () => (println!());
    ($n:expr, $($arg:tt)*) => ({
        pad($n);
        println!($($arg)*);
    })
}

macro_rules! pad4 {
    () => (println!());
    ($($arg:tt)*) => (padn!(4, $($arg)*));
}

macro_rules! pad8 {
    () => (println!());
    ($($arg:tt)*) => (padn!(8, $($arg)*));
}

fn dump_recovery(r: &Recovery) {
    let Recovery {
        cert_id,
        country,
        diagnosed,
        disease,
        issuer,
        valid_from,
        valid_until,
    } = r;

    pad4!("Recovery attestation:");
    pad8!("Cert ID: {}", cert_id);
    pad8!("Disease: {}", disease);
    pad8!("Issuer: {}", issuer);
    pad8!("Country: {}", country);
    pad8!("Tested positive: {}", diagnosed);
    pad8!("Valid from: {}", valid_from);
    pad8!("Valid until: {}", valid_until);
}

fn dump_test(t: &Test) {
    let Test {
        cert_id,
        collect_ts,
        country,
        disease,
        issuer,
        name,
        result,
        test_type,
        testing_centre,
    } = t;

    pad4!("Testing attestation:");
    pad8!("Cert ID: {}", cert_id);
    pad8!("Disease: {}", disease);
    pad8!("Result code: {}", result);
    pad8!("Samples collected at: {}", collect_ts);

    let tn_str = match name {
        TestName::NAAT { name } => format!("Nucleic Acid Amplification Test ({})", name),
        TestName::RAT { device_id } => format!("Rapid Antigen Test (device: {})", device_id),
    };

    pad8!("Test type: {}, ID: {}", tn_str, test_type);
    pad8!("Conducted by: {}", testing_centre);
    pad8!("Issuer: {}", issuer);
    pad8!("Country: {}", country);
}

fn dump_vaccination(v: &Vaccine) {
    let Vaccine {
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
    } = v;

    pad4!("Vaccination data:");

    pad8!("Cert ID: {}", cert_id);
    pad8!("Disease: {}", disease);
    pad8!("Issuer: {}", issuer);
    pad8!("Country: {}", country);
    pad8!("Vaccination date: {}", date);
    pad8!("Doses administered: {}/{}", dose_number, dose_total);
    pad8!("Product ID: {}", product);
    pad8!("Market Authorization ID: {}", market_auth);
    pad8!("Vaccine/Prophylaxis ID: {}", prophylaxis_kind);
}

fn dump_greenpass(gp: &GreenPass) {
    let GreenPass {
        date_of_birth,
        surname,
        givenname,
        std_surname,
        std_givenname,
        ver,
        entries,
    } = gp;

    pad4!("Cert version {}", ver);

    pad4!("Emitted to: {} {}", givenname, surname);
    pad4!("Standardized Name: {} {}", std_givenname, std_surname);
    pad4!("Date of birth: {}\n", date_of_birth);

    for ci in entries {
        match ci {
            CertInfo::Recovery(r) => dump_recovery(r),
            CertInfo::Test(t) => dump_test(t),
            CertInfo::Vaccine(v) => dump_vaccination(v),
        }
    }
}

fn dump_hc(hc: &HealthCert) {
    let HealthCert {
        created,
        expires,
        passes,
        some_issuer,
    } = hc;

    println!("EU Digital COVID Certificate\n");

    if let Some(issuer) = some_issuer {
        println!("Issued by: {}", issuer);
    }

    println!("Created at: {}", created);
    println!("Expires at: {}\n", expires);

    for (i, pass) in passes.iter().enumerate() {
        println!("Pass#{}:", i);
        dump_greenpass(pass);
    }
}

fn main_do() -> std::result::Result<(), anyhow::Error> {
    let Opts { file } = Opts::parse();

    let buf = if file == "-" {
        read_stdin()?
    } else {
        read(file)?
    };

    if !buf.is_empty() {
        let buf_str = String::from_utf8(buf)?;

        dump_hc(&greenpass::parse(&buf_str)?);
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_do() {
        eprintln!("error: {}", e);
        exit(-1);
    }
}
