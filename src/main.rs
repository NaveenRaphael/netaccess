mod gecko;
mod login;
use fantoccini::{Client, ClientBuilder, Locator};
use gecko::{DriverSpawn, DriverTypes};
use login::{Login, LoginTypes};
use serde_json::{self, json};

async fn navigate_site(login: Login, driver: &Client) -> Result<(), fantoccini::error::CmdError> {
    driver.goto("https://netaccess.iitm.ac.in").await?;

    println!("Opened netaccess...");

    println!("Logging in...");
    let user_name_fill = driver.wait().for_element(Locator::Id("username")).await?;
    user_name_fill.send_keys(login.username().as_str()).await?;

    let password_fill = driver.wait().for_element(Locator::Id("password")).await?;
    password_fill.send_keys(login.password().as_str()).await?;

    let first_button = driver
        .wait()
        .for_element(Locator::XPath(
            "/html/body/main/div/div/div[2]/form/div[3]/button",
        ))
        .await?;
    first_button.click().await?;

    println!("clicking approve");
    driver.goto("https://netaccess.iitm.ac.in/approve").await?;

    println!("approve site loaded");
    let select_time = driver
        .wait()
        .for_element(Locator::XPath(
            "/html/body/main/div/div/div[1]/div/div[1]/form/div[1]/select/option[2]",
        ))
        .await?;

    println!("selecting the correct time?!");
    select_time.click().await?;

    println!("selected duration");
    let second_button = driver
        .wait()
        .for_element(Locator::XPath(
            "/html/body/main/div/div/div[1]/div/div[1]/form/div[3]/button",
        ))
        .await?;
    second_button.click().await?;

    let acceptable_button = driver
        .wait()
        .for_element(Locator::Id("btnAupAccept"))
        .await?;
    acceptable_button.click().await?;

    println!("Done!");
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let login = Login::new(LoginTypes::Module);

    println!("Spawning the driver");

    let driver = match DriverSpawn::new(DriverTypes::Gecko) {
        Ok(g) => g,
        Err(a) => panic!("{}", a),
    };

    //Because Linux is just that fast smh
    //
    let mut map = fantoccini::wd::Capabilities::new();
    let cap = json!({
        "args":["-headless"]
    });

    map.insert("acceptInsecureCerts".into(), true.into());
    map.insert("moz:firefoxOptions".into(), cap);
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

    let Ok(_) = navigate_site(login, &client).await else {
        println!("Error in navigating site");
        return;
    };
    println!("Done! cleaning and closing!");

    let Ok(_) = client.close().await else {
        println!("Error in closing driver");
        return;
    };
    println!("closed the client...");
}
