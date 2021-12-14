use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use std::{
    env,
    fs::File,
    io::Write,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            .unwrap_or_else(|| manifest_dir.join("../vendor/bin/StereoKit.xml")),
    )
    .expect("failed to open doc XML file");
    let dest_summaries = File::create(
        args.next()
            .map(PathBuf::from)
            .unwrap_or_else(|| manifest_dir.join("summaries.json")),
    )
    .expect("failed to open destination JSON");
    let mut dest_blank_checklist = File::create(
        args.next()
            .map(PathBuf::from)
            .unwrap_or_else(|| manifest_dir.join("checklist.blank.md")),
    )
    .expect("failed to open destination .md");

    let parsed: Doc = serde_xml_rs::from_reader(&source).expect("deserialization failed");

    let summaries = parsed
        .members
        .members
        .iter()
        .flat_map(|m| {
            m.summary.clone().map(|s| {
                std::iter::once((m.name.clone(), s.content.clone()))
                    .chain(
                        m.params
                            .iter()
                            .flatten()
                            .map(|p| (format!("{}~{}", m.name, p.name), p.content.clone())),
                    )
                    .chain(
                        m.returns
                            .iter()
                            .flatten()
                            .map(|r| (format!("{}->", m.name.clone()), r.content.clone())),
                    )
            })
        })
        .flatten()
        .collect::<IndexMap<String, String>>();

    serde_json::to_writer(dest_summaries, &summaries).expect("failed serializing json");

    let checklist = parsed
        .members
        .members
        .iter()
        .map(|m| format!("- [ ] {}\n", m.name))
        .collect::<String>();

    dest_blank_checklist
        .write_all(checklist.as_bytes())
        .expect("writing checklist failed");
}
