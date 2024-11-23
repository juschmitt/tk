use std::{error::Error, net::TcpListener, io::{BufReader, BufRead, Write}};

use oauth2::{basic::BasicClient, ClientSecret, AuthUrl, ClientId, TokenUrl, RedirectUrl, CsrfToken, Scope, AuthorizationCode};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting OAuth process...");
    let tt_client_id = ClientId::new("O2Mbd1j8nkD7NvNS1R".to_string());
    let tt_client_secret = ClientSecret::new("WxRy01)gJDnffZ#R)_Bza2230zY5T7B&".to_string());
    let tt_auth_url = AuthUrl::new("https://ticktick.com/oauth/authorize".to_string())?;
    let tt_token_url = TokenUrl::new("https://ticktick.com/auth/token".to_string())?;
    
    // start webserver to handle redirect
    let client = BasicClient::new(
        tt_client_id, 
        Some(tt_client_secret),
        tt_auth_url,
        Some(tt_token_url),
    ).set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

    let (authorize_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    .add_scopes(vec![
        Scope::new("tasks:read".to_string()), 
        Scope::new("tasks:write".to_string())
        ])
    .url();

    println!("Visit this URL: {}", authorize_url);

    // start an tcp listener to handle redirect
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();
                let parts: Vec<&str> = request_line.split_whitespace().map(str::trim).collect();

                let code1 = parts.get(1).unwrap().split('=').next().unwrap();
                code = AuthorizationCode::new(code1.to_string());

                let state1 = parts.get(2).unwrap().split('=').next().unwrap();
                state = CsrfToken::new(state1.to_string());
            }
            
            // write into browser
            let message = "Go back to the terminal!";
            let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", message.len(), message);
            stream.write_all(response.as_bytes()).unwrap();

            println!("Authorization code: {}", code.secret());
            println!("State: Got {} and expected {}", state.secret(), csrf_token.secret());
        }
    }
    
    Ok(())
}
