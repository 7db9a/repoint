#[macro_use]
extern crate seahorse;
extern crate repoint;

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
        .command(init());

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

fn create_account_action(c: &Context) {
    let mut args = c.args.iter();
    let mut name = "";
    let mut pub_addr = "";
    let arg_count = args.clone().count();
    match arg_count {
       2 => {
           name = args.next().unwrap();
           pub_addr = args.next().unwrap();

           println!("{}\n{}", name, pub_addr)
       },
       _ => ()
    };
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
