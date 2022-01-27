// Value Sets for the Digital COVID Certificates according to https://ec.europa.eu/health/system/files/2022-01/digital-green-value-sets_en.pdf

// 2.1 Disease or agent targeted / Disease or agent the citizen has recovered from
// Fully described in the Implementing Decision.

// 2.2 COVID-19 vaccine or prophylaxis

#[derive(Debug, Clone, Copy)]
pub enum VaccineProphylaxis {
    Antigen,
    MRNA,
    Vaccine,
}

impl VaccineProphylaxis {
    pub fn values(&self) -> (&str, &str, &str, &str, &str, &str) {
        match *self {
            VaccineProphylaxis::Antigen => (
                "1119305005",                 // Code
                "SARS-CoV-2 antigen vaccine", // Display
                "SNOMED CT",                  // Code System name
                "http://snomed.info/sct",     // Code System URL
                "2.16.840.1.113883.6.96",     // Code System OID
                "2021-01-31",                 // Code System version
            ),
            VaccineProphylaxis::MRNA => (
                "1119349007",              // Code
                "SARS-CoV-2 mRNA vaccine", // Display
                "SNOMED CT",               // Code System name
                "http://snomed.info/sct",  // Code System URL
                "2.16.840.1.113883.6.96",  // Code System OID
                "2021-01-31",              // Code System version
            ),
            VaccineProphylaxis::Vaccine => (
                "J07BX03",                                               // Code
                "covid-19 vaccines",                                     // Display
                "Anatomical Therapeutic Chemical Classification System", // Code System name
                "http://www.whocc.no/atc",                               // Code System URL
                "2.16.840.1.113883.6.73",                                // Code System OID
                "2021-01",                                               // Code System version
            ),
        }
    }
}

// 2.3 Vaccine medicinal product
#[derive(Debug, Clone, Copy)]
pub enum VaccineMedicinalProduct {
    Comirnaty,
    Spikevax,
    Vaxzevria,
    COVID19VaccineJanssen,
    CVnCoV,
    NVXCoV2373,
    SputnikV,
    Convidecia,
    EpiVacCorona,
    BBIBPCorV,
    InactivatedSARSCoV2,
    VeroCell,
    CoronaVac,
    Covaxin,
    BBV152ABC,
    Covishield,
    ChAdOx1nCoV19,
    Covid19Recombinant,
    RCOVI,
    CoviVac,
    SputnikLight,
    HayatVax,
    Abdala,
    WIBPCorV,
    MVCCOVID19Vaccine,
    Nuvaxovid,
}

#[derive(Debug, Clone, Copy)]
pub enum VaccineAuthorizationStatus {
    // Union Register of medicinal products (https://ec.europa.eu/health/documents/community-register/html/)
    CentrallyAuthorized,
    // Vaccine medicinal products not centrally authorized in the EU in rolling review by EMA
    InRollingReview,
    // Vaccine medicinal products not centrally authorized in the EU
    NotAuthorized,
}

#[derive(Debug, Clone, Copy)]
pub enum CodeSystemVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
    V1_4,
    V1_5,
    V1_6,
}

