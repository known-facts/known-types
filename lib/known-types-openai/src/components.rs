// This is free and unencumbered software released into the public domain.

//! OpenAI API components

use crate::prelude::{String, Vec};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddUploadPartRequest {
    /// The chunk of bytes for this Part.
    pub r#data: String,
}

/// Represents an individual Admin API key in an org.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdminApiKey {
    /// The object type, which is always `organization.admin_api_key`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the API key
    pub r#name: String,

    /// The redacted value of the API key
    pub r#redacted_value: String,

    /// The value of the API key.
    pub r#value: String,

    /// The Unix timestamp (in seconds) of when the API key was created
    pub r#created_at: i64,

    /// The Unix timestamp (in seconds) of when the API key was last used
    pub r#last_used_at: Option<i64>,

    pub r#owner: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Annotation {
    FileCitationBody(FileCitationBody),

    UrlCitationBody(UrlCitationBody),

    FilePath(FilePath),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApiKeyList {
    pub r#object: String,

    pub r#data: Vec<AdminApiKey>,

    pub r#has_more: bool,

    pub r#first_id: String,

    pub r#last_id: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApproximateLocation {
    /// The type of location approximation.
    pub r#type: String,

    pub r#country: (/*AnyOf*/),

    pub r#region: (/*AnyOf*/),

    pub r#city: (/*AnyOf*/),

    pub r#timezone: (/*AnyOf*/),
}

/// Represents an `assistant` that can call the model and use tools.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `assistant`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the assistant was created.
    pub r#created_at: i64,

    /// The name of the assistant.
    pub r#name: Option<String>,

    /// The description of the assistant.
    pub r#description: Option<String>,

    /// ID of the model to use.
    pub r#model: String,

    /// The system instructions that the assistant uses.
    pub r#instructions: Option<String>,

    /// A list of tool enabled on the assistant.
    pub r#tools: Vec<(/*OneOf*/)>,

    /// A set of resources that are used by the assistant's tools.
    pub r#tool_resources: Option<(/*Object*/)>,

    pub r#metadata: Metadata,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

/// Represents an event emitted when streaming a Run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssistantStreamEvent {
    ThreadStreamEvent(ThreadStreamEvent),

    RunStreamEvent(RunStreamEvent),

    RunStepStreamEvent(RunStepStreamEvent),

    MessageStreamEvent(MessageStreamEvent),

    ErrorEvent(ErrorEvent),

    DoneEvent(DoneEvent),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantSupportedModels(pub String);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantToolsCode {
    /// The type of tool being defined: `code_interpreter`
    pub r#type: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantToolsFileSearch {
    /// The type of tool being defined: `file_search`
    pub r#type: String,

    /// Overrides for the file search tool.
    pub r#file_search: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantToolsFileSearchTypeOnly {
    /// The type of tool being defined: `file_search`
    pub r#type: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantToolsFunction {
    /// The type of tool being defined: `function`
    pub r#type: String,

    pub r#function: FunctionObject,
}

/// Specifies the format that the model must output.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssistantsApiResponseFormatOption {
    /// `auto` is the default value
    String(String),

    ResponseFormatText(ResponseFormatText),

    ResponseFormatJsonObject(ResponseFormatJsonObject),

    ResponseFormatJsonSchema(ResponseFormatJsonSchema),
}

/// Controls which (if any) tool is called by the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssistantsApiToolChoiceOption {
    /// `none` means the model will not call any tools and instead generates a
    /// message.
    String(String),

    AssistantsNamedToolChoice(AssistantsNamedToolChoice),
}

/// Specifies a tool the model should use.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssistantsNamedToolChoice {
    /// The type of the tool.
    pub r#type: String,

    pub r#function: (/*Object*/),
}

/// The format of the output, in one of these options: `json`, `text`, `srt`,
/// `verbose_json`, or `vtt`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AudioResponseFormat(pub String);

/// A log of a user action or configuration change within this organization.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLog {
    /// The ID of this log.
    pub r#id: String,

    pub r#type: AuditLogEventType,

    /// The Unix timestamp (in seconds) of the event.
    pub r#effective_at: i64,

    /// The project that the action was scoped to.
    pub r#project: (/*Object*/),

    pub r#actor: AuditLogActor,

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "api_key.created"))]
    pub r#api_key_created: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "api_key.updated"))]
    pub r#api_key_updated: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "api_key.deleted"))]
    pub r#api_key_deleted: (/*Object*/),

    /// The project and fine-tuned model checkpoint that the checkpoint
    /// permission was created for.
    #[cfg_attr(feature = "serde", serde(rename = "checkpoint_permission.created"))]
    pub r#checkpoint_permission_created: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "checkpoint_permission.deleted"))]
    pub r#checkpoint_permission_deleted: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "invite.sent"))]
    pub r#invite_sent: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "invite.accepted"))]
    pub r#invite_accepted: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "invite.deleted"))]
    pub r#invite_deleted: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "login.failed"))]
    pub r#login_failed: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "logout.failed"))]
    pub r#logout_failed: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "organization.updated"))]
    pub r#organization_updated: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "project.created"))]
    pub r#project_created: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "project.updated"))]
    pub r#project_updated: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "project.archived"))]
    pub r#project_archived: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "rate_limit.updated"))]
    pub r#rate_limit_updated: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "rate_limit.deleted"))]
    pub r#rate_limit_deleted: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "service_account.created"))]
    pub r#service_account_created: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "service_account.updated"))]
    pub r#service_account_updated: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "service_account.deleted"))]
    pub r#service_account_deleted: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "user.added"))]
    pub r#user_added: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "user.updated"))]
    pub r#user_updated: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "user.deleted"))]
    pub r#user_deleted: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "certificate.created"))]
    pub r#certificate_created: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "certificate.updated"))]
    pub r#certificate_updated: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "certificate.deleted"))]
    pub r#certificate_deleted: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "certificates.activated"))]
    pub r#certificates_activated: (/*Object*/),

    /// The details for events with this `type`.
    #[cfg_attr(feature = "serde", serde(rename = "certificates.deactivated"))]
    pub r#certificates_deactivated: (/*Object*/),
}

/// The actor who performed the audit logged action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActor {
    /// The type of actor.
    pub r#type: String,

    pub r#session: AuditLogActorSession,

    pub r#api_key: AuditLogActorApiKey,
}

/// The API Key used to perform the audit logged action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActorApiKey {
    /// The tracking id of the API key.
    pub r#id: String,

    /// The type of API key.
    pub r#type: String,

    pub r#user: AuditLogActorUser,

    pub r#service_account: AuditLogActorServiceAccount,
}

/// The service account that performed the audit logged action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActorServiceAccount {
    /// The service account id.
    pub r#id: String,
}

/// The session in which the audit logged action was performed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActorSession {
    pub r#user: AuditLogActorUser,

    /// The IP address from which the action was performed.
    pub r#ip_address: String,
}

/// The user who performed the audit logged action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogActorUser {
    /// The user id.
    pub r#id: String,

    /// The user email.
    pub r#email: String,
}

/// The event type.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuditLogEventType(pub String);

/// The default strategy.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AutoChunkingStrategyRequestParam {
    /// Always `auto`.
    pub r#type: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Batch {
    pub r#id: String,

    /// The object type, which is always `batch`.
    pub r#object: String,

    /// The OpenAI API endpoint used by the batch.
    pub r#endpoint: String,

    pub r#errors: (/*Object*/),

    /// The ID of the input file for the batch.
    pub r#input_file_id: String,

    /// The time frame within which the batch should be processed.
    pub r#completion_window: String,

    /// The current status of the batch.
    pub r#status: String,

    /// The ID of the file containing the outputs of successfully executed
    /// requests.
    pub r#output_file_id: String,

    /// The ID of the file containing the outputs of requests with errors.
    pub r#error_file_id: String,

    /// The Unix timestamp (in seconds) for when the batch was created.
    pub r#created_at: i64,

    /// The Unix timestamp (in seconds) for when the batch started processing.
    pub r#in_progress_at: i64,

    /// The Unix timestamp (in seconds) for when the batch will expire.
    pub r#expires_at: i64,

    /// The Unix timestamp (in seconds) for when the batch started finalizing.
    pub r#finalizing_at: i64,

    /// The Unix timestamp (in seconds) for when the batch was completed.
    pub r#completed_at: i64,

    /// The Unix timestamp (in seconds) for when the batch failed.
    pub r#failed_at: i64,

    /// The Unix timestamp (in seconds) for when the batch expired.
    pub r#expired_at: i64,

    /// The Unix timestamp (in seconds) for when the batch started cancelling.
    pub r#cancelling_at: i64,

    /// The Unix timestamp (in seconds) for when the batch was cancelled.
    pub r#cancelled_at: i64,

    /// The request counts for different statuses within the batch.
    pub r#request_counts: (/*Object*/),

    pub r#metadata: Metadata,
}

/// The per-line object of the batch input file
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchRequestInput {
    /// A developer-provided per-request id that will be used to match outputs
    /// to inputs.
    pub r#custom_id: String,

    /// The HTTP method to be used for the request.
    pub r#method: String,

    /// The OpenAI API relative URL to be used for the request.
    pub r#url: String,
}

/// The per-line object of the batch output and error files
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BatchRequestOutput {
    pub r#id: String,

    /// A developer-provided per-request id that will be used to match outputs
    /// to inputs.
    pub r#custom_id: String,

    pub r#response: Option<(/*Object*/)>,

    /// For requests that failed with a non-HTTP error, this will contain more
    /// information on the cause of the failure.
    pub r#error: Option<(/*Object*/)>,
}

/// Represents an individual `certificate` uploaded to the organization.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Certificate {
    /// The object type.
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the certificate.
    pub r#name: String,

    /// The Unix timestamp (in seconds) of when the certificate was uploaded.
    pub r#created_at: i64,

    pub r#certificate_details: (/*Object*/),

    /// Whether the certificate is currently active at the specified scope.
    pub r#active: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionDeleted {
    /// The type of object being deleted.
    pub r#object: String,

    /// The ID of the chat completion that was deleted.
    pub r#id: String,

    /// Whether the chat completion was deleted.
    pub r#deleted: bool,
}

/// Specifying a particular function via `{"name": "my_function"}` forces the
/// model to call that function.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionFunctionCallOption {
    /// The name of the function to call.
    pub r#name: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionFunctions {
    /// A description of what the function does, used by the model to choose
    /// when and how to call the function.
    pub r#description: String,

    /// The name of the function to be called.
    pub r#name: String,

    pub r#parameters: FunctionParameters,
}

/// An object representing a list of Chat Completions.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionList {
    /// The type of this object.
    pub r#object: String,

    /// An array of chat completion objects.
    pub r#data: Vec<CreateChatCompletionResponse>,

    /// The identifier of the first chat completion in the data array.
    pub r#first_id: String,

    /// The identifier of the last chat completion in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more Chat Completions available.
    pub r#has_more: bool,
}

/// An object representing a list of chat completion messages.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageList {
    /// The type of this object.
    pub r#object: String,

    /// An array of chat completion message objects.
    pub r#data: Vec<(/*AllOf*/)>,

    /// The identifier of the first chat message in the data array.
    pub r#first_id: String,

    /// The identifier of the last chat message in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more chat messages available.
    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageToolCall {
    /// The ID of the tool call.
    pub r#id: String,

    /// The type of the tool.
    pub r#type: String,

    /// The function that the model called.
    pub r#function: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageToolCallChunk {
    pub r#index: i64,

    /// The ID of the tool call.
    pub r#id: String,

    /// The type of the tool.
    pub r#type: String,

    pub r#function: (/*Object*/),
}

/// The tool calls generated by the model, such as function calls.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionMessageToolCalls(pub Vec<ChatCompletionMessageToolCall>);

/// Output types that you would like the model to generate for this request.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionModalities(pub Vec<String>);

/// Specifies a tool the model should use.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionNamedToolChoice {
    /// The type of the tool.
    pub r#type: String,

    pub r#function: (/*Object*/),
}

/// Messages sent by the model in response to user messages.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestAssistantMessage {
    /// The contents of the assistant message.
    pub r#content: Option<(/*OneOf*/)>,

    /// The refusal message by the assistant.
    pub r#refusal: Option<String>,

    /// The role of the messages author, in this case `assistant`.
    pub r#role: String,

    /// An optional name for the participant.
    pub r#name: String,

    /// Data about a previous audio response from the model.
    pub r#audio: Option<(/*Object*/)>,

    pub r#tool_calls: ChatCompletionMessageToolCalls,

    /// Deprecated and replaced by `tool_calls`.
    pub r#function_call: Option<(/*Object*/)>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChatCompletionRequestAssistantMessageContentPart {
    ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),

    ChatCompletionRequestMessageContentPartRefusal(ChatCompletionRequestMessageContentPartRefusal),
}

/// Developer-provided instructions that the model should follow, regardless of
/// messages sent by the user.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestDeveloperMessage {
    /// The contents of the developer message.
    pub r#content: (/*OneOf*/),

    /// The role of the messages author, in this case `developer`.
    pub r#role: String,

    /// An optional name for the participant.
    pub r#name: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestFunctionMessage {
    /// The role of the messages author, in this case `function`.
    pub r#role: String,

    /// The contents of the function message.
    pub r#content: Option<String>,

    /// The name of the function to call.
    pub r#name: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChatCompletionRequestMessage {
    ChatCompletionRequestDeveloperMessage(ChatCompletionRequestDeveloperMessage),

    ChatCompletionRequestSystemMessage(ChatCompletionRequestSystemMessage),

    ChatCompletionRequestUserMessage(ChatCompletionRequestUserMessage),

    ChatCompletionRequestAssistantMessage(ChatCompletionRequestAssistantMessage),

    ChatCompletionRequestToolMessage(ChatCompletionRequestToolMessage),

    ChatCompletionRequestFunctionMessage(ChatCompletionRequestFunctionMessage),
}

/// Learn about [audio inputs](/docs/guides/audio).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartAudio {
    /// The type of the content part.
    pub r#type: String,

    pub r#input_audio: (/*Object*/),
}

/// Learn about [file inputs](/docs/guides/text) for text generation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartFile {
    /// The type of the content part.
    pub r#type: String,

    pub r#file: (/*Object*/),
}

/// Learn about [image inputs](/docs/guides/vision).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartImage {
    /// The type of the content part.
    pub r#type: String,

    pub r#image_url: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartRefusal {
    /// The type of the content part.
    pub r#type: String,

    /// The refusal message generated by the model.
    pub r#refusal: String,
}

/// Learn about [text inputs](/docs/guides/text-generation).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestMessageContentPartText {
    /// The type of the content part.
    pub r#type: String,

    /// The text content.
    pub r#text: String,
}

/// Developer-provided instructions that the model should follow, regardless of
/// messages sent by the user.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestSystemMessage {
    /// The contents of the system message.
    pub r#content: (/*OneOf*/),

    /// The role of the messages author, in this case `system`.
    pub r#role: String,

    /// An optional name for the participant.
    pub r#name: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChatCompletionRequestSystemMessageContentPart {
    ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestToolMessage {
    /// The role of the messages author, in this case `tool`.
    pub r#role: String,

    /// The contents of the tool message.
    pub r#content: (/*OneOf*/),

    /// Tool call that this message is responding to.
    pub r#tool_call_id: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChatCompletionRequestToolMessageContentPart {
    ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
}

/// Messages sent by an end user, containing prompts or additional context
/// information.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRequestUserMessage {
    /// The contents of the user message.
    pub r#content: (/*OneOf*/),

    /// The role of the messages author, in this case `user`.
    pub r#role: String,

    /// An optional name for the participant.
    pub r#name: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChatCompletionRequestUserMessageContentPart {
    ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),

    ChatCompletionRequestMessageContentPartImage(ChatCompletionRequestMessageContentPartImage),

    ChatCompletionRequestMessageContentPartAudio(ChatCompletionRequestMessageContentPartAudio),

    ChatCompletionRequestMessageContentPartFile(ChatCompletionRequestMessageContentPartFile),
}

/// A chat completion message generated by the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionResponseMessage {
    /// The contents of the message.
    pub r#content: Option<String>,

    /// The refusal message generated by the model.
    pub r#refusal: Option<String>,

    pub r#tool_calls: ChatCompletionMessageToolCalls,

    /// Annotations for the message, when applicable, as when using the [web
    /// search tool](/docs/guides/tools-web-search?api-mode=chat).
    pub r#annotations: Vec<(/*Object*/)>,

    /// The role of the author of this message.
    pub r#role: String,

    /// Deprecated and replaced by `tool_calls`.
    pub r#function_call: (/*Object*/),

    /// If the audio output modality is requested, this object contains data
    /// about the audio response from the model.
    pub r#audio: Option<(/*Object*/)>,
}

/// The role of the author of a message
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionRole(pub String);

/// Options for streaming response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionStreamOptions {
    /// If set, an additional chunk will be streamed before the `data: [DONE]`
    /// message.
    pub r#include_usage: bool,
}

/// A chat completion delta generated by streamed model responses.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionStreamResponseDelta {
    /// The contents of the chunk message.
    pub r#content: Option<String>,

    /// Deprecated and replaced by `tool_calls`.
    pub r#function_call: (/*Object*/),

    pub r#tool_calls: Vec<ChatCompletionMessageToolCallChunk>,

    /// The role of the author of this message.
    pub r#role: String,

    /// The refusal message generated by the model.
    pub r#refusal: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionTokenLogprob {
    /// The token.
    pub r#token: String,

    /// The log probability of this token, if it is within the top 20 most
    /// likely tokens.
    pub r#logprob: f64,

    /// A list of integers representing the UTF-8 bytes representation of the
    /// token.
    pub r#bytes: Option<Vec<i64>>,

    /// List of the most likely tokens and their log probability, at this token
    /// position.
    pub r#top_logprobs: Vec<(/*Object*/)>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChatCompletionTool {
    /// The type of the tool.
    pub r#type: String,

    pub r#function: FunctionObject,
}

