use application::Application;
use clap::Parser;
use glium::{backend::glutin::SimpleWindowBuilder, winit::event_loop::EventLoop};
use parser::Args;

mod application;
mod background_worker;
mod parser;
mod st_shader;
mod watched_file;

fn main() {
    let args = Args::parse();
    let event_loop = EventLoop::new().unwrap();
    let (window, display) = SimpleWindowBuilder::new()
        .with_title("Shadertoy")
        .build(&event_loop);

    let mut application =
        Application::new(window, display, args.shader.into()).expect("File should exist");

    event_loop.run_app(&mut application).unwrap()
}
