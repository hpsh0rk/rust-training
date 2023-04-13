// #![allow(clippy::needless_option_as_deref)]
use gpt::{Config, Message, OpenAiClient};
use pyo3::{exceptions, prelude::*};

#[pyfunction]
pub fn chat(context: &str, output: Option<&str>) -> PyResult<String> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let client = OpenAiClient::new(Config::default()).unwrap();

    let default_system_message = Message::new_default_system();
    let mut messages = vec![default_system_message, Message::new(context)];

    let data = client.send_message_block(&messages).unwrap();
    Ok(data)
}

#[pymodule]
fn gpt_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(chat, m)?)?;
    Ok(())
}
