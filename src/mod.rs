pub mod repoint_file;
use std::process::Command;
use std::path::PathBuf;
use cmd_lib::run_fun;

pub fn init_sign(cmd: String) -> Result<std::process::Output, std::io::Error> {
    sign(
        cmd,
        String::from("0x7202"),
        String::from("c2859d6ace2072662e22bd2e197c790fffca56ac6030800139800a3d1f87866f")
    )
}

pub fn create_account_sign(cmd: String, account_name: String) -> Result<std::process::Output, std::io::Error> {
    sign(
        cmd,
        String::from("0x7203"),
        account_name
    )
}

pub fn create_repo_sign(cmd: String, repo_name: String) -> Result<std::process::Output, std::io::Error> {
    sign(
        cmd,
        String::from("0x7206"),
        repo_name
    )
}

pub fn add_url_sign(cmd: String, url: String) -> Result<std::process::Output, std::io::Error> {
    sign(
        cmd,
        String::from("0x7209"),
        url
    )
}

pub fn sign(cmd: String, opcode: String, msg: String) -> Result<std::process::Output, std::io::Error> {
     Command::new("sh")
        .arg(cmd)
        .arg(
            get_privkey()// get from account.toml
        )
        .arg(opcode)  // hard-code
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
    let privkey = toml_doc["account"]["xpriv"]
        .as_str()
        .expect("failed to parse privkey from account.toml")
        .to_string();

    assert_eq!(privkey, "5JZ4RXH4MoXpaUQMcJHo8DxhZtkf5U5VnYd9zZH8BRKZuAbxZEw");

    privkey
}

pub fn send_opreturn(test: bool) {
    if test {


    }
}

pub fn exists_opreturn(test: bool) {
    if test {
    }
}

// test get_privkey
#[cfg(test)]
mod account_toml {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_get_privkey() {

         assert_eq!(
             get_privkey(),
             "5JZ4RXH4MoXpaUQMcJHo8DxhZtkf5U5VnYd9zZH8BRKZuAbxZEw")
    }

    #[test]
    fn test_init_sign() {
         let output = init_sign(
             "opreturn.sh".to_string(),
         ).expect("opreturn shell call failed");

         let stdout = String::from_utf8_lossy(&output.stdout);

         assert_eq!(
             stdout,
             "010000000001000000000000000046006a027202406332383539643661636532303732363632653232626432653139376337393066666663613536616336303330383030313339383030613364316638373836366600000000\n"
        )
    }

    #[test]
    fn test_create_account_sign() {
         let output = create_account_sign(
             "opreturn.sh".to_string(),
             "7db9a".to_string(),
         ).expect("opreturn shell call failed");

         let stdout = String::from_utf8_lossy(&output.stdout);

         assert_eq!(
             stdout,
             "01000000000100000000000000000b006a02720305376462396100000000\n",
        )
    }

    #[test]
    fn test_create_repo_sign() {
         let output = create_repo_sign(
             "opreturn.sh".to_string(),
             "repoint".to_string(),
         ).expect("opreturn shell call failed");

         let stdout = String::from_utf8_lossy(&output.stdout);

         assert_eq!(
             stdout,
             "01000000000100000000000000000d006a027206077265706f696e7400000000\n"
        )
    }

    #[test]
    fn test_add_url_sign() {
         let output = create_repo_sign(
             "opreturn.sh".to_string(),
             "https://github.com/7db9a/repoint".to_string(),
         ).expect("opreturn shell call failed");

         let stdout = String::from_utf8_lossy(&output.stdout);

         assert_eq!(
             stdout,
             "010000000001000000000000000026006a0272062068747470733a2f2f6769746875622e636f6d2f37646239612f7265706f696e7400000000\n"
        )
    }
}
