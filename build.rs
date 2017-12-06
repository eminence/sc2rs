extern crate protoc_rust;

use std::fs::{File, read_dir};
use std::io::{Read, Write};

fn main() {
    println!("cargo:rerun-if-changed=s2client-proto/s2clientprotocol/");
    for entry in read_dir("s2client-proto/s2clientprotocol").unwrap() {
        let entry = entry.unwrap();
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }
    println!("Generating protobuf files...");
    protoc_rust::run(protoc_rust::Args {
        out_dir: "sc2-protobuf/src/protos",
        input: &["s2client-proto/s2clientprotocol/common.proto",
            "s2client-proto/s2clientprotocol/data.proto",
            "s2client-proto/s2clientprotocol/debug.proto",
            "s2client-proto/s2clientprotocol/error.proto",
            "s2client-proto/s2clientprotocol/query.proto",
            "s2client-proto/s2clientprotocol/raw.proto",
            "s2client-proto/s2clientprotocol/sc2api.proto",
            "s2client-proto/s2clientprotocol/spatial.proto",
            "s2client-proto/s2clientprotocol/score.proto",
            "s2client-proto/s2clientprotocol/ui.proto"],
        includes: &["s2client-proto"],
    }).expect("protoc");

    // patch up raw.rs, since the protobuf files include a field named "Self"
    // which is not valid rust

    let s = {
        let mut raw = File::open("sc2-protobuf/src/protos/raw.rs").unwrap();
        let mut s = String::new();
        raw.read_to_string(&mut s).unwrap();
        s
    };

    let mut raw = File::create("sc2-protobuf/src/protos/raw.rs").unwrap();
    raw.write_all(s.as_bytes()).unwrap();

}