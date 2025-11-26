use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use regex::Regex;

#[allow(dead_code)]
pub enum DriverTypes {
    Gecko,
    Edge,
    None,
}

struct DriverSpawnInner {
    app: std::process::Child,
    port: String,
}
pub struct DriverSpawn(Option<DriverSpawnInner>);

impl DriverSpawn {
    pub fn new(dr: DriverTypes) -> Result<Self, String> {
        match dr {
            DriverTypes::None => return Ok(DriverSpawn(None)),
            _ => {}
        };
        let command = match dr {
            DriverTypes::Gecko => "geckodriver",
            DriverTypes::Edge => "msedgedriver",
            DriverTypes::None => unreachable!(),
        };
        println!("spawning {command}");

        let mut child = match Command::new(command).args(["--log", "trace"]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn(){
            Ok(a) => a,
            Err(e) => match dr{
                DriverTypes::Gecko => return Err(format!("Geckodriver possibly not installed; please run `cargo install geckodriver`. Actual error: {}", e)),
                DriverTypes::Edge => return Err(format!("Edge driver possibly not installed. Actual error: {}", e)),
                DriverTypes::None => unreachable!(),

            },
        };

        let stdout = child.stdout.take().unwrap();
        let mut bufread = BufReader::new(stdout);
        let mut buf = String::new();
        let mut port = String::new();

        let re = match dr {
            DriverTypes::Gecko => Regex::new(r"127\.0\.0\.1:(\d+)").unwrap(),
            DriverTypes::Edge => Regex::new(r" (\d{5})\.").unwrap(),
            DriverTypes::None => unreachable!(),
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
        println!("made driver!");
        return Ok(DriverSpawn(Some(DriverSpawnInner { app: child, port })));
    }

    pub(crate) fn get_port(&self) -> String {
        let a = match self.0 {
            Some(ref a) => format!("http://localhost:{}", a.port),
            None => "http://localhost:4444".into(),
        };
        println!("Connecting to {}", a);
        return a;
    }
}

impl Drop for DriverSpawn {
    fn drop(&mut self) {
        let Some(a) = &mut self.0 else { return };
        a.app
            .kill()
            .expect("Killing the driver on drop should not fail");
    }
}
