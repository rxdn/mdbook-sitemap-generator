mod builder;

use std::fs;
use std::fs::ReadDir;
use std::io;
use std::io::Write;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    domain: String,

    #[clap(short, long, value_parser)]
    output: Option<String>,
}

fn main() {
    let mut args = Args::parse();

    if let Some(stripped) = args.domain.strip_suffix('/') {
        args.domain = stripped.to_string();
    }

    if !args.domain.starts_with("https") {
        args.domain = format!("https://{}", args.domain);
    } else if args.domain.starts_with("http") {
        args.domain = args.domain.replace("http", "https");
    }

    let dir = fs::read_dir("src").expect("Unable to read src directory");
    let paths = find_paths(dir, "").expect("Failed to walk directory");

    let mut urls: Vec<String> = paths
        .iter()
        .map(|path| format!("{domain}{path}", domain = args.domain))
        .collect();

    urls.insert(0, args.domain);

    let url_set = builder::UrlSet::new(urls);
    let sitemap = url_set.to_xml().expect("Failed to build sitemap.xml");

    match args.output {
        Some(path) => {
            let mut file = fs::File::create(path).expect("Failed to create file");
            file.write_all(sitemap.as_bytes())
                .expect("Failed to write to file");
        }
        None => {
            println!("{sitemap}");
        }
    }
}

fn find_paths(directory: ReadDir, current_path: &str) -> io::Result<Vec<String>> {
    let mut paths = Vec::new();

    for entry in directory {
        let entry = entry?;
        let path = entry.path();

        let name =
            path.file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| io::Error::new(
                    io::ErrorKind::Other,
                    "Unable to get file name",
                ))?;

        if path.is_dir() {
            let new_path = format!("{}/{}", current_path, name);
            let mut sub_paths = find_paths(fs::read_dir(path)?, new_path.as_str())?;
            paths.append(&mut sub_paths);
        } else if name.ends_with(".md") && name != "SUMMARY.md" {
            paths.push(format!("{}/{}", current_path, name));
        }
    }

    Ok(paths)
}
