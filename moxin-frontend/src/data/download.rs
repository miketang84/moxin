use makepad_widgets::SignalToUI;
use moxin_backend::Backend;
use moxin_protocol::data::*;
use moxin_protocol::protocol::{Command, FileDownloadResponse};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub enum DownloadFileAction {
    Progress(FileID, f64),
    StreamingDone,
}

#[derive(Debug)]
pub struct Download {
    pub file: File,
    pub model: Model,
    pub sender: Sender<DownloadFileAction>,
    pub receiver: Receiver<DownloadFileAction>,
    pub progress: f64,
    pub done: bool,
}

impl Download {
    pub fn new(file: File, model: Model, progress: f64, backend: &Backend) -> Self {
        let (tx, rx) = channel();
        let mut download = Self {
            file: file,
            model: model,
            progress,
            sender: tx,
            receiver: rx,
            done: false,
        };

        download.start(backend);
        download
    }

    pub fn start(&mut self, backend: &Backend) {
        let (tx, rx) = channel();

        let store_download_tx = self.sender.clone();
        let cmd = Command::DownloadFile(self.file.id.clone(), tx);
        backend.command_sender.send(cmd).unwrap();

        thread::spawn(move || loop {
            let mut is_done = false;
            if let Ok(response) = rx.recv() {
                match response {
                    Ok(response) => match response {
                        FileDownloadResponse::Completed(_completed) => {
                            is_done = true;
                            store_download_tx
                                .send(DownloadFileAction::StreamingDone)
                                .unwrap();
                        }
                        FileDownloadResponse::Progress(file, value) => store_download_tx
                            .send(DownloadFileAction::Progress(file, value as f64))
                            .unwrap(),
                    },
                    Err(err) => eprintln!("Error downloading file: {:?}", err),
                }
            } else {
                break
            }

            SignalToUI::set_ui_signal();
            if is_done {
                break
            }
        });
    }

    pub fn process_download_progress(&mut self) {
        for msg in self.receiver.try_iter() {
            match msg {
                DownloadFileAction::StreamingDone => {
                    self.done = true;
                    //println!("Download complete");
                }
                DownloadFileAction::Progress(_file, value) => {
                    self.progress = value;
                    // println!("Download {:?} progress: {:?}", file, value);
                }
            }
        }
    }
}
