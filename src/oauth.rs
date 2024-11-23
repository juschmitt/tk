use std::{net::TcpListener, io::{Write, BufReader, BufRead}};
use serde::{Serialize, Deserialize};
use reqwest::Url;
use crate::oauth::access_token_response::AccessTokenResponse;
use crate::oauth::oauth_client::OAuthClient;
use crate::oauth::oauth_error::OAuthError;

mod oauth_error;
mod access_token_response;
mod oauth_client;

pub fn start_oauth_process() -> Result<(), OAuthError> {
    println!("Starting OAuth process...");
    let oauth_client = OAuthClient::new(
        "O2Mbd1j8nkD7NvNS1R",
        "WxRy01)gJDnffZ#R)_Bza2230zY5T7B&",
        "https://ticktick.com/oauth/authorize",
        "https://ticktick.com/oauth/token",
        "http://localhost:8080"
    );

    println!("Visit this URL: {}", authentication_url(&oauth_client));
    let (code, _) = await_code()?;
    let token_response = exchange_code(&oauth_client, &code)?;

    Ok(())
}

fn exchange_code(oauth_client: &OAuthClient, code: &str) -> Result<AccessTokenResponse, OAuthError> {
    let client = reqwest::blocking::Client::new();
    let form_data = [
        ("client_id", oauth_client.client_id),
        ("client_secret", oauth_client.client_secret),
        ("code", code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", oauth_client.redirect_uri),
        ("scope", &String::from(&oauth_client.scopes)),
    ];
    let resp = client
        .post(oauth_client.token_url)
        .basic_auth(oauth_client.client_id, Some(oauth_client.client_secret))
        .form(&form_data)
        .send().unwrap();

    let body = resp.text()?;
    let response: AccessTokenResponse = serde_json::from_str(&body)?;
    println!("Response: {:?}", response);
    Ok(response)
}

fn await_code() -> Result<(String, String), OAuthError> {
    // start an tcp listener to handle redirect
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        return if let Ok(mut stream) = stream {
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
            Ok((code, state))
        } else {
            Err(OAuthError::TcpError)
        }
    }
    return Err(OAuthError::TcpError);
}

fn authentication_url(oauth_client: &OAuthClient) -> String {
    format!(
        "{}?scope={}&client_id={}&state={}&redirect_uri={}&response_type={}",
        oauth_client.auth_url,
        oauth_client.scopes,
        oauth_client.client_id,
        oauth_client.state,
        oauth_client.redirect_uri,
        oauth_client.response_type,
    )
}
