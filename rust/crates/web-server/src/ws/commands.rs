use serde::Deserialize;

/// A file attached to a `SendMessage` command.
#[derive(Debug, Clone, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub content: String,
}

/// Commands sent from the client to the server over WebSocket.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientCommand {
    SendMessage {
        session_id: String,
        text: String,
        #[serde(default)]
        attachments: Vec<Attachment>,
    },
    CancelTurn {
        session_id: String,
    },
    ApprovePermission {
        request_id: String,
    },
    DenyPermission {
        request_id: String,
    },
    Compact {
        session_id: String,
    },
    SwitchModel {
        model: String,
    },
    ResumeSession {
        session_id: String,
    },
}
