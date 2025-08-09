use smithay_client_toolkit::{
    delegate_registry,
    output::OutputState,
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::SeatState,
};
use tokio::sync::mpsc::{Sender, channel};
use wayland_client::protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer};

use crate::*;

mod compositor;
mod keyboard;
mod layer;
mod output;
mod pointer;
mod seat;
mod window;

pub(crate) struct Client {
    pub(crate) tx: Sender<Event>,

    pub(crate) keyboard: Option<WlKeyboard>,
    pub(crate) pointer: Option<WlPointer>,

    pub(crate) registry_state: RegistryState,
    pub(crate) seat_state: SeatState,
    pub(crate) output_state: OutputState,
}

impl Client {
    pub(crate) fn new(wl: &Wl) -> (Self, Stream) {
        let registry_state = RegistryState::new(&wl.globals);
        let seat_state = SeatState::new(&wl.globals, &wl.qh);
        let output_state = OutputState::new(&wl.globals, &wl.qh);

        let (keyboard, pointer) = (None, None);

        let (tx, rx) = channel(128);
        let stream = Stream::new(rx);

        (
            Self {
                output_state,
                seat_state,
                registry_state,

                pointer,
                keyboard,

                tx,
            },
            stream,
        )
    }

    pub(crate) fn send(&self, id: Option<SurfaceId>, kind: EventKind) {
        let tx = self.tx.clone();

        tokio::task::spawn(async move {
            tx.send(Event { id, kind }).await.unwrap_or_else(|e| {
                tracing::error!("Error {}", e);
            });
        });
    }
}

delegate_registry!(Client);

impl ProvidesRegistryState for Client {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
