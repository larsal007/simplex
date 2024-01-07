mod cli;

fn main() {
    let _matches = cli::cli().get_matches();
    println!("Hello, world!");
}

