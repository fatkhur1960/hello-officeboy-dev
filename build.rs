extern crate protoc_rust;

use protoc_rust::{Args, Customize};
use std::{
    env,
    fs,
    // fs::File,
    // io::{BufRead, BufReader, BufWriter},
};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("Cannot get OUT_DIR");

    protoc_rust::run(Args {
        out_dir: &out_dir,
        input: &["src/protos/apf.proto"],
        includes: &["src/protos"],
        customize: Customize {
            serde_derive: Some(true),
            ..Default::default()
        },
    })
    .expect("Protoc compile");

    let path = format!("{}/apf.rs", out_dir);
    let content = fs::read_to_string(&path).expect("cannot read autogen protobuf apf.rs");
    let mut new_content = vec![];

    for line in content.split("\n") {
        if line.starts_with("#![") {
            continue;
        }
        new_content.push(line);
    }

    let new_content: String = new_content.join("\n");

    let _ = fs::write(&path, new_content);

    println!("cargo:rerun-if-changed={}", "src/protos/apf.proto");
}
