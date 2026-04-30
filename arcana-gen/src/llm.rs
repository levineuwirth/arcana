//! LLM client abstraction + concrete impls for the bake-off pipeline.
//!
//! # Trait contract
//!
//! [`LlmClient`] is the single surface the bake-off driver sees.
//! Sync, one-shot, no streaming: the bake-off's next step
//! ([`crate::verify::check`]) needs the full completion before it
//! can run, so streaming would add complexity without buying
//! anything. Object-safe — the driver holds `&[&dyn LlmClient]`
//! and rotates through models.
//!
//! # Impls
//!
//! * [`OllamaClient`] — real HTTP against a local Ollama instance.
//!   Cost is always `None` (local inference is free).
//! * [`AnthropicClient`] — real HTTP against the Messages API.
//!   Per-model pricing passed in at construction so token counts
//!   get converted to USD on the fly and surface in bake-off JSONL
//!   + report output.
//! * [`OpenAiCompatibleClient`] — HTTP against any OpenAI-shaped
//!   `/v1/chat/completions` endpoint (OpenAI itself, Together,
//!   Fireworks, Groq, DeepSeek, vLLM, llama.cpp server, LMStudio, …).
//!   API key is optional (local servers don't need one). Pricing is
//!   optional (`None` → cost_usd stays `None`, the right default for
//!   local inference).
//! * [`MockClient`] — canned-response queue for unit tests.
//!
//! # Determinism
//!
//! Ollama supports `options.seed`, so reruns with the same seed
//! and same model produce identical completions. Anthropic's API
//! does not provide strong determinism guarantees even with
//! temperature=0 — document at the consumer level that Anthropic
//! completions are non-reproducible. OpenAI's `seed` parameter is
//! best-effort (may regress with model upgrades); vLLM and
//! llama.cpp server honor it strictly. We pass it through; the
//! consumer interprets.

use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

// =============================================================================
// public trait + Completion
// =============================================================================

/// Result of a single `complete` call.
#[derive(Debug, Clone)]
pub struct Completion {
    pub text: String,
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    /// Estimated USD cost — token counts × per-model pricing. Always
    /// `None` for local inference (Ollama). Filled for paid APIs.
    pub cost_usd: Option<f64>,
    /// Wall-clock duration of the HTTP call (or mock delay).
    pub duration: Duration,
}

/// Sync LLM client interface. Object-safe: bake-off driver holds
/// `&[&dyn LlmClient]`.
pub trait LlmClient {
    /// Stable identifier suitable for report bucketing + JSONL row
    /// keys. Shape is provider-specific; e.g. `"qwen3:235b-instruct"`
    /// for Ollama, `"claude-opus-4-7"` for Anthropic.
    fn model_id(&self) -> &str;

    /// Block until the full completion is available. Implementations
    /// apply whatever chat-template transformation their provider
    /// requires; the caller passes role-separated `system` and
    /// `user` strings.
    fn complete(&self, system: &str, user: &str) -> Result<Completion>;
}

// =============================================================================
// OllamaClient
// =============================================================================

/// Ollama `/api/chat` client. Block-on-full-response (streaming
/// disabled). Temperature and seed passed through to
/// `options.{temperature,seed}` so reruns are reproducible.
#[derive(Debug, Clone)]
pub struct OllamaClient {
    model_id: String,
    endpoint: String,
    timeout: Duration,
    temperature: f32,
    seed: Option<u64>,
}

impl OllamaClient {
    /// Build a client targeting `endpoint` (typically
    /// `http://localhost:11434`) for the given `model_id`. Defaults:
    /// temperature = 0.2 (low, we want correctness not creativity),
    /// timeout = 180 s (large local models doing multi-thousand-
    /// token completions take time), seed = None (non-deterministic
    /// unless the caller sets one).
    pub fn new(model_id: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
            endpoint: endpoint.into(),
            timeout: Duration::from_secs(180),
            temperature: 0.2,
            seed: None,
        }
    }

    pub fn with_timeout(mut self, t: Duration) -> Self {
        self.timeout = t;
        self
    }

    pub fn with_temperature(mut self, t: f32) -> Self {
        self.temperature = t;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
}

