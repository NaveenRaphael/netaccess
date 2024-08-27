use std::process::Command;

pub enum DriverTypes {
    Gecko,
    Edge,
}

pub struct DriverSpawn(std::process::Child);
impl DriverSpawn {
    pub fn new(dr: DriverTypes) -> Result<Self, String> {
        match dr {
            DriverTypes::Gecko => {
                match Command::new("geckodriver")
            .args(["--log", "fatal"])
            .spawn(){
            Err(e) =>Err(format!("Geckodriver possibly not installed; please run `cargo install geckodriver`. Actual error: {}", e)),
            Ok(a)=> Ok(DriverSpawn(a)),
        }
            }
            DriverTypes::Edge => todo!(),
        }
    }
}

impl Drop for DriverSpawn {
    fn drop(&mut self) {
        self.0.kill().expect("This should work, you know");
    }
}
