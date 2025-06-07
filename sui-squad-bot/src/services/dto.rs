use std::{env, fmt};

pub enum Endpoints {
    User,
    Payment,
    Withdraw,
}

impl fmt::Display for Endpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend_host = env::var("HOST").expect("HOST environment variable not set");
        let backend_url = format!("https://{}", backend_host);

        match self {
            &Endpoints::User => write!(f, "{}/user", backend_url),
            &Endpoints::Payment => write!(f, "{}/payment", backend_url),
            &Endpoints::Withdraw => write!(f, "{}/withdraw", backend_url),
        }
    }
}