#[derive(Serialize)]
struct OllamaChatRequest<'a> {
    model: &'a str,
    messages: Vec<OllamaMessage<'a>>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
}

#[derive(Deserialize)]
struct OllamaChatResponse {
    message: OllamaResponseMessage,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
}

#[derive(Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

impl LlmClient for OllamaClient {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn complete(&self, system: &str, user: &str) -> Result<Completion> {
        let agent = ureq::AgentBuilder::new()
            .timeout(self.timeout)
            .build();

        let body = OllamaChatRequest {
            model: &self.model_id,
            messages: vec![
                OllamaMessage { role: "system", content: system },
                OllamaMessage { role: "user", content: user },
            ],
            stream: false,
            options: OllamaOptions {
                temperature: self.temperature,
                seed: self.seed,
            },
        };

        let url = format!("{}/api/chat", self.endpoint.trim_end_matches('/'));

        let start = Instant::now();
        let resp: OllamaChatResponse = agent
            .post(&url)
            .set("Content-Type", "application/json")
            .send_json(&body)
            .with_context(|| format!("POST {url}"))?
            .into_json()
            .context("parsing Ollama response")?;
        let duration = start.elapsed();

        Ok(Completion {
            text: resp.message.content,
            prompt_tokens: resp.prompt_eval_count,
            completion_tokens: resp.eval_count,
            cost_usd: None,
            duration,
        })
    }
}

// =============================================================================
// AnthropicClient (skeleton — HTTP call intentionally unimplemented)
// =============================================================================

/// Anthropic Messages API client. Wires a sync HTTP call against
/// `/v1/messages`, parses the standard response shape
/// (`content[0].text` + `usage.{input,output}_tokens`), and
/// converts token counts to USD via the [`AnthropicPricing`]
/// supplied at construction.
///
/// Anthropic does not document strong determinism guarantees, even
/// with `temperature=0` — downstream consumers (bake-off) should
/// treat Anthropic rows as single-sample draws, not reproducible
/// fixtures.
#[derive(Debug, Clone)]
pub struct AnthropicClient {
    model_id: String,
    api_key: String,
    endpoint: String,
    timeout: Duration,
    max_tokens: u32,
    pricing: AnthropicPricing,
}

/// Per-model pricing in USD per million tokens. Passed in at
/// construction so changes to upstream pricing don't require
/// recompilation.
#[derive(Debug, Clone, Copy)]
pub struct AnthropicPricing {
    pub input_per_mtok: f64,
    pub output_per_mtok: f64,
}

impl AnthropicClient {
    pub fn new(
        model_id: impl Into<String>,
        api_key: impl Into<String>,
        pricing: AnthropicPricing,
    ) -> Self {
        Self {
            model_id: model_id.into(),
            api_key: api_key.into(),
            endpoint: "https://api.anthropic.com".to_string(),
            timeout: Duration::from_secs(180),
            max_tokens: 4096,
            pricing,
        }
    }

    pub fn with_max_tokens(mut self, n: u32) -> Self {
        self.max_tokens = n;
        self
    }

    pub fn with_endpoint(mut self, url: impl Into<String>) -> Self {
        self.endpoint = url.into();
        self
    }

    /// Parse an already-fetched Anthropic Messages API response
    /// body into a [`Completion`]. Shared between `complete()` and
    /// the offline unit tests so there is one parse path.
    fn parse_response(&self, body: &str, duration: Duration) -> Result<Completion> {
        let resp: AnthropicResponse = serde_json::from_str(body)
            .context("parsing Anthropic response body")?;
        let text = resp
            .content
            .iter()
            .find(|b| b.kind == "text")
            .map(|b| b.text.clone())
            .ok_or_else(|| anyhow!("no text block in Anthropic response"))?;
        let cost_usd = self.compute_cost(resp.usage.input_tokens, resp.usage.output_tokens);
        Ok(Completion {
            text,
            prompt_tokens: Some(resp.usage.input_tokens),
            completion_tokens: Some(resp.usage.output_tokens),
            cost_usd: Some(cost_usd),
            duration,
        })
    }