/// Controls which (if any) tool is called by the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChatCompletionToolChoiceOption {
    /// `none` means the model will not call any tool and instead generates a
    /// message.
    String(String),

    ChatCompletionNamedToolChoice(ChatCompletionNamedToolChoice),
}

/// The chunking strategy used to chunk the file(s).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChunkingStrategyRequestParam;

/// A click action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Click {
    /// Specifies the event type.
    pub r#type: String,

    /// Indicates which mouse button was pressed during the click.
    pub r#button: String,

    /// The x-coordinate where the click occurred.
    pub r#x: i64,

    /// The y-coordinate where the click occurred.
    pub r#y: i64,
}

/// The output of a code interpreter tool call that is a file.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeInterpreterFileOutput {
    /// The type of the code interpreter file output.
    pub r#type: String,

    pub r#files: Vec<(/*Object*/)>,
}

/// The output of a code interpreter tool call that is text.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeInterpreterTextOutput {
    /// The type of the code interpreter text output.
    pub r#type: String,

    /// The logs of the code interpreter tool call.
    pub r#logs: String,
}

/// A tool call to run code.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeInterpreterToolCall {
    /// The unique ID of the code interpreter tool call.
    pub r#id: String,

    /// The type of the code interpreter tool call.
    pub r#type: String,

    /// The code to run.
    pub r#code: String,

    /// The status of the code interpreter tool call.
    pub r#status: String,

    /// The results of the code interpreter tool call.
    pub r#results: Vec<CodeInterpreterToolOutput>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CodeInterpreterToolOutput {
    CodeInterpreterTextOutput(CodeInterpreterTextOutput),

    CodeInterpreterFileOutput(CodeInterpreterFileOutput),
}

/// A filter used to compare a specified attribute key to a given value using a
/// defined comparison operation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComparisonFilter {
    /// Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.
    pub r#type: String,

    /// The key to compare against the value.
    pub r#key: String,

    /// The value to compare against the attribute key; supports string, number,
    /// or boolean types.
    pub r#value: (/*OneOf*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompleteUploadRequest {
    /// The ordered list of Part IDs.
    pub r#part_ids: Vec<String>,

    /// The optional md5 checksum for the file contents to verify if the bytes
    /// uploaded matches what you expect.
    pub r#md5: String,
}

/// Usage statistics for the completion request.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompletionUsage {
    /// Number of tokens in the generated completion.
    pub r#completion_tokens: i64,

    /// Number of tokens in the prompt.
    pub r#prompt_tokens: i64,

    /// Total number of tokens used in the request (prompt + completion).
    pub r#total_tokens: i64,

    /// Breakdown of tokens used in a completion.
    pub r#completion_tokens_details: (/*Object*/),

    /// Breakdown of tokens used in the prompt.
    pub r#prompt_tokens_details: (/*Object*/),
}

/// Combine multiple filters using `and` or `or`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompoundFilter {
    /// Type of operation: `and` or `or`.
    pub r#type: String,

    /// Array of filters to combine.
    pub r#filters: Vec<(/*OneOf*/)>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ComputerAction {
    Click(Click),

    DoubleClick(DoubleClick),

    Drag(Drag),

    KeyPress(KeyPress),

    Move(Move),

    Screenshot(Screenshot),

    Scroll(Scroll),

    Type(Type),

    Wait(Wait),
}

/// The output of a computer tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerCallOutputItemParam {
    pub r#id: (/*AnyOf*/),

    /// The ID of the computer tool call that produced the output.
    pub r#call_id: String,

    /// The type of the computer tool call output.
    pub r#type: String,

    pub r#output: ComputerScreenshotImage,

    pub r#acknowledged_safety_checks: (/*AnyOf*/),

    pub r#status: (/*AnyOf*/),
}

/// A pending safety check for the computer call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerCallSafetyCheckParam {
    /// The ID of the pending safety check.
    pub r#id: String,

    pub r#code: (/*AnyOf*/),

    pub r#message: (/*AnyOf*/),
}

/// A computer screenshot image used with the computer use tool.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerScreenshotImage {
    /// Specifies the event type.
    pub r#type: String,

    /// The URL of the screenshot image.
    pub r#image_url: String,

    /// The identifier of an uploaded file that contains the screenshot.
    pub r#file_id: String,
}

/// A tool call to a computer use tool.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerToolCall {
    /// The type of the computer call.
    pub r#type: String,

    /// The unique ID of the computer call.
    pub r#id: String,

    /// An identifier used when responding to the tool call with output.
    pub r#call_id: String,

    pub r#action: ComputerAction,

    /// The pending safety checks for the computer call.
    pub r#pending_safety_checks: Vec<ComputerToolCallSafetyCheck>,

    /// The status of the item.
    pub r#status: String,
}

/// The output of a computer tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerToolCallOutput {
    /// The type of the computer tool call output.
    pub r#type: String,

    /// The ID of the computer tool call output.
    pub r#id: String,

    /// The ID of the computer tool call that produced the output.
    pub r#call_id: String,

    /// The safety checks reported by the API that have been acknowledged by the
    /// developer.
    pub r#acknowledged_safety_checks: Vec<ComputerToolCallSafetyCheck>,

    pub r#output: ComputerScreenshotImage,

    /// The status of the message input.
    pub r#status: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerToolCallOutputResource(pub (/*AllOf*/));

/// A pending safety check for the computer call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerToolCallSafetyCheck {
    /// The ID of the pending safety check.
    pub r#id: String,

    /// The type of the pending safety check.
    pub r#code: String,

    /// Details about the pending safety check.
    pub r#message: String,
}

/// A tool that controls a virtual computer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputerUsePreviewTool {
    /// The type of the computer use tool.
    pub r#type: String,

    /// The type of computer environment to control.
    pub r#environment: String,

    /// The width of the computer display.
    pub r#display_width: i64,

    /// The height of the computer display.
    pub r#display_height: i64,
}

/// Multi-modal input and output contents.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Content {
    InputContent(InputContent),

    OutputContent(OutputContent),
}

/// An x/y coordinate pair, e.g.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Coordinate {
    /// The x-coordinate.
    pub r#x: i64,

    /// The y-coordinate.
    pub r#y: i64,
}

/// The aggregated costs details of the specific time bucket.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CostsResult {
    pub r#object: String,

    /// The monetary value in its associated currency.
    pub r#amount: (/*Object*/),

    /// When `group_by=line_item`, this field provides the line item of the
    /// grouped costs result.
    pub r#line_item: Option<String>,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped costs result.
    pub r#project_id: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateAssistantRequest {
    /// ID of the model to use.
    pub r#model: (/*AnyOf*/),

    /// The name of the assistant.
    pub r#name: Option<String>,

    /// The description of the assistant.
    pub r#description: Option<String>,

    /// The system instructions that the assistant uses.
    pub r#instructions: Option<String>,

    pub r#reasoning_effort: ReasoningEffort,

    /// A list of tool enabled on the assistant.
    pub r#tools: Vec<(/*OneOf*/)>,

    /// A set of resources that are used by the assistant's tools.
    pub r#tool_resources: Option<(/*Object*/)>,

    pub r#metadata: Metadata,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionRequest(pub (/*AllOf*/));

/// Represents a chat completion response returned by model, based on the
/// provided input.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionResponse {
    /// A unique identifier for the chat completion.
    pub r#id: String,

    /// A list of chat completion choices.
    pub r#choices: Vec<(/*Object*/)>,

    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub r#created: i64,

    /// The model used for the chat completion.
    pub r#model: String,

    pub r#service_tier: ServiceTier,

    /// This fingerprint represents the backend configuration that the model
    /// runs with.
    pub r#system_fingerprint: String,

    /// The object type, which is always `chat.completion`.
    pub r#object: String,

    pub r#usage: CompletionUsage,
}

/// Represents a streamed chunk of a chat completion response returned by the
/// model, based on the provided input.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateChatCompletionStreamResponse {
    /// A unique identifier for the chat completion.
    pub r#id: String,

    /// A list of chat completion choices.
    pub r#choices: Vec<(/*Object*/)>,

    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub r#created: i64,

    /// The model to generate the completion.
    pub r#model: String,

    pub r#service_tier: ServiceTier,

    /// This fingerprint represents the backend configuration that the model
    /// runs with.
    pub r#system_fingerprint: String,

    /// The object type, which is always `chat.completion.chunk`.
    pub r#object: String,

    /// An optional field that will only be present when you set
    /// `stream_options: {"include_usage": true}` in your request.
    pub r#usage: Option<CompletionUsage>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCompletionRequest {
    /// ID of the model to use.
    pub r#model: (/*AnyOf*/),

    /// The prompt(s) to generate completions for, encoded as a string, array of
    /// strings, array of tokens, or array of token arrays.
    pub r#prompt: Option<(/*OneOf*/)>,

    /// Generates `best_of` completions server-side and returns the "best" (the
    /// one with the highest log probability per token).
    pub r#best_of: Option<i64>,

    /// Echo back the prompt in addition to the completion
    pub r#echo: Option<bool>,

    /// Number between -2.0 and 2.0.
    pub r#frequency_penalty: Option<f64>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    pub r#logit_bias: Option<(/*Object*/)>,

    /// Include the log probabilities on the `logprobs` most likely output
    /// tokens, as well the chosen tokens.
    pub r#logprobs: Option<i64>,

    /// The maximum number of [tokens](/tokenizer) that can be generated in the
    /// completion.
    pub r#max_tokens: Option<i64>,

    /// How many completions to generate for each prompt.
    pub r#n: Option<i64>,

    /// Number between -2.0 and 2.0.
    pub r#presence_penalty: Option<f64>,

    /// If specified, our system will make a best effort to sample
    /// deterministically, such that repeated requests with the same `seed` and
    /// parameters should return the same result.
    pub r#seed: Option<i64>,

    pub r#stop: StopConfiguration,

    /// Whether to stream back partial progress.
    pub r#stream: Option<bool>,

    pub r#stream_options: ChatCompletionStreamOptions,

    /// The suffix that comes after a completion of inserted text.
    pub r#suffix: Option<String>,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: String,
}

/// Represents a completion response from the API.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCompletionResponse {
    /// A unique identifier for the completion.
    pub r#id: String,

    /// The list of completion choices the model generated for the input prompt.
    pub r#choices: Vec<(/*Object*/)>,

    /// The Unix timestamp (in seconds) of when the completion was created.
    pub r#created: i64,

    /// The model used for completion.
    pub r#model: String,

    /// This fingerprint represents the backend configuration that the model
    /// runs with.
    pub r#system_fingerprint: String,

    /// The object type, which is always "text_completion"
    pub r#object: String,

    pub r#usage: CompletionUsage,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEmbeddingRequest {
    /// Input text to embed, encoded as a string or array of tokens.
    pub r#input: (/*OneOf*/),

    /// ID of the model to use.
    pub r#model: (/*AnyOf*/),

    /// The format to return the embeddings in.
    pub r#encoding_format: String,

    /// The number of dimensions the resulting output embeddings should have.
    pub r#dimensions: i64,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEmbeddingResponse {
    /// The list of embeddings generated by the model.
    pub r#data: Vec<Embedding>,

    /// The name of the model used to generate the embedding.
    pub r#model: String,

    /// The object type, which is always "list".
    pub r#object: String,

    /// The usage information for the request.
    pub r#usage: (/*Object*/),
}

/// A CompletionsRunDataSource object describing a model sampling configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalCompletionsRunDataSource {
    /// The type of run data source.
    pub r#type: String,

    pub r#input_messages: (/*OneOf*/),

    pub r#sampling_params: (/*Object*/),

    /// The name of the model to use for generating completions (e.g.
    pub r#model: String,

    pub r#source: (/*OneOf*/),
}

/// A CustomDataSourceConfig object that defines the schema for the data source
/// used for the evaluation runs.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalCustomDataSourceConfig {
    /// The type of data source.
    pub r#type: String,

    /// The json schema for each row in the data source.
    pub r#item_schema: (/*Object*/),

    /// Whether the eval should expect you to populate the sample namespace (ie,
    /// by generating responses off of your data source)
    pub r#include_sample_schema: bool,
}

/// A chat message that makes up the prompt or context.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalItem;

/// A JsonlRunDataSource object with that specifies a JSONL file that matches
/// the eval
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalJsonlRunDataSource {
    /// The type of data source.
    pub r#type: String,

    pub r#source: (/*OneOf*/),
}

/// A LabelModelGrader object which uses a model to assign labels to each item
/// in the evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalLabelModelGrader {
    /// The object type, which is always `label_model`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The model to use for the evaluation.
    pub r#model: String,

    /// A list of chat messages forming the prompt or context.
    pub r#input: Vec<CreateEvalItem>,

    /// The labels to classify to each item in the evaluation.
    pub r#labels: Vec<String>,

    /// The labels that indicate a passing result.
    pub r#passing_labels: Vec<String>,
}

/// A data source config which specifies the metadata property of your stored
/// completions query.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalLogsDataSourceConfig {
    /// The type of data source.
    pub r#type: String,

    /// Metadata filters for the logs data source.
    pub r#metadata: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalRequest {
    /// The name of the evaluation.
    pub r#name: String,

    pub r#metadata: Metadata,

    /// The configuration for the data source used for the evaluation runs.
    pub r#data_source_config: (/*OneOf*/),

    /// A list of graders for all eval runs in this group.
    pub r#testing_criteria: Vec<(/*OneOf*/)>,
}

/// A ResponsesRunDataSource object describing a model sampling configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalResponsesRunDataSource {
    /// The type of run data source.
    pub r#type: String,

    pub r#input_messages: (/*OneOf*/),

    pub r#sampling_params: (/*Object*/),

    /// The name of the model to use for generating completions (e.g.
    pub r#model: String,

    pub r#source: (/*OneOf*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateEvalRunRequest {
    /// The name of the run.
    pub r#name: String,

    pub r#metadata: Metadata,

    /// Details about the run's data source.
    pub r#data_source: (/*OneOf*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFileRequest {
    /// The File object (not file name) to be uploaded.
    pub r#file: String,

    /// The intended purpose of the uploaded file.
    pub r#purpose: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFineTuningCheckpointPermissionRequest {
    /// The project identifiers to grant access to.
    pub r#project_ids: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateFineTuningJobRequest {
    /// The name of the model to fine-tune.
    pub r#model: (/*AnyOf*/),

    /// The ID of an uploaded file that contains training data.
    pub r#training_file: String,

    /// The hyperparameters used for the fine-tuning job.
    pub r#hyperparameters: (/*Object*/),

    /// A string of up to 64 characters that will be added to your fine-tuned
    /// model name.
    pub r#suffix: Option<String>,

    /// The ID of an uploaded file that contains validation data.
    pub r#validation_file: Option<String>,

    /// A list of integrations to enable for your fine-tuning job.
    pub r#integrations: Option<Vec<(/*Object*/)>>,

    /// The seed controls the reproducibility of the job.
    pub r#seed: Option<i64>,

    pub r#method: FineTuneMethod,

    pub r#metadata: Metadata,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateImageEditRequest {
    /// The image(s) to edit.
    pub r#image: (/*AnyOf*/),

    /// A text description of the desired image(s).
    pub r#prompt: String,

    /// An additional image whose fully transparent areas (e.g.
    pub r#mask: String,

    /// The model to use for image generation.
    pub r#model: Option<(/*AnyOf*/)>,

    /// The number of images to generate.
    pub r#n: Option<i64>,

    /// The size of the generated images.
    pub r#size: Option<String>,

    /// The format in which the generated images are returned.
    pub r#response_format: Option<String>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: String,

    /// The quality of the image that will be generated.
    pub r#quality: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateImageRequest {
    /// A text description of the desired image(s).
    pub r#prompt: String,

    /// The model to use for image generation.
    pub r#model: Option<(/*AnyOf*/)>,

    /// The number of images to generate.
    pub r#n: Option<i64>,

    /// The quality of the image that will be generated.
    pub r#quality: Option<String>,

    /// The format in which generated images with `dall-e-2` and `dall-e-3` are
    /// returned.
    pub r#response_format: Option<String>,

    /// The format in which the generated images are returned.
    pub r#output_format: Option<String>,

    /// The compression level (0-100%) for the generated images.
    pub r#output_compression: Option<i64>,

    /// The size of the generated images.
    pub r#size: Option<String>,

    /// Control the content-moderation level for images generated by
    /// `gpt-image-1`.
    pub r#moderation: Option<String>,

    /// Allows to set transparency for the background of the generated image(s).
    pub r#background: Option<String>,

    /// The style of the generated images.
    pub r#style: Option<String>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateImageVariationRequest {
    /// The image to use as the basis for the variation(s).
    pub r#image: String,

    /// The model to use for image generation.
    pub r#model: Option<(/*AnyOf*/)>,

    /// The number of images to generate.
    pub r#n: Option<i64>,

    /// The format in which the generated images are returned.
    pub r#response_format: Option<String>,

    /// The size of the generated images.
    pub r#size: Option<String>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateMessageRequest {
    /// The role of the entity that is creating the message.
    pub r#role: String,

    pub r#content: (/*OneOf*/),

    /// A list of files attached to the message, and the tools they should be
    /// added to.
    pub r#attachments: Option<Vec<(/*Object*/)>>,

    pub r#metadata: Metadata,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModelResponseProperties(pub (/*AllOf*/));

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationRequest {
    /// Input (or inputs) to classify.
    pub r#input: (/*OneOf*/),

    /// The content moderation model you would like to use.
    pub r#model: (/*AnyOf*/),
}

