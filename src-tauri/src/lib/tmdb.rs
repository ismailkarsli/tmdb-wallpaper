use std::collections::HashMap;

use reqwest::Error;
use serde::Deserialize;

static API_URL: &str = "https://api.themoviedb.org/3";

pub struct Tmdb {
    pub api_key: String,
    account_id: Option<i32>,
    pub session_id: Option<String>,
}

#[derive(Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
}
#[derive(Deserialize)]
pub struct MovieWatchlistResponse {
    pub results: Vec<Movie>,
}
#[derive(Deserialize)]
pub struct TVShow {
    pub id: i32,
    pub name: String,
}
#[derive(Deserialize)]
pub struct TVWatchlistResponse {
    pub results: Vec<TVShow>,
}

#[derive(Deserialize)]
pub struct Image {
    pub aspect_ratio: f64,
    pub file_path: String,
    pub height: u32,
    pub width: u32,
}
#[derive(Deserialize)]
pub struct ImagesResponse {
    pub backdrops: Vec<Image>,
}

impl Tmdb {
    pub fn new(api_key: String, session_id: Option<String>) -> Self {
        Self {
            api_key,
            account_id: None,
            session_id,
        }
    }

    pub async fn create_request_token(&self) -> String {
        #[derive(Deserialize)]
        struct RequestTokenResponse {
            // success: bool,
            request_token: String,
        }
        let request_token = reqwest::get(format!(
            "{}/authentication/token/new?api_key={}",
            API_URL, self.api_key
        ))
        .await
        .unwrap()
        .json::<RequestTokenResponse>()
        .await
        .unwrap()
        .request_token;

        return request_token;
    }

    pub async fn create_session_id(&mut self, request_token: &str) -> Result<String, Error> {
        #[derive(Deserialize)]
        struct SessionIdResponse {
            // success: bool,
            session_id: String,
        }
        // Body
        let mut request_body = HashMap::new();
        request_body.insert("request_token", request_token);

        let client = reqwest::Client::new();
        let request = client
            .post(format!(
                "https://api.themoviedb.org/3/authentication/session/new?api_key={}",
                self.api_key
            ))
            .json(&request_body)
            .send()
            .await;

        match request {
            Ok(request) => {
                let deserialized = request.json::<SessionIdResponse>().await;
                match deserialized {
                    Ok(deserialized) => {
                        self.session_id = Some(deserialized.session_id.clone());
                        return Ok(deserialized.session_id);
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
                return Err(err);
            }
        }
    }

    async fn get_account_id(&mut self) -> i32 {
        if !self.account_id.is_none() {
            return self.account_id.unwrap();
        } else {
            #[derive(Deserialize)]
            struct AccountIdResponse {
                id: i32,
            }
            let request: AccountIdResponse = reqwest::get(format!(
                "{}/account?api_key={}&session_id={}",
                API_URL,
                self.api_key,
                self.session_id.as_ref().unwrap()
            ))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

            self.account_id = Some(request.id.clone());
            return request.id;
        }
    }

    pub async fn get_movie_watchlist(&mut self) -> MovieWatchlistResponse {
        let account_id = self.get_account_id().await;

        return reqwest::get(format!(
            "{}/account/{}/watchlist/movies?api_key={}&session_id={}",
            API_URL,
            account_id,
            self.api_key,
            self.session_id.as_ref().unwrap()
        ))
        .await
        .unwrap()
        .json::<MovieWatchlistResponse>()
        .await
        .unwrap();
    }

    pub async fn get_tv_watchlist(&mut self) -> TVWatchlistResponse {
        let account_id = self.get_account_id().await;

        return reqwest::get(format!(
            "{}/account/{}/watchlist/tv?api_key={}&session_id={}",
            API_URL,
            account_id,
            self.api_key,
            self.session_id.as_ref().unwrap()
        ))
        .await
        .unwrap()
        .json::<TVWatchlistResponse>()
        .await
        .unwrap();
    }

    pub async fn get_movie_favorites(&mut self) -> MovieWatchlistResponse {
        let account_id = self.get_account_id().await;

        return reqwest::get(format!(
            "{}/account/{}/favorite/movies?api_key={}&session_id={}",
            API_URL,
            account_id,
            self.api_key,
            self.session_id.as_ref().unwrap()
        ))
        .await
        .unwrap()
        .json::<MovieWatchlistResponse>()
        .await
        .unwrap();
    }

    pub async fn get_tv_favorites(&mut self) -> TVWatchlistResponse {
        let account_id = self.get_account_id().await;

        return reqwest::get(format!(
            "{}/account/{}/favorite/tv?api_key={}&session_id={}",
            API_URL,
            account_id,
            self.api_key,
            self.session_id.as_ref().unwrap()
        ))
        .await
        .unwrap()
        .json::<TVWatchlistResponse>()
        .await
        .unwrap();
    }

    pub async fn get_movie_images(
        &self,
        movie_id: i32,
        language: Option<String>,
    ) -> ImagesResponse {
        return reqwest::get(format!(
            "{}/movie/{}/images?api_key={}&include_image_language={}",
            API_URL,
            movie_id,
            self.api_key,
            format!("null,xx,{}", language.unwrap_or("".to_string()))
        ))
        .await
        .unwrap()
        .json::<ImagesResponse>()
        .await
        .unwrap();
    }

    pub async fn get_tv_images(&self, tv_id: i32, language: Option<String>) -> ImagesResponse {
        return reqwest::get(format!(
            "{}/tv/{}/images?api_key={}&include_image_language={}",
            API_URL,
            tv_id,
            self.api_key,
            format!("null,xx,{}", language.unwrap_or("".to_string()))
        ))
        .await
        .unwrap()
        .json::<ImagesResponse>()
        .await
        .unwrap();
    }
}
