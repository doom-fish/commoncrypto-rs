#[cfg(target_vendor = "apple")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::fmt::Write as _;
    use std::io::Read;
    use std::path::PathBuf;

    for argument in std::env::args_os().skip(1) {
        let path = PathBuf::from(argument);
        let mut file = std::fs::File::open(&path)?;
        let mut hasher = commoncrypto::Sha256::new()?;
        hasher.update(&[])?;
        let mut buffer = vec![0; 64 * 1024];
        loop {
            let read = file.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read])?;
        }
        let mut digest = String::with_capacity(64);
        for byte in hasher.finalize()? {
            write!(&mut digest, "{byte:02x}")?;
        }
        println!("{digest}  {}", path.display());
    }
    Ok(())
}

#[cfg(not(target_vendor = "apple"))]
fn main() {}
