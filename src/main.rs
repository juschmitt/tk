
mod oauth;

fn main() {
    let accessToken = oauth::start_oauth_process().unwrap();
}
