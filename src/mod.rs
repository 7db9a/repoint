pub mod repoint_file;
use std::process::Command;
use std::path::PathBuf;
use cmd_lib::run_fun;

pub fn init(cmd: String, msg: String) -> Result<std::process::Output, std::io::Error> {
     Command::new("sh")
        .arg(cmd)
        //.arg(privkey) // get from account.toml
        .arg("0x7202")  // hard-code
        .arg(msg)     // hard-code (app-ID)
        //.arg(fee)     // cli arg
        //.arg(rpc_url) // get from env var
        //.arg(safe)
        .output()
}

pub fn get_privkey() -> String {
     let mut pathbuf = dirs::home_dir().unwrap();
     pathbuf.push(".repoint");
     pathbuf.push("account.toml");
     let path = pathbuf.to_str().expect("fail to convert pathbuf into string");

    let toml_doc = repoint_file::open(path).expect("failed to open toml file");
    toml_doc["acount"]["xpriv"]
        .as_str()
        .expect("fail to parse toml keys")
        .to_string()
}

pub fn send_opreturn(test: bool) {
    if test {


    }
}

pub fn exists_opreturn(test: bool) {
    if test {
    }
}
