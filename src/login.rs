use std::env;

const USERNAME: &str = "username";
const PASSWORD: &str = "password";

pub enum LoginTypes {
    Environment,
    Module,
}

pub struct Login {
    username: String,
    password: String,
}

impl Login {
    pub fn new(login_type: LoginTypes) -> Self {
        match login_type {
            LoginTypes::Environment => Self::new_env(),
            LoginTypes::Module => Self::new_module(),
        }
    }

    fn new_module() -> Self {
        Login {
            username: USERNAME.to_string(),
            password: PASSWORD.to_string(),
        }
    }
    fn new_env() -> Self {
        let (Ok(username), Ok(password)) = (env::var("LDAP_USERNAME"),env::var("LDAP_PASSWORD")) else{
            panic!("LDAP Credentials are not in environment variables... either add them as explained in README.md or change the LoginType to Module"); 
        };
        Login { username, password }
    }
    pub fn username(&self) -> String {
        self.username.clone()
    }
    pub fn password(&self) -> String {
        self.password.clone()
    }
}