/// Represents if a given text input is potentially harmful.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateModerationResponse {
    /// The unique identifier for the moderation request.
    pub r#id: String,

    /// The model used to generate the moderation results.
    pub r#model: String,

    /// A list of moderation objects.
    pub r#results: Vec<(/*Object*/)>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateResponse(pub (/*AllOf*/));

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateRunRequest {
    /// The ID of the [assistant](/docs/api-reference/assistants) to use to
    /// execute this run.
    pub r#assistant_id: String,

    /// The ID of the [Model](/docs/api-reference/models) to be used to execute
    /// this run.
    pub r#model: Option<(/*AnyOf*/)>,

    pub r#reasoning_effort: ReasoningEffort,

    /// Overrides the
    /// [instructions](/docs/api-reference/assistants/createAssistant) of the
    /// assistant.
    pub r#instructions: Option<String>,

    /// Appends additional instructions at the end of the instructions for the
    /// run.
    pub r#additional_instructions: Option<String>,

    /// Adds additional messages to the thread before creating the run.
    pub r#additional_messages: Option<Vec<CreateMessageRequest>>,

    /// Override the tools the assistant can use for this run.
    pub r#tools: Option<Vec<(/*OneOf*/)>>,

    pub r#metadata: Metadata,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// If `true`, returns a stream of events that happen during the Run as
    /// server-sent events, terminating when the Run enters a terminal state
    /// with a `data: [DONE]` message.
    pub r#stream: Option<bool>,

    /// The maximum number of prompt tokens that may be used over the course of
    /// the run.
    pub r#max_prompt_tokens: Option<i64>,

    /// The maximum number of completion tokens that may be used over the course
    /// of the run.
    pub r#max_completion_tokens: Option<i64>,

    pub r#truncation_strategy: (/*AllOf*/),

    pub r#tool_choice: (/*AllOf*/),

    pub r#parallel_tool_calls: ParallelToolCalls,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateSpeechRequest {
    /// One of the available [TTS models](/docs/models#tts): `tts-1`, `tts-1-hd`
    /// or `gpt-4o-mini-tts`.
    pub r#model: (/*AnyOf*/),

    /// The text to generate audio for.
    pub r#input: String,

    /// Control the voice of your generated audio with additional instructions.
    pub r#instructions: String,

    /// The voice to use when generating the audio.
    pub r#voice: VoiceIdsShared,

    /// The format to audio in.
    pub r#response_format: String,

    /// The speed of the generated audio.
    pub r#speed: f64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadAndRunRequest {
    /// The ID of the [assistant](/docs/api-reference/assistants) to use to
    /// execute this run.
    pub r#assistant_id: String,

    pub r#thread: CreateThreadRequest,

    /// The ID of the [Model](/docs/api-reference/models) to be used to execute
    /// this run.
    pub r#model: Option<(/*AnyOf*/)>,

    /// Override the default system message of the assistant.
    pub r#instructions: Option<String>,

    /// Override the tools the assistant can use for this run.
    pub r#tools: Option<Vec<(/*OneOf*/)>>,

    /// A set of resources that are used by the assistant's tools.
    pub r#tool_resources: Option<(/*Object*/)>,

    pub r#metadata: Metadata,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// If `true`, returns a stream of events that happen during the Run as
    /// server-sent events, terminating when the Run enters a terminal state
    /// with a `data: [DONE]` message.
    pub r#stream: Option<bool>,

    /// The maximum number of prompt tokens that may be used over the course of
    /// the run.
    pub r#max_prompt_tokens: Option<i64>,

    /// The maximum number of completion tokens that may be used over the course
    /// of the run.
    pub r#max_completion_tokens: Option<i64>,

    pub r#truncation_strategy: (/*AllOf*/),

    pub r#tool_choice: (/*AllOf*/),

    pub r#parallel_tool_calls: ParallelToolCalls,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

/// Options to create a new thread.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateThreadRequest {
    /// A list of [messages](/docs/api-reference/messages) to start the thread
    /// with.
    pub r#messages: Vec<CreateMessageRequest>,

    /// A set of resources that are made available to the assistant's tools in
    /// this thread.
    pub r#tool_resources: Option<(/*Object*/)>,

    pub r#metadata: Metadata,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranscriptionRequest {
    /// The audio file object (not file name) to transcribe, in one of these
    /// formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub r#file: String,

    /// ID of the model to use.
    pub r#model: (/*AnyOf*/),

    /// The language of the input audio.
    pub r#language: String,

    /// An optional text to guide the model's style or continue a previous audio
    /// segment.
    pub r#prompt: String,

    pub r#response_format: AudioResponseFormat,

    /// The sampling temperature, between 0 and 1.
    pub r#temperature: f64,

    /// Additional information to include in the transcription response.
    #[cfg_attr(feature = "serde", serde(rename = "include[]"))]
    pub r#include: Vec<TranscriptionInclude>,

    /// The timestamp granularities to populate for this transcription.
    #[cfg_attr(feature = "serde", serde(rename = "timestamp_granularities[]"))]
    pub r#timestamp_granularities: Vec<String>,

    /// If set to true, the model response data will be streamed to the client
    /// as it is generated using [server-sent
    /// events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
    pub r#stream: Option<bool>,
}

/// Represents a transcription response returned by model, based on the provided
/// input.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranscriptionResponseJson {
    /// The transcribed text.
    pub r#text: String,

    /// The log probabilities of the tokens in the transcription.
    pub r#logprobs: Vec<(/*Object*/)>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranscriptionResponseStreamEvent(pub (/*AnyOf*/));

/// Represents a verbose json transcription response returned by model, based on
/// the provided input.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranscriptionResponseVerboseJson {
    /// The language of the input audio.
    pub r#language: String,

    /// The duration of the input audio.
    pub r#duration: f64,

    /// The transcribed text.
    pub r#text: String,

    /// Extracted words and their corresponding timestamps.
    pub r#words: Vec<TranscriptionWord>,

    /// Segments of the transcribed text and their corresponding details.
    pub r#segments: Vec<TranscriptionSegment>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranslationRequest {
    /// The audio file object (not file name) translate, in one of these
    /// formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub r#file: String,

    /// ID of the model to use.
    pub r#model: (/*AnyOf*/),

    /// An optional text to guide the model's style or continue a previous audio
    /// segment.
    pub r#prompt: String,

    /// The format of the output, in one of these options: `json`, `text`,
    /// `srt`, `verbose_json`, or `vtt`.
    pub r#response_format: String,

    /// The sampling temperature, between 0 and 1.
    pub r#temperature: f64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranslationResponseJson {
    pub r#text: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTranslationResponseVerboseJson {
    /// The language of the output translation (always `english`).
    pub r#language: String,

    /// The duration of the input audio.
    pub r#duration: f64,

    /// The translated text.
    pub r#text: String,

    /// Segments of the translated text and their corresponding details.
    pub r#segments: Vec<TranscriptionSegment>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateUploadRequest {
    /// The name of the file to upload.
    pub r#filename: String,

    /// The intended purpose of the uploaded file.
    pub r#purpose: String,

    /// The number of bytes in the file you are uploading.
    pub r#bytes: i64,

    /// The MIME type of the file.
    pub r#mime_type: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateVectorStoreFileBatchRequest {
    /// A list of [File](/docs/api-reference/files) IDs that the vector store
    /// should use.
    pub r#file_ids: Vec<String>,

    pub r#chunking_strategy: ChunkingStrategyRequestParam,

    pub r#attributes: VectorStoreFileAttributes,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateVectorStoreFileRequest {
    /// A [File](/docs/api-reference/files) ID that the vector store should use.
    pub r#file_id: String,

    pub r#chunking_strategy: ChunkingStrategyRequestParam,

    pub r#attributes: VectorStoreFileAttributes,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateVectorStoreRequest {
    /// A list of [File](/docs/api-reference/files) IDs that the vector store
    /// should use.
    pub r#file_ids: Vec<String>,

    /// The name of the vector store.
    pub r#name: String,

    pub r#expires_after: VectorStoreExpirationAfter,

    /// The chunking strategy used to chunk the file(s).
    pub r#chunking_strategy: (/*OneOf*/),

    pub r#metadata: Metadata,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteAssistantResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteCertificateResponse {
    /// The object type, must be `certificate.deleted`.
    pub r#object: String,

    /// The ID of the certificate that was deleted.
    pub r#id: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteFileResponse {
    pub r#id: String,

    pub r#object: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteFineTuningCheckpointPermissionResponse {
    /// The ID of the fine-tuned model checkpoint permission that was deleted.
    pub r#id: String,

    /// The object type, which is always "checkpoint.permission".
    pub r#object: String,

    /// Whether the fine-tuned model checkpoint permission was successfully
    /// deleted.
    pub r#deleted: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteMessageResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteModelResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteThreadResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteVectorStoreFileResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteVectorStoreResponse {
    pub r#id: String,

    pub r#deleted: bool,

    pub r#object: String,
}

/// Occurs when a stream ends.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoneEvent {
    pub r#event: String,

    pub r#data: String,
}

/// A double click action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoubleClick {
    /// Specifies the event type.
    pub r#type: String,

    /// The x-coordinate where the double click occurred.
    pub r#x: i64,

    /// The y-coordinate where the double click occurred.
    pub r#y: i64,
}

/// A drag action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Drag {
    /// Specifies the event type.
    pub r#type: String,

    /// An array of coordinates representing the path of the drag action.
    pub r#path: Vec<Coordinate>,
}

/// A message input to the model with a role indicating instruction following
/// hierarchy.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EasyInputMessage {
    /// The role of the message input.
    pub r#role: String,

    /// Text, image, or audio input to the model, used to generate a response.
    pub r#content: (/*OneOf*/),

    /// The type of the message input.
    pub r#type: String,
}

/// Represents an embedding vector returned by embedding endpoint.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Embedding {
    /// The index of the embedding in the list of embeddings.
    pub r#index: i64,

    /// The embedding vector, which is a list of floats.
    pub r#embedding: Vec<f64>,

    /// The object type, which is always "embedding".
    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Error {
    pub r#code: Option<String>,

    pub r#message: String,

    pub r#param: Option<String>,

    pub r#type: String,
}

/// Occurs when an [error](/docs/guides/error-codes#api-errors) occurs.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorEvent {
    pub r#event: String,

    pub r#data: Error,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ErrorResponse {
    pub r#error: Error,
}

/// An Eval object with a data source config and testing criteria.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Eval {
    /// The object type.
    pub r#object: String,

    /// Unique identifier for the evaluation.
    pub r#id: String,

    /// The name of the evaluation.
    pub r#name: String,

    /// Configuration of data sources used in runs of the evaluation.
    pub r#data_source_config: (/*OneOf*/),

    /// A list of testing criteria.
    pub r#testing_criteria: Vec<(/*OneOf*/)>,

    /// The Unix timestamp (in seconds) for when the eval was created.
    pub r#created_at: i64,

    pub r#metadata: Metadata,
}

/// An object representing an error response from the Eval API.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalApiError {
    /// The error code.
    pub r#code: String,

    /// The error message.
    pub r#message: String,
}

/// A CustomDataSourceConfig which specifies the schema of your `item` and
/// optionally `sample` namespaces.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalCustomDataSourceConfig {
    /// The type of data source.
    pub r#type: String,

    /// The json schema for the run data source items.
    pub r#schema: (/*Object*/),
}

/// A message input to the model with a role indicating instruction following
/// hierarchy.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalItem {
    /// The role of the message input.
    pub r#role: String,

    /// Text inputs to the model - can contain template strings.
    pub r#content: (/*OneOf*/),

    /// The type of the message input.
    pub r#type: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalJsonlFileContentSource {
    /// The type of jsonl source.
    pub r#type: String,

    /// The content of the jsonl file.
    pub r#content: Vec<(/*Object*/)>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalJsonlFileIdSource {
    /// The type of jsonl source.
    pub r#type: String,

    /// The identifier of the file.
    pub r#id: String,
}

/// A LabelModelGrader object which uses a model to assign labels to each item
/// in the evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalLabelModelGrader {
    /// The object type, which is always `label_model`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The model to use for the evaluation.
    pub r#model: String,

    pub r#input: Vec<EvalItem>,

    /// The labels to assign to each item in the evaluation.
    pub r#labels: Vec<String>,

    /// The labels that indicate a passing result.
    pub r#passing_labels: Vec<String>,
}

/// An object representing a list of evals.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalList {
    /// The type of this object.
    pub r#object: String,

    /// An array of eval objects.
    pub r#data: Vec<Eval>,

    /// The identifier of the first eval in the data array.
    pub r#first_id: String,

    /// The identifier of the last eval in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more evals available.
    pub r#has_more: bool,
}

/// A PythonGrader object that runs a python script on the input.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalPythonGrader {
    /// The object type, which is always `python`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The source code of the python script.
    pub r#source: String,

    /// The threshold for the score.
    pub r#pass_threshold: f64,

    /// The image tag to use for the python script.
    pub r#image_tag: String,
}

/// A EvalResponsesSource object describing a run data source configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalResponsesSource {
    /// The type of run data source.
    pub r#type: String,

    /// Metadata filter for the responses.
    pub r#metadata: Option<(/*Object*/)>,

    /// The name of the model to find responses for.
    pub r#model: Option<String>,

    /// Optional search string for instructions.
    pub r#instructions_search: Option<String>,

    /// Only include items created after this timestamp (inclusive).
    pub r#created_after: Option<i64>,

    /// Only include items created before this timestamp (inclusive).
    pub r#created_before: Option<i64>,

    /// Whether the response has tool calls.
    pub r#has_tool_calls: Option<bool>,

    /// Optional reasoning effort parameter.
    pub r#reasoning_effort: Option<ReasoningEffort>,

    /// Sampling temperature.
    pub r#temperature: Option<f64>,

    /// Nucleus sampling parameter.
    pub r#top_p: Option<f64>,

    /// List of user identifiers.
    pub r#users: Option<Vec<String>>,

    /// Whether to allow parallel tool calls.
    pub r#allow_parallel_tool_calls: Option<bool>,
}

/// A schema representing an evaluation run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRun {
    /// The type of the object.
    pub r#object: String,

    /// Unique identifier for the evaluation run.
    pub r#id: String,

    /// The identifier of the associated evaluation.
    pub r#eval_id: String,

    /// The status of the evaluation run.
    pub r#status: String,

    /// The model that is evaluated, if applicable.
    pub r#model: String,

    /// The name of the evaluation run.
    pub r#name: String,

    /// Unix timestamp (in seconds) when the evaluation run was created.
    pub r#created_at: i64,

    /// The URL to the rendered evaluation run report on the UI dashboard.
    pub r#report_url: String,

    /// Counters summarizing the outcomes of the evaluation run.
    pub r#result_counts: (/*Object*/),

    /// Usage statistics for each model during the evaluation run.
    pub r#per_model_usage: Vec<(/*Object*/)>,

    /// Results per testing criteria applied during the evaluation run.
    pub r#per_testing_criteria_results: Vec<(/*Object*/)>,

    /// Information about the run's data source.
    pub r#data_source: (/*OneOf*/),

    pub r#metadata: Metadata,

    pub r#error: EvalApiError,
}

/// An object representing a list of runs for an evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunList {
    /// The type of this object.
    pub r#object: String,

    /// An array of eval run objects.
    pub r#data: Vec<EvalRun>,

    /// The identifier of the first eval run in the data array.
    pub r#first_id: String,

    /// The identifier of the last eval run in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more evals available.
    pub r#has_more: bool,
}

/// A schema representing an evaluation run output item.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItem {
    /// The type of the object.
    pub r#object: String,

    /// Unique identifier for the evaluation run output item.
    pub r#id: String,

    /// The identifier of the evaluation run associated with this output item.
    pub r#run_id: String,

    /// The identifier of the evaluation group.
    pub r#eval_id: String,

    /// Unix timestamp (in seconds) when the evaluation run was created.
    pub r#created_at: i64,

    /// The status of the evaluation run.
    pub r#status: String,

    /// The identifier for the data source item.
    pub r#datasource_item_id: i64,

    /// Details of the input data source item.
    pub r#datasource_item: (/*Object*/),

    /// A list of results from the evaluation run.
    pub r#results: Vec<(/*Object*/)>,

    /// A sample containing the input and output of the evaluation run.
    pub r#sample: (/*Object*/),
}

/// An object representing a list of output items for an evaluation run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalRunOutputItemList {
    /// The type of this object.
    pub r#object: String,

    /// An array of eval run output item objects.
    pub r#data: Vec<EvalRunOutputItem>,

    /// The identifier of the first eval run output item in the data array.
    pub r#first_id: String,

    /// The identifier of the last eval run output item in the data array.
    pub r#last_id: String,

    /// Indicates whether there are more eval run output items available.
    pub r#has_more: bool,
}

