use std::{
    io::{Read, Write},
    thread::sleep,
    time::Duration,
};

use anyhow::{anyhow, bail, Context, Result};
use eventsource_stream::Eventsource;
use futures_util::StreamExt;
use reqwest::{Client, Proxy, RequestBuilder};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::runtime::Runtime;

use super::message::Message;

pub struct Model {
    name: String,
    limit_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    /// OpenAi Chat URL
    api_url: Option<String>,
    /// OpenAi Key
    api_key: Option<String>,
    /// Openai organization id
    organization_id: Option<String>,
    /// Set proxy
    proxy: Option<String>,
    /// Set a timeout in seconds for connect to gpt
    connect_timeout: usize,
    /// What sampling temperature to use, between 0 and 2
    temperature: Option<f64>,
    #[serde(skip)]
    model: (String, usize),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // TODO: api url 不应该放这，应该是每个请求维护自己的接口？
            api_url: Some("https://api.openai.com/v1/chat/completions".to_owned()),
            api_key: Some("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".to_owned()),
            proxy: None,
            connect_timeout: 10,
            model: ("gpt-3.5-turbo".into(), 4096),
            temperature: None,
            organization_id: None,
        }
    }
}

impl Config {
    pub fn get_connect_timeout(&self) -> Duration {
        Duration::from_secs(self.connect_timeout as u64)
    }
    pub fn get_model(&self) -> (String, usize) {
        self.model.clone()
    }
    // TODO: 在config里面build message，不太对吧？
    // pub fn build_messages(&self, content: &str) -> Result<Vec<Message>> {
    //     let default_system_message = Message::new_default_system();
    //     let message = Message::new(content);
    //     Ok(vec![default_system_message, message])
    // }
    pub fn get_temperature(&self) -> Option<f64> {
        self.temperature
    }
    pub fn get_api_url(&self) -> String {
        self.api_url.as_ref().expect("api url not set").into()
    }
    pub fn get_api_key(&self) -> (String, Option<String>) {
        let api_key = self.api_key.as_ref().expect("api_key not set");
        let organization_id = self.organization_id.as_ref();
        (api_key.into(), organization_id.cloned())
    }
}

#[derive(Debug)]
pub struct OpenAiClient {
    config: Config,
    runtime: Runtime,
}

impl OpenAiClient {
    pub fn new(config: Config) -> Result<Self> {
        let runtime = init_runtime()?;
        let s = Self { config, runtime };
        let _ = s.build_client()?; // check error
        Ok(s)
    }

    pub fn send_message_block(&self, messages: &Vec<Message>) -> Result<String> {
        self.runtime.block_on(async {
            self.send_message_inner(messages)
                .await
                .with_context(|| "Failed to fetch")
        })
    }

    // FIX: 不知道为啥调用这个方法报错
    pub async fn send_message(&self, messages: &Vec<Message>) -> Result<String> {
        self.runtime.block_on(async {
            self.send_message_inner(messages)
                .await
                .with_context(|| "Failed to fetch")
        })
    }

    pub fn send_message_streaming(
        &self,
        messages: &Vec<Message>,
        writer: impl Write,
    ) -> Result<()> {
        self.runtime.block_on(async {
            tokio::select! {
                ret = self.send_message_streaming_inner(messages, writer) => {
                    ret.with_context(|| "Failed to fetch stream")
                }
            }
        })
    }

    async fn send_message_inner(&self, messages: &Vec<Message>) -> Result<String> {
        let builder = self.request_builder(messages, false)?;
        let data: Value = builder.send().await?.json().await?;
        if let Some(err_msg) = data["error"]["message"].as_str() {
            bail!("Request failed, {err_msg}");
        }

        let output = data["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Unexpected response {data}"))?;

        Ok(output.to_string())
    }

    async fn send_message_streaming_inner(
        &self,
        messages: &Vec<Message>,
        mut writer: impl Write,
    ) -> Result<()> {
        let builder = self.request_builder(messages, true)?;
        let res = builder.send().await?;
        if !res.status().is_success() {
            let data: Value = res.json().await?;
            if let Some(err_msg) = data["error"]["message"].as_str() {
                bail!("Request failed, {err_msg}");
            }
            bail!("Request failed");
        }
        let mut stream = res.bytes_stream().eventsource();
        while let Some(part) = stream.next().await {
            let chunk = part?.data;
            if chunk == "[DONE]" {
                break;
            } else {
                let data: Value = serde_json::from_str(&chunk)?;
                let text = data["choices"][0]["delta"]["content"]
                    .as_str()
                    .unwrap_or_default();
                if text.is_empty() {
                    continue;
                }
                writer.write(text.as_bytes());
            }
        }

        Ok(())
    }

    fn build_client(&self) -> Result<Client> {
        let mut builder = Client::builder();
        if let Some(proxy) = self.config.proxy.as_ref() {
            builder = builder
                .proxy(Proxy::all(proxy).with_context(|| format!("Invalid proxy `{proxy}`"))?);
        }
        let timeout = self.config.get_connect_timeout();
        let client = builder
            .connect_timeout(timeout)
            .build()
            .with_context(|| "Failed to build http client")?;
        Ok(client)
    }

    fn request_builder(&self, messages: &Vec<Message>, stream: bool) -> Result<RequestBuilder> {
        let (model, _) = self.config.get_model();
        // let messages = self.config.build_messages(content)?;
        let mut body = json!({
            "model": model,
            "messages": messages,
        });

        if let Some(v) = self.config.get_temperature() {
            body.as_object_mut()
                .and_then(|m| m.insert("temperature".into(), json!(v)));
        }

        if stream {
            body.as_object_mut()
                .and_then(|m| m.insert("stream".into(), json!(true)));
        }

        let (api_key, organization_id) = self.config.get_api_key();

        let mut builder = self
            .build_client()?
            .post(self.config.get_api_url())
            .bearer_auth(api_key)
            .json(&body);

        if let Some(organization_id) = organization_id {
            builder = builder.header("OpenAI-Organization", organization_id);
        }

        Ok(builder)
    }
}

fn init_runtime() -> Result<Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .with_context(|| "Failed to init tokio")
}

#[cfg(test)]
mod test {
    use std::io;

    use super::*;

    #[test]
    fn config_check_and_connect_test() {
        let client = OpenAiClient::new(Config::default()).unwrap();

        let default_system_message = Message::new_default_system();
        let message = Message::new("hello");
        let messages = vec![default_system_message, message];

        let result = client.send_message_block(&messages).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn stream_chat_test() {
        let client = OpenAiClient::new(Config::default()).unwrap();

        let default_system_message = Message::new_default_system();
        let message = Message::new("hello");
        let messages = vec![default_system_message, message];

        client
            .send_message_streaming(&messages, io::stdout())
            .unwrap();
    }
}
