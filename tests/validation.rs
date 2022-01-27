use chrono::prelude::*;
use greenpass::{CertInfo, GreenPass, HealthCert, Recovery, Signature, Test, TestName, Vaccine};

// Quick and dirty validation tests

const RECOVERY_SAMPLE_PAYLOAD : &str = "HC1:NCFOXN%TS3DH3ZSUZK+.V0ETD%65NL-AH-XIIOOA+IWBCNQ5GJL-XVRJAKD93B4:ZH6I1$4JF 2K%5+G9F.PNF67J6UW6LEQV46PK9E:00$4*2DN43U*0CEBQ/GXQFY73CIBC:GVEBBIBBL7BIB4UNAWNJKBOJJ5PNT53/FJ8FN96B2M3-6BHI7UG55:44$28A9H0D3ZCL4JMYAZ+S-A5$XKX6T2YC 35H/ITX8GL2TK96L6SR9MU9DV5 R1BPIZKH03RW63LD3LS4JYK9EFH78$ZJ*DJ3Q4+Y5V$K2:6.77/Z6KZ5LD6E6P 9SH87/YQJ/RL35+Y5P Q*8D$JD IBCLCK5MJMS68H36DH.K:Z28AL**I3DN3F7MHFEVV%*4HBTSCNT 4C%C47TO*47*KB*KYQT3LT+*4.$S6ZC0JB%JBK:TBSH6FN-G6N%VPJ13M9K9J8*TPB12-VRSQ8XMA6EBNVC28:APQ71K:ON4K25A:I9/-HUZGOZFB04YME%6DAUNO2B+TQG/2I0A4Z5NDROBVKS0J$28XG";
const PCR_TEST_SAMPLE_PAYLOAD : &str = "HC1:NCFOXN%TS3DH3ZSUZK+.V0ETD%65NL-AHBP1IOOA+IS7C$068WAD1W7:BAT4V22F/8X*G3M9BM9Z0BFU2P4JY73JC3KD34LT7A3-4386BXSJ$IJGX8.+IIYC6Q0ZIJPKJ+LJ%2TK/IS/SR4DKJ5QWCB4DN57E-4LXKV85HZ0T+0K%I17JLXKB6J57TJK57ALT$I/+GDG6Z$U*C2OQ1:PIGEGEV4*2DN43U*0CEBQ/GXQFY73CIBC:GUC7QHBN83GG3NQN%976FNXEB.FJN83HB3EG3CAJTA3ANBXEBGM5J%44$28A9H0D3ZCL4JMYAZ+S-A5$XKX6T2YC 35H/ITX8GL2TK96L6SR9MU9DV5 R1JNI:E4I+C7*4M:KCY07LPMIH-O9XZQSH9R$FXQGDVBK*RZP3:*DG1W7SGT$7S%RMSG2UQYI96GGLXK6*K$X4FUTD14//EF.712U0$89NT2V457U8+9W2KQ-7LF9-DF07U$B97JJ1D7WKP/HLIJLRKF1MFHJP7NVDEBU1J6+2FBKBSHNAIFVV%KN$2W5+IKDP6SFPQC16LITI/-7P E:ZSYMJS$5-BPDMFAMBJ7TN7FMRHL:19XI9X5ZL36%OGODHIFE8SHLH0ZLG$DDZG$DR-00*%E%4";
const ANTIGEN_TEST_SAMPLE_PAYLOAD : &str = "HC1:NCFOXN%TS3DH3ZSUZK+.V0ETD%65NL-AH+VEIOOA+ILACMB9GJL. V/1KKD93B4:ZH6I1$4JM:IP1MUF5Z$5NF67J6QW6%PQSE6-96XNM6-6SF6IRH*PPKS9.Q6%%6%E5%NPC71RF6+17S%MBX6PF51$5DB97-59$PO1BD/9BL5$W5151-V9NVP.SA0T932QHG3JZILDB523G*S2U2V8TQEDK8CD/SYJC.0EPSTBVC9ND2SSIYDNYDF*S8/D7BCU5TL0D BD$8D8ND1UT*WD.XI5TBW1UYIJGDBGIASJLA8KOHSLOJJPAOJKCGSFC17KPDS9+E93ZM$96PZ6+Q6X46KE0.43RA3/43KD3F23/9TL4T1C9 UPVD5BT17$1MV15K1DR1FIEC2F5+1T+UC2FSH9 UP+/UEOJDKBO.AI9BVYTOCFOPS788O5L9Y4KCT:WC.L76V0VSNRCN /KU%CW.4WV2L4L$XKV7J$%25I3IC33835AL5:4A93QF08T1+G3N313SGXVA.-ND3JH/F.*OXNENHN%C36$EQU3*7LT AOH1N6OJZF3RTF7LS8BJFTGAH%BO3GA7QH*VTP7P.1GA G-/N5Y7T UDEUGBDSHNJ50/BHS2";
const VACCINE_SAMPLE_PAYLOAD : &str = "HC1:NCFOXN%TS3DH3ZSUZK+.V0ETD%65NL-AH-R6IOOA+IZJS:A8GJL*XV%O3+QI6M8SA3/-2E%5VR5VVB9ZILAPIZI.EJJ14B2MZ8DC8COVD9VC/MJK.A+ C/8DXED%JCC8C62KXJAUYCOS2QW6%PQRZMPK9I+0MCIKYJGCC:H3J1D1I3-*TW CXBDW33+ CD8CQ8C0EC%*TGHD1KT0NDPST7KDQHDN8TSVD2NDB*S6ECX%LBZI+PB/VSQOL9DLKWCZ3EBKD8IIGDB0D48UJ06J9UBSVAXCIF4LEIIPBJ7OICWK%5BBS22T9UF5LDCPF5RBQ746B46JZ0V-OEA7IB6$C94JB2E9Z3E8AE-QD+PB.QCD-H/8O3BEQ8L9VN.6A4JBLHLM7A$JD IBCLCK5MJMS68H36DH.K:Z28AL**I3DN3F7MHFEVV%*4HBTSCNT 4C%C47TO*47*KB*KYQT3LT+*4.$S6ZC0JB%JB% NHTC:OS6K7C*M9$4HOUJJ8BZR+AB9MRX.JK/DXQC*%ESNO3VVS$EO:9-14:6VEVUOEKF4PSTT:L9HRJKP4H6RAOV%Q7RB1P 9OG2O%LAULC97*JLUEH";

