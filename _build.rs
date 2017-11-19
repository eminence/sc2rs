extern crate protoc_rust;

use std::fs::read_dir;

fn main() {
    //    let mut files = Vec::new();
    //
    //    // discover the proto files we need to compile
    //    for dir in read_dir("s2client-proto/s2clientprotocol").expect("read_dir") {
    //        let dir = dir.expect("direntry");
    //        if let Some(ext) = dir.path().extension() {
    //            if ext == "proto" {
    //                files.push(dir.path().to_string_lossy().into_owned());
    //            }
    //        }
    //    }
    //
    //    let files_str : Vec<_> = files.iter().map(|f| f.as_ref()).collect();
    return;

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
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
}