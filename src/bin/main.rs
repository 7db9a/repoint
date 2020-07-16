#[macro_use]
extern crate seahorse;
extern crate repoint;
extern crate dirs;

use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use seahorse::{App, Command, Context, Flag, FlagType};
use repoint::repoint_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [name]")
        .action(default_action)
        .flag(Flag::new("bye", "cli [name] --bye(-b)", FlagType::Bool).alias("b"))
        .flag(Flag::new("age", "cli [name] --age(-a)", FlagType::Int).alias("a"))
        .command(calc_command())
        .command(create_account())
        .command(init())
        .command(send());

    app.run(args);
}

fn default_action(c: &Context) {
    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }

    if let Some(age) = c.int_flag("age") {
        println!("{:?} is {} years old", c.args, age);
    }
}

fn init() -> Command {
    Command::new()
        .name("init")
        .usage("cli [dir]")
        .action(init_action)
}

fn create_account() -> Command {
    Command::new()
        .name("create-account")
        .usage("cli [name] [pub-addr]")
        .action(create_account_action)
}

fn send() -> Command {
    Command::new()
        .name("send")
        .usage("cli [test]")
        .action(send_action)
}

fn send_action(c: &Context) {
    let mut args = c.args.iter();
    let mut test = "";
    let arg_count = args.clone().count();
    match arg_count {
       1 => {
           test = args.next().unwrap();
       },
       _ => ()
    };

    // Get file hash of account and repo tomls.
    // If no match, use toml fields to write opreturn...
    // and then save appropriate toml hashes to /tmp.

    let test: bool = (test == "test");
    if test {
        let mut account_pathbuf = dirs::home_dir().unwrap();
        account_pathbuf.push(".repoint");
        account_pathbuf.push("account.toml");
        let account_path_str = account_pathbuf.as_path().to_str().unwrap();
        repoint_file::hash_file("repoint.toml").expect("fail to write hash file");
        repoint_file::hash_file(account_path_str).expect("fail to write account hash file");
    }
    println!("test: {}", test)
}

fn create_account_action(c: &Context) {
    let mut args = c.args.iter();
    let mut name = "";
    let mut pub_addr = "";
    let mut xpriv = "";
    let arg_count = args.clone().count();
    match arg_count {
       3 => {
           name = args.next().unwrap();
           pub_addr = args.next().unwrap();
           xpriv = args.next().unwrap();

           println!("{}\n{}\n{}", name, pub_addr, xpriv)
       },
       _ => ()
    };

    let mut pathbuf = dirs::home_dir().unwrap();
    pathbuf.push(".repoint");
    pathbuf.push("account.toml");

    println!("{:?}", pathbuf);
    //File should already exist from install.
    File::create(&pathbuf).expect("Failed to create file.");

    let doc = repoint_file::init_account(
        pathbuf.as_path().to_str().unwrap(),
        name,
        pub_addr,
        xpriv
    ).unwrap();

    //repoint_file::write(doc.clone(), repoint_path.as_ref()).expect("failed to write toml to disk");
    repoint_file::write(doc.clone(),pathbuf.as_path().to_str().unwrap()).expect("failed to write toml to disk");
    //let repoint_res = repoint_file::repoint(&doc, Some("example"), "name");
}

fn init_action(c: &Context) {
    let mut args = c.args.iter();
    let mut path = "";
    let mut pathbuf: PathBuf;
    let arg_count = args.clone().count();
    match arg_count {
       1 => {
           path = args.next().unwrap();
       },
       _ => ()
    };

    pathbuf = if path == "" {
        PathBuf::from("repoint.toml")
    } else {
        let mut p = PathBuf::from(path);
        p.push("repoint.toml");
        p
    };

    println!("{:?}", pathbuf);
    File::create(&pathbuf).expect("Failed to create file.");

    let doc = repoint_file::init(
        pathbuf.as_path().to_str().unwrap(),
        env!("CARGO_PKG_VERSION"),
    ).unwrap();
    repoint_file::write(doc.clone(),pathbuf.as_path().to_str().unwrap()).expect("failed to write toml to disk");
}

fn calc_action(c: &Context) {
    match c.string_flag("operator") {
        Some(op) => {
            let sum: i32 = match &*op {
                "add" => c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum(),
                "sub" => c.args.iter().map(|n| n.parse::<i32>().unwrap() * -1).sum(),
                _ => panic!("undefined operator..."),
            };

            println!("{}", sum);
        }
        None => panic!(),
    }
}

fn calc_command() -> Command {
    Command::new()
        .name("calc")
        .usage("cli calc [nums...]")
        .action(calc_action)
        .flag(
            Flag::new(
                "operator",
                "cli calc [nums...] --operator(-op) [add | sub]",
                FlagType::String,
            )
            .alias("op"),
        )
}
