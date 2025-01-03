mod openssl;

use crate::openssl::table::Table;
use crate::openssl::remaining_days::{cmd_output, RemainingDays};
use log::{error, info};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;
use tempfile::NamedTempFile;
use tokio::{spawn};
use crate::openssl::table;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Timestamp (sec, ms, ns, none)
    #[structopt(short = "t", long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,

    /// File (hosts.txt)
    #[structopt(short = "f", long = "file", default_value = "", parse(from_os_str))]
    file: PathBuf,
    /// Domain (google.com)
    domains: Vec<String>,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();

    if !opt.domains.is_empty() {
        from_domains(opt.domains).await;
        std::process::exit(0);
    }

    let path = opt.file.as_path();
    if !path.exists() {
        error!("file {:?} does not exist", path);
        std::process::exit(1);
    }

    if path.is_dir() {
        error!("path {:?} is a directory", path);
        std::process::exit(1);
    }

    from_file(opt.file).await;
}

async fn from_file(path: PathBuf) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut table = Table::new();
    table.add(openssl::table::TableRow::header());

    let mut join_handles = Vec::new();

    for line in reader.lines() {
        let handle = spawn(process_domain_line(line));
        join_handles.push(handle);
    }

    for handle in join_handles {
        let result = handle.await;
        let remaining_days = result.unwrap();

        if add_successful_parsed_remaining_days(&mut table, remaining_days) { continue; }
    }

    //print table
    table.order_by_host().order_by_remaining_days().print();
}

fn add_successful_parsed_remaining_days(table: &mut Table, remaining_days: RemainingDays) -> bool {
    // handle stderr
    if !remaining_days.stderr.is_empty() {
        error!("adding domain {:?} failed: {}", remaining_days.domain, remaining_days.stderr);
        return true;
    }

    table.add(table::TableRow::new(
        remaining_days.domain.clone(),
        remaining_days.organisation.clone(),
        remaining_days.parse()
    ));
    false
}

async fn process_domain_line<Error: std::fmt::Debug>(line: Result<String, Error>) -> RemainingDays {
    let url = line.expect("failed to read line");
    let file = NamedTempFile::new().expect("failed to create temp file");
    let path = file.path();

    info!("adding domain {:?} to {:?}", url, path);
    let organisation = write_tmp_get_organisation(&url, path);
    let remaining_days =
        cmd_output(path.display(), organisation.clone(), url.clone());

    file.close().unwrap();

    remaining_days
}

async fn from_domains(domains: Vec<String>) {
    let mut table = Table::new();
    table.add(openssl::table::TableRow::header());

    for domain in domains {
        let line: Result<String, Error> = Ok(domain);
        let result = process_domain_line(line).await;

        _ = add_successful_parsed_remaining_days(&mut table, result)
    }

    table.print();
}

fn write_tmp_get_organisation(host: &String, pem_file: &Path) -> String {
    // get certificate from url
    let pem = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "openssl s_client -connect {host}:443 </dev/null | openssl x509 -outform PEM > {file}",
            host = host,
            file = pem_file.display()
        ))
        .output()
        .expect("failed to execute process");

    let mut organisation = String::new();

    // handle stderr
    if !pem.stderr.is_empty() {
        String::from_utf8_lossy(&pem.stderr)
            .split("\n")
            .filter(|&x| x.contains("depth=1"))
            .for_each(|x| organisation = get_organisation(x));
    }
    organisation
}

// i example: depth=1 C = US, O = Let's Encrypt, CN = R3
// o example: Let's Encrypt
fn get_organisation(i: &str) -> String {
    //get organisation
    let mut o = i.split("O = ").collect::<Vec<&str>>();
    let mut s = " ="; //splitter
    let mut l = 4; //length after split

    if o.len() == 1 {
        o = i.split("O=").collect::<Vec<&str>>();
        s = ",";
        l = 0;
    }
    let o = o[1].split(s).collect::<Vec<&str>>();
    //remove last 4 chars
    let o = &o[0][..o[0].len() - l];
    //remove \" from string
    let o = o.replace("\"", "");
    o.to_string()
}
