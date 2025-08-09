use std::time::Duration;

use smithay_client_toolkit::{
    compositor::CompositorState,
    shell::{wlr_layer::LayerShell, xdg::XdgShell},
};

use wayland_client::{
    Connection, QueueHandle,
    globals::{GlobalList, registry_queue_init},
};

use crate::*;

pub struct Wl {
    pub(crate) xdg_shell: XdgShell,
    pub(crate) layer_shell: LayerShell,

    pub(crate) compositor_state: CompositorState,

    pub(crate) qh: QueueHandle<Client>,
    pub(crate) globals: GlobalList,

    pub(crate) connection: Connection,
}

impl Wl {
    pub(crate) fn new() -> Result<(Self, Stream)> {
        let connection = Connection::connect_to_env()?;

        let (globals, mut event_queue) = registry_queue_init::<Client>(&connection)?;
        let qh = event_queue.handle();

        let compositor_state = CompositorState::bind(&globals, &qh)?;
        let xdg_shell = XdgShell::bind(&globals, &qh)?;
        let layer_shell = LayerShell::bind(&globals, &qh)?;

        let wl = Wl {
            xdg_shell,
            layer_shell,
            compositor_state,
            qh,
            globals,
            connection,
        };

        let (mut client, stream) = Client::new(&wl);

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(16)).await;

                if let Err(e) = event_queue.flush() {
                    tracing::error!("Error flushing events: {}", e);
                    continue;
                }

                if let Some(guard) = event_queue.prepare_read() {
                    if let Err(e) = guard.read_without_dispatch() {
                        tracing::error!("Error reading events: {:?}", e);
                    }
                }

                event_queue.dispatch_pending(&mut client).unwrap();
            }
        });

        Ok((wl, stream))
    }
}
