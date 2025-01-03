use std::path;

pub struct RemainingDays {
    pub stdout: String,
    pub stderr: String,
    pub organisation: String,
    pub domain: String,
}

impl RemainingDays {
    pub fn new(stdout: String, stderr: String, organisation: String, domain: String) -> RemainingDays {
        RemainingDays { stdout, stderr, organisation, domain }
    }

    pub fn parse(&self) -> i64 {
        let output = self.stdout.trim();
        let output = output.split("=").collect::<Vec<&str>>();

        let datetime = output[1].split(" ").collect::<Vec<&str>>();
        let datetime = datetime.iter().filter(|&x| !x.is_empty()).collect::<Vec<&&str>>();

        let year = datetime[3];
        let month = datetime[0];
        let day = datetime[1];

        let today = chrono::Local::now();
        let remaining = chrono::NaiveDate::parse_from_str(&format!("{}{}{}", year, month, day), "%Y%b%d").unwrap() - chrono::NaiveDate::parse_from_str(&today.format("%Y%m%d").to_string(), "%Y%m%d").unwrap();
        remaining.num_days()
    }
}

///
///
/// # Arguments
///
/// * `p`: path to cert file, which is used by the openssl command
/// * `organisation`: cert organisation e.g. "Let's Encrypt"
/// * `domain`: domain to check "domain.tld"
///
/// returns: RemainingDays
///
/// # Examples
///
/// ```
///
/// ```
pub fn cmd_output(p: path::Display, organisation: String, domain: String) -> RemainingDays {
    let p = p.to_string();
    let output = std::process::Command::new("openssl")
        .args(&["x509", "-in", &p, "-noout", "-enddate"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    RemainingDays::new(stdout, stderr, organisation, domain)
}