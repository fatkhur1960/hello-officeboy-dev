extern crate protoc_rust;

use protoc_rust::{Args, Customize};

fn main() {
    protoc_rust::run(Args {
        out_dir: "src/protos",
        input: &["src/protos/apf.proto"],
        includes: &["src/protos"],
        customize: Customize {
            serde_derive: Some(true),
            ..Default::default()
        },
    })
    .expect("Protoc compile");

    println!("cargo:rerun-if-changed={}", "src/protos");
}
