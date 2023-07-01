use std::process::Command;

pub struct Gecko(std::process::Child);
impl Gecko {
    pub fn new() -> Result<Self, String> {
        match Command::new("geckodriver")
            .args(["--log", "fatal"])
            .spawn(){
            Err(e) =>Err(format!("Geckodriver possibly not installed; please run `cargo install geckodriver`. Actual error: {}", e)),
            Ok(a)=> Ok(Gecko(a)),
        }
    }
}

impl Drop for Gecko {
    fn drop(&mut self) {
        self.0.kill().expect("This should work, you know");
    }
}
