use genai::{
    Client,
    chat::{ChatMessage, ChatOptions, ChatRequest, ChatResponseFormat, JsonSpec},
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WishError {
    #[error("genai error: {0}")]
    GenAiError(#[from] genai::Error),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("custom error: {0}")]
    Custom(&'static str),
    #[error("gemini error: {0}")]
    Gemini(String),
}

impl From<&'static str> for WishError {
    fn from(value: &'static str) -> Self {
        WishError::Custom(value)
    }
}

impl From<String> for WishError {
    fn from(value: String) -> Self {
        WishError::Gemini(value)
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    status: String,
    commands: Vec<String>,
    error_msg: String,
}

pub struct CmdGen {
    client: Client,
    chat_req: ChatRequest,
    chat_options: ChatOptions,
    model: &'static str,
}

impl Default for CmdGen {
    fn default() -> Self {
        Self::new()
    }
}

impl CmdGen {
    pub fn new() -> Self {
        let system = concat!(
            "Your purpose is to help developers run the correct commands in their terminal. Your",
            " response to each of my messages should only be raw JSON respecting the provided schema.",
            " Do not send anything else - just JSON - with the fields: status, error_msg, and commands.",
            " If you feel a request was too vague or unclear, return status 'error'. For example, if I",
            " say 'create a new project' you should return an error, because I did not specify what",
            " kind of project. If there is an error, also include a suitable error message in 'error_msg'",
            " and an empty list for 'commands'. If there is no error, status should be 'ok' and ",
            " 'error_msg' should be an empty string. Do not include any instructions in the list of",
            " commands, only commands that would work. For example, do not include something like",
            " 'cargo new <NAME> # replace <NAME> with the name of your project'. This would not work.",
        );

        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["ok", "error"]
                },
                "commands": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                "error_msg": {
                    "type": "string"
                }
            },
            "required": ["status", "commands", "error_msg"]
        });

        CmdGen {
            client: Client::default(),
            chat_req: ChatRequest::default().with_system(system),
            chat_options: ChatOptions::default().with_response_format(
                ChatResponseFormat::JsonSpec(JsonSpec {
                    name: "commands-format".to_string(),
                    description: None,
                    schema,
                }),
            ),
            model: "gemini-2.5-flash",
        }
    }

    pub async fn generate_commands(&mut self, prompt: String) -> Result<Vec<String>, WishError> {
        let mut chat_req = self
            .chat_req
            .clone()
            .append_message(ChatMessage::user(prompt));

        let chat_res = self
            .client
            .exec_chat(self.model, chat_req.clone(), Some(&self.chat_options))
            .await?;

        let assistant_res = chat_res
            .content_text_into_string()
            .ok_or("error: couldn't get content")?;
        chat_req = chat_req.append_message(ChatMessage::assistant(&assistant_res));

        self.chat_req = chat_req;

        let response: Response = serde_json::from_str(assistant_res.as_str())?;

        match response.status.as_str() {
            "ok" => return Ok(response.commands),
            "error" => return Err(response.error_msg.into()),
            _ => unimplemented!(),
        }
    }
}
