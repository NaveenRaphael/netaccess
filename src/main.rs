mod gecko;
mod login;
use gecko::Gecko;
use login::{Login, LoginTypes};
use thirtyfour::prelude::{By, DesiredCapabilities, WebDriver, WebDriverResult};

async fn navigate_site(login: Login, driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://netaccess.iitm.ac.in").await?;

    let username_fill = driver.find(By::Id("username")).await?;
    username_fill.send_keys(login.username()).await?;

    let password_fill = driver.find(By::Id("password")).await?;
    password_fill.send_keys(login.password()).await?;

    let first_button = driver.find(By::Id("submit")).await?;
    first_button.click().await?;

    driver
        .goto("https://netaccess.iitm.ac.in/account/approve")
        .await?;
    let select_time = driver.find(By::Id("radios-1")).await?;
    select_time.click().await?;

    let second_button = driver.find(By::Id("approveBtn")).await?;
    second_button.click().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let login = Login::new(LoginTypes::Environment);

    let _gecko = match Gecko::new() {
        Ok(g) => g,
        Err(a) => panic!("{}", a),
    };

    let driver = WebDriver::new("http://localhost:4444", DesiredCapabilities::firefox()).await?;

    navigate_site(login, &driver).await?;

    driver.quit().await?;

    Ok(())
}
