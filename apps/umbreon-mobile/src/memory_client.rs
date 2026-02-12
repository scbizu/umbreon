#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct MemoryClient {
    base_url: String,
    user_id: String,
    client: reqwest::Client,
}

impl MemoryClient {
    pub fn new(base_url: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            user_id: user_id.into(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    pub async fn add_memory(&self, memory: impl Into<String>) -> Result<AddMemoryResponse, String> {
        let body = AddMemoryRequest {
            user_id: self.user_id.clone(),
            action: "addMemory".into(),
            memory: memory.into(),
        };

        self.post_json(body).await
    }

    pub async fn search_memories(
        &self,
        query: impl Into<String>,
        limit: Option<u32>,
        include_full_docs: Option<bool>,
    ) -> Result<SearchMemoriesResponse, String> {
        let body = SearchMemoriesRequest {
            user_id: self.user_id.clone(),
            action: "searchMemories".into(),
            query: query.into(),
            limit,
            include_full_docs,
        };

        self.post_json(body).await
    }

    pub async fn chat(
        &self,
        message: impl Into<String>,
        mode: Option<String>,
        conversation_id: Option<String>,
        add_memory: Option<String>,
    ) -> Result<ChatResponse, String> {
        let body = ChatRequest {
            user_id: self.user_id.clone(),
            message: message.into(),
            mode,
            conversation_id,
            add_memory,
        };

        self.post_json(body).await
    }

    async fn post_json<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        payload: T,
    ) -> Result<R, String> {
        let response = self
            .client
            .post(&self.base_url)
            .json(&payload)
            .send()
            .await
            .map_err(|err| format!("memory server request failed: {err}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("memory server error {status}: {body}"));
        }

        response
            .json::<R>()
            .await
            .map_err(|err| format!("failed to parse memory server response: {err}"))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AddMemoryRequest {
    user_id: String,
    action: String,
    memory: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchMemoriesRequest {
    user_id: String,
    action: String,
    query: String,
    limit: Option<u32>,
    include_full_docs: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChatRequest {
    user_id: String,
    message: String,
    mode: Option<String>,
    conversation_id: Option<String>,
    add_memory: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMemoryResponse {
    pub success: bool,
    pub memory: Option<MemoryRecord>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMemoriesResponse {
    pub success: bool,
    pub results: Option<Vec<MemorySearchResult>>,
    pub count: Option<usize>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatResponse {
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryRecord {
    pub id: String,
    pub status: Option<String>,
    pub workflow_instance_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemorySearchResult {
    pub id: String,
    pub content: String,
    pub score: Option<f32>,
}