#[test]
fn parse_recovery() {
    let rec_hc = HealthCert {
        some_issuer: Some("AT".into()),
        created: Utc.ymd(2021, 07, 02).and_hms(21, 24, 42),
        expires: Utc.ymd(2022, 07, 02).and_hms(21, 24, 42),
        passes: vec![GreenPass {
            date_of_birth: "1998-02-26".into(),
            surname: "Musterfrau-Gößinger".into(),
            givenname: "Gabriele".into(),
            std_surname: "MUSTERFRAU<GOESSINGER".into(),
            std_givenname: "GABRIELE".into(),
            ver: "1.2.1".into(),
            entries: vec![CertInfo::Recovery(Recovery {
                cert_id: "URN:UVCI:01:AT:858CC18CFCF5965EF82F60E493349AA5#K".into(),
                country: "AT".into(),
                diagnosed: NaiveDate::from_ymd(2021, 02, 20),
                disease: "840539006".into(),
                issuer: "Ministry of Health, Austria".into(),
                valid_from: NaiveDate::from_ymd(2021, 04, 04),
                valid_until: NaiveDate::from_ymd(2021, 10, 04),
            })],
        }],
        signature: Signature {
            kid: vec![217, 25, 55, 95, 193, 231, 182, 178],
            algorithm: -7i128,
            signature: vec![
                209, 100, 158, 183, 159, 208, 106, 164, 97, 164, 140, 184, 100, 84, 73, 247, 52,
                136, 185, 139, 78, 154, 207, 178, 248, 215, 95, 172, 83, 202, 1, 39, 111, 254, 39,
                212, 136, 60, 54, 144, 171, 14, 55, 241, 213, 9, 232, 132, 86, 223, 157, 37, 146,
                235, 232, 94, 228, 57, 56, 11, 175, 15, 141, 229,
            ],
        },
    };

    assert_eq!(greenpass::parse(RECOVERY_SAMPLE_PAYLOAD).unwrap(), rec_hc);
}

