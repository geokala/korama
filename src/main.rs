use std::env;
use std::fs::File;
use std::io::BufReader;
use rodio::Source;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <mp3 file path>", &args[0]);
    } else {
        let filename = &args[1].trim();
        println!("Playing {}", &filename);
        let device = rodio::default_output_device().unwrap();

        let file = File::open(&filename).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
                        .expect("Failed to read line");
    }
}

