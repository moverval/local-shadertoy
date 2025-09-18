use std::{
    path::PathBuf,
    process::exit,
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};

use crate::background_worker::{BackgroundWorker, WorkerUpdate};
use crate::st_shader::ShadertoyShader;
use crate::watched_file::WatchedFile;
use glium::{
    DrawParameters,
    ProgramCreationError::{self},
    Surface,
    glutin::surface::WindowSurface,
    program::SourceCode,
    uniforms::UniformsStorage,
    vertex::{EmptyVertexAttributes, VerticesSource},
    winit::{application::ApplicationHandler, event::WindowEvent, window::Window},
};

pub enum UpdateError {
    Compilation(ProgramCreationError),
    Skip,
}

pub struct Application {
    window: Window,
    display: glium::Display<WindowSurface>,
    max_frames: f64,
    shader: ShadertoyShader,
    has_errors: bool,
    frames_made: u32,
    created: Instant,
    updates: Receiver<WorkerUpdate>,
    mouse_x: f64,
    mouse_y: f64,
    average_frames: f64,
    pressed: bool,
}

impl Application {
    pub fn new(
        window: Window,
        display: glium::Display<WindowSurface>,
        path: PathBuf,
    ) -> Option<Application> {
        let watched_file = WatchedFile::new(path)?;
        let shader = ShadertoyShader::new(watched_file.read()?);

        let (mut background_worker, updates) =
            BackgroundWorker::new(watched_file, Duration::from_millis(200))
                .expect("File should exist");
        std::thread::spawn(move || {
            background_worker.work();
        });

        Some(Application {
            window,
            display,
            updates,
            shader,
            max_frames: 60.,
            average_frames: 60.,
            frames_made: 0,
            has_errors: false,
            mouse_x: 0.,
            mouse_y: 0.,
            pressed: false,
            created: Instant::now(),
        })
    }

    pub fn update(&mut self) -> Result<(), UpdateError> {
        while let Ok(update) = self.updates.try_recv() {
            match update {
                WorkerUpdate::NewShader(shader) => {
                    println!("New shader");
                    self.shader = shader;
                    self.has_errors = false;
                }
            }
        }

        // if self.frames_made % 500 == 0 {
        //     if let Some(monitor) = self.window.current_monitor() {
        //         if let Some(rr) = monitor.refresh_rate_millihertz() {
        //             self.max_frames = rr as f64 / 1000.;
        //         }
        //     }
        // }

        if self.has_errors {
            return Err(UpdateError::Skip);
        }

        let mut frame = self.display.draw();
        frame.clear_color(0., 0., 0., 1.);

        let source_code = SourceCode {
            vertex_shader: include_str!("res/vertex.glsl"),
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: self.shader.as_glsl(),
        };

        let program = match glium::program::Program::new(&self.display, source_code) {
            Ok(program) => program,
            Err(err) => {
                self.has_errors = true;
                frame.finish().ok();
                return Err(UpdateError::Compilation(err));
            }
        };

        let elapsed = self.created.elapsed();

        let size: (u32, u32) = self.window.inner_size().into();
        let uniforms_storage = UniformsStorage::new("iTime", elapsed.as_secs_f32());
        let uniforms_storage = uniforms_storage.add("iResolution", (size.0 as f32, size.1 as f32));
        let uniforms_storage = uniforms_storage.add("iFrame", self.frames_made as i32);
        let uniforms_storage =
            uniforms_storage.add("iMouse", (self.mouse_x as f32, self.mouse_y as f32, 0., 0.));
        let uniforms_storage = uniforms_storage.add("iFrameRate", self.average_frames as f32);

        self.frames_made += 1;

        frame
            .draw(
                VerticesSource::from(EmptyVertexAttributes { len: 3 }),
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &program,
                &uniforms_storage,
                &DrawParameters::default(),
            )
            .expect("This should draw");
        frame.finish().unwrap();

        Ok(())
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, _event_loop: &glium::winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &glium::winit::event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                exit(0);
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                if self.pressed {
                    self.mouse_x = position.x;
                    self.mouse_y = position.y;
                }
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button: glium::winit::event::MouseButton::Left,
            } => {
                self.pressed = state.is_pressed();
            }
            WindowEvent::RedrawRequested => {
                let start = Instant::now();
                let update = self.update();

                if let Err(UpdateError::Compilation(err)) = update {
                    println!("{err}");
                }

                let duration = start.elapsed();
                let justified = 1e6 / self.max_frames;

                if (duration.as_micros() as f64) < justified {
                    std::thread::sleep(Duration::from_micros(
                        (justified - (duration.as_micros() as f64)) as u64,
                    ));
                }

                let duration = start.elapsed();
                let frames = 1e6 / (duration.as_micros() as f64);
                self.average_frames *= 0.9 * frames / self.average_frames;

                self.window.request_redraw();
            }
            _ => {}
        }
    }
}
