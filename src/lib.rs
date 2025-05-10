use wgpu::{Backends, Instance, InstanceDescriptor};
use winit::{application::ApplicationHandler, event_loop::{self, EventLoop}, window::{Window, WindowAttributes}};

pub trait Widget{
	fn build();
	fn draw();
	fn layout();
}

#[derive(Debug,Default)]
pub struct App{
	window: Option<Window>
}

impl App{
	pub fn new() -> Self{
		Self::default()
	}
	pub fn run(mut self){
		let event_loop = EventLoop::new().unwrap();

		event_loop.run_app(&mut self).unwrap();
	}	
}

impl ApplicationHandler for App{
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
		let attrs = WindowAttributes::default();
		let window = event_loop.create_window(attrs)
			.expect("Failed to create window");
		self.window = Some(window);
	}
	
	fn window_event(
		&mut self,
		event_loop: &winit::event_loop::ActiveEventLoop,
		window_id: winit::window::WindowId,
		event: winit::event::WindowEvent,
	) {
	}
}



pub fn render(){
	let instance = Instance::new(&InstanceDescriptor{
		backends: Backends::PRIMARY,
		..Default::default()
	});
}


