mod gen;
mod parse;
mod process;
mod google {
    mod protobuf {
        mod descriptor;
        mod compiler {
            mod plugin;
        }
    }
}

use std::{
    fs,
    io::{self, Read},
};

use anyhow::Result;
use bytes::Bytes;
use clap::App;
use parse::parse_request;
use process::process_files;

use crate::gen::gen_file;

fn main() -> Result<()> {
    App::new("tobu-gen-rust").version("0.1.0").get_matches();
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let req = parse_request(Bytes::from(buf))?;
    let files = process_files(&req.proto_file)?;
    for file in &files {
        if let Some(parent) = file.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file.path, format!("{}", gen_file(file)))?;
    }
    Ok(())
}
