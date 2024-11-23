use std::{error::Error, net::TcpListener, io::{BufReader, BufRead, Write}};

use oauth2::{basic::BasicClient, ClientSecret, AuthUrl, ClientId, TokenUrl, RedirectUrl, CsrfToken, Scope, AuthorizationCode};
use reqwest::Url;

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
                let redirect_url= request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let (_, code_value) = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();
                code = AuthorizationCode::new(code_value.into_owned());

                    let (_, state_value) = url
                    .query_pairs()
                    .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "state"
                    }).unwrap();
                state = CsrfToken::new(state_value.to_string());
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
