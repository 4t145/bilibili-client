use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;
pub fn find_all_proto(proto_dir: &str) -> Vec<PathBuf> {
    WalkDir::new(proto_dir)
        .into_iter()
        .filter_map(|entry| {
            let path = entry.unwrap().path().to_owned();
            if path.is_file() && path.extension().unwrap_or_default() == "proto" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn create_grpc_mod(proto_dir: impl AsRef<Path>, outdir: impl AsRef<Path>) {
    #[derive(Default, Debug)]
    pub struct ModTree {
        submods: HashMap<String, ModTree>,
    }

    impl ModTree {
        pub fn is_leaf(&self) -> bool {
            self.submods.is_empty()
        }
        pub fn add<'i>(&mut self, path: impl IntoIterator<Item = &'i str>) {
            let mut iter = path.into_iter();
            if let Some(key) = iter.next() {
                self.submods
                    .entry(key.to_string())
                    .or_insert(Default::default())
                    .add(iter)
            }
        }
        pub fn write(self, name: String, from: &Path, to: &Path, path: &mut Vec<String>) {
            path.push(name);
            let mut source_filename: String = path.join(".");
            source_filename.push_str(".rs");
            let from_path = from.join(source_filename.trim_start_matches("grpc."));
            let mut to_path = to.to_path_buf();
            to_path.extend(path.iter());
            if !self.is_leaf() {
                let _e = fs::create_dir(&to_path);
                to_path.push("mod");
            }
            if let Some(f) = to_path.file_name().map(|x| x.to_str().unwrap().to_string()) {
                to_path.set_file_name(f.trim_start_matches("r#"))
            }
            to_path.set_extension("rs");
            println!(
                "cargo:warning=from {from_path} to {to_path}",
                from_path = from_path.display(),
                to_path = to_path.display()
            );
            let mut output_file = File::create(to_path).expect("fail to create file");
            for submod in self.submods.keys() {
                writeln!(output_file, "pub mod {submod};").unwrap();
            }
            if from_path.exists() {
                let mut buf = Vec::new();
                File::open(from_path)
                    .unwrap()
                    .read_to_end(&mut buf)
                    .unwrap();
                output_file.write_all(&buf).unwrap();
            }
            for (submod, tree) in self.submods {
                tree.write(submod, from, to, path)
            }
            path.pop();
        }
    }
    let mut tree = ModTree::default();
    let proto_dir = proto_dir.as_ref();
    let out_dir = outdir.as_ref();
    let grpc_dir = out_dir.join("grpc");
    if grpc_dir.exists() {
        fs::remove_dir_all(&grpc_dir).expect("fail to delete");
    }
    fs::create_dir(&grpc_dir).expect("fail to create");
    // copy all files
    for f_entry in fs::read_dir(proto_dir).expect("fail to read proto_dir") {
        let f_entry = f_entry.expect("file to entry");
        if !f_entry.path().is_file() {
            continue;
        }
        if !f_entry.path().extension().unwrap_or_default().eq("rs") {
            continue;
        }
        tree.add(
            f_entry
                .file_name()
                .to_str()
                .unwrap()
                .trim_end_matches(".rs")
                .split('.'),
        );
    }
    tree.write(
        "grpc".to_string(),
        proto_dir,
        outdir.as_ref(),
        &mut Vec::new(),
    );
    // scan to add mod
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = find_all_proto("bilibili-API-collect/grpc_api/bilibili/");
    let out_dir = PathBuf::from("proto_build/");
    let proto_version = out_dir.join("version");
    let output = Command::new("git")
        .args(["submodule", "status", "bilibili-API-collect"])
        .output()
        .expect("failed to execute process");
    let sha1 = String::from_utf8_lossy(&output.stdout).trim().split(' ').next().unwrap().trim().to_string();
    // 上游sha相同就没必要重新编译了
    if let Ok(prev_sha1) = fs::read(&proto_version) {
        if prev_sha1 == sha1.as_bytes() {
            return Ok(());
        }
    }
    fs::create_dir_all(&out_dir).expect("fail to create directory: ");
    tonic_build::configure()
        .build_client(true)
        .out_dir(&out_dir)
        .compile_well_known_types(true)
        .build_server(false)
        .compile(&proto_files, &["bilibili_api/grpc_api/"])?;
    create_grpc_mod(&out_dir, "src/");
    // let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut f = fs::File::open(&proto_version).expect("fail to create version");
    f.write_all(sha1.as_bytes()).expect("fail to write version");
    Ok(())
}
