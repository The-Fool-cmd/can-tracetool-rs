fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: can-tracetool stats <file>");
    } else {
        println!("Arguments: ");
        for arg in &args {
            println!("{}", arg);
        }
    }
}
