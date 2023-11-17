// use std::fs;
use md5::compute;
// use std::fs::rename;
// use std::path::Path;
use walkdir::WalkDir;
fn main() {
    let img_path = "/media/pipi/0123-4567/Images/";

    let paths_list = vec![
        "/media/pipi/0123-4567/jpg/",
        "/media/pipi/0123-4567/png/",
        "/media/pipi/0123-4567/bmp/",
        "/media/pipi/0123-4567/AV/",
    ];
    let _zlist = paths_list
        .iter()
        .map(|x| std::fs::create_dir_all(x))
        .collect::<Vec<_>>();

    let mlist = ["avi", "AVI", "mpg", "MPG", "mp4", "MP4"];

    for e in WalkDir::new(img_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            println!("Processing: {}", fname);
            let digest = compute(fname.clone());
            let parts = &fname.split(".").collect::<Vec<&str>>();
            let ext = parts.last().unwrap();
            if ext == &"jpg" || ext == &"JPG" || ext == &"jpeg" || ext == &"JPEG" {
                let new_jpg_addr = format!("{:?}", digest) + ".jpg";
                let jpg_rename_resp =
                    std::fs::rename(fname.clone(), paths_list[0].to_owned() + &new_jpg_addr + ".jpg");
                match jpg_rename_resp {
                    Ok(_) => println!("{} moved to {}", fname, new_jpg_addr),
                    Err(e) => {
                        std::fs::remove_file(fname.clone()).unwrap();
                        println!("Error {} not moved so removing {}", e, fname)
                    }
                }
            } else if ext == &"png" || ext == &"PNG" {
                let new_png_addr = format!("{:?}", digest) + ".png";
                let png_rename_resp =
                    std::fs::rename(fname.clone(), paths_list[1].to_owned() + &new_png_addr + ".png");
                match png_rename_resp {
                    Ok(_) => println!("{} moved to {}", fname, new_png_addr),
                    Err(e) => {
                        std::fs::remove_file(fname.clone()).unwrap();
                        println!("Error {} not moved so removing {}", e, fname)
                    }
                }
            } else if ext == &"bmp" || ext == &"BMP" {
                let new_bmp_addr = format!("{:?}", digest) + ".bmp";
                let bmp_rename_resp =
                    std::fs::rename(fname.clone(), paths_list[2].to_owned() + &new_bmp_addr + ".bmp");
                match bmp_rename_resp {
                    Ok(_) => println!("{} moved to {}", fname, new_bmp_addr),
                    Err(e) => {
                        std::fs::remove_file(fname.clone()).unwrap();
                        println!("Error {} not moved so removing {}", e, fname)
                    }
                }
            } else if mlist.contains(&ext) {
                let new_mov_addr = format!("{:?}", digest);
                std::fs::rename(
                    fname.clone(),
                    paths_list[4].to_owned() + &new_mov_addr + "." + ext.to_lowercase().as_str(),
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
