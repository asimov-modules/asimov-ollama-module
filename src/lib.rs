// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

use asimov_module::{prelude::*, tracing};
use core::error::Error;
use serde_json::{Value, json};

#[derive(Clone, Debug, bon::Builder)]
#[builder(on(String, into))]
pub struct Options {
    #[builder(default = "http://localhost:11434")]
    pub endpoint: String,

    pub model: String,
}

pub fn generate(input: impl AsRef<str>, options: &Options) -> Result<Vec<String>, Box<dyn Error>> {
    let req = json!({
        "model": options.model,
        "prompt": input.as_ref(),
        "stream": false
    });

    let mut resp = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .user_agent("asimov-ollama-module")
        .build()
        .new_agent()
        .post(format!("{}/api/generate", options.endpoint))
        .header("content-type", "application/json")
        .send_json(&req)
        .inspect_err(|e| tracing::error!("HTTP request failed: {e}"))?;
    tracing::debug!(response = ?resp);

    let status = resp.status();
    tracing::debug!(status = status.to_string());

    let resp: Value = resp
        .body_mut()
        .read_json()
        .inspect_err(|e| tracing::error!("unable to read HTTP response body: {e}"))?;
    tracing::debug!(body = ?resp);

    if !status.is_success() {
        tracing::error!("Received an error response: {status}");

        // {
        //   "error": "model 'foobar' not found"
        // }
        if let Some(message) = resp["error"].as_str() {
            return Err(message.into());
        }
    }

    // {
    //   "context": [ ... ],
    //   "created_at": "2025-09-23T12:46:45.876878Z",
    //   "done": true,
    //   "done_reason": "stop",
    //   "eval_count": 139,
    //   "eval_duration": 10118193708,
    //   "load_duration": 80743666,
    //   "model": "deepseek-r1:14b",
    //   "prompt_eval_count": 10,
    //   "prompt_eval_duration": 1175246542,
    //   "response": "...",
    //   "total_duration": 11374760875
    // }

    let mut responses = Vec::new();

    if let Some(response) = resp["response"].as_str() {
        responses.push(response.to_string())
    }

    if let Some(done_reason) = resp["done_reason"].as_str() {
        tracing::debug!(done_reason)
    }

    Ok(responses)
}
