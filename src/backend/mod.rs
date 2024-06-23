use crate::{config::Backend, key::Key};
use crossbeam_channel::Receiver;
use std::thread::JoinHandle;

#[cfg(feature = "backend-evdev")]
mod evdev;

#[must_use]
pub fn start_backend(backend: Backend) -> (Receiver<Event>, JoinHandle<()>) {
    match backend {
        #[cfg(feature = "backend-evdev")]
        Backend::Evdev { device, retry } => evdev::start(device, retry),
    }
}

pub enum Event {
    Stop,
    Pressed(Key),
    Released(Key),
}
