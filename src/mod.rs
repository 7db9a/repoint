pub mod repoint_file;
use std::process::Command;
use cmd_lib::run_fun;

pub fn init(cmd: String)/* -> Result<std::process::Output, std::io::Error>*/ {
    let output = run_fun!("{}", cmd).unwrap();
    //let output_res = Command::new("bash")
    //    .arg("opreturn-script.sh")
    //    .output();

    //let output = output_res.unwrap();

    //println!("results:\n {}", String::from_utf8_lossy(&output.stdout));
    println!("results:\n {}", output);
}

pub fn send_opreturn(test: bool) {
    if test {


    }
}

pub fn exists_opreturn(test: bool) {
    if test {
    }
}
