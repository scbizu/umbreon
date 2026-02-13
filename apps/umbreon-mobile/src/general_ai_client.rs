use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
};
use tracing::warn;

fn build_client(endpoint: &str, api_key: &str) -> Result<Client<OpenAIConfig>, String> {
    let api_key = api_key.trim();
    if api_key.is_empty() {
        return Err("API key is empty.".to_string());
    }
    let endpoint = endpoint.trim().trim_end_matches('/').to_string();
    let mut config = OpenAIConfig::new().with_api_key(api_key);
    if !endpoint.is_empty() {
        config = config.with_api_base(endpoint);
    }
    Ok(Client::with_config(config))
}

pub async fn fetch_models(endpoint: &str, api_key: &str) -> Result<Vec<String>, String> {
    let client = build_client(endpoint, api_key)?;
    let response = client.models().list().await.map_err(|err| {
        warn!(error = %err, endpoint = endpoint, "fetch models failed");
        format!("fetch models failed (endpoint: {endpoint}): {err}")
    })?;

    let mut models = response
        .data
        .into_iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    models.sort();
    models.dedup();
    Ok(models)
}

pub async fn test_chat(
    endpoint: &str,
    api_key: &str,
    model: &str,
    prompt: &str,
) -> Result<String, String> {
    let model = model.trim();
    if model.is_empty() {
        return Err("Model is empty.".to_string());
    }
    let client = build_client(endpoint, api_key)?;

    let message = ChatCompletionRequestUserMessageArgs::default()
        .content(prompt)
        .build()
        .map_err(|err| format!("build chat message failed: {err}"))?;

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([message.into()])
        .build()
        .map_err(|err| format!("build chat request failed: {err}"))?;

    let response = client.chat().create(request).await.map_err(|err| {
        warn!(error = %err, endpoint = endpoint, model = model, "chat request failed");
        format!("chat request failed (endpoint: {endpoint}, model: {model}): {err}")
    })?;

    let text = response
        .choices
        .first()
        .and_then(|choice| choice.message.content.clone())
        .unwrap_or_default();

    if text.is_empty() {
        return Err("LLM returned empty response.".to_string());
    }

    Ok(text)
}
