use std::ffi::OsString;
use std::fs;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MasterData {
    master: SequoiaKeyGenerate,
}

#[derive(Debug, Serialize, Deserialize)]
struct SequoiaKeyGenerate {
    #[serde(default = "default_cipher_suite")]
    cipher_suite: CipherSuite,
    #[serde(default = "default_authenticate")]
    authenticate: bool,
    #[serde(default = "default_encrypt")]
    encrypt: CanEncrypt,
    #[serde(default = "default_sign")]
    sign: bool,
    #[serde(default = "default_expiration")]
    expiration: String,
    #[serde(default = "default_output")]
    output: String,
    #[serde(default = "default_rev_cert")]
    rev_cert: String,
    userids: Vec<UserId>,
    #[serde(default = "default_with_password")]
    with_password: bool,
}

fn default_authenticate() -> bool {
    true
}

fn default_sign() -> bool {
    true
}

fn default_expiration() -> String {
    "3y".to_string()
}

fn default_output() -> String {
    "key.pgp".to_string()
}

fn default_rev_cert() -> String {
    "key.pgp.rev".to_string()
}

fn default_cipher_suite() -> CipherSuite {
    CipherSuite::Cv25519
}

fn default_with_password() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
enum CipherSuite {
    Cv25519,
    RSA3k,
    P256,
    P384,
    P521,
    RSA2k,
    RSA4k,
}

impl std::fmt::Display for CipherSuite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

fn default_encrypt() -> CanEncrypt {
    CanEncrypt {
        can: true,
        purpose: CanEncryptPurpose::Universal,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CanEncrypt {
    can: bool,
    purpose: CanEncryptPurpose,
}

#[derive(Debug, Serialize, Deserialize)]
// need lower case to match the cli interface
#[serde(rename_all = "lowercase")]
enum CanEncryptPurpose {
    Universal,
    Transport,
    Storage,
}

impl std::fmt::Display for CanEncryptPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserId {
    name: String,
    comment: Option<String>,
    email: String,
}

fn parse_master(data: &MasterData) -> Vec<String> {
    let authenticate: String = if data.master.authenticate {
        "--can-authenticate".to_string()
    } else {
        "--cannot-authenticate".to_string()
    };

    let encrypt: String = if data.master.encrypt.can {
        format!("--can-encrypt={}", data.master.encrypt.purpose)
    } else {
        "--cannot-encrypt".to_string()
    };

    let sign: String = if data.master.sign {
        "--can-sign".to_string()
    } else {
        "--cannot-sign".to_string()
    };

    let with_password: String = if data.master.with_password {
        "--with-password".to_string()
    } else {
        "".to_string()
    };

    let userids: Vec<String> = data
        .master
        .userids
        .iter()
        .map(|elem| match &elem.comment {
            Some(comment) => format!("--userid={} ({}) <{}>", elem.name, comment, elem.email),
            None => format!("--userid={} <{}>", elem.name, elem.email),
        })
        .collect();

    vec![
        vec![
            format!("--cipher-suite={}", data.master.cipher_suite),
            authenticate,
            encrypt,
            sign,
            format!("--expiration={}", data.master.expiration),
            format!("--output={}", data.master.output),
            format!("--rev-cert={}", data.master.rev_cert),
            with_password,
        ],
        userids,
    ]
    .concat()
    .into_iter()
    .filter(|elem| elem != "")
    .collect()
}

pub fn generate(path: OsString) -> () {
    let contents = fs::read_to_string(path).unwrap();
    let data: MasterData = toml::from_str(&contents).unwrap();
    println!("{:?}", &data);

    let result: Vec<String> = parse_master(&data);
    println!("{:?}", result);

    let output = Command::new("sq")
        .args(["key", "generate"])
        .args(result)
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

    // let toml = toml::to_string(&data).unwrap();
}
