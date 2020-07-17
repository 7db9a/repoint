pub mod repoint_file;
use std::process::Command;
use cmd_lib::run_fun;

pub fn init(cmd: String) -> Result<std::process::Output, std::io::Error> {
    //let output = run_fun!("{}", cmd).unwrap();
     Command::new("sh")
        .arg(cmd)
        .output()


    //println!("{}", String::from_utf8_lossy(&output.stdout));
    //println!("results:\n {}", output);
}

pub fn send_opreturn(test: bool) {
    if test {


    }
}

pub fn exists_opreturn(test: bool) {
    if test {
    }
}
