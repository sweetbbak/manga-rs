use reqwest::header::USER_AGENT;
use select::document::Document;
use select::predicate::Name;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 && args.len() != 4 {
        println!("Usage: rust-script <url> [--list-images] [--download-images <output-dir>]");
        return Ok(());
    }
    let url = &args[1];
    let list_images = args.get(2).map_or(false, |arg| arg == "--list-images");
    let download_images = args.get(2).map_or(false, |arg| arg == "--download-images");
    let output_dir = args.get(3).map_or("", |arg| arg.as_str());

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
        for node in document.find(Name("img")) {
            if let Some(img_src) = node.attr("src") {
                let filename = Path::new(img_src).file_name().unwrap();
                let output_path = Path::new(output_dir).join(filename);

                let mut output_file = File::create(output_path)?;
                let img_data = client.get(img_src).send()?.bytes()?;
                output_file.write_all(&img_data)?;
            }
        }
    }

    Ok(())
}
