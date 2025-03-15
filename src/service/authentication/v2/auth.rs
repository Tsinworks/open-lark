use crate::core::api_req::ApiRequest;
use crate::core::api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat};
use crate::core::config::Config;
use crate::core::constants::AccessTokenType;
use crate::core::http::Transport;
use crate::core::SDKResult;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone)]
pub struct TokenService {
    config: Config,
}

#[derive(Debug, Serialize)]
struct RequestBody {
    client_id: String,
    client_secret: String,
    grant_type: String,
    code: String,
    redirect_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_in: Option<i32>,
    pub scope: String,
    pub token_type: String,
}

impl ApiResponseTrait for UserToken {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

impl TokenService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn get_user_token(&self, code: impl ToString, redirect_uri: Option<String>) -> SDKResult<BaseResponse<UserToken>> {
        let body = serde_json::to_vec(&RequestBody {
            client_id: self.config.app_id.clone(),
            client_secret: self.config.app_secret.clone(),
            grant_type: "authorization_code".to_string(),
            code: code.to_string(),
            redirect_uri,
        })?;
        let api_req = ApiRequest {
            api_path: "/open-apis/authen/v2/oauth/token".to_string(),
            http_method: reqwest::Method::POST,
            body,
            supported_access_token_types: vec![AccessTokenType::None],
            ..Default::default()
        };
        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }
}