    fn compute_cost(&self, input_tokens: u32, output_tokens: u32) -> f64 {
        let input = (input_tokens as f64) / 1_000_000.0 * self.pricing.input_per_mtok;
        let output = (output_tokens as f64) / 1_000_000.0 * self.pricing.output_per_mtok;
        input + output
    }
}

#[derive(Serialize)]
struct AnthropicRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    system: &'a str,
    messages: Vec<AnthropicRequestMessage<'a>>,
}

#[derive(Serialize)]
struct AnthropicRequestMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContentBlock>,
    usage: AnthropicUsage,
}

#[derive(Deserialize)]
struct AnthropicContentBlock {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    text: String,
}

#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

impl LlmClient for AnthropicClient {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn complete(&self, system: &str, user: &str) -> Result<Completion> {
        let agent = ureq::AgentBuilder::new().timeout(self.timeout).build();

        let body = AnthropicRequest {
            model: &self.model_id,
            max_tokens: self.max_tokens,
            system,
            messages: vec![AnthropicRequestMessage { role: "user", content: user }],
        };

        let url = format!("{}/v1/messages", self.endpoint.trim_end_matches('/'));

        let start = Instant::now();
        let response = match agent
            .post(&url)
            .set("x-api-key", &self.api_key)
            .set("anthropic-version", "2023-06-01")
            .set("content-type", "application/json")
            .send_json(&body)
        {
            Ok(r) => r,
            // Unpack 4xx/5xx bodies — bad API keys, rate limits, and
            // malformed requests all surface here and the response
            // body is the actionable content.
            Err(ureq::Error::Status(code, resp)) => {
                let body_text = resp.into_string().unwrap_or_default();
                return Err(anyhow!(
                    "Anthropic API returned HTTP {code}: {body_text}"
                ));
            }
            Err(e) => {
                return Err(anyhow!("Anthropic API transport error: {e}"));
            }
        };
        let body_text = response
            .into_string()
            .context("reading Anthropic response body")?;
        let duration = start.elapsed();

        self.parse_response(&body_text, duration)
    }
}

// =============================================================================
// OpenAiCompatibleClient
// =============================================================================

/// Client for any OpenAI-shaped `/v1/chat/completions` endpoint.
///
/// Works against:
///   * OpenAI itself (`https://api.openai.com/v1`)
///   * Hosted drop-ins (Together, Fireworks, Groq, DeepSeek, …)
///   * Local inference servers exposing an OpenAI-compatible API
///     (vLLM, llama.cpp server, LMStudio, TabbyAPI, …)
///
/// API key and pricing are both optional — the no-key / no-pricing
/// configuration is what you want for a local vLLM or llama.cpp
/// server on the LAN.
#[derive(Debug, Clone)]
pub struct OpenAiCompatibleClient {
    model_id: String,
    endpoint: String,
    api_key: Option<String>,
    pricing: Option<OpenAiPricing>,
    timeout: Duration,
    temperature: f32,
    seed: Option<u64>,
    max_tokens: u32,
    /// Provider-specific keys merged into every request body.
    /// Common use: Qwen3 thinking-mode toggle via
    /// `{"chat_template_kwargs": {"enable_thinking": false}}`.
    /// Keys here override the client's own (`model`, `messages`,
    /// etc.) — caller's responsibility not to clobber.
    extra_body: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Per-model pricing in USD per million tokens. Mirrors the
/// Anthropic shape; keep as its own type so a CLI typo mixing the
/// two can't silently typecheck.
#[derive(Debug, Clone, Copy)]
pub struct OpenAiPricing {
    pub input_per_mtok: f64,
    pub output_per_mtok: f64,
}

impl OpenAiCompatibleClient {
    /// Build a client for `model_id` at `endpoint` (the `/v1` base;
    /// the chat-completions path is appended by `complete()`).
    /// Defaults: temperature = 0.2, timeout = 180s, max_tokens = 4096,
    /// no API key, no pricing, no seed.
    pub fn new(
        model_id: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            model_id: model_id.into(),
            endpoint: endpoint.into(),
            api_key: None,
            pricing: None,
            timeout: Duration::from_secs(180),
            temperature: 0.2,
            seed: None,
            max_tokens: 4096,
            extra_body: None,
        }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn with_pricing(mut self, pricing: OpenAiPricing) -> Self {
        self.pricing = Some(pricing);
        self
    }

