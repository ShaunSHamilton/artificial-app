//! Artificial App uses Google Gemini to enact a user browser experience through a LLM.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    env,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct AppState {
    client: Client,
    api_key: String,
    history: Arc<Mutex<Vec<Content>>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiResponse {
    candidates: Vec<Candidate>,
    usage_metadata: UsageMetadata,
    model_version: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Candidate {
    content: Content,
    finish_reason: String,
    safety_ratings: Vec<SafetyRating>,
    avg_logprobs: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct UsageMetadata {
    prompt_token_count: i32,
    candidates_token_count: i32,
    total_token_count: i32,
    prompt_tokens_details: Vec<TokenDetails>,
    candidates_tokens_details: Vec<TokenDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TokenDetails {
    modality: String,
    token_count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct SafetyRating {
    category: String,
    probability: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Prompt {
    system_instruction: SystemInstruction,
    contents: Vec<Content>,
    safety_settings: Vec<SafetySettings>,
    generation_config: GenerationConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SystemInstruction {
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Content {
    parts: Vec<Part>,
    role: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SafetySettings {
    category: String,
    threshold: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GenerationConfig {
    stop_sequences: Vec<String>,
    temperature: f32,
    max_output_tokens: i32,
    top_p: f32,
    top_k: i32,
}

#[derive(Debug, Deserialize)]
struct Interaction {
    element_id: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let client = Client::new();
    let history = Arc::new(Mutex::new(Vec::new()));

    let app_state = AppState {
        client,
        api_key,
        history,
    };

    let app = Router::new()
        .route("/", get(handle_get_index))
        .route("/interaction", get(handle_interaction))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// When any interaction is made with the webpage, this function is called.
async fn handle_interaction(
    State(state): State<AppState>,
    Query(interaction): Query<Interaction>,
) -> Result<Html<String>, (StatusCode, String)> {
    state.history.lock().unwrap().push(Content {
        parts: vec![Part {
            text: interaction.element_id.clone(),
        }],
        role: Some("user".to_string()),
    });

    let prompt = create_prompt(&state.history.lock().unwrap());
    let response = call_gemini_api(&state.client, &state.api_key, &prompt).await?;

    let candidates = response.candidates;

    let html = candidates[0].content.parts[0].text.clone();

    // Remove codeblocks by stripping the first and last line
    let html = html
        .lines()
        .skip(1)
        .take(html.lines().count() - 2)
        .collect::<Vec<&str>>()
        .join("\n");
    state.history.lock().unwrap().push(Content {
        parts: vec![Part { text: html.clone() }],
        role: Some("model".to_string()),
    });
    Ok(Html(html))
}

async fn handle_get_index(State(state): State<AppState>) -> Html<String> {
    state.history.lock().unwrap().clear();
    state.history.lock().unwrap().push(Content {
        parts: vec![Part {
            text: "Start by responding with a random webpage of your choice. Ensure the styling is a nice, dark theme."
                .to_string(),
        }],
        role: Some("user".to_string()),
    });

    let prompt = create_prompt(&state.history.lock().unwrap());

    let response = call_gemini_api(&state.client, &state.api_key, &prompt)
        .await
        .unwrap();
    let html = response.candidates[0].content.parts[0].text.clone();

    // Remove codeblocks by stripping the first and last line
    let html = html
        .lines()
        .skip(1)
        .take(html.lines().count() - 2)
        .collect::<Vec<&str>>()
        .join("\n");
    state.history.lock().unwrap().push(Content {
        parts: vec![Part { text: html.clone() }],
        role: Some("model".to_string()),
    });

    Html(html)
}

async fn call_gemini_api(
    client: &Client,
    api_key: &str,
    prompt: &Prompt,
) -> Result<GeminiResponse, (StatusCode, String)> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    println!("{:#?}", prompt.contents);

    let prompt: String = serde_json::to_string(prompt).unwrap();
    let response = client.post(&url).body(prompt).send().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to call Gemini API: {}", e),
        )
    })?;

    if response.status().is_success() {
        let prompt_response: String = response.text().await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse Gemini API response: {}", e),
            )
        })?;

        let gemini_response = serde_json::from_str(&prompt_response).unwrap();
        Ok(gemini_response)
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Gemini API returned error: {}", response.status()),
        ))
    }
}

const SYSTEM_PROMPT: &str = include_str!("../prompt.txt");

fn create_prompt(history: &Vec<Content>) -> Prompt {
    Prompt {
        system_instruction: SystemInstruction {
            parts: vec![Part {
                text: SYSTEM_PROMPT.to_string(),
            }],
        },
        contents: history.clone(),
        safety_settings: vec![SafetySettings {
            category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
            threshold: "BLOCK_ONLY_HIGH".to_string(),
        }],
        generation_config: GenerationConfig {
            stop_sequences: vec!["Title".to_string()],
            temperature: 1.0,
            max_output_tokens: 800,
            top_p: 0.8,
            top_k: 10,
        },
    }
}
