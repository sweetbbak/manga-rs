use reqwest::header::USER_AGENT;
use select::document::Document;
use select::predicate::Name;
use std::env;
use std::fs::{self,File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
let args: Vec<String> = env::args().collect();
    if !(2..=4).contains(&args.len()) {
        println!("Usage: rust-script <url> [--list-images] [--download-images <output-dir>]");
        return Ok(());
    }
    let url = &args[1];
    let list_images = args.contains(&String::from("--list-images"));
    let download_images = args.contains(&String::from("--download-images"));
    let output_dir = if let Some(index) = args.iter().position(|arg| arg == "--download-images") {
        sanitize_directory_name(args.get(index + 1).unwrap_or(url))
    } else {
        url.to_string()
    };    
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).header(USER_AGENT, "rust-script").send()?;
    let body = res.text()?;
    let document = Document::from(body.as_str());

    if list_images {
        let images: Vec<_> = document
            .find(Name("img"))
            .map(|node| node.attr("src").unwrap_or(""))
            .collect();
        println!("{:?}", images);
    } else {
        println!("{}", body);
    }

    if download_images {
        if output_dir.is_empty() {
            println!("Please provide an output directory.");
            return Ok(());
        }
        if !Path::new(&output_dir).exists() {
            fs::create_dir_all(&output_dir)?;
            println!("Directory '{}' created for output.", &output_dir);
        }

        for node in document.find(Name("img")) {
            if let Some(img_src) = node.attr("src") {
                let filename = Path::new(img_src).file_name().unwrap();
                let output_path = PathBuf::from(&output_dir).join(filename);

                let mut output_file = File::create(output_path)?;
                let img_data = client.get(img_src).send()?.bytes()?;
                output_file.write_all(&img_data)?;
            }
        }
    }

    Ok(())
}

fn sanitize_directory_name(dir_name: &str) -> String {
    dir_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}
