#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate glob;
extern crate docopt;
extern crate copy_dir;

use std::fs::OpenOptions;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use docopt::Docopt;
use serde_json::Value as Json;

const USAGE: &'static str = "
docset-dashing

Usage:
  docset-dashing <indir> <outdir>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_indir: String,
    arg_outdir: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let indir = Path::new(&args.arg_indir);
    let outdir = Path::new(&args.arg_outdir);
    let docset_name = indir.file_name().unwrap().to_str().unwrap().to_owned();

    copy_to_builddir(indir, outdir, docset_name.clone());
    edit_css(&builddir(outdir).join("rustdoc.css"));
    write_dashing_json(outdir, docset_name);
}

fn builddir(outdir: &Path) -> PathBuf {
    outdir.join("build")
}

fn copy_to_builddir(indir: &Path, outdir: &Path, docset_name: String) {
    let doc_basedir = indir.parent().unwrap();
    let builddir = builddir(outdir);

    fs::create_dir(&outdir).unwrap();
    fs::create_dir(&builddir).unwrap();
    ::copy_dir::copy_dir(indir, builddir.join(&docset_name)).unwrap();
    copy_html_dependencies(doc_basedir, &builddir, "*.css");
    copy_html_dependencies(doc_basedir, &builddir, "*.js");
    copy_html_dependencies(doc_basedir, &builddir, "*.woff");
}

fn copy_html_dependencies(doc_basedir: &Path, builddir: &Path, pattern: &str) {
    for file in glob::glob(doc_basedir.join(pattern).to_str().unwrap()).unwrap() {
        let path = file.unwrap();
        let filename = path.file_name().unwrap().clone();
        fs::copy(&path, builddir.join(filename)).unwrap();
    }
}

fn edit_css(path: &Path) {
    let mut file = OpenOptions::new().append(true).open(path).unwrap();

    file.write_all(r#"
.sidebar, .sub, .srclink {
    display: none;
}

.content {
    margin-left: 0px;
    max-width: inherit;
}
    "#.as_bytes()).unwrap();
}

fn write_dashing_json(outdir: &Path, docset_name: String) {
    let base_json = include_str!("../dashing.json");
    let mut outfile = OpenOptions::new()
        .write(true)
        .create(true)
        .open(outdir.join("dashing.json"))
        .unwrap();

    let mut json: Json = serde_json::from_str(base_json).unwrap();
    json["name"] = json!{docset_name};
    json["package"] = json!{docset_name};

    outfile.write_all(json.to_string().as_bytes()).unwrap();
}
