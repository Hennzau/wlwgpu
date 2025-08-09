use wlwgpu::*;

#[tokio::main]
async fn main() -> Result<()> {
    let (mut wlwgpu, mut stream) = wlwgpu().await?;

    let surface = SurfaceBuilder::default()
        .window(true)
        .build(&mut wlwgpu)
        .await?;

    let handle_event = async |wlwgpu: &mut WlWgpu, event: Event| -> Result<bool> {
        let id = event.id;
        let kind = event.kind;

        match kind {
            EventKind::Close => {
                if let Some(id) = id {
                    wlwgpu.destroy_surface(&id);
                }

                if wlwgpu.surfaces() == 0 {
                    return Ok(true);
                }
            }
            EventKind::Configure { width, height } => {
                if let Some(id) = id {
                    wlwgpu.resize_surface(&id, width, height);
                }
            }
        }

        Ok(false)
    };

    let mut scene = Scene::new();

    let mut x = 0.0;

    loop {
        tokio::select! {
            Ok(event) = stream.next() => {
                let exit = handle_event(&mut wlwgpu, event).await?;

                if exit {
                    println!("No more surfaces, exiting...");
                    break;
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(16)) => {
                scene.clear();

                let (width, height) = wlwgpu.size(&surface)?;
                scene.fill(width, height, Color::new([1.0, 1.0, 1.0, 1.0]));

                let y = height as f64 / 2.0;

                scene.add_circle(x, y, 100.0, Color::new([0.9529, 0.5451, 0.6588, 1.]));

                wlwgpu.render(&surface, &scene)?;

                x += 1000.0 * 16.0 / 1000.0;
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Received Ctrl+C, exiting...");
                break;
            }
        }
    }

    Ok(())
}
