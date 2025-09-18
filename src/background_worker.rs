use std::{path::PathBuf, process::exit, sync::mpsc::{self, Receiver, Sender}, time::{Duration, Instant}};

use crate::{st_shader::ShadertoyShader, watched_file::WatchedFile};

pub struct BackgroundWorker {
    watched_file: WatchedFile,
    update_delta: Duration,
    last_update: Instant,
    no_file_reported: usize,
    sender: Sender<WorkerUpdate>,
}

#[derive(Debug)]
pub struct NoFile;

pub enum WorkAbortion {
    NoFile,
}

pub enum WorkerUpdate {
    NewShader(ShadertoyShader),
}

impl BackgroundWorker {
    pub fn new(watched_file: WatchedFile, update_delta: Duration) -> Result<(BackgroundWorker, Receiver<WorkerUpdate>), NoFile> {
        let (sender, receiver) = mpsc::channel();

        Ok((BackgroundWorker {
            watched_file,
            update_delta,
            last_update: Instant::now(),
            sender,
            no_file_reported: 0,
        }, receiver))
    }

    pub fn work(&mut self) -> WorkAbortion {
        loop {
            let elapsed = self.last_update.elapsed();

            if elapsed < self.update_delta {
                std::thread::yield_now();
                continue
            }

            let modified = self.watched_file.is_modified();

            let modified = match modified {
                Some(modified) => modified,
                None => if self.watched_file.exists() {
                    self.no_file_reported = 0;
                    continue
                } else {
                    if self.no_file_reported >= 5 {
                        println!("Shader File was deleted. Quitting Program...");
                        exit(-1);
                    } else {
                        self.no_file_reported += 1;
                        self.last_update = Instant::now();
                        continue
                    }
                },
            };

            if !modified {
                self.last_update = Instant::now();
                continue
            }

            let content = match self.watched_file.read() {
                Some(content) => content,
                None => {
                    println!("File not readable. Try again later");
                    continue
                }
            };

            let shader = ShadertoyShader::new(content);

            self.sender.send(WorkerUpdate::NewShader(shader)).expect("Should send Shader");
            self.last_update = Instant::now();
            self.watched_file.accept_changes();
        }
    }
}
