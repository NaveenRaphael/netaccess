mod gecko;
mod login;
use fantoccini::{Client, ClientBuilder, Locator};
use gecko::{DriverSpawn, DriverTypes};
use login::{Login, LoginTypes};
use std::{thread, time::Duration};

async fn navigate_site(login: Login, driver: &Client) -> Result<(), fantoccini::error::CmdError> {
    driver.goto("https://netaccess.iitm.ac.in").await?;

    let user_name_fill = driver.wait().for_element(Locator::Id("username")).await?;
    user_name_fill.send_keys(login.username().as_str()).await?;

    let password_fill = driver.wait().for_element(Locator::Id("password")).await?;
    password_fill.send_keys(login.password().as_str()).await?;

    let first_button = driver.wait().for_element(Locator::Id("submit")).await?;
    first_button.click().await?;

    driver
        .goto("https://netaccess.iitm.ac.in/account/approve")
        .await?;
    let select_time = driver.wait().for_element(Locator::Id("radios-1")).await?;
    select_time.click().await?;

    let second_button = driver.wait().for_element(Locator::Id("approveBtn")).await?;
    second_button.click().await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let login = Login::new(LoginTypes::Environment);

    let _gecko = match DriverSpawn::new(DriverTypes::Gecko) {
        Ok(g) => g,
        Err(a) => panic!("{}", a),
    };

    //Because Linux is just that fast smh
    let driver = match ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
    {
        Ok(a) => a,
        Err(_e) => {
            println!("Waiting for some more time...");
            thread::sleep(Duration::from_millis(10));
            match ClientBuilder::native()
                .connect("http://localhost:4444")
                .await
            {
                Ok(a) => a,
                Err(e) => panic!("Cannot connect because {e}"),
            }
        }
    };

    let Ok(_) = navigate_site(login, &driver).await else {
        println!("Error in navigating site");
        return;
    };

    let Ok(_) = driver.close().await else {
        println!("Error in closing driver");
        return;
    };
}
