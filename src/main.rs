mod openssl;

use std::io::BufRead;
use std::path::Path;
use tempfile::NamedTempFile;

fn main() {
    if std::env::args().len() == 1 {
        from_stdin();
    } else {
        from_pos_arg();
    }
}

fn from_stdin() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut table = openssl::table::Table::new();
    table.add(openssl::table::TableRow::header());

    while let Some(line) = lines.next() {
        let url = line.expect("failed to read line");
        let file = NamedTempFile::new().expect("failed to create tempfile");
        let path = file.path();

        let organisation = write_tmp_get_organisation(&url, path);
        let remaining_days = openssl::remaining_days_cmd_output(path.display(), organisation.clone(), url.clone());

        // handle stderr
        if !remaining_days.stderr.is_empty() {
            continue;
        }

        table.add(openssl::table::TableRow::new(remaining_days.host.clone(), remaining_days.organisation.clone(), remaining_days.parse()));

        //remove file
        file.close().unwrap();
    }

    //print table
    table
        .order_by_host()
        .order_by_remaining_days()
        .print();
}

fn from_pos_arg() {
    //url from args
    let url = std::env::args().nth(1).expect("no url given");

    let file = NamedTempFile::new().expect("failed to create tempfile");
    let path = file.path();

    let organisation = write_tmp_get_organisation(&url, path);
    let remaining_days = openssl::remaining_days_cmd_output(path.display(), organisation.clone(), url.clone());

    // handle stderr
    if !remaining_days.stderr.is_empty() {
        let stderr = remaining_days.stderr;
        println!("stderr: {}", stderr);
        return;
    }

    //print table
    openssl::table::Table::new()
        .add(openssl::table::TableRow::header())
        .add(openssl::table::TableRow::new(remaining_days.host.clone(), remaining_days.organisation.clone(), remaining_days.parse()))
        .print();

    //remove file
    file.close().unwrap();
}

fn write_tmp_get_organisation(host: &String, pem_file: &Path) -> String {
// get certificate from url
    let pem = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("openssl s_client -connect {host}:443 </dev/null | openssl x509 -outform PEM > {file}", host=host, file= pem_file.display()))
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
    let i = i.split("O = ").collect::<Vec<&str>>();
    let i = i[1].split(" =").collect::<Vec<&str>>();
    //remove last 4 chars
    let i = &i[0][..i[0].len() - 4];
    //remove \" from string
    let i = i.replace("\"", "");
    i.to_string()
}