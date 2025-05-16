// This is free and unencumbered software released into the public domain.

impl ChatCompletionRequestMessage {
    pub fn role(&self) -> &str {
        match self {
            ChatCompletionRequestMessage::ChatCompletionRequestDeveloperMessage(
                ChatCompletionRequestDeveloperMessage { role, .. },
            ) => role.as_str(),
            ChatCompletionRequestMessage::ChatCompletionRequestSystemMessage(
                ChatCompletionRequestSystemMessage { role, .. },
            ) => role.as_str(),
            ChatCompletionRequestMessage::ChatCompletionRequestUserMessage(
                ChatCompletionRequestUserMessage { role, .. },
            ) => role.as_str(),
            ChatCompletionRequestMessage::ChatCompletionRequestAssistantMessage(
                ChatCompletionRequestAssistantMessage { role, .. },
            ) => role.as_str(),
            ChatCompletionRequestMessage::ChatCompletionRequestToolMessage(
                ChatCompletionRequestToolMessage { role, .. },
            ) => role.as_str(),
            ChatCompletionRequestMessage::ChatCompletionRequestFunctionMessage(
                ChatCompletionRequestFunctionMessage { role, .. },
            ) => role.as_str(),
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            ChatCompletionRequestMessage::ChatCompletionRequestDeveloperMessage(
                ChatCompletionRequestDeveloperMessage { name, .. },
            ) => name.as_ref().map(|s| s.as_str()),
            ChatCompletionRequestMessage::ChatCompletionRequestSystemMessage(
                ChatCompletionRequestSystemMessage { name, .. },
            ) => name.as_ref().map(|s| s.as_str()),
            ChatCompletionRequestMessage::ChatCompletionRequestUserMessage(
                ChatCompletionRequestUserMessage { name, .. },
            ) => name.as_ref().map(|s| s.as_str()),
            ChatCompletionRequestMessage::ChatCompletionRequestAssistantMessage(
                ChatCompletionRequestAssistantMessage { name, .. },
            ) => name.as_ref().map(|s| s.as_str()),
            ChatCompletionRequestMessage::ChatCompletionRequestToolMessage(..) => return None,
            ChatCompletionRequestMessage::ChatCompletionRequestFunctionMessage(
                ChatCompletionRequestFunctionMessage { name, .. },
            ) => {
                if name.as_str().is_empty() {
                    None
                } else {
                    Some(name.as_str())
                }
            }
        }
    }

    pub fn text_content(&self) -> Option<&str> {
        match self {
            ChatCompletionRequestMessage::ChatCompletionRequestDeveloperMessage(
                ChatCompletionRequestDeveloperMessage { content, .. },
            ) => content.text_content(),
            ChatCompletionRequestMessage::ChatCompletionRequestSystemMessage(
                ChatCompletionRequestSystemMessage { content, .. },
            ) => content.text_content(),
            ChatCompletionRequestMessage::ChatCompletionRequestUserMessage(
                ChatCompletionRequestUserMessage { content, .. },
            ) => content.text_content(),
            ChatCompletionRequestMessage::ChatCompletionRequestAssistantMessage(
                ChatCompletionRequestAssistantMessage { content, .. },
            ) => content.as_ref().and_then(|c| c.text_content()),
            ChatCompletionRequestMessage::ChatCompletionRequestToolMessage(
                ChatCompletionRequestToolMessage { content, .. },
            ) => content.text_content(),
            ChatCompletionRequestMessage::ChatCompletionRequestFunctionMessage(
                ChatCompletionRequestFunctionMessage { content, .. },
            ) => content.as_ref().map(|s| s.as_str()),
        }
    }
}

impl ChatCompletionRequestDeveloperMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestDeveloperMessage_Content::*;
        match self {
            String(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}

impl ChatCompletionRequestSystemMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestSystemMessage_Content::*;
        match self {
            String(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}

impl ChatCompletionRequestUserMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestUserMessage_Content::*;
        match self {
            String(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}

impl ChatCompletionRequestAssistantMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestAssistantMessage_Content::*;
        match self {
            String(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}

impl ChatCompletionRequestToolMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestToolMessage_Content::*;
        match self {
            String(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}
