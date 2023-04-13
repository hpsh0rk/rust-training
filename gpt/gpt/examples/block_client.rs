use std::io::{self, Write};

use gpt::{Config, Message, OpenAiClient};

fn main() {
    let client = OpenAiClient::new(Config::default()).unwrap();

    let default_system_message = Message::new_default_system();
    let mut messages = vec![default_system_message];
    loop {
        print!("You: ");
        io::stdout().flush().unwrap(); // 刷新输出缓冲区

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // 读取用户输入

        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }
        messages.push(Message::new(input.trim()));
        let result = client.send_message_block(&messages).unwrap();
        messages.push(Message::new_assistant(&result));

        println!("Gpt: {}", result);
    }
}
