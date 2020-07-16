/*
This module manages the specifics of the repoint file.
*/
extern crate toml;
extern crate toml_edit;
pub use toml_edit::{value, Document};
use easy_hasher::easy_hasher::sha256;

use fixture;
use err::Error;
pub use err::{ErrorKind, RepointFileError};

use std::fs::File;
pub use std::fs::read_to_string;
use std::io::Write; // Not sure why, but file.write_all doesn't work without it. Not explicit to me.
use std::path::PathBuf;
use std::fs::{create_dir_all, OpenOptions};
use std::io::prelude::*;

/// Reveals the state of the repoint file.
#[derive(Clone, Debug, PartialEq)]
pub enum RepointFileState {
    NonExistant,
    Valid,
    Invalid,
}

/// Creates a repoint file with basic info.
pub fn init<T: AsRef<str>>(path: T, version: T) -> Result<Document, RepointFileError> {
    let toml = format!(
        r#"['repository']
version = "{}""#,
        version.as_ref(),
    );

    let doc = toml.parse::<Document>()?;

    Ok(doc)
}

/// Creates a repoint file with basic info.
pub fn init_account<T: AsRef<str>>(path: T, name: T, pubaddr: T, xpriv: T) -> Result<Document, RepointFileError> {
    let toml = format!(
        r#"['account']
name = "{}"
pubaddr = "{}"
xpriv = "{}""#,
        name.as_ref(),
        pubaddr.as_ref(),
        xpriv.as_ref(),
    );

    let doc = toml.parse::<Document>()?;

    Ok(doc)
}

/// Open a repoint file.
pub fn open<T: AsRef<str>>(path: T) -> Result<Document, RepointFileError> {
    let data = read_to_string(path.as_ref())?;
    let doc = data.parse::<Document>()?;

    Ok(doc)
}

/// Write to a repoint file.
pub fn write<T: AsRef<str>>(toml_doc: Document, path: T) -> Result<(), RepointFileError> {
    let toml_string = toml_doc.to_string();
    let mut file = File::create(path.as_ref())?;
    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

/// Valid if the version field can be read. Should rename pass
/// toml value into method, that other fields can be validated.
pub fn is_valid(doc: &Document) -> RepointFileState {
    let mut valid: RepointFileState;
    let version = entry_exists(&doc, "repository", Some("version"));

    if version {
        valid = RepointFileState::Valid;
    } else {
        valid = RepointFileState::Invalid;
    }

    valid
}

///! Retrieve field data from a repoint file. For example, if the file name is provided, it will attempt to retrieve the field `repoint` nested in the `README.md` entry.
///!  ```ignore
///!  [README.md]
///!  repoint = "The README."
///!  ```
///! If no file name is given, it will retrieve all the nested value in the key and not necessarily a specific field.
pub fn repoint<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
) -> Result<String, RepointFileError> {
    if file_name.is_some() {
        if let Some(data) = doc[file_name.unwrap().as_ref()][key.as_ref()].as_str() {
            Ok(data.to_string())
        } else {
            let err = Error::new(
                "Invalid nested entry in repoint file",
                ErrorKind::InvalidKey,
            );
            Err(RepointFileError::from(err))
        }
    } else if let Some(data) = doc[key.as_ref()].as_str() {
        Ok(data.to_string())
    } else {
        let err = Error::new("Invalid entry in repoint file", ErrorKind::InvalidKey);
        Err(RepointFileError::from(err))
    }
}

/// A crude way to find if an entry exits. Doesn't work for nested etnries.
/// `path` paramaters is the path to the repoint file.
pub fn exists<T: AsRef<str>>(path: T, name: T) -> bool {
    let doc = open(path.as_ref()).unwrap();
    repoint(&doc, Some(name.as_ref()), "name").is_ok()
}

/// See if an entry exists, with an optional nested key.
/// `path` paramater is the path to the repoint file.
pub fn entry_exists<T: AsRef<str>>(doc: &Document, key: T, key_nested: Option<T>) -> bool {
    if let Some(_key_nested) = key_nested {
        if let Some(table) = doc[key.as_ref()].as_table() {
            table.contains_key(_key_nested.as_ref())
        } else {
            false
        }
    } else {
        let table = doc.as_table();
        table.contains_key(key.as_ref())
    }
}

pub fn insert_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
    repoint: T,
) -> Result<Document, RepointFileError> {
    let status = is_valid(&doc);
    if status == RepointFileState::Valid {
        insert_entry_same_doc(&doc, file_name, key, repoint)
    } else if status == RepointFileState::NonExistant && file_name.is_some() {
        insert_entry_new_doc(&doc, file_name.unwrap(), key, repoint)
    } else {
        // Invalid
        let err = Error::new("invalid repoint file", ErrorKind::InvalidFile);
        Err(RepointFileError::from(err))
    }
}

