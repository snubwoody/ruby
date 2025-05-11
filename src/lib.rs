use bytemuck::{Pod, Zeroable};
use wgpu::{
    include_wgsl, wgt::DeviceDescriptor, Backends, Device, Features, Instance, InstanceDescriptor, PipelineLayoutDescriptor, Queue, RenderPass, RequestAdapterOptions, ShaderModuleDescriptor, ShaderStages, Surface, SurfaceConfiguration, TextureFormat, TextureUsages, VertexFormat
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, EventLoop},
    window::{Window, WindowAttributes},
};

pub trait Widget {
    fn build();
    fn draw();
    fn layout();
}

#[derive(Debug, Default)]
pub struct App {
    window: Option<Window>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attrs = WindowAttributes::default();
        let window = event_loop
            .create_window(attrs)
            .expect("Failed to create window");

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = self.window.as_ref().expect("No window present");

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                window.pre_present_notify();
            }
            _ => {}
        }
    }
}

/// Represents a single vertex with a 2D position, color and uv coordinates.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Pod, Default, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

impl Vertex {
    /// Creates a new [`Vertex`]
    pub fn new(x: f32, y: f32, color: [f32; 4]) -> Self {
        Self {
            position: [x, y],
            color,
            uv: [1.0, 1.0],
        }
    }

    /// Creates a `Vec` of 6 `Vertices` in a quad layout.
    ///
    /// # Example
    /// ```
    /// use helium_core::{Size,Position,Color};
    /// use helium_renderer::Vertex;
    ///
    /// let size = Size::new(50.0,75.0);
    /// let position = Position::default();
    /// let color = Color::default();
    ///
    /// let vertices = Vertex::quad(size,position,color);
    ///
    /// assert_eq!(vertices[0].position[0],position.x);
    /// assert_eq!(vertices[5].position[0],position.x + size.width);
    /// ```
    pub fn quad(width: f32, height: f32, x: f32, y: f32) -> Vec<Self>{

        let vertex1 = Vertex::new(x, y, [0.0, 0.0,0.0,1.0]); //Top left
        let vertex2 = Vertex::new(x + width, y, [0.0, 0.0,0.0,1.0]); // Top right
        let vertex3 = Vertex::new(x, y + height,[0.0, 0.0,0.0,1.0]); //Bottom left
        let vertex4 = Vertex::new(x + width, y, [0.0, 0.0,0.0,1.0]); //Top right
        let vertex5 = Vertex::new(x, y + height, [0.0, 0.0,0.0,1.0]); // Bottom left
        let vertex6 = Vertex::new(x + width, y + height, [0.0, 0.0,0.0,1.0]); //Bottom right

        return vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    }
}

pub struct State<'a>{
	surface: Surface<'a>,
	device: Device,
	queue: Queue,
	config: SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
	window: &'a Window
}

impl<'a> State<'a> {
	async fn new(window: &'a Window) -> Self{
		let size = window.inner_size();

		let instance = Instance::new(&InstanceDescriptor { 
			backends: wgpu::Backends::PRIMARY, 
			..Default::default()
		});

		let surface = instance.create_surface(window).unwrap();

		let adapter = instance.request_adapter(&RequestAdapterOptions{
			compatible_surface: Some(&surface),
			..Default::default()
		}).await.unwrap();

		let (device,queue) = adapter.request_device(&Default::default())
			.await
			.unwrap();

		let caps = surface.get_capabilities(&adapter);

		let format = caps.formats
			.iter()
			.find(|f|f.is_srgb())
			.copied()
			.unwrap_or(caps.formats[0]);

		let config = wgpu::SurfaceConfiguration{
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			width: size.width,
			height: size.height,
			present_mode: caps.present_modes[0],
			alpha_mode: caps.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
			format,
		};

		Self{
			surface,
			device,
			queue,
			config,
			size,
			window
		}
	}

	pub fn window(&self) -> &Window{
		&self.window
	}

	/// Resize the surface size when the window size changes.
	/// 
	/// Attempting to draw when the `Surface` and `Window` are 
	/// different sizes will cause the program to crash.
	fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}