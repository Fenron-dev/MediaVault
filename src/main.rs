fn main() {
    if let Err(error) = mediavault::run() {
        eprintln!("MediaVault could not start: {error}");
        std::process::exit(1);
    }

    println!("MediaVault foundation scaffold is ready.");
}
