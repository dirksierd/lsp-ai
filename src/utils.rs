use lsp_server::ResponseError;

use crate::{config::ChatMessage, memory_backends::ContextAndCodePrompt};

pub trait ToResponseError {
    fn to_response_error(&self, code: i32) -> ResponseError;
}

impl ToResponseError for anyhow::Error {
    fn to_response_error(&self, code: i32) -> ResponseError {
        ResponseError {
            code,
            message: self.to_string(),
            data: None,
        }
    }
}

pub fn tokens_to_estimated_characters(tokens: usize) -> usize {
    tokens * 4
}

pub fn format_chat_messages(
    messages: &[ChatMessage],
    prompt: &ContextAndCodePrompt,
) -> Vec<ChatMessage> {
    messages
        .iter()
        .map(|m| {
            ChatMessage::new(
                m.role.to_owned(),
                m.content
                    .replace("{CONTEXT}", &prompt.context)
                    .replace("{CODE}", &prompt.code),
            )
        })
        .collect()
}

pub fn format_context_code(context: &str, code: &str) -> String {
    format!("{context}\n\n{code}")
}
