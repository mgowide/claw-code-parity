use serde::Deserialize;

/// Commands sent from the client to the server over WebSocket.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientCommand {
    SendMessage {
        session_id: String,
        text: String,
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
