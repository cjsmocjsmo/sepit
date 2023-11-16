// use std::fs;
use md5::compute;
// use std::fs::rename;
// use std::path::Path;
use walkdir::WalkDir;
fn main() {
    let img_path = "/media/pipi/0123-4567/Images/";
    // for e in WalkDir::new(img_path)
    //     .follow_links(true)
    //     .into_iter()
    //     .filter_map(|e| e.ok())
    // {
    //     if e.metadata().unwrap().is_file() {
    //         let fname = e.path();
    //         let _new_filename = sanitize_filename(fname);
    //     }
    // };

    let jpg_path = "/media/pipi/0123-4567/Images/jpg/";
    let png_path = "/media/pipi/0123-4567/Images/png/";
    let gif_path = "/media/pipi/0123-4567/Images/gif/";
    let bmp_path = "/media/pipi/0123-4567/Images/bmp/";
    let tif_path = "/media/pipi/0123-4567/Images/tif/";
    let mov_path = "/media/pipi/0123-4567/AV/";
    let paths_list = vec![jpg_path, png_path, gif_path, bmp_path, tif_path, mov_path];
    let _zlist = paths_list.iter().map(|x| std::fs::create_dir_all(x)).collect::<Vec<_>>().unwrap();

    let mlist = ["avi", "AVI", "mpg", "MPG", "mp4", "MP4"];

    for e in WalkDir::new(img_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let digest = compute(fname.clone());
            let parts = &fname.split(".").collect::<Vec<&str>>();
            let ext = parts.last().unwrap();
            if ext == &"jpg" || ext == &"JPG" || ext == &"jpeg" || ext == &"JPEG" {
                let new_jpg_addr = format!("{:?}", digest) + ".jpg";
                std::fs::rename(fname.clone(), jpg_path.to_owned() + &new_jpg_addr + ".jpg")
                    .unwrap();
            } else if ext == &"png" || ext == &"PNG" {
                let new_png_addr = format!("{:?}", digest) + ".png";
                std::fs::rename(fname.clone(), png_path.to_owned() + &new_png_addr + ".png")
                    .unwrap();
            } else if ext == &"gif" || ext == &"GIF" {
                let new_gif_addr = format!("{:?}", digest) + ".gif";
                std::fs::rename(fname.clone(), gif_path.to_owned() + &new_gif_addr + ".gif")
                    .unwrap();
            } else if ext == &"bmp" || ext == &"BMP" {
                let new_bmp_addr = format!("{:?}", digest) + ".bmp";
                std::fs::rename(fname.clone(), bmp_path.to_owned() + &new_bmp_addr + ".bmp")
                    .unwrap();
            } else if ext == &"tif" || ext == &"TIF" || ext == &"tiff" || ext == &"TIFF" {
                let new_tif_addr = format!("{:?}", digest) + ".tif";
                std::fs::rename(fname.clone(), tif_path.to_owned() + &new_tif_addr + ".tif")
                    .unwrap();
            } else if mlist.contains(&ext) {
                let new_mov_addr = format!("{:?}", digest);
                std::fs::rename(
                    fname.clone(),
                    mov_path.to_owned() + &new_mov_addr + "." + ext,
                )
                .unwrap();
            }
        }
    }
}

// fn sanitize_filename(path: &Path) -> Result<String, std::io::Error> {
//     let filename = path.file_name().unwrap().to_str().unwrap();
//     let mut new_filename = String::new();
//     for c in filename.chars() {
//         if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
//             new_filename.push(c);
//         }
//     }
//     let new_filename = new_filename.to_lowercase();
//     let new_path = path.parent().unwrap().join(&new_filename);
//     println!("new_path: \n\t{:?}\n\t{:?}\n", path, new_path);
//     rename(path, &new_path)?;

//     Ok(new_filename)
// }
