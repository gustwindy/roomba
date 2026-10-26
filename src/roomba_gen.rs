use std::collections::HashMap;

use poise::serenity_prelude::futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
};
use serde_json::Value;
use tokio::time::{timeout, Duration};

#[derive(Deserialize, Serialize)]
pub struct HistoryMessage {
    pub author_id: u64,
    pub id: u64,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct AskReplyRequest {
    pub user_ids: HashMap<u64, String>,
    pub bot_id: u64,
    pub messages: Vec<HistoryMessage>,
}

#[derive(Deserialize, Serialize)]
struct Request {
    content: AskReplyRequest,
    msg_type: String,
}

pub async fn send_to_roomba(uri: &str, ask_reply_req: AskReplyRequest) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let (mut ws, _) = connect_async(uri).await?;

    let request = Request {
        msg_type: String::from("ask_reply"),
        content: ask_reply_req
    };

    ws.send(Message::text(serde_json::to_string(&request)?)).await?;

    let response = timeout(Duration::from_secs(2), async {
        while let Some(message) = ws.next().await {
            if let Message::Text(text) = message? {
                let value: Value = serde_json::from_str(&text)?;

                match value["type"].as_str() {
                    Some("pong") => {
                        return Ok::<_, Box<dyn std::error::Error + Send + Sync>>(None);
                    }
                    Some("send") => {
                        return Ok(Some(value));
                    }
                    Some("error") => {
                        return Err(format!("Server error: {}", value["content"]).into());
                    }
                    _ => continue,
                }
            }
        }

        Err("WebSocket closed before receiving a response".into())
    })
    .await??;

    if let Some(value) = response {
        return Ok(value);
    }

    let response = timeout(Duration::from_secs(15), async {
        while let Some(message) = ws.next().await {
            if let Message::Text(text) = message? {
                let value: Value = serde_json::from_str(&text)?;

                if value["type"] == "send" {
                    return Ok::<_, Box<dyn std::error::Error + Send + Sync>>(value);
                }
            }
        }

        Err("WebSocket closed before receiving a send message".into())
    })
    .await??;

    Ok(response)
}