/// A ScoreModelGrader object that uses a model to assign a score to the input.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalScoreModelGrader {
    /// The object type, which is always `score_model`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The model to use for the evaluation.
    pub r#model: String,

    /// The sampling parameters for the model.
    pub r#sampling_params: (/*Object*/),

    /// The input text.
    pub r#input: Vec<EvalItem>,

    /// The threshold for the score.
    pub r#pass_threshold: f64,

    /// The range of the score.
    pub r#range: Vec<f64>,
}

/// A StoredCompletionsDataSourceConfig which specifies the metadata property of
/// your stored completions query.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalStoredCompletionsDataSourceConfig {
    /// The type of data source.
    pub r#type: String,

    pub r#metadata: Metadata,

    /// The json schema for the run data source items.
    pub r#schema: (/*Object*/),
}

/// A StoredCompletionsRunDataSource configuration describing a set of filters
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalStoredCompletionsSource {
    /// The type of source.
    pub r#type: String,

    pub r#metadata: Metadata,

    /// An optional model to filter by (e.g., 'gpt-4o').
    pub r#model: Option<String>,

    /// An optional Unix timestamp to filter items created after this time.
    pub r#created_after: Option<i64>,

    /// An optional Unix timestamp to filter items created before this time.
    pub r#created_before: Option<i64>,

    /// An optional maximum number of items to return.
    pub r#limit: Option<i64>,
}

/// A StringCheckGrader object that performs a string comparison between input
/// and reference using a specified operation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalStringCheckGrader {
    /// The object type, which is always `string_check`.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The input text.
    pub r#input: String,

    /// The reference text.
    pub r#reference: String,

    /// The string check operation to perform.
    pub r#operation: String,
}

/// A TextSimilarityGrader object which grades text based on similarity metrics.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EvalTextSimilarityGrader {
    /// The type of grader.
    pub r#type: String,

    /// The name of the grader.
    pub r#name: String,

    /// The text being graded.
    pub r#input: String,

    /// The text being graded against.
    pub r#reference: String,

    /// A float score where a value greater than or equal indicates a passing
    /// grade.
    pub r#pass_threshold: f64,

    /// The evaluation metric to use.
    pub r#evaluation_metric: String,
}

/// A citation to a file.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileCitationBody {
    /// The type of the file citation.
    pub r#type: String,

    /// The ID of the file.
    pub r#file_id: String,

    /// The index of the file in the list of files.
    pub r#index: i64,
}

/// A path to a file.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FilePath {
    /// The type of the file path.
    pub r#type: String,

    /// The ID of the file.
    pub r#file_id: String,

    /// The index of the file in the list of files.
    pub r#index: i64,
}

/// The ranker to use for the file search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSearchRanker(pub String);

/// The ranking options for the file search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSearchRankingOptions {
    pub r#ranker: FileSearchRanker,

    /// The score threshold for the file search.
    pub r#score_threshold: f64,
}

/// A tool that searches for relevant content from uploaded files.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSearchTool {
    /// The type of the file search tool.
    pub r#type: String,

    /// The IDs of the vector stores to search.
    pub r#vector_store_ids: Vec<String>,

    /// The maximum number of results to return.
    pub r#max_num_results: i64,

    /// Ranking options for search.
    pub r#ranking_options: RankingOptions,

    pub r#filters: (/*AnyOf*/),
}

/// The results of a file search tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileSearchToolCall {
    /// The unique ID of the file search tool call.
    pub r#id: String,

    /// The type of the file search tool call.
    pub r#type: String,

    /// The status of the file search tool call.
    pub r#status: String,

    /// The queries used to search for files.
    pub r#queries: Vec<String>,

    /// The results of the file search tool call.
    pub r#results: Option<Vec<(/*Object*/)>>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Filters(pub (/*AnyOf*/));

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneChatCompletionRequestAssistantMessage(pub (/*AllOf*/));

/// The per-line training example of a fine-tuning input file for chat models
/// using the supervised method.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneChatRequestInput {
    pub r#messages: Vec<(/*OneOf*/)>,

    /// A list of tools the model may generate JSON inputs for.
    pub r#tools: Vec<ChatCompletionTool>,

    pub r#parallel_tool_calls: ParallelToolCalls,

    /// A list of functions the model may generate JSON inputs for.
    pub r#functions: Vec<ChatCompletionFunctions>,
}

/// The per-line training example of a fine-tuning input file for completions
/// models
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneCompletionRequestInput {
    /// The input prompt for this training example.
    pub r#prompt: String,

    /// The desired completion for this training example.
    pub r#completion: String,
}

/// Configuration for the DPO fine-tuning method.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneDPOMethod {
    /// The hyperparameters used for the fine-tuning job.
    pub r#hyperparameters: (/*Object*/),
}

/// The method used for fine-tuning.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneMethod {
    /// The type of method.
    pub r#type: String,

    pub r#supervised: FineTuneSupervisedMethod,

    pub r#dpo: FineTuneDPOMethod,
}

/// The per-line training example of a fine-tuning input file for chat models
/// using the dpo method.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTunePreferenceRequestInput {
    pub r#input: (/*Object*/),

    /// The preferred completion message for the output.
    pub r#preferred_completion: Vec<(/*OneOf*/)>,

    /// The non-preferred completion message for the output.
    pub r#non_preferred_completion: Vec<(/*OneOf*/)>,
}

/// Configuration for the supervised fine-tuning method.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuneSupervisedMethod {
    /// The hyperparameters used for the fine-tuning job.
    pub r#hyperparameters: (/*Object*/),
}

/// The `checkpoint.permission` object represents a permission for a fine-tuned
/// model checkpoint.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningCheckpointPermission {
    /// The permission identifier, which can be referenced in the API endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the permission was created.
    pub r#created_at: i64,

    /// The project identifier that the permission is for.
    pub r#project_id: String,

    /// The object type, which is always "checkpoint.permission".
    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningIntegration {
    /// The type of the integration being enabled for the fine-tuning job
    pub r#type: String,

    /// The settings for your integration with Weights and Biases.
    pub r#wandb: (/*Object*/),
}

/// The `fine_tuning.job` object represents a fine-tuning job that has been
/// created through the API.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJob {
    /// The object identifier, which can be referenced in the API endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was
    /// created.
    pub r#created_at: i64,

    /// For fine-tuning jobs that have `failed`, this will contain more
    /// information on the cause of the failure.
    pub r#error: Option<(/*Object*/)>,

    /// The name of the fine-tuned model that is being created.
    pub r#fine_tuned_model: Option<String>,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was
    /// finished.
    pub r#finished_at: Option<i64>,

    /// The hyperparameters used for the fine-tuning job.
    pub r#hyperparameters: (/*Object*/),

    /// The base model that is being fine-tuned.
    pub r#model: String,

    /// The object type, which is always "fine_tuning.job".
    pub r#object: String,

    /// The organization that owns the fine-tuning job.
    pub r#organization_id: String,

    /// The compiled results file ID(s) for the fine-tuning job.
    pub r#result_files: Vec<String>,

    /// The current status of the fine-tuning job, which can be either
    /// `validating_files`, `queued`, `running`, `succeeded`, `failed`, or
    /// `cancelled`.
    pub r#status: String,

    /// The total number of billable tokens processed by this fine-tuning job.
    pub r#trained_tokens: Option<i64>,

    /// The file ID used for training.
    pub r#training_file: String,

    /// The file ID used for validation.
    pub r#validation_file: Option<String>,

    /// A list of integrations to enable for this fine-tuning job.
    pub r#integrations: Option<Vec<(/*OneOf*/)>>,

    /// The seed used for the fine-tuning job.
    pub r#seed: i64,

    /// The Unix timestamp (in seconds) for when the fine-tuning job is
    /// estimated to finish.
    pub r#estimated_finish: Option<i64>,

    pub r#method: FineTuneMethod,

    pub r#metadata: Metadata,
}

/// The `fine_tuning.job.checkpoint` object represents a model checkpoint for a
/// fine-tuning job that is ready to use.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJobCheckpoint {
    /// The checkpoint identifier, which can be referenced in the API endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the checkpoint was created.
    pub r#created_at: i64,

    /// The name of the fine-tuned checkpoint model that is created.
    pub r#fine_tuned_model_checkpoint: String,

    /// The step number that the checkpoint was created at.
    pub r#step_number: i64,

    /// Metrics at the step number during the fine-tuning job.
    pub r#metrics: (/*Object*/),

    /// The name of the fine-tuning job that this checkpoint was created from.
    pub r#fine_tuning_job_id: String,

    /// The object type, which is always "fine_tuning.job.checkpoint".
    pub r#object: String,
}

/// Fine-tuning job event object
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FineTuningJobEvent {
    /// The object type, which is always "fine_tuning.job.event".
    pub r#object: String,

    /// The object identifier.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the fine-tuning job was
    /// created.
    pub r#created_at: i64,

    /// The log level of the event.
    pub r#level: String,

    /// The message of the event.
    pub r#message: String,

    /// The type of event.
    pub r#type: String,

    /// The data associated with the event.
    pub r#data: (/*Object*/),
}

/// The output of a function tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionCallOutputItemParam {
    pub r#id: (/*AnyOf*/),

    /// The unique ID of the function tool call generated by the model.
    pub r#call_id: String,

    /// The type of the function tool call output.
    pub r#type: String,

    /// A JSON string of the output of the function tool call.
    pub r#output: String,

    pub r#status: (/*AnyOf*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionObject {
    /// A description of what the function does, used by the model to choose
    /// when and how to call the function.
    pub r#description: String,

    /// The name of the function to be called.
    pub r#name: String,

    pub r#parameters: FunctionParameters,

    /// Whether to enable strict schema adherence when generating the function
    /// call.
    pub r#strict: Option<bool>,
}

/// The parameters the functions accepts, described as a JSON Schema object.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionParameters;

/// Defines a function in your own code the model can choose to call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionTool {
    /// The type of the function tool.
    pub r#type: String,

    /// The name of the function to call.
    pub r#name: String,

    pub r#description: (/*AnyOf*/),

    pub r#parameters: (/*AnyOf*/),

    pub r#strict: (/*AnyOf*/),
}

/// A tool call to run a function.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCall {
    /// The unique ID of the function tool call.
    pub r#id: String,

    /// The type of the function tool call.
    pub r#type: String,

    /// The unique ID of the function tool call generated by the model.
    pub r#call_id: String,

    /// The name of the function to run.
    pub r#name: String,

    /// A JSON string of the arguments to pass to the function.
    pub r#arguments: String,

    /// The status of the item.
    pub r#status: String,
}

/// The output of a function tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCallOutput {
    /// The unique ID of the function tool call output.
    pub r#id: String,

    /// The type of the function tool call output.
    pub r#type: String,

    /// The unique ID of the function tool call generated by the model.
    pub r#call_id: String,

    /// A JSON string of the output of the function tool call.
    pub r#output: String,

    /// The status of the item.
    pub r#status: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCallOutputResource(pub (/*AllOf*/));

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionToolCallResource(pub (/*AllOf*/));

/// Represents the content or the URL of an image generated by the OpenAI API.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image {
    /// The base64-encoded JSON of the generated image.
    pub r#b64_json: String,

    /// When using `dall-e-2` or `dall-e-3`, the URL of the generated image if
    /// `response_format` is set to `url` (default value).
    pub r#url: String,

    /// For `dall-e-3` only, the revised prompt that was used to generate the
    /// image.
    pub r#revised_prompt: String,
}

/// The response from the image generation endpoint.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImagesResponse {
    /// The Unix timestamp (in seconds) of when the image was created.
    pub r#created: i64,

    /// The list of generated images.
    pub r#data: Vec<Image>,

    /// For `gpt-image-1` only, the token usage information for the image
    /// generation.
    pub r#usage: (/*Object*/),
}

/// Specify additional output data to include in the model response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Includable(pub String);

/// An audio input to the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputAudio {
    /// The type of the input item.
    pub r#type: String,

    /// Base64-encoded audio data.
    pub r#data: String,

    /// The format of the audio data.
    pub r#format: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputContent {
    InputTextContent(InputTextContent),

    InputImageContent(InputImageContent),

    InputFileContent(InputFileContent),
}

/// A file input to the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputFileContent {
    /// The type of the input item.
    pub r#type: String,

    pub r#file_id: (/*AnyOf*/),

    /// The name of the file to be sent to the model.
    pub r#filename: String,

    /// The content of the file to be sent to the model.
    pub r#file_data: String,
}

/// An image input to the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputImageContent {
    /// The type of the input item.
    pub r#type: String,

    pub r#image_url: (/*AnyOf*/),

    pub r#file_id: (/*AnyOf*/),

    /// The detail level of the image to be sent to the model.
    pub r#detail: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputItem {
    EasyInputMessage(EasyInputMessage),

    /// An item representing part of the context for the response to be
    /// generated by the model.
    Item(Item),

    ItemReferenceParam(ItemReferenceParam),
}

/// A message input to the model with a role indicating instruction following
/// hierarchy.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputMessage {
    /// The type of the message input.
    pub r#type: String,

    /// The role of the message input.
    pub r#role: String,

    /// The status of item.
    pub r#status: String,

    pub r#content: InputMessageContentList,
}

/// A list of one or many input items to the model, containing different content
/// types.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputMessageContentList(pub Vec<InputContent>);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputMessageResource(pub (/*AllOf*/));

/// A text input to the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputTextContent {
    /// The type of the input item.
    pub r#type: String,

    /// The text input to the model.
    pub r#text: String,
}

/// Represents an individual `invite` to the organization.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Invite {
    /// The object type, which is always `organization.invite`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The email address of the individual to whom the invite was sent
    pub r#email: String,

    /// `owner` or `reader`
    pub r#role: String,

    /// `accepted`,`expired`, or `pending`
    pub r#status: String,

    /// The Unix timestamp (in seconds) of when the invite was sent.
    pub r#invited_at: i64,

    /// The Unix timestamp (in seconds) of when the invite expires.
    pub r#expires_at: i64,

    /// The Unix timestamp (in seconds) of when the invite was accepted.
    pub r#accepted_at: i64,

    /// The projects that were granted membership upon acceptance of the invite.
    pub r#projects: Vec<(/*Object*/)>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InviteDeleteResponse {
    /// The object type, which is always `organization.invite.deleted`
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InviteListResponse {
    /// The object type, which is always `list`
    pub r#object: String,

    pub r#data: Vec<Invite>,

    /// The first `invite_id` in the retrieved `list`
    pub r#first_id: String,

    /// The last `invite_id` in the retrieved `list`
    pub r#last_id: String,

    /// The `has_more` property is used for pagination to indicate there are
    /// additional results.
    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InviteRequest {
    /// Send an email to this address
    pub r#email: String,

    /// `owner` or `reader`
    pub r#role: String,

    /// An array of projects to which membership is granted at the same time the
    /// org invite is accepted.
    pub r#projects: Vec<(/*Object*/)>,
}

/// Content item used to generate a response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Item;

/// An internal identifier for an item to reference.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemReferenceParam {
    pub r#type: (/*AnyOf*/),

    /// The ID of the item to reference.
    pub r#id: String,
}

/// Content item used to generate a response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ItemResource {
    InputMessageResource(InputMessageResource),

    OutputMessage(OutputMessage),

    FileSearchToolCall(FileSearchToolCall),

    ComputerToolCall(ComputerToolCall),

    ComputerToolCallOutputResource(ComputerToolCallOutputResource),

    WebSearchToolCall(WebSearchToolCall),

    FunctionToolCallResource(FunctionToolCallResource),

    FunctionToolCallOutputResource(FunctionToolCallOutputResource),
}

/// A collection of keypresses the model would like to perform.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyPress {
    /// Specifies the event type.
    pub r#type: String,

    /// The combination of keys the model is requesting to be pressed.
    pub r#keys: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListAssistantsResponse {
    pub r#object: String,

    pub r#data: Vec<AssistantObject>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListAuditLogsResponse {
    pub r#object: String,

    pub r#data: Vec<AuditLog>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListBatchesResponse {
    pub r#data: Vec<Batch>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,

    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListCertificatesResponse {
    pub r#data: Vec<Certificate>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,

    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListFilesResponse {
    pub r#object: String,

    pub r#data: Vec<OpenAIFile>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListFineTuningCheckpointPermissionResponse {
    pub r#data: Vec<FineTuningCheckpointPermission>,

    pub r#object: String,

    pub r#first_id: Option<String>,

    pub r#last_id: Option<String>,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListFineTuningJobCheckpointsResponse {
    pub r#data: Vec<FineTuningJobCheckpoint>,

    pub r#object: String,

    pub r#first_id: Option<String>,

    pub r#last_id: Option<String>,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListFineTuningJobEventsResponse {
    pub r#data: Vec<FineTuningJobEvent>,

    pub r#object: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListMessagesResponse;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListModelsResponse {
    pub r#object: String,

    pub r#data: Vec<Model>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListPaginatedFineTuningJobsResponse {
    pub r#data: Vec<FineTuningJob>,

    pub r#has_more: bool,

    pub r#object: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListRunStepsResponse;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListRunsResponse {
    pub r#object: String,

    pub r#data: Vec<RunObject>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListVectorStoreFilesResponse;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListVectorStoresResponse;

/// A log probability object.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LogProbProperties {
    /// The token that was used to generate the log probability.
    pub r#token: String,

    /// The log probability of the token.
    pub r#logprob: f64,

    /// The bytes that were used to generate the log probability.
    pub r#bytes: Vec<i64>,
}

/// References an image [File](/docs/api-reference/files) in the content of a
/// message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentImageFileObject {
    /// Always `image_file`.
    pub r#type: String,

    pub r#image_file: (/*Object*/),
}

