use reqwest::Client;
use serde_json::{Value, json};
use tokio::time::{sleep, Duration};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let client = Client::new();
    let mut offset = 0;

    loop {
        let get_updates_url = format!("https://api.telegram.org/bot{}/getUpdates?offset={}", token, offset);
        let updates_res = client.get(&get_updates_url)
            .send()
            .await
            .expect("Failed to get updates");

        let updates: Value = updates_res.json().await.expect("Failed to parse JSON");

        for update in updates["result"].as_array().unwrap_or(&vec![]) {
            let chat_id = update["message"]["chat"]["id"].as_i64().unwrap();
            let message_text = update["message"]["text"].as_str().unwrap_or("");

            let response_text = match message_text {
                "/start" => {
                    let keyboard = json!({
                        "inline_keyboard": [
                            [
                                {"text": "Кнопка 1", "callback_data": "button1"},
                                {"text": "Кнопка 2", "callback_data": "button2"}
                            ]
                        ]
                    });

                    let send_message_url = format!("https://api.telegram.org/bot{}/sendMessage", token);
                    client.post(&send_message_url)
                        .json(&json!({
                            "chat_id": chat_id,
                            "text": "Выберите кнопку:",
                            "reply_markup": keyboard,
                        }))
                        .send()
                        .await
                        .expect("Failed to send message");

                    offset = update["update_id"].as_i64().unwrap() + 1;
                    continue;
                },
                "/help" => "Доступные команды: /start, /help.".to_string(),
                _ => format!("Вы написали: {}", message_text),
            };

            if message_text != "/start" {
                let send_message_url = format!("https://api.telegram.org/bot{}/sendMessage", token);
                client.post(&send_message_url)
                    .json(&json!({
                        "chat_id": chat_id,
                        "text": response_text,
                    }))
                    .send()
                    .await
                    .expect("Failed to send message");
            }

            offset = update["update_id"].as_i64().unwrap() + 1;
        }

        sleep(Duration::from_secs(1)).await;
    }
}
