mod gecko;
mod login;
mod my_error_enum;

use fantoccini::{Client, ClientBuilder, Locator};
use gecko::{DriverSpawn, DriverTypes};
use login::{Login, LoginTypes};
use my_error_enum::MyError;
use serde_json::{self, json};

// These are the variables that users need to change. Note that headless only works on Geckodriver(Mozilla firefox) right now.
const DRIVER_TYPE: DriverTypes = DriverTypes::Gecko;
// const HEADLESS: bool = false;
const HEADLESS: bool = true;
const LOGIN_TYPE: LoginTypes = LoginTypes::Module;

async fn navigate_site(login: Login, driver: &Client) -> Result<(), MyError> {
    println!("Opening netaccess...");
    driver.goto("https://cc.iitm.ac.in/").await?;

    println!("Logging in...");

    let user_name_fill = driver.wait().for_element(Locator::Id("username")).await?;
    user_name_fill.send_keys(login.username().as_str()).await?;

    let password_fill = driver.wait().for_element(Locator::Id("password")).await?;
    password_fill.send_keys(login.password().as_str()).await?;

    let first_button = driver
        .wait()
        .for_element(Locator::Css("html body.index-page section#hero.hero.section.dark-background div#front-carousel.carousel.slide.carousel-fade div.carousel-item form.form button.btn.btn-primary.btn-lg"))
        .await?;
    first_button.click().await?;
    //Using find did not work for some reason...
    match driver.find_all(Locator::Id("username")).await {
        Ok(a) => {
            if a.len() == 1 {
                return Err(MyError::Cred("Invalid Credentials!".into()));
            }
        }
        Err(_) => {}
    };

    println!("clicking approve");
    driver.goto("https://cc.iitm.ac.in/account/approve").await?;

    println!("approve site loaded");
    let select_time = driver
        .wait()
        .for_element(Locator::XPath(r#"//*[@id="radios-1"]"#))
        .await?;

    println!("selecting the correct time?!");
    select_time.click().await?;

    println!("selected duration");
    let second_button = driver
        .wait()
        .for_element(Locator::XPath(r#"//*[@id="use-policy"]"#))
        .await?;
    second_button.click().await?;

    let acceptable_button = driver.wait().for_element(Locator::Id("approveBtn")).await?;
    acceptable_button.click().await?;

    println!("Done!");
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let login = Login::new(LOGIN_TYPE);

    println!("Spawning the driver");

    let driver = match DriverSpawn::new(DRIVER_TYPE) {
        Ok(g) => g,
        Err(a) => panic!("{}", a),
    };

    // Headless version
    let mut map = fantoccini::wd::Capabilities::new();
    if HEADLESS {
        // If you do not want headless, set this to false
        let cap = json!({
            // "args":["-headless"]
        });
        map.insert("acceptInsecureCerts".into(), true.into());
        map.insert("moz:firefoxOptions".into(), cap);
    }
    println!("Spawning client");
    let client = match ClientBuilder::native()
        .capabilities(map)
        .connect(driver.get_port().as_str())
        .await
    {
        Ok(a) => a,
        Err(e) => panic!("Cannot connect because {e}"),
    };
    println!("Connecting client");

    match navigate_site(login, &client).await {
        Ok(_) => {}
        Err(a) => panic!("Error in navigating site! {a}"), // panic!("Error in navigating site");
    };
    println!("Done! cleaning and closing!");

    let Ok(_) = client.close().await else {
        panic!("Error in closing driver");
    };
    println!("closed the client...");
}