/// References an image URL in the content of a message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentImageUrlObject {
    /// The type of the content part.
    pub r#type: String,

    pub r#image_url: (/*Object*/),
}

/// The refusal content generated by the assistant.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentRefusalObject {
    /// Always `refusal`.
    pub r#type: String,

    pub r#refusal: String,
}

/// A citation within the message that points to a specific quote from a
/// specific File associated with the assistant or the message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentTextAnnotationsFileCitationObject {
    /// Always `file_citation`.
    pub r#type: String,

    /// The text in the message content that needs to be replaced.
    pub r#text: String,

    pub r#file_citation: (/*Object*/),

    pub r#start_index: i64,

    pub r#end_index: i64,
}

/// A URL for the file that's generated when the assistant used the
/// `code_interpreter` tool to generate a file.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentTextAnnotationsFilePathObject {
    /// Always `file_path`.
    pub r#type: String,

    /// The text in the message content that needs to be replaced.
    pub r#text: String,

    pub r#file_path: (/*Object*/),

    pub r#start_index: i64,

    pub r#end_index: i64,
}

/// The text content that is part of a message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageContentTextObject {
    /// Always `text`.
    pub r#type: String,

    pub r#text: (/*Object*/),
}

/// References an image [File](/docs/api-reference/files) in the content of a
/// message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentImageFileObject {
    /// The index of the content part in the message.
    pub r#index: i64,

    /// Always `image_file`.
    pub r#type: String,

    pub r#image_file: (/*Object*/),
}

/// References an image URL in the content of a message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentImageUrlObject {
    /// The index of the content part in the message.
    pub r#index: i64,

    /// Always `image_url`.
    pub r#type: String,

    pub r#image_url: (/*Object*/),
}

/// The refusal content that is part of a message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentRefusalObject {
    /// The index of the refusal part in the message.
    pub r#index: i64,

    /// Always `refusal`.
    pub r#type: String,

    pub r#refusal: String,
}

/// A citation within the message that points to a specific quote from a
/// specific File associated with the assistant or the message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
    /// The index of the annotation in the text content part.
    pub r#index: i64,

    /// Always `file_citation`.
    pub r#type: String,

    /// The text in the message content that needs to be replaced.
    pub r#text: String,

    pub r#file_citation: (/*Object*/),

    pub r#start_index: i64,

    pub r#end_index: i64,
}

/// A URL for the file that's generated when the assistant used the
/// `code_interpreter` tool to generate a file.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentTextAnnotationsFilePathObject {
    /// The index of the annotation in the text content part.
    pub r#index: i64,

    /// Always `file_path`.
    pub r#type: String,

    /// The text in the message content that needs to be replaced.
    pub r#text: String,

    pub r#file_path: (/*Object*/),

    pub r#start_index: i64,

    pub r#end_index: i64,
}

/// The text content that is part of a message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaContentTextObject {
    /// The index of the content part in the message.
    pub r#index: i64,

    /// Always `text`.
    pub r#type: String,

    pub r#text: (/*Object*/),
}

/// Represents a message delta i.e.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageDeltaObject {
    /// The identifier of the message, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.message.delta`.
    pub r#object: String,

    /// The delta containing the fields that have changed on the Message.
    pub r#delta: (/*Object*/),
}

/// Represents a message within a [thread](/docs/api-reference/threads).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.message`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the message was created.
    pub r#created_at: i64,

    /// The [thread](/docs/api-reference/threads) ID that this message belongs
    /// to.
    pub r#thread_id: String,

    /// The status of the message, which can be either `in_progress`,
    /// `incomplete`, or `completed`.
    pub r#status: String,

    /// On an incomplete message, details about why the message is incomplete.
    pub r#incomplete_details: Option<(/*Object*/)>,

    /// The Unix timestamp (in seconds) for when the message was completed.
    pub r#completed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the message was marked as
    /// incomplete.
    pub r#incomplete_at: Option<i64>,

    /// The entity that produced the message.
    pub r#role: String,

    /// The content of the message in array of text and/or images.
    pub r#content: Vec<(/*OneOf*/)>,

    /// If applicable, the ID of the [assistant](/docs/api-reference/assistants)
    /// that authored this message.
    pub r#assistant_id: Option<String>,

    /// The ID of the [run](/docs/api-reference/runs) associated with the
    /// creation of this message.
    pub r#run_id: Option<String>,

    /// A list of files attached to the message, and the tools they were added
    /// to.
    pub r#attachments: Option<Vec<(/*Object*/)>>,

    pub r#metadata: Metadata,
}

/// The text content that is part of a message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageRequestContentTextObject {
    /// Always `text`.
    pub r#type: String,

    /// Text content to be sent to the model
    pub r#text: String,
}

include!("components/message_stream_event.rs");

/// Set of 16 key-value pairs that can be attached to an object.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metadata;

/// Describes an OpenAI model offering that can be used with the API.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Model;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModelIds(pub (/*AnyOf*/));

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModelIdsResponses(pub (/*AnyOf*/));

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModelIdsShared(pub (/*AnyOf*/));

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModelResponseProperties {
    pub r#metadata: Metadata,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    /// A unique identifier representing your end-user, which can help OpenAI to
    /// monitor and detect abuse.
    pub r#user: String,

    pub r#service_tier: ServiceTier,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyAssistantRequest {
    /// ID of the model to use.
    pub r#model: (/*AnyOf*/),

    pub r#reasoning_effort: ReasoningEffort,

    /// The name of the assistant.
    pub r#name: Option<String>,

    /// The description of the assistant.
    pub r#description: Option<String>,

    /// The system instructions that the assistant uses.
    pub r#instructions: Option<String>,

    /// A list of tool enabled on the assistant.
    pub r#tools: Vec<(/*OneOf*/)>,

    /// A set of resources that are used by the assistant's tools.
    pub r#tool_resources: Option<(/*Object*/)>,

    pub r#metadata: Metadata,

    /// What sampling temperature to use, between 0 and 2.
    pub r#temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass.
    pub r#top_p: Option<f64>,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyCertificateRequest {
    /// The updated name for the certificate
    pub r#name: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyMessageRequest {
    pub r#metadata: Metadata,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyRunRequest {
    pub r#metadata: Metadata,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModifyThreadRequest {
    /// A set of resources that are made available to the assistant's tools in
    /// this thread.
    pub r#tool_resources: Option<(/*Object*/)>,

    pub r#metadata: Metadata,
}

/// A mouse move action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Move {
    /// Specifies the event type.
    pub r#type: String,

    /// The x-coordinate to move to.
    pub r#x: i64,

    /// The y-coordinate to move to.
    pub r#y: i64,
}

/// The `File` object represents a document that has been uploaded to OpenAI.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenAIFile;

/// This is returned when the chunking strategy is unknown.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OtherChunkingStrategyResponseParam {
    /// Always `other`.
    pub r#type: String,
}

/// An audio output from the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputAudio {
    /// The type of the output audio.
    pub r#type: String,

    /// Base64-encoded audio data from the model.
    pub r#data: String,

    /// The transcript of the audio data from the model.
    pub r#transcript: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OutputContent {
    OutputTextContent(OutputTextContent),

    RefusalContent(RefusalContent),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputItem(pub (/*AnyOf*/));

/// An output message from the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputMessage {
    /// The unique ID of the output message.
    pub r#id: String,

    /// The type of the output message.
    pub r#type: String,

    /// The role of the output message.
    pub r#role: String,

    /// The content of the output message.
    pub r#content: Vec<OutputContent>,

    /// The status of the message input.
    pub r#status: String,
}

/// A text output from the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputTextContent {
    /// The type of the output text.
    pub r#type: String,

    /// The text output from the model.
    pub r#text: String,

    /// The annotations of the text output.
    pub r#annotations: Vec<Annotation>,
}

/// Whether to enable [parallel function
/// calling](/docs/guides/function-calling#configuring-parallel-function-calling)
/// during tool use.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParallelToolCalls(pub bool);

/// Static predicted output content, such as the content of a text file that is
/// being regenerated.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PredictionContent {
    /// The type of the predicted content you want to provide.
    pub r#type: String,

    /// The content that should be matched when generating a model response.
    pub r#content: (/*OneOf*/),
}

/// Represents an individual project.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Project {
    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The object type, which is always `organization.project`
    pub r#object: String,

    /// The name of the project.
    pub r#name: String,

    /// The Unix timestamp (in seconds) of when the project was created.
    pub r#created_at: i64,

    /// The Unix timestamp (in seconds) of when the project was archived or
    /// `null`.
    pub r#archived_at: Option<i64>,

    /// `active` or `archived`
    pub r#status: String,
}

/// Represents an individual API key in a project.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectApiKey {
    /// The object type, which is always `organization.project.api_key`
    pub r#object: String,

    /// The redacted value of the API key
    pub r#redacted_value: String,

    /// The name of the API key
    pub r#name: String,

    /// The Unix timestamp (in seconds) of when the API key was created
    pub r#created_at: i64,

    /// The Unix timestamp (in seconds) of when the API key was last used.
    pub r#last_used_at: i64,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    pub r#owner: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectApiKeyDeleteResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectApiKeyListResponse {
    pub r#object: String,

    pub r#data: Vec<ProjectApiKey>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectCreateRequest {
    /// The friendly name of the project, this name appears in reports.
    pub r#name: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectListResponse {
    pub r#object: String,

    pub r#data: Vec<Project>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

/// Represents a project rate limit config.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectRateLimit {
    /// The object type, which is always `project.rate_limit`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The model this rate limit applies to.
    pub r#model: String,

    /// The maximum requests per minute.
    pub r#max_requests_per_1_minute: i64,

    /// The maximum tokens per minute.
    pub r#max_tokens_per_1_minute: i64,

    /// The maximum images per minute.
    pub r#max_images_per_1_minute: i64,

    /// The maximum audio megabytes per minute.
    pub r#max_audio_megabytes_per_1_minute: i64,

    /// The maximum requests per day.
    pub r#max_requests_per_1_day: i64,

    /// The maximum batch input tokens per day.
    pub r#batch_1_day_max_input_tokens: i64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectRateLimitListResponse {
    pub r#object: String,

    pub r#data: Vec<ProjectRateLimit>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectRateLimitUpdateRequest {
    /// The maximum requests per minute.
    pub r#max_requests_per_1_minute: i64,

    /// The maximum tokens per minute.
    pub r#max_tokens_per_1_minute: i64,

    /// The maximum images per minute.
    pub r#max_images_per_1_minute: i64,

    /// The maximum audio megabytes per minute.
    pub r#max_audio_megabytes_per_1_minute: i64,

    /// The maximum requests per day.
    pub r#max_requests_per_1_day: i64,

    /// The maximum batch input tokens per day.
    pub r#batch_1_day_max_input_tokens: i64,
}

/// Represents an individual service account in a project.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccount {
    /// The object type, which is always `organization.project.service_account`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the service account
    pub r#name: String,

    /// `owner` or `member`
    pub r#role: String,

    /// The Unix timestamp (in seconds) of when the service account was created
    pub r#created_at: i64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountApiKey {
    /// The object type, which is always
    /// `organization.project.service_account.api_key`
    pub r#object: String,

    pub r#value: String,

    pub r#name: String,

    pub r#created_at: i64,

    pub r#id: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountCreateRequest {
    /// The name of the service account being created.
    pub r#name: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountCreateResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#name: String,

    /// Service accounts can only have one role of type `member`
    pub r#role: String,

    pub r#created_at: i64,

    pub r#api_key: ProjectServiceAccountApiKey,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountDeleteResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectServiceAccountListResponse {
    pub r#object: String,

    pub r#data: Vec<ProjectServiceAccount>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUpdateRequest {
    /// The updated name of the project, this name appears in reports.
    pub r#name: String,
}

/// Represents an individual user in a project.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUser {
    /// The object type, which is always `organization.project.user`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the user
    pub r#name: String,

    /// The email address of the user
    pub r#email: String,

    /// `owner` or `member`
    pub r#role: String,

    /// The Unix timestamp (in seconds) of when the project was added.
    pub r#added_at: i64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUserCreateRequest {
    /// The ID of the user.
    pub r#user_id: String,

    /// `owner` or `member`
    pub r#role: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUserDeleteResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUserListResponse {
    pub r#object: String,

    pub r#data: Vec<ProjectUser>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProjectUserUpdateRequest {
    /// `owner` or `member`
    pub r#role: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RankingOptions {
    /// The ranker to use for the file search.
    pub r#ranker: String,

    /// The score threshold for the file search, a number between 0 and 1.
    pub r#score_threshold: f64,
}

/// A realtime client event.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEvent(pub (/*AnyOf*/));

/// Add a new Item to the Conversation's context, including messages, function
/// calls, and function call responses.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventConversationItemCreate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.create`.
    pub r#type: String,

    /// The ID of the preceding item after which the new item will be inserted.
    pub r#previous_item_id: String,

    pub r#item: RealtimeConversationItem,
}

/// Send this event when you want to remove any item from the conversation
/// history.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventConversationItemDelete {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.delete`.
    pub r#type: String,

    /// The ID of the item to delete.
    pub r#item_id: String,
}

/// Send this event when you want to retrieve the server's representation of a
/// specific item in the conversation history.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventConversationItemRetrieve {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.retrieve`.
    pub r#type: String,

    /// The ID of the item to retrieve.
    pub r#item_id: String,
}

/// Send this event to truncate a previous assistant message’s audio.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventConversationItemTruncate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.truncate`.
    pub r#type: String,

    /// The ID of the assistant message item to truncate.
    pub r#item_id: String,

    /// The index of the content part to truncate.
    pub r#content_index: i64,

    /// Inclusive duration up to which audio is truncated, in milliseconds.
    pub r#audio_end_ms: i64,
}

/// Send this event to append audio bytes to the input audio buffer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventInputAudioBufferAppend {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.append`.
    pub r#type: String,

    /// Base64-encoded audio bytes.
    pub r#audio: String,
}

/// Send this event to clear the audio bytes in the buffer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventInputAudioBufferClear {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.clear`.
    pub r#type: String,
}

/// Send this event to commit the user input audio buffer, which will create a
/// new user message item in the conversation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventInputAudioBufferCommit {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.commit`.
    pub r#type: String,
}

/// **WebRTC Only:** Emit to cut off the current audio response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventOutputAudioBufferClear {
    /// The unique ID of the client event used for error handling.
    pub r#event_id: String,

    /// The event type, must be `output_audio_buffer.clear`.
    pub r#type: String,
}

/// Send this event to cancel an in-progress response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventResponseCancel {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `response.cancel`.
    pub r#type: String,

    /// A specific response ID to cancel - if not provided, will cancel an
    /// in-progress response in the default conversation.
    pub r#response_id: String,
}

/// This event instructs the server to create a Response, which means triggering
/// model inference.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventResponseCreate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `response.create`.
    pub r#type: String,

    pub r#response: RealtimeResponseCreateParams,
}

/// Send this event to update the session’s default configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventSessionUpdate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `session.update`.
    pub r#type: String,

    pub r#session: RealtimeSessionCreateRequest,
}

/// Send this event to update a transcription session.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeClientEventTranscriptionSessionUpdate {
    /// Optional client-generated ID used to identify this event.
    pub r#event_id: String,

    /// The event type, must be `transcription_session.update`.
    pub r#type: String,

    pub r#session: RealtimeTranscriptionSessionCreateRequest,
}

/// The item to add to the conversation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeConversationItem {
    /// The unique ID of the item, this can be generated by the client to help
    /// manage server-side context, but is not required because the server will
    /// generate one if not provided.
    pub r#id: String,

    /// The type of the item (`message`, `function_call`,
    /// `function_call_output`).
    pub r#type: String,

    /// Identifier for the API object being returned - always `realtime.item`.
    pub r#object: String,

    /// The status of the item (`completed`, `incomplete`).
    pub r#status: String,

    /// The role of the message sender (`user`, `assistant`, `system`), only
    /// applicable for `message` items.
    pub r#role: String,

    /// The content of the message, applicable for `message` items.
    pub r#content: Vec<(/*Object*/)>,

    /// The ID of the function call (for `function_call` and
    /// `function_call_output` items).
    pub r#call_id: String,

    /// The name of the function being called (for `function_call` items).
    pub r#name: String,

    /// The arguments of the function call (for `function_call` items).
    pub r#arguments: String,

    /// The output of the function call (for `function_call_output` items).
    pub r#output: String,
}

/// The item to add to the conversation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeConversationItemWithReference {
    /// For an item of type (`message` | `function_call` |
    /// `function_call_output`) this field allows the client to assign the
    /// unique ID of the item.
    pub r#id: String,

    /// The type of the item (`message`, `function_call`,
    /// `function_call_output`, `item_reference`).
    pub r#type: String,

    /// Identifier for the API object being returned - always `realtime.item`.
    pub r#object: String,

    /// The status of the item (`completed`, `incomplete`).
    pub r#status: String,

    /// The role of the message sender (`user`, `assistant`, `system`), only
    /// applicable for `message` items.
    pub r#role: String,

    /// The content of the message, applicable for `message` items.
    pub r#content: Vec<(/*Object*/)>,

    /// The ID of the function call (for `function_call` and
    /// `function_call_output` items).
    pub r#call_id: String,

    /// The name of the function being called (for `function_call` items).
    pub r#name: String,

    /// The arguments of the function call (for `function_call` items).
    pub r#arguments: String,

    /// The output of the function call (for `function_call_output` items).
    pub r#output: String,
}

