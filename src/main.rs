fn main() {
    if let Err(error) = mediavault::run() {
        eprintln!("MediaVault konnte nicht starten: {error}");
        std::process::exit(1);
    }
}
