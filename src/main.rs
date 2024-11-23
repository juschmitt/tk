use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener
};
use reqwest::Url;

fn main() {
    println!("Starting OAuth process...");
    let tt_client_id_str = "O2Mbd1j8nkD7NvNS1R";
    let tt_client_secret_str = "WxRy01)gJDnffZ#R)_Bza2230zY5T7B&";

    let tt_auth_url = "https://ticktick.com/oauth/authorize";
    let tt_token_url = "https://ticktick.com/oauth/token";

   
    let scopes = vec!["tasks:read", "tasks:write"];

    let authorize_url = "https://something.com";

    println!("Visit this URL: {}", authorize_url);

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

            // exchange code for token
            
        
            break;
        }
    }
}