fn insert_entry_new_doc<T: AsRef<str>>(
    doc: &Document,
    file_name: T,
    key: T,
    repoint: T,
) -> Result<Document, RepointFileError> {
    let mut toml_add: String;
    let toml = doc.to_string();
    if key.as_ref() == "repository" {
        toml_add = format!(
            r#"
['{}']
repoint = "{}""#,
            file_name.as_ref(),
            repoint.as_ref()
        );
    } else {
        toml_add = format!("['{}']", file_name.as_ref());
    }

    let toml = toml + &toml_add;

    Ok(toml.parse::<Document>()?)
}

fn insert_entry_same_doc<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
    repoint: T,
) -> Result<Document, RepointFileError> {
    if let Some(_file_name) = file_name {
        let mut doc = doc.clone();
        if !entry_exists(&doc, _file_name.as_ref(), None) {
            let toml = doc.to_string();
            if key.as_ref() == "name" {
                let toml_add = format!(
                    r#"
['{}']
name = "{}""#,
                    _file_name.as_ref(),
                    repoint.as_ref()
                );

                let toml = toml + &toml_add;
                doc = toml.parse::<Document>().expect("failed to get toml doc");

                Ok(doc)
            } else {
                let err = Error::new(
                    "no sub-keys to file/dir entries other than 'repoint' is allowed",
                    ErrorKind::InvalidKey,
                );
                Err(RepointFileError::from(err))
            }
        } else {
            doc[_file_name.as_ref()][key.as_ref()] = value(repoint.as_ref());

            Ok(doc)
        }
    } else {
        let mut doc = doc.clone();
        doc[key.as_ref()] = value(repoint.as_ref());

        Ok(doc)
    }
}

pub fn add_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    name: T,
    repoint: T,
) -> Result<Document, RepointFileError> {
    let file_state = is_valid(&doc);
    if file_state == RepointFileState::NonExistant {
        let err = Error::new("repoint file doesn't exist", ErrorKind::NoFile);
        Err(RepointFileError::from(err))
    } else if file_name.is_none() {
        let entry_exists = entry_exists(&doc, "repository", Some(name.as_ref()));
        if !entry_exists {
            insert_entry(&doc, None, name.as_ref(), repoint.as_ref())
        } else {
            let err = Error::new(
                "failed to add sub-entry to about field repoint file",
                ErrorKind::DuplicateKey,
            );
            Err(RepointFileError::from(err))
        }
    } else {
        let file_name = file_name.unwrap();
        let entry_exists = entry_exists(&doc, file_name.as_ref(), None);
        if !entry_exists {
            insert_entry(
                &doc,
                Some(file_name.as_ref()),
                name.as_ref(),
                repoint.as_ref(),
            )
        } else {
            let err = Error::new(
                "failed to add entry to repoint file",
                ErrorKind::DuplicateKey,
            );
            Err(RepointFileError::from(err))
        }
    }
}

pub fn update_entry<T: AsRef<str>>(
    doc: &Document,
    file_name: Option<T>,
    key: T,
    repoint: T,
) -> Result<Document, RepointFileError> {
    let file_state = is_valid(&doc);
    if file_state == RepointFileState::NonExistant {
        let err = Error::new("repoint file doesn't exist", ErrorKind::InvalidFile);
        Err(RepointFileError::from(err))
    } else if file_name.is_some() {
        let file_name = file_name.unwrap();
        let entry_exists = entry_exists(&doc, file_name.as_ref(), None);
        if entry_exists {
            insert_entry(
                &doc,
                Some(file_name.as_ref()),
                key.as_ref(),
                repoint.as_ref(),
            )
        } else {
            let err = Error::new(
                "repoint entry doesn't exist in repointfile",
                ErrorKind::InvalidKey,
            );
            Err(RepointFileError::from(err))
        }
    } else {
        let entry_exists = entry_exists(&doc, "repository", Some(repoint.as_ref()));
        if entry_exists {
            insert_entry(&doc, Some("repository"), key.as_ref(), repoint.as_ref())
        } else {
            let err = Error::new(
                "file entry doesn't exist in repoint file",
                ErrorKind::InvalidKey,
            );
            Err(RepointFileError::from(err))
        }
    }
}

