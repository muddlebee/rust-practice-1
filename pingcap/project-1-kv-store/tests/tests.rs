use assert_cmd::prelude::*;
use kvs::{KvStore, KvStoreDisk};
use predicates::str::contains;
use std::process::Command;
use predicates::prelude::predicate;

// `kvs` with no args should exit with a non-zero code.
#[test]
fn cli_no_args() {
    Command::cargo_bin("kvs").unwrap().assert().failure();
}

// `kvs -V` should print the version
#[test]
fn cli_version() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}



// `kvs set <KEY> <VALUE>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_set() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(["set", "key1", "value1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("key: key1, value: value1"));
}
// `kvs get <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_get() {

    Command::cargo_bin("kvs")
        .unwrap()
        .args(["set", "key1", "value1"])
        .assert()
        .success();

    Command::cargo_bin("kvs")
        .unwrap()
        .args(["get", "key1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("key: key1, value: value1"));
}


// `kvs rm <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_rm() {

    Command::cargo_bin("kvs")
        .unwrap()
        .args(["set", "key1", "value1"])
        .assert()
        .success();

    Command::cargo_bin("kvs")
        .unwrap()
        .args(["rm", "key1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("key: key1, value: value1"));
}

#[test]
fn cli_invalid_get() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(["get"])
        .assert()
        .failure();

    Command::cargo_bin("kvs")
        .unwrap()
        .args(["get", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_set() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(["set"])
        .assert()
        .failure();

    Command::cargo_bin("kvs")
        .unwrap()
        .args(["set", "missing_field"])
        .assert()
        .failure();

    Command::cargo_bin("kvs")
        .unwrap()
        .args(["set", "extra", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_rm() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(["rm"])
        .assert()
        .failure();

    Command::cargo_bin("kvs")
        .unwrap()
        .args(["rm", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_subcommand() {
    Command::cargo_bin("kvs")
        .unwrap()
        .args(["unknown", "subcommand"])
        .assert()
        .failure();
}

// Should get previously stored value
#[test]
fn get_stored_value() {
    let mut store = KvStore::new();

    store.set("key1".to_owned(), "value1".to_owned());
    store.set("key2".to_owned(), "value2".to_owned());

    assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    assert_eq!(store.get("key2".to_owned()), Some("value2".to_owned()));
}

// Should overwrite existent value
#[test]
fn overwrite_value() {
    let mut store = KvStore::new();

    store.set("key1".to_owned(), "value1".to_owned());
    assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));

    store.set("key1".to_owned(), "value2".to_owned());
    assert_eq!(store.get("key1".to_owned()), Some("value2".to_owned()));
}

// Should get `None` when getting a non-existent key
#[test]
fn get_non_existent_value() {
    let mut store = KvStore::new();

    store.set("key1".to_owned(), "value1".to_owned());
    assert_eq!(store.get("key2".to_owned()), None);
}

#[test]
fn remove_key() {
    let mut store = KvStore::new();

    store.set("key1".to_owned(), "value1".to_owned());
    store.remove("key1".to_owned());
    assert_eq!(store.get("key1".to_owned()), None);
}


// add tests for KvStoreDisk
#[test]
fn set_and_get_value() {
    let mut store = KvStoreDisk::new().expect("Failed to create KvStoreDisk");
    let key = "test_key".to_string();
    let value = "test_value".to_string();

    store.set(key.clone(), value.clone()).expect("Failed to set value");
    assert_eq!(store.get(key).unwrap(), Some(value));
}

#[test]
fn get_non_existent_key() {
    let store = KvStoreDisk::new().expect("Failed to create KvStoreDisk");
    let key = "non_existent_key".to_string();
    assert_eq!(store.get(key).unwrap(), None);
}

#[test]
fn set_and_remove_key() {
    let mut store = KvStoreDisk::new().expect("Failed to create KvStoreDisk");
    let key = "key_to_remove".to_string();
    store.set(key.clone(), "value".to_string()).expect("Failed to set value");

    assert_eq!(store.remove(key.clone()).unwrap(), Some("value".to_string()));
    assert_eq!(store.get(key).unwrap(), None);
}