use std::io::Read;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "d", long = "directory", default_value = "twiggy_results")]
    input_dir: String,
}

fn main() {
    let opt = Opt::from_args();
    let files = glob::glob(&format!("{}/*.csv", opt.input_dir)).unwrap();
    let mut total_diff = 0;
    for file in files {
        match file {
            Ok(path) => {
                let mut contents = String::new();
                let mut f = std::fs::File::open(&path).expect("Couldn't read file.");
                f.read_to_string(&mut contents).unwrap();
                total_diff += contents
                    .lines()
                    .last()
                    .unwrap()
                    .split(",")
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            }
            Err(error) => panic!("Encountered an error: {:?}", error),
        }
    }
    println!(
        "The difference between this commit and master is {} bytes.",
        total_diff
    );
}
