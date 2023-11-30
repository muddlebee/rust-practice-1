//! A simple key/value store.

pub use kv::KvStore;
pub use kv_disk::KvStoreDisk;
mod kv;
mod kv_disk;
