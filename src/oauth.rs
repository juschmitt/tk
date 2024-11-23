use std::{net::TcpListener, io::{Write, BufReader, BufRead}};

use reqwest::Url;


pub fn start_oauth_process() -> Result<(), OAuthError> {
    println!("Starting OAuth process...");
    let tt_client_id = "O2Mbd1j8nkD7NvNS1R";
    let tt_client_secret = "WxRy01)gJDnffZ#R)_Bza2230zY5T7B&";

    let tt_auth_url = "https://ticktick.com/oauth/authorize";
    let tt_token_url = "https://ticktick.com/oauth/token";

   
    let scopes = vec!["tasks:read", "tasks:write"];

    let authorize_url = "https://something.com";

    println!("Visit this URL: {}", authorize_url);
    let (code, state) = await_code()?;
    exchange_code(tt_token_url, &code, tt_client_id, tt_client_secret, tt_auth_url, tt_token_url, scopes).unwrap();
    Ok(())
}

fn exchange_code<'resp>(url: &str, code: &str, tt_client_id: &str, tt_client_secret: &str, tt_auth_url: &str, tt_token_url: &str, scopes: Vec<&str>) -> Result<&'resp str, OAuthError> {
    let client = reqwest::blocking::Client::new();
    let form = [
        ("client_id", tt_client_id),
        ("client_secret", tt_client_secret),
        ("code", code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", "http://localhost:8080"),
        ("scope", &scopes.join(" ")),
    ];
    let resp = client
        .post(url)
        .basic_auth(tt_client_id, Some(tt_client_secret))
        .form(&form)
        .send().unwrap();

    let body = resp.text().unwrap();
    println!("Response: {}", body);
    Ok("ok")
}

fn await_code() -> Result<(String, String), OAuthError> {
    // start an tcp listener to handle redirect
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();
                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let (_, code_value) = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();
                code = code_value.into_owned();

                let (_, state_value) = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();
                state = state_value.into_owned();
            }
            // write into browser
            let message = "Go back to the terminal! :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            println!("Authorization code: {}", code);
            println!(
                "State: Got {}",
                state
            );
            return Ok((code, state));
        } else {
            return Err(OAuthError::TcpError);
        }
    }
    return Err(OAuthError::TcpError);
}

#[derive(Debug)]
pub enum OAuthError {
    TcpError,
}