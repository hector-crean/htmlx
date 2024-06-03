use color_eyre::eyre;
use dotenv::dotenv;
use open_ai_client::{
    chat::{
        request::{ChatCompletionMessage, ChatCompletionRequest},
        response::ChatCompletionResponse,
    },
    role::OpenAiRole,
    ChatCompletionModel, OpenAiClient, OpenAiClientError,
};
use reqwest::Client;
use secrecy::ExposeSecret;
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};
use tracing::{error, info};
use walkdir::WalkDir;

fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

const ROOT_DIR: &str = r#"C:\Users\Hector.C\rust\htmx\crates\blocks\src\block\icon\icons"#;

fn open_ai_client() -> OpenAiClient {
    dotenv().ok();

    // let open_ai_key = Secret::new(env::var("OPEN_AI_KEY").expect("Failed to get OpenAI key"));
    let open_ai_key = Secret::new(String::from(
    ));

    OpenAiClient::new(open_ai_key)
}

fn split_code_into_chunks(code: &str, chunk_size: usize) -> Vec<String> {
    code.chars()
        .collect::<Vec<char>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

async fn code_refactor(open_ai: &OpenAiClient, code: &str) -> Result<String, OpenAiClientError> {
    //Roughly: 4 chars to a token. We've set the max tokens as 4095
    let chunk_size = 4095 * 4; // Adjust the chunk size as necessary
    let chunks = split_code_into_chunks(code, chunk_size);
    let mut full_response = String::new();

    for chunk in chunks {
        let mut remaining_code = chunk.clone();
        let mut continue_flag = true;

        while continue_flag {
            let msgs = vec![
                ChatCompletionMessage {
                    role: OpenAiRole::System,
                    content: Some(String::from(
                        r#"
                        You are refactoring this svg to only use inline stroke/fill etc, rather than classes. Please do not give any steps. Just refactor the code, and send exclusively that in a message back"#,
                    )),
                    function_call: None,
                    name: None,
                },
                ChatCompletionMessage {
                    role: OpenAiRole::User,
                    content: Some(remaining_code.clone()),
                    function_call: None,
                    name: None,
                },
            ];

            let req = ChatCompletionRequest::new(ChatCompletionModel::GPT4o, msgs)
                .temperature(0.8)
                .max_tokens(4095)
                .top_p(1.)
                .frequency_penalty(0.)
                .presence_penalty(0.);

            let resp = open_ai
                .client
                .post("https://api.openai.com/v1/chat/completions")
                .header(
                    "Authorization",
                    format!("Bearer {}", open_ai.api_key.expose_secret()),
                )
                .header("Content-Type", "application/json")
                .json(&req)
                .send()
                .await?;

            let resp_body = resp.json::<ChatCompletionResponse>().await?;
            if let Some(response_message) = resp_body
                .choices
                .get(0)
                .and_then(|choice| choice.message.content.clone())
            {
                // Append the response and prepare for continuation if necessary
                full_response.push_str(&response_message);
                if response_message.trim().ends_with("...") {
                    remaining_code = response_message.trim().to_string();
                } else {
                    continue_flag = false;
                }
            } else {
                // Handle the case where there is no valid response
                return Err(OpenAiClientError::NoValidResponse);
            }
        }
    }

    Ok(full_response)
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    // let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let open_ai = open_ai_client();

    for entry in WalkDir::new(ROOT_DIR).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_path_buf();

        // Check if the entry is a file with a .ts extension
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("svg") {
            info!("Processing file: {:?}", path);

            match read_file_to_string(&path) {
                Ok(code) => match code_refactor(&open_ai, &code).await {
                    Ok(refactored_code) => {
                        info!("Refactor successful: {:?}", refactored_code);
                        write_string_to_file(path, &refactored_code)?;
                    }

                    Err(e) => {
                        error!("Error in code refactor: {}", e);
                    }
                },
                Err(e) => {
                    error!("Error reading file {:?}: {}", path, e);
                }
            }
        }
    }

    Ok(())
}