#[test]
fn parse_test_pcr() {
    let pcr_hc = HealthCert {
        some_issuer: Some("AT".into()),
        created: Utc.ymd(2021, 07, 02).and_hms(20, 54, 37),
        expires: Utc.ymd(2022, 07, 02).and_hms(20, 54, 37),
        passes: vec![GreenPass {
            date_of_birth: "1998-02-26".into(),
            surname: "Musterfrau-Gößinger".into(),
            givenname: "Gabriele".into(),
            std_surname: "MUSTERFRAU<GOESSINGER".into(),
            std_givenname: "GABRIELE".into(),
            ver: "1.2.1".into(),
            entries: vec![CertInfo::Test(Test {
                cert_id: "URN:UVCI:01:AT:B5921A35D6A0D696421B3E2462178297#I".into(),
                collect_ts: FixedOffset::east(0).ymd(2021, 02, 20).and_hms(04, 34, 56),
                country: "AT".into(),
                disease: "840539006".into(),
                issuer: "Ministry of Health, Austria".into(),
                name: TestName::NAAT {
                    name: "Roche LightCycler qPCR".into(),
                },
                result: "260415000".into(),
                test_type: "LP6464-4".into(),
                testing_centre: "Testing center Vienna 1".into(),
            })],
        }],
        signature: Signature {
            kid: vec![217, 25, 55, 95, 193, 231, 182, 178],
            algorithm: -7i128,
            signature: vec![
                30, 197, 243, 131, 131, 7, 254, 58, 77, 84, 188, 63, 28, 198, 211, 148, 166, 29,
                91, 207, 181, 181, 1, 249, 124, 210, 68, 36, 199, 6, 140, 98, 133, 126, 159, 7,
                138, 79, 21, 151, 82, 210, 97, 150, 104, 182, 12, 24, 152, 214, 136, 110, 23, 75,
                31, 33, 184, 58, 21, 60, 152, 84, 92, 62,
            ],
        },
    };

    assert_eq!(greenpass::parse(PCR_TEST_SAMPLE_PAYLOAD).unwrap(), pcr_hc);
}

#[test]
fn parse_test_antigen() {
    let rat_hc = HealthCert {
        some_issuer: Some("AT".into()),
        created: Utc.ymd(2021, 07, 02).and_hms(20, 55, 37),
        expires: Utc.ymd(2022, 07, 02).and_hms(20, 55, 37),
        passes: vec![GreenPass {
            date_of_birth: "1998-02-26".into(),
            surname: "Musterfrau-Gößinger".into(),
            givenname: "Gabriele".into(),
            std_surname: "MUSTERFRAU<GOESSINGER".into(),
            std_givenname: "GABRIELE".into(),
            ver: "1.2.1".into(),
            entries: vec![CertInfo::Test(Test {
                cert_id: "URN:UVCI:01:AT:71EE2559DE38C6BF7304FB65A1A451EC#3".into(),
                collect_ts: FixedOffset::east(0).ymd(2021, 02, 20).and_hms(12, 34, 56),
                country: "AT".into(),
                disease: "840539006".into(),
                issuer: "Ministry of Health, Austria".into(),
                name: TestName::RAT {
                    device_id: "1232".into(),
                },
                result: "260415000".into(),
                test_type: "LP217198-3".into(),
                testing_centre: "Testing center Vienna 1".into(),
            })],
        }],
        signature: Signature {
            kid: vec![217, 25, 55, 95, 193, 231, 182, 178],
            algorithm: -7i128,
            signature: vec![
                193, 252, 36, 214, 235, 99, 31, 183, 220, 20, 71, 57, 149, 206, 192, 104, 143, 126,
                114, 136, 177, 151, 245, 38, 90, 83, 247, 183, 207, 113, 171, 228, 203, 152, 59,
                84, 47, 48, 23, 27, 237, 140, 37, 142, 90, 18, 143, 254, 10, 87, 220, 200, 45, 222,
                229, 140, 74, 159, 247, 188, 40, 129, 44, 209,
            ],
        },
    };

    assert_eq!(
        greenpass::parse(ANTIGEN_TEST_SAMPLE_PAYLOAD).unwrap(),
        rat_hc
    );
}

#[test]
fn parse_vaccination() {
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
        signature: Signature {
            kid: vec![217, 25, 55, 95, 193, 231, 182, 178],
            algorithm: -7i128,
            signature: vec![
                69, 146, 203, 61, 178, 10, 23, 233, 18, 29, 183, 22, 5, 63, 6, 141, 66, 205, 176,
                236, 63, 106, 217, 0, 79, 215, 218, 174, 181, 202, 83, 194, 246, 241, 147, 249, 82,
                113, 129, 96, 123, 124, 210, 64, 179, 128, 25, 64, 173, 6, 78, 72, 231, 20, 86, 77,
                99, 148, 85, 166, 136, 245, 61, 119,
            ],
        },
    };

    assert_eq!(greenpass::parse(VACCINE_SAMPLE_PAYLOAD).unwrap(), vac_hc);
}