    pub fn with_timeout(mut self, t: Duration) -> Self {
        self.timeout = t;
        self
    }

    pub fn with_temperature(mut self, t: f32) -> Self {
        self.temperature = t;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_max_tokens(mut self, n: u32) -> Self {
        self.max_tokens = n;
        self
    }

    /// Merge `extra` into every request body sent by this client.
    /// Use for provider-specific kwargs the standard
    /// `/v1/chat/completions` schema doesn't carry — most commonly
    /// `chat_template_kwargs` for Qwen3 / similar thinking-mode
    /// toggles. Replaces any previous extra_body call.
    pub fn with_extra_body(
        mut self,
        extra: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        self.extra_body = Some(extra);
        self
    }

    /// Parse an already-fetched `/v1/chat/completions` body into a
    /// [`Completion`]. Shared between `complete()` and the offline
    /// unit tests. Tolerates a missing `usage` object: some local
    /// servers omit it entirely, in which case token counts and
    /// cost all come back `None`.
    fn parse_response(&self, body: &str, duration: Duration) -> Result<Completion> {
        let resp: OpenAiResponse = serde_json::from_str(body)
            .context("parsing OpenAI-compatible response body")?;
        let text = resp
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("no choices in OpenAI-compatible response"))?
            .message
            .content;
        let (prompt_tokens, completion_tokens, cost_usd) = match (resp.usage, self.pricing) {
            (Some(u), Some(p)) => {
                let cost = (u.prompt_tokens as f64) / 1_000_000.0 * p.input_per_mtok
                    + (u.completion_tokens as f64) / 1_000_000.0 * p.output_per_mtok;
                (Some(u.prompt_tokens), Some(u.completion_tokens), Some(cost))
            }
            (Some(u), None) => (Some(u.prompt_tokens), Some(u.completion_tokens), None),
            (None, _) => (None, None, None),
        };
        Ok(Completion {
            text,
            prompt_tokens,
            completion_tokens,
            cost_usd,
            duration,
        })
    }
}

#[derive(Serialize)]
struct OpenAiRequest<'a> {
    model: &'a str,
    messages: Vec<OpenAiRequestMessage<'a>>,
    temperature: f32,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    stream: bool,
}

#[derive(Serialize)]
struct OpenAiRequestMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
    #[serde(default)]
    usage: Option<OpenAiUsage>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiResponseMessage,
}

#[derive(Deserialize)]
struct OpenAiResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct OpenAiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

