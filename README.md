[![Crate](https://img.shields.io/crates/v/greenpass.svg)](https://crates.io/crates/greenpass)
[![dependency status](https://deps.rs/repo/github/mcilloni/greenpass/status.svg)](https://deps.rs/repo/github/mcilloni/greenpass)
![Build](https://github.com/mcilloni/greenpass/workflows/Build/badge.svg)

# greenpass
A Rust crate to parse EU Digital Green Certificates for COVID-19, with a simple utility to dump certificates with a well formatted output.

Based on the [JSON specification](https://ec.europa.eu/health/sites/default/files/ehealth/docs/covid-certificate_json_specification_en.pdf) and [Technical Specifications](https://ec.europa.eu/health/sites/default/files/ehealth/docs/digital-green-certificates_v1_en.pdf)  for Digital Green Certificates as published by the EU.
Verification of cryptographic signatures is not implemented. Do not use this code to validate certificates for non-educational purposes.

## Usage
This crate is able to load Base45-encoded DGC payloads. 
It does not directly support barcode parsing, but can be used alongside [ZBar](http://zbar.sourceforge.net/) to read Digital Green Certificates from images: 

```shell
$ greenpass <(zbarimg -q pass.png | sed s/QR-Code://)
EU Digital COVID Certificate

Issued by: AT
Created at: 2021-07-02 23:58:57 UTC
Expires at: 2022-07-02 23:58:57 UTC

Pass#0:
    Cert version 1.2.1
    Emitted to: Gabriele Musterfrau-Gößinger
    Standardized Name: GABRIELE MUSTERFRAU<GOESSINGER
    Date of birth: 1998-02-26

    Vaccination data:
        Cert ID: URN:UVCI:01:AT:10807843F94AEE0EE5093FBC254BD813#B
        Disease: 840539006
        Issuer: Ministry of Health, Austria
        Country: AT
        Vaccination date: 2021-02-18
        Doses administered: 1/2
        Product ID: EU/1/20/1528
        Market Authorization ID: ORG-100030215
        Vaccine/Prophylaxis ID: 1119349007
 ```

The certificate above is fictitional, and has been generated using [this utility](https://dgc.a-sit.at/ehn/).

## Parse certificates from code

The crate can also be used as a library:

```Rust
let vac_hc = HealthCert {
    some_issuer: Some("AT".into()),
    created: Utc.ymd(2021, 07, 02).and_hms(23, 58, 57),
    expires: Utc.ymd(2022, 07, 02).and_hms(23, 58, 57),
    passes: vec![GreenPass {
        date_of_birth: "1998-02-26".into(),
        surname: "Musterfrau-Gößinger".into(),
        givenname: "Gabriele".into(),
        std_surname: "MUSTERFRAU<GOESSINGER".into(),
        std_givenname: "GABRIELE".into(),
        ver: "1.2.1".into(),
        entries: vec![CertInfo::Vaccine(Vaccine {
            cert_id: "URN:UVCI:01:AT:10807843F94AEE0EE5093FBC254BD813#B".into(),
            country: "AT".into(),
            date: NaiveDate::from_ymd(2021, 02, 18),
            disease: "840539006".into(),
            dose_number: 1,
            dose_total: 2,
            issuer: "Ministry of Health, Austria".into(),
            market_auth: "ORG-100030215".into(),
            product: "EU/1/20/1528".into(),
            prophylaxis_kind: "1119349007".into(),
        })],
    }],
};

let buf_str = std::fs::read_to_str("vac_base45.txt")?;
let hc_parsed = greenpass::parse(&buf_str)?;

assert_eq!(hc_parsed, vac_hc);
```

## Fuzzing

To run the fuzzer, cargo-fuzz is required

`cargo install cargo-fuzz`

Then to do a basic fuzzing campaign

`cargo +nightly fuzz run greenpass_parse`

There are more options for cargo-fuzz in their documentation.

