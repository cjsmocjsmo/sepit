// use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use std::fs::rename;

fn main() {
    let img_path = "/media/pipi/0123-4567/Images/";
    for e in WalkDir::new(img_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path();
            let _new_filename = sanitize_filename(fname);
        }
    };


    let jpg_path = "/media/pipi/0123-4567/Images/jpg/";
    let png_path = "/media/pipi/0123-4567/Images/png/";
    let gif_path = "/media/pipi/0123-4567/Images/gif/";
    let bmp_path = "/media/pipi/0123-4567/Images/bmp/";
    let tif_path = "/media/pipi/0123-4567/Images/tif/";
    let mov_path = "/media/pipi/0123-4567/AV/";
    let mlist = ["avi", "AVI", "mpg", "MPG", "mp4", "MP4"];

    for e in WalkDir::new(img_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let parts = &fname.split(".").collect::<Vec<&str>>();
            let ext = parts.last().unwrap();
            if ext == &"jpg" || ext == &"jpeg" {
                std::fs::rename(
                    fname.clone(),
                    jpg_path.to_owned() + parts[parts.len() - 2] + ".jpg",
                )
                .unwrap();
            } else if ext == &"png" {
                std::fs::rename(
                    fname.clone(),
                    png_path.to_owned() + parts[parts.len() - 2] + ".png",
                )
                .unwrap();
            } else if ext == &"gif" {
                std::fs::rename(
                    fname.clone(),
                    gif_path.to_owned() + parts[parts.len() - 2] + ".gif",
                )
                .unwrap();
            } else if ext == &"bmp" {
                std::fs::rename(
                    fname.clone(),
                    bmp_path.to_owned() + parts[parts.len() - 2] + ".bmp",
                )
                .unwrap();
            } else if ext == &"tif" || ext == &"tiff" {
                std::fs::rename(
                    fname.clone(),
                    tif_path.to_owned() + parts[parts.len() - 2] + ".tif",
                )
                .unwrap();
            } else if mlist.contains(&ext) {
                std::fs::rename(fname.clone(), mov_path.to_owned() + parts[parts.len() - 2] + "." + ext).unwrap();
            }
        }
    }
}

fn sanitize_filename(path: &Path) -> Result<String, std::io::Error> {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let mut new_filename = String::new();
    for c in filename.chars() {
        if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
            new_filename.push(c);
        }
    }
    let new_filename = new_filename.to_lowercase();
    let new_path = path.parent().unwrap().join(&new_filename);
    println!("new_path: \n\t{:?}\n\t{:?}\n", path, new_path);
    rename(path, &new_path)?;

    Ok(new_filename)
}
