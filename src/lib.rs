pub use eyre::{Report, Result};

mod event;
pub use event::*;

mod stream;
pub use stream::*;

mod surface;
pub use surface::*;

mod client;
pub(crate) use client::*;

mod wgpu;
pub use wgpu::*;

mod wl;
pub use wl::*;

mod scene;
pub use scene::*;

pub struct WlWgpu {
    pub(crate) wl: Wl,
    pub(crate) wgpu: Wgpu,
}

impl WlWgpu {
    pub fn destroy_surface(&mut self, id: &SurfaceId) {
        self.wgpu.destroy_surface(id);
    }

    pub fn resize_surface(&mut self, id: &SurfaceId, width: u32, height: u32) {
        self.wgpu.resize_surface(id, width, height);
    }

    pub fn render(&mut self, id: &SurfaceId, scene: &Scene) -> Result<()> {
        self.wgpu.render(id, scene)
    }

    pub fn surfaces(&self) -> usize {
        self.wgpu.surfaces.len()
    }

    pub fn size(&self, id: &SurfaceId) -> Result<(u32, u32)> {
        self.wgpu.size(id)
    }
}

pub async fn wlwgpu() -> Result<(WlWgpu, Stream)> {
    let wgpu = Wgpu::new();
    let (wl, stream) = Wl::new()?;

    let wlwgpu = WlWgpu { wl, wgpu };

    Ok((wlwgpu, stream))
}
