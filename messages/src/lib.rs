//! Messages
//!
//! Library that implements Maelstorm Challenge's RPC protocol.
//! Ref: https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md

use serde::{Deserialize, Serialize};

type MessageId = Option<i32>;

/// Message implements the basic RPC json fields for all Maelstorm
/// communication.
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

/// Body represents a Maelstorm RPC's primary payload.
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct Body {
    #[serde(rename = "type", flatten)]
    kind: MessageType,

    #[serde(rename = "msg_id", skip_serializing_if = "Option::is_none")]
    id: MessageId,

    #[serde(rename = "in_reply_to", skip_serializing_if = "Option::is_none")]
    reply_id: MessageId,
}

/// MessageType is used to extend the various request & response RPC types
/// for Maelstorm payload's.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MessageType {
    /// The request/response sent to Maelstorm Node's by the test.
    /// This is the first RPC call sent to any Maelstorm Node's and is mainly
    /// used to initialize the Maelstorm's challenege environment.
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,

    Unknown,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Message {
    fn new(src: String, dst: String) -> Self {
        Message {
            src,
            dst,
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> MessageType {
        self.body.kind.clone()
    }

    pub fn respond(&self) -> Self {
        let mut response = Message::new(self.dst.clone(), self.src.clone());

        match self.body.kind {
            MessageType::Init {
                node_id: _,
                node_ids: _,
            } => {
                response.body = Body {
                    kind: MessageType::InitOk,
                    id: None,
                    reply_id: self.body.id,
                }
            }
            _ => (),
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn init_is_deserialized() -> Result<()> {
        let json = r#"{
            "type":     "init",
            "msg_id":   1,
            "node_id":  "n3",
            "node_ids": ["n1", "n2", "n3"]
        }"#;

        let want = Body {
            kind: MessageType::Init {
                node_id: "n3".to_string(),
                node_ids: vec!["n1".to_string(), "n2".to_string(), "n3".to_string()],
            },
            id: Some(1),
            reply_id: None,
        };
        let got = serde_json::from_str::<Body>(&json)?;

        println!("\n-> Want:\n{:#?}", want);
        println!("\n-> Got:\n{:#?}", got);

        assert_eq!(want, got);

        Ok(())
    }

    #[test]
    fn init_ok_is_serialized() -> Result<()> {
        let body = Body {
            kind: MessageType::InitOk,
            id: None,
            reply_id: Some(1),
        };

        let want = r#"{"type":"init_ok","in_reply_to":1}"#;
        let got = serde_json::to_string(&body)?;

        println!("\n-> Want:\n{:#?}", want);
        println!("\n-> Got:\n{:#?}", got);

        assert_eq!(want, got);

        Ok(())
    }
}
