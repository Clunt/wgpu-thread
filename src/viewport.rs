use std::sync::Arc;
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

#[derive(Debug)]
pub struct Viewport {
    window: Arc<Window>,
    instance: Instance,
    surface: Surface<'static>,
    adapter: Option<Adapter>,
    device: Option<Device>,
    queue: Option<Queue>,
    config: Option<SurfaceConfiguration>,
}

impl Viewport {
    pub fn new(window: Arc<Window>) -> Self {
        let instance = Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        Self {
            window,
            instance,
            surface,
            adapter: None,
            device: None,
            queue: None,
            config: None,
        }
    }

    pub fn build(&mut self) {
        println!("viewport build");

        let adapter = pollster::block_on(async {
            self.instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    // Request an adapter which can render to our surface
                    compatible_surface: Some(&self.surface),
                    ..Default::default()
                })
                .await
                .unwrap()
        });

        let (device, queue) = pollster::block_on(async {
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::default(),
                        // required_limits: wgpu::Limits::downlevel_defaults(),
                        // required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        //     .using_resolution(adapter.limits()),
                    },
                    None,
                )
                .await
                .unwrap()
        });
        println!("device limits {:?}", device.limits());

        let size = self.window.inner_size();
        println!("build size {size:?}");

        let config = self
            .surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        self.surface.configure(&device, &config);

        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);

        self.window.request_redraw();
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        println!("viewport resize {size:?}");

        let config = self.config.as_mut().unwrap();
        config.width = size.width;
        config.height = size.height;

        if let Some(device) = self.device.as_ref() {
            self.surface.configure(device, config);
        }
    }

    pub fn paint(&self) {
        println!("viewport paint");
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .as_ref()
            .unwrap()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.3,
                            g: 0.2,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.as_ref().unwrap().submit(Some(encoder.finish()));
        frame.present();
    }
}
