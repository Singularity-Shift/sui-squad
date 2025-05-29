use std::{fmt, env};

pub enum Endpoints {
    Auth,
    User,
}

impl fmt::Display for Endpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend_host = env::var("HOST").expect("HOST environment variable not set");
        let backend_url = format!("https://{}", backend_host);

        match self {
            &Endpoints::Auth => write!(f, "{}/auth", backend_url),
            &Endpoints::User => write!(f, "{}/user", backend_url),
        }
    }
}