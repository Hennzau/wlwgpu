use crate::*;

use smithay_client_toolkit::{
    delegate_layer,
    reexports::client::{Connection, QueueHandle},
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShellHandler, LayerSurface, LayerSurfaceConfigure},
    },
};
use wayland_client::Proxy;

delegate_layer!(Client);

impl LayerShellHandler for Client {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, layer: &LayerSurface) {
        self.send(Some(layer.wl_surface().id().into()), EventKind::Close);
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        layer.set_size(configure.new_size.0, configure.new_size.1);
        layer.commit();

        self.send(
            Some(layer.wl_surface().id().into()),
            EventKind::Configure {
                width: configure.new_size.0,
                height: configure.new_size.1,
            },
        );
    }
}