/// Build the JSON request body for `/v1/chat/completions`, merging
/// in any caller-supplied `extra_body` keys at the top level.
/// Extracted from `complete()` so the merge logic can be tested
/// without spawning HTTP. Caller-supplied keys overwrite the
/// client's own (`model`, `messages`, etc.); the caller is
/// responsible for not clobbering required fields.
fn build_openai_request_body(
    model: &str,
    system: &str,
    user: &str,
    temperature: f32,
    max_tokens: u32,
    seed: Option<u64>,
    extra_body: Option<&serde_json::Map<String, serde_json::Value>>,
) -> Result<serde_json::Value> {
    let body = OpenAiRequest {
        model,
        messages: vec![
            OpenAiRequestMessage { role: "system", content: system },
            OpenAiRequestMessage { role: "user", content: user },
        ],
        temperature,
        max_tokens,
        seed,
        stream: false,
    };
    let mut value = serde_json::to_value(&body)
        .context("serializing OpenAI request body")?;
    if let Some(extra) = extra_body {
        if let serde_json::Value::Object(map) = &mut value {
            for (k, v) in extra {
                map.insert(k.clone(), v.clone());
            }
        }
    }
    Ok(value)
}

impl LlmClient for OpenAiCompatibleClient {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn complete(&self, system: &str, user: &str) -> Result<Completion> {
        let agent = ureq::AgentBuilder::new().timeout(self.timeout).build();

        let body = build_openai_request_body(
            &self.model_id,
            system,
            user,
            self.temperature,
            self.max_tokens,
            self.seed,
            self.extra_body.as_ref(),
        )?;

        let url = format!(
            "{}/chat/completions",
            self.endpoint.trim_end_matches('/')
        );

        let start = Instant::now();
        let mut req = agent
            .post(&url)
            .set("content-type", "application/json");
        if let Some(key) = &self.api_key {
            req = req.set("authorization", &format!("Bearer {key}"));
        }
        let response = match req.send_json(&body) {
            Ok(r) => r,
            Err(ureq::Error::Status(code, resp)) => {
                let body_text = resp.into_string().unwrap_or_default();
                return Err(anyhow!(
                    "OpenAI-compatible endpoint at {url} returned HTTP {code}: {body_text}"
                ));
            }
            Err(e) => {
                return Err(anyhow!(
                    "OpenAI-compatible endpoint transport error at {url}: {e}"
                ));
            }
        };
        let body_text = response
            .into_string()
            .context("reading OpenAI-compatible response body")?;
        let duration = start.elapsed();

        self.parse_response(&body_text, duration)
    }
}

// =============================================================================
// MockClient (tests only, but public so external integration tests can use it)
// =============================================================================

/// FIFO canned-response client. Pops one response per `complete`
/// call, panics if exhausted. Use for unit tests of consumers that
/// need a model but shouldn't spawn real HTTP.
///
/// Interior mutability via `Mutex<VecDeque>` so the trait stays
/// object-safe (`&self` on `complete`). Single-threaded tests are
/// fine either way; multi-threaded tests share the queue, which is
/// usually what you want for a shared mock.
pub struct MockClient {
    model_id: String,
    responses: Mutex<VecDeque<String>>,
}

impl std::fmt::Debug for MockClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockClient")
            .field("model_id", &self.model_id)
            .field("remaining", &self.remaining())
            .finish()
    }
}

impl MockClient {
    pub fn new(model_id: impl Into<String>, responses: Vec<String>) -> Self {
        Self {
            model_id: model_id.into(),
            responses: Mutex::new(responses.into()),
        }
    }

    pub fn remaining(&self) -> usize {
        self.responses.lock().unwrap().len()
    }
}

impl LlmClient for MockClient {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn complete(&self, _system: &str, _user: &str) -> Result<Completion> {
        let mut q = self.responses.lock().unwrap();
        let text = q.pop_front().ok_or_else(|| {
            anyhow!(
                "MockClient({}) exhausted — add more canned responses",
                self.model_id
            )
        })?;
        Ok(Completion {
            text,
            prompt_tokens: None,
            completion_tokens: None,
            cost_usd: None,
            duration: Duration::from_millis(1),
        })
    }
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_client_returns_responses_in_order() {
        let m = MockClient::new(
            "mock",
            vec!["first".into(), "second".into(), "third".into()],
        );
        assert_eq!(m.model_id(), "mock");
        assert_eq!(m.remaining(), 3);
        assert_eq!(m.complete("s", "u").unwrap().text, "first");
        assert_eq!(m.complete("s", "u").unwrap().text, "second");
        assert_eq!(m.remaining(), 1);
        assert_eq!(m.complete("s", "u").unwrap().text, "third");
        assert_eq!(m.remaining(), 0);
    }

