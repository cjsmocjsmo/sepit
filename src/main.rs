// use std::fs;
// use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let img_path = "/media/pipi/0123-4567/Images/";
    let jpg_path = "/media/pipi/0123-4567/Images/jpg/";
    let jpg2_path = "/media/pipi/0123-4567/Images/JPG/";
    let png_path = "/media/pipi/0123-4567/Images/png/";
    let gif_path = "/media/pipi/0123-4567/Images/gif/";
    let bmp_path = "/media/pipi/0123-4567/Images/bmp/";
    let tif_path = "/media/pipi/0123-4567/Images/tif/";

    for e in WalkDir::new(img_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let parts = &fname.split(".").collect::<Vec<&str>>();
            let ext = parts.last().unwrap();
            if ext == &"jpg" {
                std::fs::rename(fname.clone(), jpg_path.to_owned() + parts[parts.len() - 2] + ".jpg")
                    .unwrap();
            } else if ext == &"JPG" {
                std::fs::rename(fname.clone(), jpg2_path.to_owned() + parts[parts.len() - 2] + ".jpg")
                    .unwrap();
            } else if ext == &"png" {
                std::fs::rename(fname.clone(), png_path.to_owned() + parts[parts.len() - 2] + ".png")
                    .unwrap();
            } else if ext == &"gif" {
                std::fs::rename(fname.clone(), gif_path.to_owned() + parts[parts.len() - 2] + ".gif")
                    .unwrap();
            } else if ext == &"bmp" {
                std::fs::rename(fname.clone(), bmp_path.to_owned() + parts[parts.len() - 2] + ".bmp")
                    .unwrap();
            } else if ext == &"tif" {
                std::fs::rename(fname.clone(), tif_path.to_owned() + parts[parts.len() - 2] + ".tif")
                    .unwrap();
            }

        }
    }

}
