use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use regex::Regex;

pub enum DriverTypes {
    Gecko,
    Edge,
}

pub struct DriverSpawn {
    app: std::process::Child,
    port: String,
}

impl DriverSpawn {
    pub fn new(dr: DriverTypes) -> Result<Self, String> {
        let command = match dr {
            DriverTypes::Gecko => "geckodriver",
            DriverTypes::Edge => "msedgedriver",
        };

        let mut child = match Command::new(command).stdout(Stdio::piped()).spawn(){
            Ok(a) => a,
            Err(e) => match dr{
                DriverTypes::Gecko => return Err(format!("Geckodriver possibly not installed; please run `cargo install geckodriver`. Actual error: {}", e)),
                DriverTypes::Edge => return Err(format!("Edge driver possibly not installed. Actual error: {}", e)),
            },
        };

        let stdout = child.stdout.take().unwrap();
        let mut bufread = BufReader::new(stdout);
        let mut buf = String::new();
        let mut port = String::new();

        let re = match dr {
            DriverTypes::Gecko => Regex::new(r"127\.0\.0\.1:(\d+)").unwrap(),
            DriverTypes::Edge => Regex::new(r" (\d{5})\.").unwrap(),
        };
        while let Ok(n) = bufread.read_line(&mut buf) {
            if n > 0 {
                println!("Line: {}", buf.as_str());
                let val = re.captures(buf.trim());
                if let Some(a) = val {
                    port = port
                        + a.get(1)
                            .expect("Getting the second entry; why did this fail!")
                            .as_str()
                            .to_string()
                            .trim();
                    println!("Captured port to be: {port}");
                    break;
                }
                buf.clear();
            } else {
                break;
            }
        }
        child.stdout.replace(bufread.into_inner());
        return Ok(DriverSpawn { app: child, port });
    }

    pub(crate) fn get_port(&self) -> String {
        let a = format!("http://localhost:{}", self.port);
        println!("{}", a);
        return a;
    }
}

impl Drop for DriverSpawn {
    fn drop(&mut self) {
        self.app.kill().expect("This should work, you know");
    }
}
