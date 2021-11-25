use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug)]
struct Doc {
    members: Members,
}

#[derive(Deserialize, Debug)]
struct Members {
    #[serde(rename = "member")]
    members: Vec<Member>,
}

#[derive(Deserialize, Debug)]
struct Member {
    name: String,
    summary: Option<Summary>,
    #[serde(rename = "param")]
    params: Option<Vec<Param>>,
    #[serde(rename = "returns")]
    returns: Option<Vec<Returns>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Summary {
    #[serde(rename = "$value")]
    content: String,
}

#[derive(Deserialize, Debug)]
struct Param {
    name: String,
    #[serde(rename = "$value")]
    content: String,
}

#[derive(Deserialize, Debug)]
struct Returns {
    #[serde(rename = "$value")]
    content: String,
}

fn main() {
    let mut args = env::args_os();
    args.next().unwrap();

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = File::open(
        args.next()
            .map(PathBuf::from)
            .unwrap_or_else(|| manifest_dir.join("../sys/vendor/bin/StereoKit.xml")),
    )
    .expect("failed to open doc XML file");
    let dest = File::create(
        args.next()
            .map(PathBuf::from)
            .unwrap_or_else(|| manifest_dir.join("summaries.json")),
    )
    .expect("failed to open doc XML file");

    let parsed: Doc = serde_xml_rs::from_reader(&source).expect("deserialization failed");

    let summaries = parsed
        .members
        .members
        .into_iter()
        .filter_map(|m| m.summary.map(|s| (m.name, s)))
        .collect::<IndexMap<_, _>>();

    serde_json::to_writer(dest, &summaries).expect("failed serializing json");
}
