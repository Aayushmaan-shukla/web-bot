use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response, Headers};
use serde_json::json;

#[wasm_bindgen]
pub async fn process_llm_request(provider: String, api_key: String, model: String, prompt: String, context: String) -> Result<JsValue, JsValue> {
    let system_instruction = "You are an AI assistant that answers questions based on the provided webpage context. If the user's question relates to concepts, 
    topics, or code snippets present in the webpage context, provide a helpful and valid response. 
    Only if the question is completely unrelated to the provided context, reply saying that the information asked is beyond the scope of the webpage data provided.";
    let full_prompt = format!("{}\n\nContext from web page:\n{}\n\nUser Question:\n{}", system_instruction, context, prompt);

    let (url, body, auth_header, auth_val, extra_headers) = match provider.as_str() {
        "openai" => {
            let body = json!({
                "model": model,
                "messages": [{"role": "user", "content": full_prompt}]
            });
            ("https://api.openai.com/v1/chat/completions".to_string(), body, "Authorization", format!("Bearer {}", api_key), vec![])
        },
        "anthropic" => {
            let body = json!({
                "model": model,
                "max_tokens": 1024,
                "messages": [{"role": "user", "content": full_prompt}]
            });
            ("https://api.anthropic.com/v1/messages".to_string(), body, "x-api-key", api_key, vec![
                ("anthropic-version", "2023-06-01"),
                ("anthropic-dangerous-direct-browser-access", "true")
            ])
        },
        "gemini" => {
            let body = json!({
                "contents": [{"parts": [{"text": full_prompt}]}]
            });

            (url, body, "", "".to_string(), vec![])
        },
        "grok" => {
            let body = json!({
                "model": model,
                "messages": [{"role": "user", "content": full_prompt}]
            });
            ("https://api.x.ai/v1/chat/completions".to_string(), body, "Authorization", format!("Bearer {}", api_key), vec![])
        },
        "glm" => {
            let body = json!({
                "model": model,
                "messages": [{"role": "user", "content": full_prompt}]
            });
            ("https://open.bigmodel.cn/api/paas/v4/chat/completions".to_string(), body, "Authorization", format!("Bearer {}", api_key), vec![])
        },
        _ => return Err(JsValue::from_str("Unknown provider"))
    };

    let mut opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    
    let body_str = body.to_string();
    opts.set_body(&JsValue::from_str(&body_str));

    let request = Request::new_with_str_and_init(&url, &opts)?;
    let headers = request.headers();
    headers.set("Content-Type", "application/json")?;
    
    if !auth_header.is_empty() {
        headers.set(auth_header, &auth_val)?;
    }
    for (k, v) in extra_headers {
        headers.set(k, v)?;
    }

    console::log_1(&JsValue::from_str(&format!("[Rust] Sending HTTP POST to {}", url)));
    console::time_with_label("rust-http-fetch");

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    
    console::time_end_with_label("rust-http-fetch");
    console::log_1(&JsValue::from_str(&format!("[Rust] HTTP Response received. Status: {}", resp.status())));
    console::time_with_label("rust-json-parse");
    
    let json_value = JsFuture::from(resp.json()?).await?;
    
    console::time_end_with_label("rust-json-parse");
    
    
    let stringified = js_sys::JSON::stringify(&json_value).unwrap_or_else(|_| js_sys::JsString::from("{}"));
    console::log_1(&JsValue::from_str(&format!("[Rust] JSON response: {}", stringified.as_string().unwrap_or_default())));
    
    
    let res_text: String = match provider.as_str() {
        "openai" | "grok" | "glm" => {
            let js_obj = js_sys::Reflect::get(&json_value, &JsValue::from_str("choices"))?;
            if js_obj.is_undefined() {
                return Err(JsValue::from_str(&format!("API Error: {}", stringified.as_string().unwrap_or_default())));
            }
            let arr = js_sys::Array::from(&js_obj);
            let first = arr.get(0);
            let message = js_sys::Reflect::get(&first, &JsValue::from_str("message"))?;
            let content = js_sys::Reflect::get(&message, &JsValue::from_str("content"))?;
            content.as_string().unwrap_or_default()
        },
        "anthropic" => {
            let js_obj = js_sys::Reflect::get(&json_value, &JsValue::from_str("content"))?;
            if js_obj.is_undefined() {
                let stringified = js_sys::JSON::stringify(&json_value).unwrap_or_else(|_| js_sys::JsString::from("{}"));
                return Err(JsValue::from_str(&format!("API Error: {}", stringified.as_string().unwrap_or_default())));
            }
            let arr = js_sys::Array::from(&js_obj);
            let first = arr.get(0);
            let text = js_sys::Reflect::get(&first, &JsValue::from_str("text"))?;
            text.as_string().unwrap_or_default()
        },
        "gemini" => {
            let candidates = js_sys::Reflect::get(&json_value, &JsValue::from_str("candidates"))?;
            if candidates.is_undefined() {
                let stringified = js_sys::JSON::stringify(&json_value).unwrap_or_else(|_| js_sys::JsString::from("{}"));
                return Err(JsValue::from_str(&format!("API Error: {}", stringified.as_string().unwrap_or_default())));
            }
            let arr = js_sys::Array::from(&candidates);
            let first = arr.get(0);
            let content = js_sys::Reflect::get(&first, &JsValue::from_str("content"))?;
            let parts = js_sys::Reflect::get(&content, &JsValue::from_str("parts"))?;
            let parts_arr = js_sys::Array::from(&parts);
            let first_part = parts_arr.get(0);
            let text = js_sys::Reflect::get(&first_part, &JsValue::from_str("text"))?;
            text.as_string().unwrap_or_default()
        },
        _ => String::new()
    };

    Ok(JsValue::from_str(&res_text))
}
