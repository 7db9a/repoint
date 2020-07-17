pub mod repoint_file;
use std::process::Command;

pub fn init(cmd: String, args: Vec<String>)/* -> Result<std::process::Output, std::io::Error>*/ {
    let output_res = Command::new(cmd)
        .args(args)
        .output();

    let output = output_res.unwrap();

    println!("results:\n {}", String::from_utf8_lossy(&output.stdout));
}

pub fn send_opreturn(test: bool) {
    if test {


    }
}

pub fn exists_opreturn(test: bool) {
    if test {
    }
}
