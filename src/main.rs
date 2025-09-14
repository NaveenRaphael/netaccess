mod gecko;
mod login;
use std::{collections::HashMap, iter::Map};

use fantoccini::{Client, ClientBuilder, Locator};
use gecko::{DriverSpawn, DriverTypes};
use login::{Login, LoginTypes};

async fn navigate_site(login: Login, driver: &Client) -> Result<(), fantoccini::error::CmdError> {
    driver.goto("https://netaccess.iitm.ac.in").await?;

    let user_name_fill = driver.wait().for_element(Locator::Id("username")).await?;
    user_name_fill.send_keys(login.username().as_str()).await?;

    let password_fill = driver.wait().for_element(Locator::Id("password")).await?;
    password_fill.send_keys(login.password().as_str()).await?;

    // let first_button = driver.wait().for_element(Locator::Id("submit")).await?;
    let first_button = driver
        .wait()
        .for_element(Locator::XPath(
            "/html/body/main/div/div/div[2]/form/div[3]/button",
        ))
        .await?;
    first_button.click().await?;

    // // let first_button = driver
    // //     .wait()
    // //     .for_element(Locator::Css(".btn-primary"))
    // //     .await?;
    driver.goto("https://netaccess.iitm.ac.in/approve").await?;
    // // let select_time = driver.wait().for_element(Locator::Id("radios-1")).await?;
    let select_time = driver
        .wait()
        .for_element(Locator::XPath(
            "/html/body/main/div/div/div[1]/div/div[1]/form/div[1]/select/option[3]",
        ))
        .await?;
    select_time.click().await?;

    // let second_button = driver.wait().for_element(Locator::Id("approveBtn")).await?;
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

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let login = Login::new(LoginTypes::Module);

    let driver = match DriverSpawn::new(DriverTypes::Gecko) {
        Ok(g) => g,
        Err(a) => panic!("{}", a),
    };

    //Because Linux is just that fast smh
    //
    let mut map = fantoccini::wd::Capabilities::new();
    map.insert("acceptInsecureCerts".into(), true.into());
    let client = match ClientBuilder::native()
        .capabilities(map)
        .connect(driver.get_port().as_str())
        .await
    {
        Ok(a) => a,
        Err(e) => panic!("Cannot connect because {e}"),
    };

    let Ok(_) = navigate_site(login, &client).await else {
        println!("Error in navigating site");
        return;
    };

    let Ok(_) = client.close().await else {
        println!("Error in closing driver");
        return;
    };
}