pub fn delete_entry<T: AsRef<str>>(
    doc: Document,
    file_name: T,
) -> Result<Document, RepointFileError> {
    let doc: Result<Document, RepointFileError> = {
        let mut _doc = doc.clone();
        let table = _doc.as_table_mut();
        table.set_implicit(true);
        let item = table.remove(file_name.as_ref());
        if let Some(mut _item) = item {
            let doc = {
                table.set_implicit(true);
                let table_string = table.to_string();
                table_string.parse::<Document>()?
            };

            Ok(doc)
        } else {
            let err = Error::new(
                "failed to delete entry in repoint file",
                ErrorKind::InvalidKey,
            );
            Err(RepointFileError::from(err))
        }
    };

    doc
}

pub fn hash_file(file: &str) -> std::io::Result<()> {
   //file.write_all(stuff.as_bytes()).unwrap();
   let mut repoint_path = PathBuf::new();
   repoint_path.push(file);

   let mut file = OpenOptions::new()
      .read(true)
      .write(true)
      .append(true)
      .open(repoint_path)?;//path.clone().into_os_string().into_string().unwrap())

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

   let repoint_hash = sha256(&contents);

   println!("repoint-hash: {:#?}", repoint_hash.to_hex_string());

   let mut hash_path = PathBuf::from("/tmp");
   hash_path.push("repoint");
   hash_path.push("test");
   hash_path.push("mock_send_filehashes");
   let mut repoint_hash_path = hash_path.clone();
   create_dir_all(&hash_path).expect("Failed to create directories.");
   repoint_hash_path.push(repoint_hash.to_hex_string());
   std::fs::File::create(&repoint_hash_path).expect("failed to create hash file");
   //if let Err(e) = writeln!(file, "{}", hash.to_hex_string()) {
   //    eprintln!("Couldn't write to file: {}", e);
   //}

   Ok(())
}

mod err {
    pub use toml_edit::TomlError;
    
    #[derive(Debug)]
    pub enum RepointFileError {
        IoError(std::io::Error),
        TomlError(TomlError),
        BoxRepointFileError(std::boxed::Box<RepointFileError>),
        Error(Error),
    }
    
    #[derive(Debug, PartialEq)]
    pub enum ErrorKind {
        InvalidKey,
        InvalidFile,
        DuplicateKey,
        NoFile,
    }
    
    #[derive(Debug)]
    pub struct Error {
        pub details: String,
        pub kind: ErrorKind,
    }
    
    impl Error {
        pub fn new(msg: &str, kind: ErrorKind) -> Error {
            Error {
                details: msg.to_string(),
                kind,
            }
        }
    }
    
    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.details)
        }
    }
    
    impl From<std::boxed::Box<RepointFileError>> for RepointFileError {
        fn from(error: std::boxed::Box<RepointFileError>) -> Self {
            RepointFileError::BoxRepointFileError(error)
        }
    }
    
    impl From<std::io::Error> for RepointFileError {
        fn from(error: std::io::Error) -> Self {
            RepointFileError::IoError(error)
        }
    }
    
    impl From<TomlError> for RepointFileError {
        fn from(error: TomlError) -> Self {
            RepointFileError::TomlError(error)
        }
    }
    
    impl std::error::Error for Error {
        fn description(&self) -> &str {
            &self.details
        }
    }
    
    impl From<Error> for RepointFileError {
        fn from(error: Error) -> Self {
            RepointFileError::Error(error)
        }
    }
}

#[cfg(test)]
mod toml_edit_integration {
    use super::*;

