pub mod chat;
pub mod finish_reason;
pub mod function;
pub mod role;

use chat::{request::ChatCompletionRequest, response::ChatCompletionResponse};

use dotenv::dotenv;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, EnumString, AsRefStr)]
pub enum ChatCompletionModel {
    #[strum(to_string = "gpt-4")]
    GPT4,
    #[strum(to_string = "gpt-4-0613")]
    GPT4_0613,
    #[strum(to_string = "gpt-4-32k")]
    GPT4_32K,
    #[strum(to_string = "gpt-4-32k-0613")]
    GPT4_32K_0613,
    #[strum(to_string = "gpt-3.5-turbo")]
    GPT3_5Turbo,
    #[strum(to_string = "gpt-3.5-turbo-0613")]
    GPT3_5Turbo_0613,
    #[strum(to_string = "gpt-3.5-turbo-16k")]
    GPT3_5Turbo_16K,
    #[strum(to_string = "gpt-3.5-turbo-16k-0613")]
    GPT3_5Turbo_16K_0613,
    #[strum(to_string = "gpt-4o-2024-05-13")]
    GPT4o,
}

#[derive(thiserror::Error, Debug)]
pub enum OpenAiClientError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("no valid response")]
    NoValidResponse,
}
#[derive(Clone)]
pub struct OpenAiClient {
    pub api_key: Secret<String>,
    pub client: reqwest::Client,
}

impl OpenAiClient {
    pub fn new(api_key: Secret<String>) -> Self {
        let client = reqwest::Client::new();

        Self { client, api_key }
    }
    pub fn api_key(self) -> Secret<String> {
        self.api_key
    }
    pub async fn chat(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, OpenAiClientError> {
        // Create a `CreateUser` instance

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.expose_secret()),
            )
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await?
            .json::<ChatCompletionResponse>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    use serde_json::Value;

    use super::{chat::request::ChatCompletionMessage, role::OpenAiRole, *};

    fn open_ai_client() -> OpenAiClient {
        dotenv().ok();

        let open_ai_key = Secret::new(env::var("OPEN_AI_KEY").expect("Failed to get OpenAI key"));

        OpenAiClient::new(open_ai_key)
    }

    #[tokio::test]
    async fn basic_chat() -> Result<(), OpenAiClientError> {
        let open_ai = open_ai_client();

        let msgs = vec![
            ChatCompletionMessage {
                role: OpenAiRole::System,
                content: Some(String::from(
                    r#"
                You are a Socratic tutor. Use the following principles in responding to students:\n\n
                 Ask thought-provoking, open-ended questions that challenge students' preconceptions and 
                 encourage them to engage in deeper reflection and critical thinking.\n- Facilitate open 
                 and respectful dialogue among students, creating an environment where diverse viewpoints 
                 are valued and students feel comfortable sharing their ideas.\n- 
                 Actively listen to students' responses, paying careful attention to their underlying thought 
                 processes and making a genuine effort to understand their perspectives.\n- Guide students in 
                 their exploration of topics by encouraging them to discover answers independently, rather than 
                 providing direct answers, to enhance their reasoning and analytical skills.\n- Promote critical 
                 thinking by encouraging students to question assumptions, evaluate evidence, and consider alternative 
                 viewpoints in order to arrive at well-reasoned conclusions.\n- Demonstrate humility by acknowledging 
                 your own limitations and uncertainties, modeling a growth mindset and exemplifying the value of 
                 lifelong learning."#,
                )),
                function_call: None,
                name: None,
            },
            ChatCompletionMessage {
                role: OpenAiRole::User,
                content: Some(String::from(
                    "Help me to understand the future of artificial intelligence.",
                )),
                function_call: None,
                name: None,
            },
        ];

        let req = ChatCompletionRequest::new(ChatCompletionModel::GPT4o, msgs)
            .temperature(0.8)
            .max_tokens(1024)
            .top_p(1.)
            .frequency_penalty(0.)
            .presence_penalty(0.);

        //Serialize the request to a pretty-printed JSON string and print it
        let req_json = serde_json::to_string_pretty(&req)?;
        println!("Request JSON: {}", req_json);

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

        // Read the response body as a string
        // let resp_body = resp.text().await?;

        // Deserialize the response body and pretty-print it
        // let resp_json = serde_json::to_string_pretty(&resp_body)?;

        let resp_body = resp.json::<ChatCompletionResponse>().await?;

        println!("{:?}", &resp_body);

        Ok(())
    }
}