/// The response resource.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponse {
    /// The unique ID of the response.
    pub r#id: String,

    /// The object type, must be `realtime.response`.
    pub r#object: String,

    /// The final status of the response (`completed`, `cancelled`, `failed`, or
    /// `incomplete`).
    pub r#status: String,

    /// Additional details about the status.
    pub r#status_details: (/*Object*/),

    /// The list of output items generated by the response.
    pub r#output: Vec<RealtimeConversationItem>,

    pub r#metadata: Metadata,

    /// Usage statistics for the Response, this will correspond to billing.
    pub r#usage: (/*Object*/),

    /// Which conversation the response is added to, determined by the
    /// `conversation` field in the `response.create` event.
    pub r#conversation_id: String,

    /// The voice the model used to respond.
    pub r#voice: VoiceIdsShared,

    /// The set of modalities the model used to respond.
    pub r#modalities: Vec<String>,

    /// The format of output audio.
    pub r#output_audio_format: String,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: f64,

    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls, that was used in this response.
    pub r#max_output_tokens: (/*OneOf*/),
}

/// Create a new Realtime response with these parameters
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeResponseCreateParams {
    /// The set of modalities the model can respond with.
    pub r#modalities: Vec<String>,

    /// The default system instructions (i.e.
    pub r#instructions: String,

    /// The voice the model uses to respond.
    pub r#voice: VoiceIdsShared,

    /// The format of output audio.
    pub r#output_audio_format: String,

    /// Tools (functions) available to the model.
    pub r#tools: Vec<(/*Object*/)>,

    /// How the model chooses tools.
    pub r#tool_choice: String,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: f64,

    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls.
    pub r#max_response_output_tokens: (/*OneOf*/),

    /// Controls which conversation the response is added to.
    pub r#conversation: (/*OneOf*/),

    pub r#metadata: Metadata,

    /// Input items to include in the prompt for the model.
    pub r#input: Vec<RealtimeConversationItemWithReference>,
}

/// A realtime server event.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEvent(pub (/*AnyOf*/));

/// Returned when a conversation is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationCreated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.created`.
    pub r#type: String,

    /// The conversation resource.
    pub r#conversation: (/*Object*/),
}

/// Returned when a conversation item is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemCreated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.created`.
    pub r#type: String,

    /// The ID of the preceding item in the Conversation context, allows the
    /// client to understand the order of the conversation.
    pub r#previous_item_id: String,

    pub r#item: RealtimeConversationItem,
}

/// Returned when an item in the conversation is deleted by the client with a
/// `conversation.item.delete` event.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemDeleted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.deleted`.
    pub r#type: String,

    /// The ID of the item that was deleted.
    pub r#item_id: String,
}

/// This event is the output of audio transcription for user audio written to
/// the user audio buffer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be
    /// `conversation.item.input_audio_transcription.completed`.
    pub r#type: String,

    /// The ID of the user message item containing the audio.
    pub r#item_id: String,

    /// The index of the content part containing the audio.
    pub r#content_index: i64,

    /// The transcribed text.
    pub r#transcript: String,

    /// The log probabilities of the transcription.
    pub r#logprobs: Option<Vec<LogProbProperties>>,
}

/// Returned when the text value of an input audio transcription content part is
/// updated.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be
    /// `conversation.item.input_audio_transcription.delta`.
    pub r#type: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The text delta.
    pub r#delta: String,

    /// The log probabilities of the transcription.
    pub r#logprobs: Option<Vec<LogProbProperties>>,
}

/// Returned when input audio transcription is configured, and a transcription
/// request for a user message failed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be
    /// `conversation.item.input_audio_transcription.failed`.
    pub r#type: String,

    /// The ID of the user message item.
    pub r#item_id: String,

    /// The index of the content part containing the audio.
    pub r#content_index: i64,

    /// Details of the transcription error.
    pub r#error: (/*Object*/),
}

/// Returned when a conversation item is retrieved with
/// `conversation.item.retrieve`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemRetrieved {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.retrieved`.
    pub r#type: String,

    pub r#item: RealtimeConversationItem,
}

/// Returned when an earlier assistant audio message item is truncated by the
/// client with a `conversation.item.truncate` event.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventConversationItemTruncated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `conversation.item.truncated`.
    pub r#type: String,

    /// The ID of the assistant message item that was truncated.
    pub r#item_id: String,

    /// The index of the content part that was truncated.
    pub r#content_index: i64,

    /// The duration up to which the audio was truncated, in milliseconds.
    pub r#audio_end_ms: i64,
}

/// Returned when an error occurs, which could be a client problem or a server
/// problem.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventError {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `error`.
    pub r#type: String,

    /// Details of the error.
    pub r#error: (/*Object*/),
}

/// Returned when the input audio buffer is cleared by the client with a
/// `input_audio_buffer.clear` event.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventInputAudioBufferCleared {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.cleared`.
    pub r#type: String,
}

/// Returned when an input audio buffer is committed, either by the client or
/// automatically in server VAD mode.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventInputAudioBufferCommitted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.committed`.
    pub r#type: String,

    /// The ID of the preceding item after which the new item will be inserted.
    pub r#previous_item_id: String,

    /// The ID of the user message item that will be created.
    pub r#item_id: String,
}

/// Sent by the server when in `server_vad` mode to indicate that speech has
/// been detected in the audio buffer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.speech_started`.
    pub r#type: String,

    /// Milliseconds from the start of all audio written to the buffer during
    /// the session when speech was first detected.
    pub r#audio_start_ms: i64,

    /// The ID of the user message item that will be created when speech stops.
    pub r#item_id: String,
}

/// Returned in `server_vad` mode when the server detects the end of speech in
/// the audio buffer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `input_audio_buffer.speech_stopped`.
    pub r#type: String,

    /// Milliseconds since the session started when speech stopped.
    pub r#audio_end_ms: i64,

    /// The ID of the user message item that will be created.
    pub r#item_id: String,
}

/// **WebRTC Only:** Emitted when the output audio buffer is cleared.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventOutputAudioBufferCleared {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `output_audio_buffer.cleared`.
    pub r#type: String,

    /// The unique ID of the response that produced the audio.
    pub r#response_id: String,
}

/// **WebRTC Only:** Emitted when the server begins streaming audio to the
/// client.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventOutputAudioBufferStarted {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `output_audio_buffer.started`.
    pub r#type: String,

    /// The unique ID of the response that produced the audio.
    pub r#response_id: String,
}

/// **WebRTC Only:** Emitted when the output audio buffer has been completely
/// drained on the server, and no more audio is forthcoming.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventOutputAudioBufferStopped {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `output_audio_buffer.stopped`.
    pub r#type: String,

    /// The unique ID of the response that produced the audio.
    pub r#response_id: String,
}

/// Emitted at the beginning of a Response to indicate the updated rate limits.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventRateLimitsUpdated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `rate_limits.updated`.
    pub r#type: String,

    /// List of rate limit information.
    pub r#rate_limits: Vec<(/*Object*/)>,
}

/// Returned when the model-generated audio is updated.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseAudioDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.audio.delta`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// Base64-encoded audio data delta.
    pub r#delta: String,
}

/// Returned when the model-generated audio is done.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseAudioDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.audio.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,
}

/// Returned when the model-generated transcription of audio output is updated.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseAudioTranscriptDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.audio_transcript.delta`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The transcript delta.
    pub r#delta: String,
}

/// Returned when the model-generated transcription of audio output is done
/// streaming.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseAudioTranscriptDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.audio_transcript.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The final transcript of the audio.
    pub r#transcript: String,
}

/// Returned when a new content part is added to an assistant message item
/// during response generation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseContentPartAdded {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.content_part.added`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item to which the content part was added.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The content part that was added.
    pub r#part: (/*Object*/),
}

/// Returned when a content part is done streaming in an assistant message item.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseContentPartDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.content_part.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The content part that is done.
    pub r#part: (/*Object*/),
}

/// Returned when a new Response is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseCreated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.created`.
    pub r#type: String,

    pub r#response: RealtimeResponse,
}

/// Returned when a Response is done streaming.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.done`.
    pub r#type: String,

    pub r#response: RealtimeResponse,
}

/// Returned when the model-generated function call arguments are updated.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.function_call_arguments.delta`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the function call item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The ID of the function call.
    pub r#call_id: String,

    /// The arguments delta as a JSON string.
    pub r#delta: String,
}

/// Returned when the model-generated function call arguments are done
/// streaming.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.function_call_arguments.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the function call item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The ID of the function call.
    pub r#call_id: String,

    /// The final arguments as a JSON string.
    pub r#arguments: String,
}

/// Returned when a new Item is created during Response generation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseOutputItemAdded {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.output_item.added`.
    pub r#type: String,

    /// The ID of the Response to which the item belongs.
    pub r#response_id: String,

    /// The index of the output item in the Response.
    pub r#output_index: i64,

    pub r#item: RealtimeConversationItem,
}

/// Returned when an Item is done streaming.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseOutputItemDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.output_item.done`.
    pub r#type: String,

    /// The ID of the Response to which the item belongs.
    pub r#response_id: String,

    /// The index of the output item in the Response.
    pub r#output_index: i64,

    pub r#item: RealtimeConversationItem,
}

/// Returned when the text value of a "text" content part is updated.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseTextDelta {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.text.delta`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The text delta.
    pub r#delta: String,
}

/// Returned when the text value of a "text" content part is done streaming.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventResponseTextDone {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `response.text.done`.
    pub r#type: String,

    /// The ID of the response.
    pub r#response_id: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item in the response.
    pub r#output_index: i64,

    /// The index of the content part in the item's content array.
    pub r#content_index: i64,

    /// The final text content.
    pub r#text: String,
}

/// Returned when a Session is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventSessionCreated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `session.created`.
    pub r#type: String,

    pub r#session: RealtimeSession,
}

/// Returned when a session is updated with a `session.update` event, unless
/// there is an error.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventSessionUpdated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `session.updated`.
    pub r#type: String,

    pub r#session: RealtimeSession,
}

/// Returned when a transcription session is updated with a
/// `transcription_session.update` event, unless there is an error.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeServerEventTranscriptionSessionUpdated {
    /// The unique ID of the server event.
    pub r#event_id: String,

    /// The event type, must be `transcription_session.updated`.
    pub r#type: String,

    pub r#session: RealtimeTranscriptionSessionCreateResponse,
}

/// Realtime session object configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSession {
    /// Unique identifier for the session that looks like
    /// `sess_1234567890abcdef`.
    pub r#id: String,

    /// The set of modalities the model can respond with.
    pub r#modalities: Vec<String>,

    /// The Realtime model used for this session.
    pub r#model: String,

    /// The default system instructions (i.e.
    pub r#instructions: String,

    /// The voice the model uses to respond.
    pub r#voice: VoiceIdsShared,

    /// The format of input audio.
    pub r#input_audio_format: String,

    /// The format of output audio.
    pub r#output_audio_format: String,

    /// Configuration for input audio transcription, defaults to off and can be 
    /// set to `null` to turn off once on.
    pub r#input_audio_transcription: (/*Object*/),

    /// Configuration for turn detection, ether Server VAD or Semantic VAD.
    pub r#turn_detection: (/*Object*/),

    /// Configuration for input audio noise reduction.
    pub r#input_audio_noise_reduction: (/*Object*/),

    /// Tools (functions) available to the model.
    pub r#tools: Vec<(/*Object*/)>,

    /// How the model chooses tools.
    pub r#tool_choice: String,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: f64,

    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls.
    pub r#max_response_output_tokens: (/*OneOf*/),
}

/// Realtime session object configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateRequest {
    /// The set of modalities the model can respond with.
    pub r#modalities: Vec<String>,

    /// The Realtime model used for this session.
    pub r#model: String,

    /// The default system instructions (i.e.
    pub r#instructions: String,

    /// The voice the model uses to respond.
    pub r#voice: VoiceIdsShared,

    /// The format of input audio.
    pub r#input_audio_format: String,

    /// The format of output audio.
    pub r#output_audio_format: String,

    /// Configuration for input audio transcription, defaults to off and can be 
    /// set to `null` to turn off once on.
    pub r#input_audio_transcription: (/*Object*/),

    /// Configuration for turn detection, ether Server VAD or Semantic VAD.
    pub r#turn_detection: (/*Object*/),

    /// Configuration for input audio noise reduction.
    pub r#input_audio_noise_reduction: (/*Object*/),

    /// Tools (functions) available to the model.
    pub r#tools: Vec<(/*Object*/)>,

    /// How the model chooses tools.
    pub r#tool_choice: String,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: f64,

    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls.
    pub r#max_response_output_tokens: (/*OneOf*/),
}

/// A new Realtime session configuration, with an ephermeral key.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeSessionCreateResponse {
    /// Ephemeral key returned by the API.
    pub r#client_secret: (/*Object*/),

    /// The set of modalities the model can respond with.
    pub r#modalities: Vec<String>,

    /// The default system instructions (i.e.
    pub r#instructions: String,

    /// The voice the model uses to respond.
    pub r#voice: VoiceIdsShared,

    /// The format of input audio.
    pub r#input_audio_format: String,

    /// The format of output audio.
    pub r#output_audio_format: String,

    /// Configuration for input audio transcription, defaults to off and can be
    /// set to `null` to turn off once on.
    pub r#input_audio_transcription: (/*Object*/),

    /// Configuration for turn detection.
    pub r#turn_detection: (/*Object*/),

    /// Tools (functions) available to the model.
    pub r#tools: Vec<(/*Object*/)>,

    /// How the model chooses tools.
    pub r#tool_choice: String,

    /// Sampling temperature for the model, limited to [0.6, 1.2].
    pub r#temperature: f64,

    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls.
    pub r#max_response_output_tokens: (/*OneOf*/),
}

/// Realtime transcription session object configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateRequest {
    /// The set of modalities the model can respond with.
    pub r#modalities: Vec<String>,

    /// The format of input audio.
    pub r#input_audio_format: String,

    /// Configuration for input audio transcription.
    pub r#input_audio_transcription: (/*Object*/),

    /// Configuration for turn detection, ether Server VAD or Semantic VAD.
    pub r#turn_detection: (/*Object*/),

    /// Configuration for input audio noise reduction.
    pub r#input_audio_noise_reduction: (/*Object*/),

    /// The set of items to include in the transcription.
    pub r#include: Vec<String>,
}

/// A new Realtime transcription session configuration.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealtimeTranscriptionSessionCreateResponse {
    /// Ephemeral key returned by the API.
    pub r#client_secret: (/*Object*/),

    /// The set of modalities the model can respond with.
    pub r#modalities: Vec<String>,

    /// The format of input audio.
    pub r#input_audio_format: String,

    /// Configuration of the transcription model.
    pub r#input_audio_transcription: (/*Object*/),

    /// Configuration for turn detection.
    pub r#turn_detection: (/*Object*/),
}

/// **o-series models only** Configuration options for [reasoning
/// models](https://platform.openai.com/docs/guides/reasoning).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reasoning {
    pub r#effort: ReasoningEffort,

    /// A summary of the reasoning performed by the model.
    pub r#summary: Option<String>,

    /// **Deprecated:** use `summary` instead.
    pub r#generate_summary: Option<String>,
}

/// **o-series models only** Constrains effort on reasoning for [reasoning
/// models](https://platform.openai.com/docs/guides/reasoning).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReasoningEffort(pub String);

/// A description of the chain of thought used by a reasoning model while
/// generating a response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReasoningItem {
    /// The type of the object.
    pub r#type: String,

    /// The unique identifier of the reasoning content.
    pub r#id: String,

    /// Reasoning text contents.
    pub r#summary: Vec<(/*Object*/)>,

    /// The status of the item.
    pub r#status: String,
}

/// A refusal from the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefusalContent {
    /// The type of the refusal.
    pub r#type: String,

    /// The refusal explanationfrom the model.
    pub r#refusal: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Response(pub (/*AllOf*/));

/// Emitted when there is a partial audio response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseAudioDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// A chunk of Base64 encoded response audio bytes.
    pub r#delta: String,
}

/// Emitted when the audio response is complete.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseAudioDoneEvent {
    /// The type of the event.
    pub r#type: String,
}

/// Emitted when there is a partial transcript of audio.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseAudioTranscriptDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The partial transcript of the audio response.
    pub r#delta: String,
}

/// Emitted when the full audio transcript is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseAudioTranscriptDoneEvent {
    /// The type of the event.
    pub r#type: String,
}

/// Emitted when a partial code snippet is added by the code interpreter.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    /// The partial code snippet added by the code interpreter.
    pub r#delta: String,
}

/// Emitted when code snippet output is finalized by the code interpreter.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallCodeDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    /// The final code snippet output by the code interpreter.
    pub r#code: String,
}

/// Emitted when the code interpreter call is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallCompletedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    pub r#code_interpreter_call: CodeInterpreterToolCall,
}

/// Emitted when a code interpreter call is in progress.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallInProgressEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    pub r#code_interpreter_call: CodeInterpreterToolCall,
}

/// Emitted when the code interpreter is actively interpreting the code snippet.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCodeInterpreterCallInterpretingEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the code interpreter call is in
    /// progress.
    pub r#output_index: i64,

    pub r#code_interpreter_call: CodeInterpreterToolCall,
}

/// Emitted when the model response is complete.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCompletedEvent {
    /// The type of the event.
    pub r#type: String,

    /// Properties of the completed response.
    pub r#response: Response,
}

