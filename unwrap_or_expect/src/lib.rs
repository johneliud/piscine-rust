pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),

        Security::Message => server
            .unwrap_or_else(|_| panic!("ERROR: program stops"))
            .to_string(),

        Security::Warning => match server {
            Ok(url) => url.to_string(),
            Err(_) => "WARNING: check the server".to_string(),
        },

        Security::NotFound => match server {
            Ok(url) => url.to_string(),
            Err(msg) => format!("Not found: {}", msg),
        },

        Security::UnexpectedUrl => match server {
            Ok(url) => panic!("{}", url),
            Err(msg) => msg.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_data() {
        let server = Ok("https://example.com");
        assert_eq!(fetch_data(server, Security::Unknown), "https://example.com");

        let server = Err("Server not reachable");
        assert_eq!(
            fetch_data(server, Security::Message),
            "ERROR: program stops"
        );

        let server = Ok("https://example.com");
        assert_eq!(fetch_data(server, Security::Warning), "https://example.com");

        let server = Err("Server not reachable");
        assert_eq!(
            fetch_data(server, Security::NotFound),
            "Not found: Server not reachable"
        );

        let server = Ok("https://example.com");
        assert_eq!(fetch_data(server, Security::UnexpectedUrl), "https://example.com");

        let server = Err("Server not reachable");
        assert_eq!(
            fetch_data(server, Security::UnexpectedUrl),
            "Server not reachable"
        );
    }
}