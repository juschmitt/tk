use uuid::Uuid;

#[derive(Debug)]
pub struct OAuthClient<'oauth> {
    pub client_id: &'oauth str,
    pub client_secret: &'oauth str,
    pub auth_url: &'oauth str,
    pub token_url: &'oauth str,
    pub redirect_uri: &'oauth str,
    pub response_type: &'oauth str,
    pub scopes: Scopes<'oauth>,
    pub state: String,
}

impl <'oauth> OAuthClient<'oauth> {
    pub fn new(
        client_id: &'oauth str,
        client_secret: &'oauth str,
        auth_url: &'oauth str,
        token_url: &'oauth str,
        redirect_uri: &'oauth str,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            auth_url,
            token_url,
            redirect_uri,
            response_type: "code",
            scopes: Scopes::new(vec!["tasks:read", "tasks:write"]),
            state: Uuid::new_v4().to_string(),
        }
    }
}


#[derive(Debug)]
pub struct Scopes<'oauth> {
    scopes: Vec<&'oauth str>
}

impl <'oauth> Scopes<'oauth> {
    pub fn new(scopes: Vec<&'oauth str>) -> Self {
        Self {
            scopes
        }
    }

    pub fn url_encoded(&self) -> String {
        self.scopes.join("%20")
    }

    pub fn join_whitespace(&self) -> String {
        self.scopes.join(" ")
    }
}