    #[test]
    fn mock_client_errors_when_exhausted() {
        let m = MockClient::new("mock", vec!["only".into()]);
        let _ = m.complete("s", "u").unwrap();
        let err = m.complete("s", "u").unwrap_err();
        assert!(err.to_string().contains("exhausted"));
    }

    #[test]
    fn ollama_client_builder_defaults() {
        let c = OllamaClient::new("qwen3:235b", "http://localhost:11434");
        assert_eq!(c.model_id(), "qwen3:235b");
        assert_eq!(c.temperature, 0.2);
        assert_eq!(c.timeout, Duration::from_secs(180));
        assert_eq!(c.seed, None);
    }

    #[test]
    fn ollama_client_builder_overrides() {
        let c = OllamaClient::new("m", "e")
            .with_temperature(0.7)
            .with_timeout(Duration::from_secs(30))
            .with_seed(42);
        assert_eq!(c.temperature, 0.7);
        assert_eq!(c.timeout, Duration::from_secs(30));
        assert_eq!(c.seed, Some(42));
    }

    #[test]
    fn anthropic_parse_extracts_text_and_usage_and_cost() {
        let body = r#"{
            "id": "msg_01",
            "type": "message",
            "role": "assistant",
            "content": [
                {"type": "text", "text": "Hello, world!"}
            ],
            "model": "claude-opus-4-7",
            "stop_reason": "end_turn",
            "usage": {"input_tokens": 1000, "output_tokens": 500}
        }"#;
        let client = AnthropicClient::new(
            "claude-opus-4-7",
            "dummy-key",
            AnthropicPricing { input_per_mtok: 15.0, output_per_mtok: 75.0 },
        );
        let comp = client
            .parse_response(body, Duration::from_millis(100))
            .expect("parse");
        assert_eq!(comp.text, "Hello, world!");
        assert_eq!(comp.prompt_tokens, Some(1000));
        assert_eq!(comp.completion_tokens, Some(500));
        // 1000 in @ 15/MT = 0.015; 500 out @ 75/MT = 0.0375; total 0.0525.
        let cost = comp.cost_usd.expect("cost was computed");
        assert!(
            (cost - 0.0525).abs() < 1e-9,
            "expected ~0.0525, got {cost}"
        );
    }

    #[test]
    fn anthropic_parse_errors_on_missing_text_block() {
        let body = r#"{
            "content": [{"type": "tool_use", "text": ""}],
            "usage": {"input_tokens": 10, "output_tokens": 5}
        }"#;
        let client = AnthropicClient::new(
            "claude",
            "k",
            AnthropicPricing { input_per_mtok: 0.0, output_per_mtok: 0.0 },
        );
        let err = client
            .parse_response(body, Duration::from_millis(1))
            .unwrap_err();
        assert!(err.to_string().contains("no text block"));
    }

    #[test]
    fn llm_client_is_object_safe() {
        // Forces trait object construction at compile time so a
        // future change that breaks object-safety fails loudly
        // here instead of deep in the bake-off driver.
        let clients: Vec<Box<dyn LlmClient>> = vec![
            Box::new(MockClient::new("a", vec![])),
            Box::new(OllamaClient::new("m", "http://localhost:11434")),
            Box::new(AnthropicClient::new(
                "claude",
                "k",
                AnthropicPricing { input_per_mtok: 0.0, output_per_mtok: 0.0 },
            )),
            Box::new(OpenAiCompatibleClient::new("gpt-x", "http://localhost:8000/v1")),
        ];
        assert_eq!(clients.len(), 4);
    }

    // -- OpenAI-compatible client --------------------------------------

    #[test]
    fn openai_parse_extracts_text_and_usage_and_cost() {
        let body = r#"{
            "id": "chatcmpl-1",
            "object": "chat.completion",
            "choices": [
                {"index": 0, "message": {"role": "assistant", "content": "Hello!"}}
            ],
            "usage": {"prompt_tokens": 100, "completion_tokens": 50, "total_tokens": 150}
        }"#;
        let client = OpenAiCompatibleClient::new("gpt-mock", "http://x/v1").with_pricing(
            OpenAiPricing { input_per_mtok: 10.0, output_per_mtok: 30.0 },
        );
        let comp = client
            .parse_response(body, Duration::from_millis(42))
            .expect("parse");
        assert_eq!(comp.text, "Hello!");
        assert_eq!(comp.prompt_tokens, Some(100));
        assert_eq!(comp.completion_tokens, Some(50));
        // 100 @ 10/M = 0.001; 50 @ 30/M = 0.0015; total 0.0025.
        assert!((comp.cost_usd.unwrap() - 0.0025).abs() < 1e-9);
    }

    #[test]
    fn openai_parse_no_usage_no_pricing_yields_none_fields() {
        // Many local servers (old llama.cpp, etc.) omit the usage
        // block entirely. The client must tolerate that and return
        // None for token counts + cost rather than erroring.
        let body = r#"{
            "choices": [
                {"message": {"role": "assistant", "content": "hi"}}
            ]
        }"#;
        let client = OpenAiCompatibleClient::new("local", "http://x/v1");
        let comp = client
            .parse_response(body, Duration::from_millis(1))
            .expect("parse");
        assert_eq!(comp.text, "hi");
        assert!(comp.prompt_tokens.is_none());
        assert!(comp.completion_tokens.is_none());
        assert!(comp.cost_usd.is_none());
    }

    #[test]
    fn openai_parse_usage_present_but_no_pricing_gives_tokens_but_not_cost() {
        // Hosted endpoint returns usage, but the user didn't supply
        // pricing — we still record token counts for later analysis
        // and leave cost unset.
        let body = r#"{
            "choices": [{"message": {"role": "assistant", "content": "ok"}}],
            "usage": {"prompt_tokens": 10, "completion_tokens": 5}
        }"#;
        let client = OpenAiCompatibleClient::new("local", "http://x/v1");
        let comp = client
            .parse_response(body, Duration::from_millis(1))
            .expect("parse");
        assert_eq!(comp.prompt_tokens, Some(10));
        assert_eq!(comp.completion_tokens, Some(5));
        assert!(comp.cost_usd.is_none());
    }

    #[test]
    fn openai_parse_errors_on_empty_choices() {
        let body = r#"{"choices": []}"#;
        let client = OpenAiCompatibleClient::new("local", "http://x/v1");
        let err = client
            .parse_response(body, Duration::from_millis(1))
            .unwrap_err();
        assert!(err.to_string().contains("no choices"));
    }

    #[test]
    fn openai_client_builder_defaults() {
        let c = OpenAiCompatibleClient::new("gpt", "http://localhost:8000/v1");
        assert_eq!(c.model_id(), "gpt");
        assert_eq!(c.temperature, 0.2);
        assert_eq!(c.timeout, Duration::from_secs(180));
        assert_eq!(c.seed, None);
        assert!(c.api_key.is_none());
        assert!(c.pricing.is_none());
        assert!(c.extra_body.is_none());
    }

    #[test]
    fn openai_request_body_without_extras_has_standard_shape() {
        let body = build_openai_request_body(
            "qwen3", "you are terse", "say ok", 0.2, 16, Some(42), None,
        )
        .expect("build");
        let obj = body.as_object().unwrap();
        assert_eq!(obj["model"], "qwen3");
        assert_eq!(obj["max_tokens"], 16);
        assert_eq!(obj["seed"], 42);
        assert_eq!(obj["stream"], false);
        assert!(obj.contains_key("messages"));
        // No accidental extra keys.
        let keys: std::collections::HashSet<&str> =
            obj.keys().map(|s| s.as_str()).collect();
        for required in ["model", "messages", "temperature", "max_tokens", "seed", "stream"] {
            assert!(keys.contains(required), "missing key {required}");
        }
    }

    #[test]
    fn openai_request_body_merges_extra_keys_at_top_level() {
        // Qwen3 thinking-mode toggle is the canonical use case.
        let mut extra = serde_json::Map::new();
        extra.insert(
            "chat_template_kwargs".to_string(),
            serde_json::json!({ "enable_thinking": false }),
        );
        let body = build_openai_request_body(
            "qwen3", "sys", "user", 0.2, 16, None, Some(&extra),
        )
        .expect("build");
        let obj = body.as_object().unwrap();
        // Standard keys preserved.
        assert_eq!(obj["model"], "qwen3");
        assert!(obj.contains_key("messages"));
        // Extra key merged at top level.
        assert_eq!(
            obj["chat_template_kwargs"],
            serde_json::json!({ "enable_thinking": false })
        );
    }

    #[test]
    fn openai_request_body_extra_keys_override_standard() {
        // Document the override semantic. Caller's responsibility
        // not to clobber required fields, but if they do, they win.
        let mut extra = serde_json::Map::new();
        extra.insert(
            "temperature".to_string(),
            serde_json::Value::from(0.99),
        );
        let body = build_openai_request_body(
            "qwen3", "sys", "user", 0.2, 16, None, Some(&extra),
        )
        .expect("build");
        assert_eq!(body["temperature"], 0.99);
    }

    /// Live API smoke test. Uses a cheap OpenAI model when
    /// `OPENAI_API_KEY` is set; skips otherwise. Endpoint defaults to
    /// `https://api.openai.com/v1` here (in the CLI the user supplies
    /// it explicitly to avoid accidental spending).
    #[test]
    #[ignore]
    fn openai_complete_smoke_test_live() {
        let Ok(api_key) = std::env::var("OPENAI_API_KEY") else {
            eprintln!("OPENAI_API_KEY not set; skipping");
            return;
        };
        let model = std::env::var("OPENAI_SMOKE_MODEL")
            .unwrap_or_else(|_| "gpt-4o-mini".to_string());
        let endpoint = std::env::var("OPENAI_SMOKE_ENDPOINT")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let client = OpenAiCompatibleClient::new(model, endpoint)
            .with_api_key(api_key)
            .with_max_tokens(16);
        let comp = client
            .complete(
                "You are terse. Reply with exactly one word.",
                "Respond with: ok",
            )
            .expect("API call succeeded");
        assert!(!comp.text.is_empty(), "response text was empty");
    }

    /// Live API smoke test. Skipped unless `ANTHROPIC_API_KEY` is set
    /// — a bare `cargo test --ignored` on a machine without the key
    /// no-ops rather than fails. Uses Haiku to minimize spend
    /// (ballpark $0.0001/call).
    #[test]
    #[ignore]
    fn anthropic_complete_smoke_test_live() {
        let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") else {
            eprintln!("ANTHROPIC_API_KEY not set; skipping");
            return;
        };
        let client = AnthropicClient::new(
            "claude-haiku-4-5-20251001",
            api_key,
            AnthropicPricing { input_per_mtok: 1.0, output_per_mtok: 5.0 },
        );
        let comp = client
            .complete(
                "You are terse. Reply with exactly one word.",
                "Respond with the word: ok",
            )
            .expect("API call succeeded");
        assert!(!comp.text.is_empty(), "response text was empty");
        assert!(comp.prompt_tokens.unwrap() > 0);
        assert!(comp.completion_tokens.unwrap() > 0);
        assert!(comp.cost_usd.unwrap() > 0.0);
    }
}
