use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TicketId(pub u64);

impl Display for TicketId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TicketId({})", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: String,
    pub description: String,
    pub status: Status,
}

impl Display for Ticket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ticket {{ id: {}, title: {}, description: {}, status: {:?} }}",
            self.id, self.title, self.description, self.status
        )
    }
} 

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketPatch {
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