    #[test]
    fn toml_edit_insert() {
        let toml = r#"
"hello" = 'toml!' # comment
['a'.b]
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        // let's add a new key/value pair inside a.b: c = {d = "hello"}
        doc["a"]["b"]["c"]["d"] = value("hello");
        // autoformat inline table a.b.c: { d = "hello" }
        doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
"hello" = 'toml!' # comment
['a'.b]
c = { d = "hello" }
        "#;
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn toml_edit_set() {
        let toml = "";
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        // let's add a new key/value
        doc["a"] = value("hello");

        let expected = r#"a = "hello"
"#;
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn toml_edit_set_nested() {
        let toml = r#"
['a'.b]
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        // let's add a new key/value pair inside a.b: c = {d = "hello"}
        doc["a"]["b"]["c"]["d"] = value("hello");
        // autoformat inline table a.b.c: { d = "hello" }
        doc["a"]["b"]["c"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
['a'.b]
c = { d = "hello" }
        "#;
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn toml_edit_set_file_realistic() {
        let toml = r#"
['example']
name = "name"
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        doc["example"]["name"] = value("name");
        // Commenting out won't fail test.
        doc["example"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
['example']
name = "name"
        "#;
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn toml_edit_get_nested_item() {
        let toml = r#"
['example']
name = "name"
        "#;
        let doc = toml.parse::<Document>().expect("invalid doc");
        let repoint = doc["example"]["name"].as_str();
        let expected_repoint = "name";

        assert_eq!(repoint.unwrap(), expected_repoint)
    }

    #[test]
    fn toml_edit_set_get_nested_realistic() {
        let toml = r#"
['example']
name = "name"
        "#;
        let mut doc = toml.parse::<Document>().expect("invalid doc");
        assert_eq!(doc.to_string(), toml);
        doc["example"]["name"] = value("name");
        // Commenting out won't fail test.
        doc["example"].as_inline_table_mut().map(|t| t.fmt());

        let expected = r#"
['example']
name = "name"
        "#;

        assert_eq!(doc.to_string(), expected);
        assert_eq!(
            doc["example"]["name"].as_str().unwrap(),
            "name"
        );
        assert_eq!(doc["example"]["name"].as_str().unwrap(), "name")
    }

    #[test]
    fn toml_append() {
        let repoint_fields = r#"['repository']
version = "0.1.0""#;

        let toml = repoint_fields
            .parse::<Document>()
            .expect("invalid doc");
        let toml_string = toml.to_string();

        let repoint_fields = r#"
['example']
name = "name""#;

        let expected = r#"['repository']
version = "0.1.0"

['example']
name = "name"
"#;

        let new_toml_string = toml_string + repoint_fields;
        let new_toml = new_toml_string.parse::<Document>().expect("invalid doc");

        assert_eq!(new_toml.to_string(), expected);
    }
}

#[cfg(test)]
mod integration {
    use super::*;
    use fixture::Fixture;

    pub fn setup_test<T: AsRef<str>>(
        path: T,
        version: T
    ) -> Fixture {
        let fixture = Fixture::new()
            .add_dirpath(path.as_ref().to_string())
            .build();

        let repoint_path = path.as_ref().to_string() + "/repoint";
        let doc = init(
            repoint_path.as_ref(),
            version.as_ref(),
        ).unwrap();
        write(doc.clone(), repoint_path).expect("failed to write toml to disk");

        fixture
    }

    pub fn setup_add<T: AsRef<str>>(
        repoint_path: T,
    ) -> (Document, Result<String, RepointFileError>) {
        let doc = open(repoint_path.as_ref()).unwrap();
        let doc = add_entry(
            &doc,
            Some("example"),
            "name",
            "name",
        )
        .unwrap();
        write(doc.clone(), repoint_path.as_ref()).expect("failed to write toml to disk");
        let repoint_res = repoint(&doc, Some("example"), "name");

        (doc, repoint_res)
    }

    #[test]
    fn repointfile_init() {
        let path = "/tmp/repoint_tests";
        let gpath = "/tmp/repoint_tests/repoint";
        let mut fixture = setup_test(path, "0.1.0");
        let doc = open(gpath).unwrap();
        let is_valid = is_valid(&doc);
        let doc = open(gpath).unwrap();
        let expected = r#"['repository']
version = "0.1.0"
"#;
        fixture.teardown(true);
        assert_eq!(is_valid, RepointFileState::Valid);
        assert_eq!(doc.to_string(), expected);
    }

    #[test]
    fn repointfile_add_entry() {
        let path = "/tmp/repoint_tests";
        let gpath = "/tmp/repoint_tests/repoint";
        let mut fixture = setup_test(path, "0.1.0");
        let doc = open(gpath).unwrap();
        let doc = add_entry(
            &doc,
            Some("example"),
            "name",
            "name",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let repoint_res = repoint(&doc, Some("example"), "name").unwrap();
        fixture.teardown(true);
        assert_eq!(repoint_res, "name");
    }

    #[test]
    fn repointfile_error_add_entry() {
        let path = "/tmp/repoint_tests";
        let gpath = "/tmp/repoint_tests/repoint";
        let mut fixture = setup_test(path, "0.1.0");
        let (doc, repoint) = setup_add(gpath);

        // Focus of test.
        let result = add_entry(
            &doc,
            Some("example"),
            "name",
            "name_OTHER",
        );

        fixture.teardown(true);

        assert_eq!(repoint.unwrap(), "name");
        assert!(result.is_err());
    }


    #[test]
    fn repointfile_error_update_entry() {
        let path = "/tmp/repoint_tests";
        let gpath = "/tmp/repoint_tests/repoint";
        let mut fixture = setup_test(path, "0.1.0");
        let doc = open(gpath).unwrap();
        let result = update_entry(
            &doc,
            Some("example"),
            "name",
            "name",
        );

        fixture.teardown(true);

        assert!(result.is_err());
    }

    // Verifies there is no unexpected whitespace or formatting issuees for a basic case.
    #[test]
    fn format_repointfile_file_add_entry() {
        let path = "/tmp/repoint_tests";
        let gpath = "/tmp/repoint_tests/repoint";
        let mut fixture = setup_test(path, "0.1.0");
        let (_, _) = setup_add(gpath);

        // Focus of test.
        let toml_string = read_to_string(gpath).expect("failed to read repointfile");

        let doc = open(gpath).unwrap();

        //let mut doc = toml_string.parse::<Document>().expect("failed to get toml doc");
        //doc["example"].as_inline_table_mut().map(|t| t.fmt());
        let expected = r#"['repository']
version = "0.1.0"

['example']
name = "name"
"#;

        fixture.teardown(true);

        assert_eq!(doc.to_string(), expected);
        assert_eq!(toml_string, expected);
    }

    #[test]
    fn repointfile_entry_exists() {
        let path = "/tmp/repoint_tests";
        let gpath = "/tmp/repoint_tests/repoint";
        let mut fixture = setup_test(path, "0.1.0");
        let (doc, _) = setup_add(gpath);

        assert_eq!(entry_exists(&doc, "example", None), true);

        assert_eq!(exists(gpath, "example"), true);

        assert_eq!(entry_exists(&doc, "NOT_REAL_BITCON_ADD_A", None), false);

        assert_eq!(exists(gpath, "NOT_REAL_BITCOIN_ADD_B"), false);

        fixture.teardown(true);
    }

    #[test]
    fn repointfile_update_entry() {
        let path = "/tmp/repoint_tests";
        let gpath = "/tmp/repoint_tests/repoint";
        let mut fixture = setup_test(path, "0.1.0");
        let (doc, repoint_res) = setup_add(gpath);
        // Focus of test.
        let doc = update_entry(
            &doc,
            Some("example"),
            "name",
            "SHOULDNT DO THIS",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let updated_repoint_res = repoint(&doc, Some("example"), "name").unwrap();

        fixture.teardown(true);

        assert_eq!(repoint_res.unwrap(), "name");
        assert_eq!(updated_repoint_res, "SHOULDNT DO THIS");
    }

    fn helper_repointfile_delete_entry_thorough_check<T: AsRef<str>>(path_to_dir: T) {
        let path = path_to_dir;
        let gpath = path.as_ref().to_string() + "/repoint";
        let _fixture = setup_test(path.as_ref(), "0.1.0") ;

        let (doc, _) = setup_add(gpath.as_str());

        let lib_exists = entry_exists(&doc, "example", None);

        let doc = add_entry(&doc, Some("1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt"), "name", "name").unwrap();

        write(doc.clone(), gpath.as_str()).expect("failed to write toml to disk");

        let new_doc = delete_entry(doc, "1JvFXyZMC31ShnD8PSKgN1HKQ2kGQLVpCt").unwrap();
        write(new_doc.clone(), gpath).expect("failed to write toml to disk");

        let expected = r#"['repository']
version = "0.1.0"

['example']
name = "name"
"#;

        assert_eq!(lib_exists, true);
        assert_eq!(new_doc.to_string(), expected)
    }

     #[test]
     fn repointfile_delete_entry_thorough_assert() {
         let path = "/tmp/repoint_tests";
         helper_repointfile_delete_entry_thorough_check(path);

         Fixture::new().add_dirpath(path.to_string()).teardown(true);
     }


    #[test]
    fn repointfile_delete_file_entry() {
        let path = "/tmp/repoint_tests";
        let gpath = "/tmp/repoint_tests/repoint";
        let mut fixture = setup_test(path, "0.1.0");
        let doc = open(gpath).unwrap();
        let doc = add_entry(
            &doc,
            Some("example"),
            "name",
            "name",
        )
        .unwrap();
        write(doc.clone(), gpath).expect("failed to write toml to disk");
        let repoint_res = repoint(&doc, Some("example"), "name").unwrap();

        assert_eq!(repoint_res, "name");

        // Focus of test.
        let doc = open(gpath).unwrap();
        let doc = delete_entry(doc.clone(), "example").expect("failed to delete entry");
        write(doc, gpath).expect("failed to write toml to disk");

        let result = {
            let doc = open("/tmp/repoint_tests/repoint").unwrap();
            repoint(&doc, Some("example"), "name")
        };

        assert_eq!(result.is_ok(), false);

        fixture.teardown(true);
    }
}
