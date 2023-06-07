use std::process::Command;

pub mod image;
pub mod limits;
pub mod rgb;

const URL: &str = "https://user-images.githubusercontent.com/6933510/107239146-dcc3fd00-6a28-11eb-8c7b-41aaf6618935.png";



fn main() {
    let path = std::path::PathBuf::from("out.png");
    let img_bytes = reqwest::blocking::get(URL).unwrap().bytes().unwrap();

    match image::load_img(&img_bytes) {
        Ok(img) => {
            // let mut line = Array2::from(img);
            // let binding = [Rgb::white(); 20];
            // let ones = ArrayView::from(&binding);
            // let binding = [Rgb::black(); 40];
            // let zeros = ArrayView::from(&binding);
            // line.append(Axis(0), zeros).unwrap();
            // line.append(Axis(0), ones).unwrap();
            // line.append(Axis(0), zeros).unwrap();
            img.save(path.to_str().unwrap());
            Command::new("viu").arg(path).spawn().unwrap();
        }
        Err(e) => println!("{:?}", e),
    }
}