impl VaccineMedicinalProduct {
    pub fn values(
        &self,
    ) -> (
        &str,
        &str,
        VaccineAuthorizationStatus,
        Option<CodeSystemVersion>,
    ) {
        match *self {
            VaccineMedicinalProduct::Comirnaty => (
                "EU/1/20/1528",
                "Comirnaty",
                VaccineAuthorizationStatus::CentrallyAuthorized,
                None,
            ),
            VaccineMedicinalProduct::Spikevax => (
                "EU/1/20/1507",
                "Spikevax",
                VaccineAuthorizationStatus::CentrallyAuthorized,
                None,
            ),
            VaccineMedicinalProduct::Vaxzevria => (
                "EU/1/21/1529",
                "Vaxzevria",
                VaccineAuthorizationStatus::CentrallyAuthorized,
                None,
            ),
            VaccineMedicinalProduct::COVID19VaccineJanssen => (
                "EU/1/20/1525",
                "COVID-19 Vaccine Janssen",
                VaccineAuthorizationStatus::CentrallyAuthorized,
                None,
            ),
            VaccineMedicinalProduct::CVnCoV => (
                "CVnCoV",
                "CVnCoV",
                VaccineAuthorizationStatus::InRollingReview,
                Some(CodeSystemVersion::V1_0),
            ),
            VaccineMedicinalProduct::NVXCoV2373 => (
                "NVX-CoV2373 (deprecated,see Annex A for more instructions",
                "NVX-CoV2373",
                VaccineAuthorizationStatus::InRollingReview,
                Some(CodeSystemVersion::V1_0),
            ),
            VaccineMedicinalProduct::SputnikV => (
                "
                Sputnik-V",
                "Sputnik V",
                VaccineAuthorizationStatus::InRollingReview,
                Some(CodeSystemVersion::V1_0),
            ),
            VaccineMedicinalProduct::Convidecia => (
                "Convidecia",
                "Convidecia",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_0),
            ),
            VaccineMedicinalProduct::EpiVacCorona => (
                "EpiVacCorona",
                "EpiVacCorona",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_0),
            ),
            VaccineMedicinalProduct::BBIBPCorV => (
                "BBIBP-CorV",
                "BBIBP-CorV",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_0),
            ),
            VaccineMedicinalProduct::InactivatedSARSCoV2 | VaccineMedicinalProduct::VeroCell => (
                "Inactivated-SARS-CoV-2-Vero-Cell (deprecated, see Annex A for more instructions)",
                "Inactivated SARS-CoV-2 (Vero Cell)",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_0),
            ),
            VaccineMedicinalProduct::CoronaVac => (
                "CoronaVac",
                "CoronaVac",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_0),
            ),
            VaccineMedicinalProduct::Covaxin | VaccineMedicinalProduct::BBV152ABC => (
                "Covaxin",
                "Covaxin (also known as BBV152 A, B, C)",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_0),
            ),
            VaccineMedicinalProduct::Covishield | VaccineMedicinalProduct::ChAdOx1nCoV19 => (
                "Covishield",
                "Covishield (ChAdOx1_n CoV-19)",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_2),
            ),
            VaccineMedicinalProduct::Covid19Recombinant => (
                "Covid-19-recombinant",
                "Covid-19 (recombinant)",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_3),
            ),
            VaccineMedicinalProduct::RCOVI => (
                "R-COVI",
                "R-COVI",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_3),
            ),
            VaccineMedicinalProduct::CoviVac => (
                "CoviVac",
                "CoviVac",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_4),
            ),
            VaccineMedicinalProduct::SputnikLight => (
                "Sputnik-Light",
                "Sputnik Light",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_4),
            ),
            VaccineMedicinalProduct::HayatVax => (
                "Hayat-Vax",
                "Hayat-Vax",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_4),
            ),
            VaccineMedicinalProduct::Abdala => (
                "Abdala",
                "Abdala",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_5),
            ),
            VaccineMedicinalProduct::WIBPCorV => (
                "WIBP-CorV",
                "WIBP-CorV",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_5),
            ),
            VaccineMedicinalProduct::MVCCOVID19Vaccine => (
                "MVC-COV1901",
                "MVC COVID-19 vaccine",
                VaccineAuthorizationStatus::NotAuthorized,
                Some(CodeSystemVersion::V1_6),
            ),
            VaccineMedicinalProduct::Nuvaxovid => (
                "EU/1/21/1618",
                "Nuvaxovid",
                VaccineAuthorizationStatus::CentrallyAuthorized,
                None,
            ),
        }
    }
}

// 2.4 COVID-19 vaccine marketing authorization holder or manufacturer
#[derive(Debug, Clone, Copy)]
pub enum ManufacturerInOMS {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy)]
pub enum Manufacturer {
    AstraZeneca,
    BiontechManufacturing,
    JanssenCilagInternational,
    ModernaBiotechSpain,
    Curevac,
    CanSinoBiologics,
    ChinaSinopharm,
    SinopharmWeiqidaPrague,
    SinopharmZhijun,
    Novavax,
    GamaleyaResearchInstitute,
    VectorInstitute,
    SinovacBiotech,
    BharatBiotech,
    SerumInstituteOfIndia,
    Fiocruz,
    RPharmCJSC,
    Chumakov,
    GulfPharmaceutical,
    CIGB,
    SinopharmWuhan,
    Medigen,
}

