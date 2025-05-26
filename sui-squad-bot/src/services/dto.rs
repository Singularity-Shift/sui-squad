use std::{fmt, env};

pub enum Endpoints {
    Auth,
}

impl fmt::Display for Endpoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend_port = env::var("PORT").expect("PORT environment variable not set");
        let backend_host = env::var("HOST").expect("HOST environment variable not set");
        let backend_url = format!("https://{}:{}", backend_host, backend_port);

        match self {
            &Endpoints::Auth => write!(f, "{}/auth", backend_url),
        }
    }
}