/// Emitted when a new content part is added.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseContentPartAddedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the content part was added to.
    pub r#item_id: String,

    /// The index of the output item that the content part was added to.
    pub r#output_index: i64,

    /// The index of the content part that was added.
    pub r#content_index: i64,

    /// The content part that was added.
    pub r#part: OutputContent,
}

/// Emitted when a content part is done.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseContentPartDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the content part was added to.
    pub r#item_id: String,

    /// The index of the output item that the content part was added to.
    pub r#output_index: i64,

    /// The index of the content part that is done.
    pub r#content_index: i64,

    /// The content part that is done.
    pub r#part: OutputContent,
}

/// An event that is emitted when a response is created.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseCreatedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The response that was created.
    pub r#response: Response,
}

/// An error object returned when the model fails to generate a Response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseError {
    pub r#code: ResponseErrorCode,

    /// A human-readable description of the error.
    pub r#message: String,
}

/// The error code for the response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseErrorCode(pub String);

/// Emitted when an error occurs.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseErrorEvent {
    /// The type of the event.
    pub r#type: String,

    /// The error code.
    pub r#code: Option<String>,

    /// The error message.
    pub r#message: String,

    /// The error parameter.
    pub r#param: Option<String>,
}

/// An event that is emitted when a response fails.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFailedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The response that failed.
    pub r#response: Response,
}

/// Emitted when a file search call is completed (results found).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFileSearchCallCompletedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the file search call is initiated.
    pub r#output_index: i64,

    /// The ID of the output item that the file search call is initiated.
    pub r#item_id: String,
}

/// Emitted when a file search call is initiated.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFileSearchCallInProgressEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the file search call is initiated.
    pub r#output_index: i64,

    /// The ID of the output item that the file search call is initiated.
    pub r#item_id: String,
}

/// Emitted when a file search is currently searching.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFileSearchCallSearchingEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the file search call is searching.
    pub r#output_index: i64,

    /// The ID of the output item that the file search call is initiated.
    pub r#item_id: String,
}

/// JSON object response format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFormatJsonObject {
    /// The type of response format being defined.
    pub r#type: String,
}

/// JSON Schema response format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFormatJsonSchema {
    /// The type of response format being defined.
    pub r#type: String,

    /// Structured Outputs configuration options, including a JSON Schema.
    pub r#json_schema: (/*Object*/),
}

/// The schema for the response format, described as a JSON Schema object.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFormatJsonSchemaSchema;

/// Default response format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFormatText {
    /// The type of response format being defined.
    pub r#type: String,
}

/// Emitted when there is a partial function-call arguments delta.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the function-call arguments delta is
    /// added to.
    pub r#item_id: String,

    /// The index of the output item that the function-call arguments delta is
    /// added to.
    pub r#output_index: i64,

    /// The function-call arguments delta that is added.
    pub r#delta: String,
}

/// Emitted when function-call arguments are finalized.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseFunctionCallArgumentsDoneEvent {
    pub r#type: String,

    /// The ID of the item.
    pub r#item_id: String,

    /// The index of the output item.
    pub r#output_index: i64,

    /// The function-call arguments.
    pub r#arguments: String,
}

/// Emitted when the response is in progress.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseInProgressEvent {
    /// The type of the event.
    pub r#type: String,

    /// The response that is in progress.
    pub r#response: Response,
}

/// An event that is emitted when a response finishes as incomplete.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseIncompleteEvent {
    /// The type of the event.
    pub r#type: String,

    /// The response that was incomplete.
    pub r#response: Response,
}

/// A list of Response items.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseItemList {
    /// The type of object returned, must be `list`.
    pub r#object: String,

    /// A list of items used to generate this response.
    pub r#data: Vec<ItemResource>,

    /// Whether there are more items available.
    pub r#has_more: bool,

    /// The ID of the first item in the list.
    pub r#first_id: String,

    /// The ID of the last item in the list.
    pub r#last_id: String,
}

/// Output types that you would like the model to generate.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseModalities(pub Vec<String>);

/// Emitted when a new output item is added.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseOutputItemAddedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that was added.
    pub r#output_index: i64,

    /// The output item that was added.
    pub r#item: OutputItem,
}

/// Emitted when an output item is marked done.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseOutputItemDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that was marked done.
    pub r#output_index: i64,

    /// The output item that was marked done.
    pub r#item: OutputItem,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseProperties {
    /// The unique ID of the previous response to the model.
    pub r#previous_response_id: Option<String>,

    /// Model ID used to generate the response, like `gpt-4o` or `o3`.
    pub r#model: ModelIdsResponses,

    pub r#reasoning: Option<Reasoning>,

    /// An upper bound for the number of tokens that can be generated for a
    /// response, including visible output tokens and [reasoning
    /// tokens](/docs/guides/reasoning).
    pub r#max_output_tokens: Option<i64>,

    /// Inserts a system (or developer) message as the first item in the model's
    /// context.
    pub r#instructions: Option<String>,

    /// Configuration options for a text response from the model.
    pub r#text: (/*Object*/),

    /// An array of tools the model may call while generating a response.
    pub r#tools: Vec<Tool>,

    /// How the model should select which tool (or tools) to use when generating
    /// a response.
    pub r#tool_choice: (/*OneOf*/),

    /// The truncation strategy to use for the model response.
    pub r#truncation: Option<String>,
}

/// Emitted when a new reasoning summary part is added.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryPartAddedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the item this summary part is associated with.
    pub r#item_id: String,

    /// The index of the output item this summary part is associated with.
    pub r#output_index: i64,

    /// The index of the summary part within the reasoning summary.
    pub r#summary_index: i64,

    /// The summary part that was added.
    pub r#part: (/*Object*/),
}

/// Emitted when a reasoning summary part is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryPartDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the item this summary part is associated with.
    pub r#item_id: String,

    /// The index of the output item this summary part is associated with.
    pub r#output_index: i64,

    /// The index of the summary part within the reasoning summary.
    pub r#summary_index: i64,

    /// The completed summary part.
    pub r#part: (/*Object*/),
}

/// Emitted when a delta is added to a reasoning summary text.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryTextDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the item this summary text delta is associated with.
    pub r#item_id: String,

    /// The index of the output item this summary text delta is associated with.
    pub r#output_index: i64,

    /// The index of the summary part within the reasoning summary.
    pub r#summary_index: i64,

    /// The text delta that was added to the summary.
    pub r#delta: String,
}

/// Emitted when a reasoning summary text is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseReasoningSummaryTextDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the item this summary text is associated with.
    pub r#item_id: String,

    /// The index of the output item this summary text is associated with.
    pub r#output_index: i64,

    /// The index of the summary part within the reasoning summary.
    pub r#summary_index: i64,

    /// The full text of the completed reasoning summary.
    pub r#text: String,
}

/// Emitted when there is a partial refusal text.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseRefusalDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the refusal text is added to.
    pub r#item_id: String,

    /// The index of the output item that the refusal text is added to.
    pub r#output_index: i64,

    /// The index of the content part that the refusal text is added to.
    pub r#content_index: i64,

    /// The refusal text that is added.
    pub r#delta: String,
}

/// Emitted when refusal text is finalized.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseRefusalDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the refusal text is finalized.
    pub r#item_id: String,

    /// The index of the output item that the refusal text is finalized.
    pub r#output_index: i64,

    /// The index of the content part that the refusal text is finalized.
    pub r#content_index: i64,

    /// The refusal text that is finalized.
    pub r#refusal: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseStreamEvent(pub (/*AnyOf*/));

/// Emitted when a text annotation is added.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseTextAnnotationDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the text annotation was added to.
    pub r#item_id: String,

    /// The index of the output item that the text annotation was added to.
    pub r#output_index: i64,

    /// The index of the content part that the text annotation was added to.
    pub r#content_index: i64,

    /// The index of the annotation that was added.
    pub r#annotation_index: i64,

    pub r#annotation: Annotation,
}

/// Emitted when there is an additional text delta.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseTextDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the text delta was added to.
    pub r#item_id: String,

    /// The index of the output item that the text delta was added to.
    pub r#output_index: i64,

    /// The index of the content part that the text delta was added to.
    pub r#content_index: i64,

    /// The text delta that was added.
    pub r#delta: String,
}

/// Emitted when text content is finalized.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseTextDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The ID of the output item that the text content is finalized.
    pub r#item_id: String,

    /// The index of the output item that the text content is finalized.
    pub r#output_index: i64,

    /// The index of the content part that the text content is finalized.
    pub r#content_index: i64,

    /// The text content that is finalized.
    pub r#text: String,
}

/// Represents token usage details including input tokens, output tokens, a
/// breakdown of output tokens, and the total tokens used.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseUsage {
    /// The number of input tokens.
    pub r#input_tokens: i64,

    /// A detailed breakdown of the input tokens.
    pub r#input_tokens_details: (/*Object*/),

    /// The number of output tokens.
    pub r#output_tokens: i64,

    /// A detailed breakdown of the output tokens.
    pub r#output_tokens_details: (/*Object*/),

    /// The total number of tokens used.
    pub r#total_tokens: i64,
}

/// Emitted when a web search call is completed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseWebSearchCallCompletedEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the web search call is associated
    /// with.
    pub r#output_index: i64,

    /// Unique ID for the output item associated with the web search call.
    pub r#item_id: String,
}

/// Emitted when a web search call is initiated.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseWebSearchCallInProgressEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the web search call is associated
    /// with.
    pub r#output_index: i64,

    /// Unique ID for the output item associated with the web search call.
    pub r#item_id: String,
}

/// Emitted when a web search call is executing.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResponseWebSearchCallSearchingEvent {
    /// The type of the event.
    pub r#type: String,

    /// The index of the output item that the web search call is associated
    /// with.
    pub r#output_index: i64,

    /// Unique ID for the output item associated with the web search call.
    pub r#item_id: String,
}

/// Usage statistics related to the run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunCompletionUsage {
    /// Number of completion tokens used over the course of the run.
    pub r#completion_tokens: i64,

    /// Number of prompt tokens used over the course of the run.
    pub r#prompt_tokens: i64,

    /// Total number of tokens used (prompt + completion).
    pub r#total_tokens: i64,
}

/// Represents an execution run on a [thread](/docs/api-reference/threads).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.run`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the run was created.
    pub r#created_at: i64,

    /// The ID of the [thread](/docs/api-reference/threads) that was executed on
    /// as a part of this run.
    pub r#thread_id: String,

    /// The ID of the [assistant](/docs/api-reference/assistants) used for
    /// execution of this run.
    pub r#assistant_id: String,

    /// The status of the run, which can be either `queued`, `in_progress`,
    /// `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`,
    /// `incomplete`, or `expired`.
    pub r#status: String,

    /// Details on the action required to continue the run.
    pub r#required_action: Option<(/*Object*/)>,

    /// The last error associated with this run.
    pub r#last_error: Option<(/*Object*/)>,

    /// The Unix timestamp (in seconds) for when the run will expire.
    pub r#expires_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run was started.
    pub r#started_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run was cancelled.
    pub r#cancelled_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run failed.
    pub r#failed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run was completed.
    pub r#completed_at: Option<i64>,

    /// Details on why the run is incomplete.
    pub r#incomplete_details: Option<(/*Object*/)>,

    /// The model that the [assistant](/docs/api-reference/assistants) used for
    /// this run.
    pub r#model: String,

    /// The instructions that the [assistant](/docs/api-reference/assistants)
    /// used for this run.
    pub r#instructions: String,

    /// The list of tools that the [assistant](/docs/api-reference/assistants)
    /// used for this run.
    pub r#tools: Vec<(/*OneOf*/)>,

    pub r#metadata: Metadata,

    pub r#usage: RunCompletionUsage,

    /// The sampling temperature used for this run.
    pub r#temperature: Option<f64>,

    /// The nucleus sampling value used for this run.
    pub r#top_p: Option<f64>,

    /// The maximum number of prompt tokens specified to have been used over the
    /// course of the run.
    pub r#max_prompt_tokens: Option<i64>,

    /// The maximum number of completion tokens specified to have been used over
    /// the course of the run.
    pub r#max_completion_tokens: Option<i64>,

    pub r#truncation_strategy: (/*AllOf*/),

    pub r#tool_choice: (/*AllOf*/),

    pub r#parallel_tool_calls: ParallelToolCalls,

    pub r#response_format: Option<AssistantsApiResponseFormatOption>,
}

/// Usage statistics related to the run step.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepCompletionUsage {
    /// Number of completion tokens used over the course of the run step.
    pub r#completion_tokens: i64,

    /// Number of prompt tokens used over the course of the run step.
    pub r#prompt_tokens: i64,

    /// Total number of tokens used (prompt + completion).
    pub r#total_tokens: i64,
}

/// Represents a run step delta i.e.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaObject {
    /// The identifier of the run step, which can be referenced in API
    /// endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.run.step.delta`.
    pub r#object: String,

    /// The delta containing the fields that have changed on the run step.
    pub r#delta: (/*Object*/),
}

/// Details of the message creation by the run step.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsMessageCreationObject {
    /// Always `message_creation`.
    pub r#type: String,

    pub r#message_creation: (/*Object*/),
}

/// Details of the Code Interpreter tool call the run step was involved in.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsCodeObject {
    /// The index of the tool call in the tool calls array.
    pub r#index: i64,

    /// The ID of the tool call.
    pub r#id: String,

    /// The type of tool call.
    pub r#type: String,

    /// The Code Interpreter tool call definition.
    pub r#code_interpreter: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
    /// The index of the output in the outputs array.
    pub r#index: i64,

    /// Always `image`.
    pub r#type: String,

    pub r#image: (/*Object*/),
}

/// Text output from the Code Interpreter tool call as part of a run step.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
    /// The index of the output in the outputs array.
    pub r#index: i64,

    /// Always `logs`.
    pub r#type: String,

    /// The text output from the Code Interpreter tool call.
    pub r#logs: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject {
    /// The index of the tool call in the tool calls array.
    pub r#index: i64,

    /// The ID of the tool call object.
    pub r#id: String,

    /// The type of tool call.
    pub r#type: String,

    /// For now, this is always going to be an empty object.
    pub r#file_search: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObject {
    /// The index of the tool call in the tool calls array.
    pub r#index: i64,

    /// The ID of the tool call object.
    pub r#id: String,

    /// The type of tool call.
    pub r#type: String,

    /// The definition of the function that was called.
    pub r#function: (/*Object*/),
}

/// Details of the tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDeltaStepDetailsToolCallsObject {
    /// Always `tool_calls`.
    pub r#type: String,

    /// An array of tool calls the run step was involved in.
    pub r#tool_calls: Vec<(/*OneOf*/)>,
}

/// Details of the message creation by the run step.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsMessageCreationObject {
    /// Always `message_creation`.
    pub r#type: String,

    pub r#message_creation: (/*Object*/),
}

/// Details of the Code Interpreter tool call the run step was involved in.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsCodeObject {
    /// The ID of the tool call.
    pub r#id: String,

    /// The type of tool call.
    pub r#type: String,

    /// The Code Interpreter tool call definition.
    pub r#code_interpreter: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsCodeOutputImageObject {
    /// Always `image`.
    pub r#type: String,

    pub r#image: (/*Object*/),
}

/// Text output from the Code Interpreter tool call as part of a run step.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsCodeOutputLogsObject {
    /// Always `logs`.
    pub r#type: String,

    /// The text output from the Code Interpreter tool call.
    pub r#logs: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFileSearchObject {
    /// The ID of the tool call object.
    pub r#id: String,

    /// The type of tool call.
    pub r#type: String,

    /// For now, this is always going to be an empty object.
    pub r#file_search: (/*Object*/),
}

/// The ranking options for the file search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFileSearchRankingOptionsObject {
    pub r#ranker: FileSearchRanker,

    /// The score threshold for the file search.
    pub r#score_threshold: f64,
}

/// A result instance of the file search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFileSearchResultObject {
    /// The ID of the file that result was found in.
    pub r#file_id: String,

    /// The name of the file that result was found in.
    pub r#file_name: String,

    /// The score of the result.
    pub r#score: f64,

    /// The content of the result that was found.
    pub r#content: Vec<(/*Object*/)>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsFunctionObject {
    /// The ID of the tool call object.
    pub r#id: String,

    /// The type of tool call.
    pub r#type: String,

    /// The definition of the function that was called.
    pub r#function: (/*Object*/),
}

/// Details of the tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepDetailsToolCallsObject {
    /// Always `tool_calls`.
    pub r#type: String,

    /// An array of tool calls the run step was involved in.
    pub r#tool_calls: Vec<(/*OneOf*/)>,
}

/// Represents a step in execution of a run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunStepObject {
    /// The identifier of the run step, which can be referenced in API
    /// endpoints.
    pub r#id: String,

    /// The object type, which is always `thread.run.step`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the run step was created.
    pub r#created_at: i64,

    /// The ID of the [assistant](/docs/api-reference/assistants) associated
    /// with the run step.
    pub r#assistant_id: String,

    /// The ID of the [thread](/docs/api-reference/threads) that was run.
    pub r#thread_id: String,

    /// The ID of the [run](/docs/api-reference/runs) that this run step is a
    /// part of.
    pub r#run_id: String,

    /// The type of run step, which can be either `message_creation` or
    /// `tool_calls`.
    pub r#type: String,

    /// The status of the run step, which can be either `in_progress`,
    /// `cancelled`, `failed`, `completed`, or `expired`.
    pub r#status: String,

    /// The details of the run step.
    pub r#step_details: (/*OneOf*/),

    /// The last error associated with this run step.
    pub r#last_error: Option<(/*Object*/)>,

    /// The Unix timestamp (in seconds) for when the run step expired.
    pub r#expired_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run step was cancelled.
    pub r#cancelled_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run step failed.
    pub r#failed_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the run step completed.
    pub r#completed_at: Option<i64>,

    pub r#metadata: Metadata,

    pub r#usage: RunStepCompletionUsage,
}

