use std::env;

fn print_help() {
    println!("usage: image_to_text /path/to/image -rw 1 -rh 2");
}

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();

    if paths.is_empty() {
        print_help();
        return;
    }
    let (image_path, rw, rh) = {
        if paths.len() == 1 {
            (paths[0].clone(), 1, 1)
        } else if paths.len() >= 3 && paths.len() < 5 {
            match paths[1].as_str() {
                "-rw" => (paths[0].clone(), paths[2].parse().unwrap(), 1),
                "-rh" => (paths[0].clone(), 1, paths[2].parse().unwrap()),
                _ => {
                    println!("Wrong input");
                    print_help();
                    return;
                }
            }
        } else {
            match (paths[1].as_str(), paths[3].as_str()) {
                ("-rw", "-rh") => (
                    paths[0].clone(),
                    paths[2].parse().unwrap(),
                    paths[4].parse().unwrap(),
                ),
                ("-rh", "-rw") => (
                    paths[0].clone(),
                    paths[4].parse().unwrap(),
                    paths[2].parse().unwrap(),
                ),
                _ => {
                    println!("Wrong input");
                    print_help();
                    return;
                }
            }
        }
    };

    let picture = match image::open(&image_path) {
        Ok(i) => i.into_rgba8(),
        Err(e) => {
            println!("Failed to open image {}.", paths[0]);
            println!("Error was: {e:?}");
            return;
        }
    };
    let width = picture.width() / rw;
    let height = picture.height() / rh;
    let picture = image::imageops::resize(
        &picture,
        width,
        height,
        image::imageops::FilterType::Nearest,
    );
    for row in picture.rows() {
        for pixmap in row {
            if pixmap.0[0] > 200 && pixmap.0[1] > 200 && pixmap.0[2] > 200 {
                print!(" ");
            } else if pixmap.0[1] > 200 {
                print!("~");
            } else {
                print!("#");
            }
        }
        print!("\n");
    }
}
