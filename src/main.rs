use argh::FromArgs;
use std::process::Command;

pub mod image;
pub mod rgb;

const URL: &str = "https://user-images.githubusercontent.com/6933510/107239146-dcc3fd00-6a28-11eb-8c7b-41aaf6618935.png";

#[derive(Debug, FromArgs)]
/// A command line tool to manipulate images.
struct Args {
    /// image to process
    #[argh(positional, short = 's')]
    src: Option<String>,

    /// make colors more red
    #[argh(switch, short = 'r')]
    reds: Option<bool>,

    /// make colors more red
    #[argh(switch, short = 'i')]
    invert: Option<bool>,

    /// quantize image
    #[argh(switch, short = 'q')]
    quantize: Option<bool>,

    /// quantize image
    #[argh(switch, short = 'm')]
    mean: Option<bool>,
}

fn main() {
    let args: Args = argh::from_env();
    let out = "out.png";
    let src = match args.src {
        Some(src) => src,
        None => URL.to_string(),
    };

    match image::Image::new(&src) {
        Ok(mut img) => {
            if args.reds.unwrap_or(false) {
                println!("> making colors more red");
                img.into_reds().save(&out);
            } else if args.invert.unwrap_or(false) {
                println!("> inverting colors");
                img.invert().save(&out);
            } else if args.quantize.unwrap_or(false) {
                println!("> quantizing colors");
                img.quantize().save(&out);
            } else if args.mean.unwrap_or(false) {
                println!("> mean of colors");
                img.mean().save(&out);
            }
            Command::new("viu").arg(out).spawn().unwrap();
        }
        Err(e) => println!("{:?}", e),
    }
}
