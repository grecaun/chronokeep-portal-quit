use serde::{Deserialize, Serialize};

use crate::notifications;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag="command", rename_all="snake_case")]
pub enum Request {
    Quit,
    SetNoficiation {
        kind: notifications::Notification,
    },
}
