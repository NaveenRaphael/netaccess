use std::process::Command;
use thirtyfour::prelude::*;

use std::env;

struct LoginDetails {
    username: String,
    password: String,
}
impl LoginDetails {
    pub fn get() -> Result<LoginDetails, env::VarError> {
        let username = env::var("LDAP_USERNAME")?;
        let password = env::var("LDAP_PASSWORD")?;
        Ok(LoginDetails { username, password })
    }
}

async fn navigate_site(login: LoginDetails) -> WebDriverResult<()> {
    let caps = DesiredCapabilities::firefox();

    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // Navigate to https://wikipedia.org.
    driver.goto("https://netaccess.iitm.ac.in").await?;

    let username_fill = driver.find(By::Id("username")).await?;
    username_fill.send_keys(login.username).await?;

    let password_fill = driver.find(By::Id("password")).await?;
    password_fill.send_keys(login.password).await?;

    let first_button = driver.find(By::Id("submit")).await?;
    first_button.click().await?;

    driver
        .goto("https://netaccess.iitm.ac.in/account/approve")
        .await?;
    let select_time = driver.find(By::Id("radios-1")).await?;
    select_time.click().await?;

    let second_button = driver.find(By::Id("approveBtn")).await?;
    second_button.click().await?;

    driver.quit().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let login = LoginDetails::get().expect("LDAP Credentials not in environment variables...");

    let mut gecko = Command::new("geckodriver")
        .spawn()
        .expect("Geckodriver not found; please run cargo install geckodriver first");
    let e = navigate_site(login).await;
    // Always explicitly close the browser.

    gecko.kill().expect("Well oopse");

    e
}
