use std::env;

pub enum LoginTypes {
    Environment,
    Module,
}

pub trait LoginDetails {
    fn username(&self) -> String;
    fn password(&self) -> String;
}

struct Module;

impl LoginDetails for Module {
    fn username(&self) -> String {
        return "username".to_string();
    }
    fn password(&self) -> String {
        return "password".to_string();
    }
}

struct Environment {
    username: String,
    password: String,
}

impl Environment {
    fn new() -> Environment {
        let (Ok(username), Ok(password)) = (env::var("LDAP_USERNAME"),env::var("LDAP_PASSWORD")) else{
            panic!("LDAP Credentials are not in environment variables... either add them as explained in README.md or change the LoginType to Module"); 
        };
        Environment { username, password }
    }
}
impl LoginDetails for Environment {
    fn username(&self) -> String {
        self.username.clone()
    }
    fn password(&self) -> String {
        self.password.clone()
    }
}

pub struct Login {
    value: Box<dyn LoginDetails>,
}

impl Login {
    pub fn new(login_type: LoginTypes) -> Self {
        match login_type {
            LoginTypes::Environment => Login {
                value: Box::new(Environment::new()),
            },
            LoginTypes::Module => Login {
                value: Box::new(Module),
            },
        }
    }
}

impl LoginDetails for Login {
    fn username(&self) -> String {
        self.value.username()
    }
    fn password(&self) -> String {
        self.value.password()
    }
}
