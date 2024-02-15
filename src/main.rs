fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("No arguments were provided type 'touch help' for more")
    } else if args[1] == "help" {
        println!("Use of the command: 'touch <file name>'")
    } else {
        let file_name = String::from(&args[1]);
        std::fs::File::create(file_name).expect("Failed to create file");
    }
}