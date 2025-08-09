use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    Close,
    Configure { width: u32, height: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: Option<SurfaceId>,
    pub kind: EventKind,
}