include!("components/run_step_stream_event.rs");

include!("components/run_stream_event.rs");

/// Tool call objects
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RunToolCallObject {
    /// The ID of the tool call.
    pub r#id: String,

    /// The type of tool call the output is required for.
    pub r#type: String,

    /// The function definition.
    pub r#function: (/*Object*/),
}

/// A screenshot action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Screenshot {
    /// Specifies the event type.
    pub r#type: String,
}

/// A scroll action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scroll {
    /// Specifies the event type.
    pub r#type: String,

    /// The x-coordinate where the scroll occurred.
    pub r#x: i64,

    /// The y-coordinate where the scroll occurred.
    pub r#y: i64,

    /// The horizontal scroll distance.
    pub r#scroll_x: i64,

    /// The vertical scroll distance.
    pub r#scroll_y: i64,
}

/// Specifies the latency tier to use for processing the request.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ServiceTier(pub String);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StaticChunkingStrategy {
    /// The maximum number of tokens in each chunk.
    pub r#max_chunk_size_tokens: i64,

    /// The number of tokens that overlap between chunks.
    pub r#chunk_overlap_tokens: i64,
}

/// Customize your own chunking strategy by setting chunk size and chunk
/// overlap.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StaticChunkingStrategyRequestParam {
    /// Always `static`.
    pub r#type: String,

    pub r#static: StaticChunkingStrategy,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StaticChunkingStrategyResponseParam {
    /// Always `static`.
    pub r#type: String,

    pub r#static: StaticChunkingStrategy,
}

/// Not supported with latest reasoning models `o3` and `o4-mini`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StopConfiguration {
    Null,

    String(Option<String>),

    Array(Vec<String>),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubmitToolOutputsRunRequest {
    /// A list of tools for which the outputs are being submitted.
    pub r#tool_outputs: Vec<(/*Object*/)>,

    /// If `true`, returns a stream of events that happen during the Run as
    /// server-sent events, terminating when the Run enters a terminal state
    /// with a `data: [DONE]` message.
    pub r#stream: Option<bool>,
}

/// An object specifying the format that the model must output.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextResponseFormatConfiguration {
    ResponseFormatText(ResponseFormatText),

    TextResponseFormatJsonSchema(TextResponseFormatJsonSchema),

    ResponseFormatJsonObject(ResponseFormatJsonObject),
}

/// JSON Schema response format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextResponseFormatJsonSchema {
    /// The type of response format being defined.
    pub r#type: String,

    /// A description of what the response format is for, used by the model to
    /// determine how to respond in the format.
    pub r#description: String,

    /// The name of the response format.
    pub r#name: String,

    pub r#schema: ResponseFormatJsonSchemaSchema,

    /// Whether to enable strict schema adherence when generating the output.
    pub r#strict: Option<bool>,
}

/// Represents a thread that contains [messages](/docs/api-reference/messages).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThreadObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `thread`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the thread was created.
    pub r#created_at: i64,

    /// A set of resources that are made available to the assistant's tools in
    /// this thread.
    pub r#tool_resources: Option<(/*Object*/)>,

    pub r#metadata: Metadata,
}

include!("components/thread_stream_event.rs");

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToggleCertificatesRequest {
    pub r#certificate_ids: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Tool {
    FileSearchTool(FileSearchTool),

    FunctionTool(FunctionTool),

    WebSearchPreviewTool(WebSearchPreviewTool),

    ComputerUsePreviewTool(ComputerUsePreviewTool),
}

/// Use this option to force the model to call a specific function.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToolChoiceFunction {
    /// For function calling, the type is always `function`.
    pub r#type: String,

    /// The name of the function to call.
    pub r#name: String,
}

/// Controls which (if any) tool is called by the model.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToolChoiceOptions(pub String);

/// Indicates that the model should use a built-in tool to generate a response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToolChoiceTypes {
    /// The type of hosted tool the model should to use.
    pub r#type: String,
}

/// Emitted when there is an additional text delta.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptTextDeltaEvent {
    /// The type of the event.
    pub r#type: String,

    /// The text delta that was additionally transcribed.
    pub r#delta: String,

    /// The log probabilities of the delta.
    pub r#logprobs: Vec<(/*Object*/)>,
}

/// Emitted when the transcription is complete.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptTextDoneEvent {
    /// The type of the event.
    pub r#type: String,

    /// The text that was transcribed.
    pub r#text: String,

    /// The log probabilities of the individual tokens in the transcription.
    pub r#logprobs: Vec<(/*Object*/)>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptionInclude(pub String);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptionSegment {
    /// Unique identifier of the segment.
    pub r#id: i64,

    /// Seek offset of the segment.
    pub r#seek: i64,

    /// Start time of the segment in seconds.
    pub r#start: f64,

    /// End time of the segment in seconds.
    pub r#end: f64,

    /// Text content of the segment.
    pub r#text: String,

    /// Array of token IDs for the text content.
    pub r#tokens: Vec<i64>,

    /// Temperature parameter used for generating the segment.
    pub r#temperature: f64,

    /// Average logprob of the segment.
    pub r#avg_logprob: f64,

    /// Compression ratio of the segment.
    pub r#compression_ratio: f64,

    /// Probability of no speech in the segment.
    pub r#no_speech_prob: f64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranscriptionWord {
    /// The text content of the word.
    pub r#word: String,

    /// Start time of the word in seconds.
    pub r#start: f64,

    /// End time of the word in seconds.
    pub r#end: f64,
}

/// Controls for how a thread will be truncated prior to the run.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TruncationObject {
    /// The truncation strategy to use for the thread.
    pub r#type: String,

    /// The number of most recent messages from the thread when constructing the
    /// context for the run.
    pub r#last_messages: Option<i64>,
}

/// An action to type in text.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Type {
    /// Specifies the event type.
    pub r#type: String,

    /// The text to type.
    pub r#text: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateVectorStoreFileAttributesRequest {
    pub r#attributes: VectorStoreFileAttributes,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateVectorStoreRequest {
    /// The name of the vector store.
    pub r#name: Option<String>,

    pub r#expires_after: (/*AllOf*/),

    pub r#metadata: Metadata,
}

/// The Upload object can accept byte chunks in the form of Parts.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Upload {
    /// The Upload unique identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the Upload was created.
    pub r#created_at: i64,

    /// The name of the file to be uploaded.
    pub r#filename: String,

    /// The intended number of bytes to be uploaded.
    pub r#bytes: i64,

    /// The intended purpose of the file.
    pub r#purpose: String,

    /// The status of the Upload.
    pub r#status: String,

    /// The Unix timestamp (in seconds) for when the Upload will expire.
    pub r#expires_at: i64,

    /// The object type, which is always "upload".
    pub r#object: String,

    pub r#file: (/*AllOf*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UploadCertificateRequest {
    /// An optional name for the certificate
    pub r#name: String,

    /// The certificate content in PEM format
    pub r#content: String,
}

/// The upload Part represents a chunk of bytes we can add to an Upload object.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UploadPart {
    /// The upload Part unique identifier, which can be referenced in API
    /// endpoints.
    pub r#id: String,

    /// The Unix timestamp (in seconds) for when the Part was created.
    pub r#created_at: i64,

    /// The ID of the Upload object that this Part was added to.
    pub r#upload_id: String,

    /// The object type, which is always `upload.part`.
    pub r#object: String,
}

/// A citation for a web resource used to generate a model response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UrlCitationBody {
    /// The type of the URL citation.
    pub r#type: String,

    /// The URL of the web resource.
    pub r#url: String,

    /// The index of the first character of the URL citation in the message.
    pub r#start_index: i64,

    /// The index of the last character of the URL citation in the message.
    pub r#end_index: i64,

    /// The title of the web resource.
    pub r#title: String,
}

/// The aggregated audio speeches usage details of the specific time bucket.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageAudioSpeechesResult {
    pub r#object: String,

    /// The number of characters processed.
    pub r#characters: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

/// The aggregated audio transcriptions usage details of the specific time
/// bucket.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageAudioTranscriptionsResult {
    pub r#object: String,

    /// The number of seconds processed.
    pub r#seconds: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

/// The aggregated code interpreter sessions usage details of the specific time
/// bucket.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageCodeInterpreterSessionsResult {
    pub r#object: String,

    /// The number of code interpreter sessions.
    pub r#num_sessions: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,
}

/// The aggregated completions usage details of the specific time bucket.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageCompletionsResult {
    pub r#object: String,

    /// The aggregated number of text input tokens used, including cached
    /// tokens.
    pub r#input_tokens: i64,

    /// The aggregated number of text input tokens that has been cached from
    /// previous requests.
    pub r#input_cached_tokens: i64,

    /// The aggregated number of text output tokens used.
    pub r#output_tokens: i64,

    /// The aggregated number of audio input tokens used, including cached
    /// tokens.
    pub r#input_audio_tokens: i64,

    /// The aggregated number of audio output tokens used.
    pub r#output_audio_tokens: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,

    /// When `group_by=batch`, this field tells whether the grouped usage result
    /// is batch or not.
    pub r#batch: Option<bool>,
}

/// The aggregated embeddings usage details of the specific time bucket.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageEmbeddingsResult {
    pub r#object: String,

    /// The aggregated number of input tokens used.
    pub r#input_tokens: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

/// The aggregated images usage details of the specific time bucket.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageImagesResult {
    pub r#object: String,

    /// The number of images processed.
    pub r#images: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=source`, this field provides the source of the grouped
    /// usage result, possible values are `image.generation`, `image.edit`,
    /// `image.variation`.
    pub r#source: Option<String>,

    /// When `group_by=size`, this field provides the image size of the grouped
    /// usage result.
    pub r#size: Option<String>,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

/// The aggregated moderations usage details of the specific time bucket.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageModerationsResult {
    pub r#object: String,

    /// The aggregated number of input tokens used.
    pub r#input_tokens: i64,

    /// The count of requests made to the model.
    pub r#num_model_requests: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,

    /// When `group_by=user_id`, this field provides the user ID of the grouped
    /// usage result.
    pub r#user_id: Option<String>,

    /// When `group_by=api_key_id`, this field provides the API key ID of the
    /// grouped usage result.
    pub r#api_key_id: Option<String>,

    /// When `group_by=model`, this field provides the model name of the grouped
    /// usage result.
    pub r#model: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageResponse {
    pub r#object: String,

    pub r#data: Vec<UsageTimeBucket>,

    pub r#has_more: bool,

    pub r#next_page: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageTimeBucket {
    pub r#object: String,

    pub r#start_time: i64,

    pub r#end_time: i64,

    pub r#result: Vec<(/*OneOf*/)>,
}

/// The aggregated vector stores usage details of the specific time bucket.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageVectorStoresResult {
    pub r#object: String,

    /// The vector stores usage in bytes.
    pub r#usage_bytes: i64,

    /// When `group_by=project_id`, this field provides the project ID of the
    /// grouped usage result.
    pub r#project_id: Option<String>,
}

/// Represents an individual `user` within an organization.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct User {
    /// The object type, which is always `organization.user`
    pub r#object: String,

    /// The identifier, which can be referenced in API endpoints
    pub r#id: String,

    /// The name of the user
    pub r#name: String,

    /// The email address of the user
    pub r#email: String,

    /// `owner` or `reader`
    pub r#role: String,

    /// The Unix timestamp (in seconds) of when the user was added.
    pub r#added_at: i64,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserDeleteResponse {
    pub r#object: String,

    pub r#id: String,

    pub r#deleted: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserListResponse {
    pub r#object: String,

    pub r#data: Vec<User>,

    pub r#first_id: String,

    pub r#last_id: String,

    pub r#has_more: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserRoleUpdateRequest {
    /// `owner` or `reader`
    pub r#role: String,
}

/// The expiration policy for a vector store.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreExpirationAfter {
    /// Anchor timestamp after which the expiration policy applies.
    pub r#anchor: String,

    /// The number of days after the anchor time that the vector store will
    /// expire.
    pub r#days: i64,
}

/// Set of 16 key-value pairs that can be attached to an object.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileAttributes;

/// A batch of files attached to a vector store.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileBatchObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `vector_store.file_batch`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the vector store files batch
    /// was created.
    pub r#created_at: i64,

    /// The ID of the [vector store](/docs/api-reference/vector-stores/object)
    /// that the [File](/docs/api-reference/files) is attached to.
    pub r#vector_store_id: String,

    /// The status of the vector store files batch, which can be either
    /// `in_progress`, `completed`, `cancelled` or `failed`.
    pub r#status: String,

    pub r#file_counts: (/*Object*/),
}

/// Represents the parsed content of a vector store file.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileContentResponse {
    /// The object type, which is always `vector_store.file_content.page`
    pub r#object: String,

    /// Parsed content of the file.
    pub r#data: Vec<(/*Object*/)>,

    /// Indicates if there are more content pages to fetch.
    pub r#has_more: bool,

    /// The token for the next page, if any.
    pub r#next_page: Option<String>,
}

/// A list of files attached to a vector store.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreFileObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `vector_store.file`.
    pub r#object: String,

    /// The total vector store usage in bytes.
    pub r#usage_bytes: i64,

    /// The Unix timestamp (in seconds) for when the vector store file was
    /// created.
    pub r#created_at: i64,

    /// The ID of the [vector store](/docs/api-reference/vector-stores/object)
    /// that the [File](/docs/api-reference/files) is attached to.
    pub r#vector_store_id: String,

    /// The status of the vector store file, which can be either `in_progress`,
    /// `completed`, `cancelled`, or `failed`.
    pub r#status: String,

    /// The last error associated with this vector store file.
    pub r#last_error: Option<(/*Object*/)>,

    /// The strategy used to chunk the file.
    pub r#chunking_strategy: (/*OneOf*/),

    pub r#attributes: VectorStoreFileAttributes,
}

/// A vector store is a collection of processed files can be used by the
/// `file_search` tool.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreObject {
    /// The identifier, which can be referenced in API endpoints.
    pub r#id: String,

    /// The object type, which is always `vector_store`.
    pub r#object: String,

    /// The Unix timestamp (in seconds) for when the vector store was created.
    pub r#created_at: i64,

    /// The name of the vector store.
    pub r#name: String,

    /// The total number of bytes used by the files in the vector store.
    pub r#usage_bytes: i64,

    pub r#file_counts: (/*Object*/),

    /// The status of the vector store, which can be either `expired`,
    /// `in_progress`, or `completed`.
    pub r#status: String,

    pub r#expires_after: VectorStoreExpirationAfter,

    /// The Unix timestamp (in seconds) for when the vector store will expire.
    pub r#expires_at: Option<i64>,

    /// The Unix timestamp (in seconds) for when the vector store was last
    /// active.
    pub r#last_active_at: Option<i64>,

    pub r#metadata: Metadata,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreSearchRequest {
    /// A query string for a search
    pub r#query: (/*OneOf*/),

    /// Whether to rewrite the natural language query for vector search.
    pub r#rewrite_query: bool,

    /// The maximum number of results to return.
    pub r#max_num_results: i64,

    /// A filter to apply based on file attributes.
    pub r#filters: (/*OneOf*/),

    /// Ranking options for search.
    pub r#ranking_options: (/*Object*/),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreSearchResultContentObject {
    /// The type of content.
    pub r#type: String,

    /// The text content returned from search.
    pub r#text: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreSearchResultItem {
    /// The ID of the vector store file.
    pub r#file_id: String,

    /// The name of the vector store file.
    pub r#filename: String,

    /// The similarity score for the result.
    pub r#score: f64,

    pub r#attributes: VectorStoreFileAttributes,

    /// Content chunks from the file.
    pub r#content: Vec<VectorStoreSearchResultContentObject>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VectorStoreSearchResultsPage {
    /// The object type, which is always `vector_store.search_results.page`
    pub r#object: String,

    pub r#search_query: Vec<String>,

    /// The list of search result items.
    pub r#data: Vec<VectorStoreSearchResultItem>,

    /// Indicates if there are more results to fetch.
    pub r#has_more: bool,

    /// The token for the next page, if any.
    pub r#next_page: Option<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VoiceIdsShared(pub (/*AnyOf*/));

/// A wait action.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wait {
    /// Specifies the event type.
    pub r#type: String,
}

/// High level guidance for the amount of context window space to use for the
/// search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WebSearchContextSize(pub String);

/// Approximate location parameters for the search.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WebSearchLocation {
    /// The two-letter [ISO country
    /// code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user, e.g.
    pub r#country: String,

    /// Free text input for the region of the user, e.g.
    pub r#region: String,

    /// Free text input for the city of the user, e.g.
    pub r#city: String,

    /// The [IANA timezone](https://timeapi.io/documentation/iana-timezones) of
    /// the user, e.g.
    pub r#timezone: String,
}

/// This tool searches the web for relevant results to use in a response.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WebSearchPreviewTool {
    /// The type of the web search tool.
    pub r#type: String,

    pub r#user_location: (/*AnyOf*/),

    /// High level guidance for the amount of context window space to use for
    /// the search.
    pub r#search_context_size: String,
}

/// The results of a web search tool call.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WebSearchToolCall {
    /// The unique ID of the web search tool call.
    pub r#id: String,

    /// The type of the web search tool call.
    pub r#type: String,

    /// The status of the web search tool call.
    pub r#status: String,
}