impl Manufacturer {
    pub fn values(&self) -> (&str, &str, ManufacturerInOMS, Option<CodeSystemVersion>) {
        match *self {
            Manufacturer::AstraZeneca => (
                "ORG-100001699",
                "AstraZeneca AB",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::BiontechManufacturing => (
                "ORG-100030215",
                "Biontech Manufacturing GmbH",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::JanssenCilagInternational => (
                "ORG-100001417",
                "Janssen-Cilag International",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::ModernaBiotechSpain => (
                "ORG-100031184",
                "Moderna Biotech Spain S.L.",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::Curevac => (
                "ORG-100006270",
                "Curevac AG",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::CanSinoBiologics => (
                "ORG-100013793",
                "CanSino Biologics",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::ChinaSinopharm => (
                "ORG-100020693",
                "China Sinopharm International Corp. - Beijing location",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::SinopharmWeiqidaPrague => (
                "ORG-100010771",
                "Sinopharm Weiqida Europe Pharmaceutical s.r.o. - Prague location",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::SinopharmZhijun => (
                "ORG-100024420",
                "Sinopharm Zhijun (Shenzhen) Pharmaceutical Co. Ltd. - Shenzhen location",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::Novavax => (
                "ORG-100032020",
                "Novavax CZ a.s.",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::GamaleyaResearchInstitute => (
                "Gamaleya-Research-Institute",
                "Gamaleya Research Institute",
                ManufacturerInOMS::No,
                Some(CodeSystemVersion::V1_0),
            ),
            Manufacturer::VectorInstitute => (
                "Vector-Institute",
                "Vector Institute",
                ManufacturerInOMS::No,
                Some(CodeSystemVersion::V1_0),
            ),
            Manufacturer::SinovacBiotech => (
                "Sinovac-Biotech",
                "Sinovac Biotech",
                ManufacturerInOMS::No,
                Some(CodeSystemVersion::V1_0),
            ),
            Manufacturer::BharatBiotech => (
                "Bharat-Biotech",
                "Bharat Biotech",
                ManufacturerInOMS::No,
                Some(CodeSystemVersion::V1_0),
            ),
            Manufacturer::SerumInstituteOfIndia => (
                "ORG-100001981",
                "Serum Institute Of India Private Limited",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::Fiocruz => (
                "Fiocruz",
                "Fiocruz",
                ManufacturerInOMS::No,
                Some(CodeSystemVersion::V1_3),
            ),
            Manufacturer::RPharmCJSC => (
                "ORG-100007893",
                "R-Pharm CJSC",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::Chumakov => (
                "Chumakov-Federal-Scientific-Center",
                "Chumakov Federal Scientific Center for Research and Development of Immune-and-Biological Products",
                ManufacturerInOMS::No,
                Some(CodeSystemVersion::V1_4),
            ),
            Manufacturer::GulfPharmaceutical => (
                "ORG-100023050",
                "Gulf Pharmaceutical Industries",
                ManufacturerInOMS::Yes,
                None,
            ),
            Manufacturer::CIGB => (
                "CIGB",
                "Center for Genetic Engineering and Biotechnology (CIGB)",
                ManufacturerInOMS::No,
                Some(CodeSystemVersion::V1_5),
            ),
            Manufacturer::SinopharmWuhan => (
                "Sinopharm-WIBP",
                "Sinopharm - Wuhan Institute of Biological Products",
                ManufacturerInOMS::No,
                Some(CodeSystemVersion::V1_5),
            ),
            Manufacturer::Medigen => (
                "ORG-100033914",
                "Medigen Vaccine Biologics Corporation",
                ManufacturerInOMS::Yes,
                None,
            ),
        }
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
