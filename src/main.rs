mod gecko;
mod login;
use fantoccini::{Client, ClientBuilder, Locator};
use gecko::{DriverSpawn, DriverTypes};
use login::{Login, LoginTypes};

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

    let driver = match DriverSpawn::new(DriverTypes::Edge) {
        Ok(g) => g,
        Err(a) => panic!("{}", a),
    };

    //Because Linux is just that fast smh
    let client = match ClientBuilder::native()
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
