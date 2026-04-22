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
//! * [`AnthropicClient`] — skeleton only. Response parsing is wired
//!   (so when we flip it on, we trust the extraction path); the
//!   HTTP call itself `unimplemented!()`s to keep us from burning
//!   credits on infra shakedown. Flip the switch when the budget
//!   question is resolved.
//! * [`MockClient`] — canned-response queue for unit tests.
//!
//! # Determinism
//!
//! Ollama supports `options.seed`, so reruns with the same seed
//! and same model produce identical completions. Anthropic's API
//! does not provide strong determinism guarantees even with
//! temperature=0 — document at the consumer level that Anthropic
//! completions are non-reproducible.

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

/// Anthropic Messages API client. **Response parsing is wired**
/// (trusted shape: extract `content[0].text`, token counts from
/// `usage.{input,output}_tokens`, cost from token counts × model
/// pricing). **HTTP call is [`unimplemented!()`]** — we're not
/// spending credits on infrastructure shakedown. Flip it on when
/// the budget question resolves. When you do, rerun bake-off
/// measurements from scratch; don't mix skeleton-phase data with
/// live-phase data.
///
/// Fields are `#[allow(dead_code)]` because they're held for the
/// eventual live `complete()` implementation. `parse_response` and
/// `compute_cost` are exercised by tests today to lock in the
/// response-extraction path.
#[allow(dead_code)]
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
    /// body into a [`Completion`]. Exposed so tests (and the
    /// eventual wired `complete()`) share one parse path.
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn compute_cost(&self, input_tokens: u32, output_tokens: u32) -> f64 {
        let input = (input_tokens as f64) / 1_000_000.0 * self.pricing.input_per_mtok;
        let output = (output_tokens as f64) / 1_000_000.0 * self.pricing.output_per_mtok;
        input + output
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContentBlock>,
    usage: AnthropicUsage,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct AnthropicContentBlock {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    text: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

impl LlmClient for AnthropicClient {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn complete(&self, _system: &str, _user: &str) -> Result<Completion> {
        // Intentionally unwired — see struct doc. The parse path is
        // available via `parse_response` and tested independently.
        unimplemented!(
            "AnthropicClient HTTP call is intentionally unwired; flip \
             on when budget is resolved. Use OllamaClient for local \
             bake-offs today."
        )
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
    #[should_panic(expected = "intentionally unwired")]
    fn anthropic_complete_panics_until_wired() {
        let c = AnthropicClient::new(
            "claude",
            "k",
            AnthropicPricing { input_per_mtok: 0.0, output_per_mtok: 0.0 },
        );
        let _ = c.complete("s", "u");
    }

    #[test]
    fn llm_client_is_object_safe() {
        // Doesn't actually call anything — just forces the trait
        // object construction at compile time so a future change
        // that breaks object-safety fails this test loudly.
        let clients: Vec<Box<dyn LlmClient>> = vec![
            Box::new(MockClient::new("a", vec![])),
            Box::new(OllamaClient::new("m", "http://localhost:11434")),
        ];
        assert_eq!(clients.len(), 2);
    }
}
