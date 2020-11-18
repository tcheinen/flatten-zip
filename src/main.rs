use log::{error, warn};
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about, author)]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

// Drops .zip from the end of a string
fn drop_zip(name: &str) -> String {
    let mut segments = name.split(".").collect::<Vec<_>>();
    if let Some(last) = segments.last() {
        if *last == "zip" {
            segments.truncate(segments.len() - 1);
        }
    }
    segments.join(".")
}

fn main() {
    let opt = Opt::from_args();
    pretty_env_logger::init();

    let mut root_zip = {
        let f = std::fs::File::open(&opt.file)
            .expect("file argument should exist (or maybe io error idk)");
        zip::ZipArchive::new(f).expect("zip should open (probably invalid zip)")
    };

    root_zip
        .file_names()
        .filter(|x| x.ends_with(".zip")) // this is a flawed assumption, a valid zip file doesn't need to end in .zip
        .map(String::from)
        .collect::<Vec<_>>()
        .iter()
        .filter_map(|name| {
            let mut buf: Vec<u8> = Vec::new();
            if let Err(e) = root_zip.by_name(&name).unwrap().read_to_end(&mut buf) {
                error!("Couldn't read file {:?} for some reason: {}", &name, e);
            }

            let cursor = Cursor::new(buf);
            if let Ok(zip) = zip::ZipArchive::new(cursor) {
                Some((name, zip))
            } else {
                error!(
                    "Could not construct ZipArchive from {}, likely because it isn't a valid zip",
                    name
                );
                None
            }
        })
        .for_each(|(name, mut zip)| {
            let root_folder_name = drop_zip(&opt.file.as_path().display().to_string());
            let sub_folder_name = drop_zip(name);
            if let Err(e) =
                fs::create_dir_all(format!("{}/{}", &root_folder_name, &sub_folder_name))
            {
                error!("{}", e);
            }

            let inner_zip_names = zip
                .file_names()
                .filter(|name| !name.ends_with("/"))
                .map(String::from)
                .collect::<Vec<_>>();

            for file_name in inner_zip_names {
                let output_name = file_name.split("/").last().unwrap_or(&file_name);

                if Path::new(&format!(
                    "{}/{}/{}",
                    &root_folder_name, &sub_folder_name, &output_name
                ))
                .exists()
                {
                    warn!(
                        "Duplicate file name {:?} for sub-zip {:?}",
                        &output_name, &sub_folder_name
                    );
                }

                let mut buf: Vec<u8> = Vec::new();
                if let Err(e) = zip.by_name(&file_name).unwrap().read_to_end(&mut buf) {
                    error!("Couldn't read file {:?} for some reason: {}", &file_name, e);
                }
                if let Err(e) = fs::write(
                    format!(
                        "{}/{}/{}",
                        &root_folder_name, &sub_folder_name, &output_name
                    ),
                    &buf,
                ) {
                    error!("couldn't write output file for some reason: {}", e);
                }
            }
        });
}
