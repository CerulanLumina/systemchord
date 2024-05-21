use std::collections::HashSet;
use std::process::{Command, Stdio};

use crossbeam_channel::Receiver;

use crate::backend::{Key, KeyEdge, KeyEvent};
use crate::config::CoreConfig;

fn scan_chords(current_keys: &HashSet<Key>, config: &CoreConfig) {
    for chord in config.hotchords.iter().filter(|a| a.matches(current_keys)) {
        run_action(&chord.action);
    }
}

fn run_action(action: &[String]) {
    log::info!("Running `{}`", action.join(" "));
    let mut com = Command::new(&action[0]);
    com.args(action.iter().skip(1));
    com.stdout(Stdio::null());
    com.stdin(Stdio::null());
    com.stderr(Stdio::null());
    match com.spawn() {
        Ok(mut c) => {
            std::thread::spawn(move || {
                if let Err(err) = c.wait() {
                    log::error!("Failed to run action, process didn't start. Error: {err:?}");
                }
            });
        },
        Err(err) => {
            log::error!("Failed to run action. Error: {err:?}");
        }
    }
}

pub fn start(receiver: Receiver<KeyEvent>, config: CoreConfig) {
    log::info!("Loaded {} hotchords.", config.hotchords.len());
    let mut current_keys = HashSet::new();
    for event in receiver {
        match event.edge() {
            KeyEdge::Pressed => {
                if !current_keys.insert(event.key()) {
                    log::debug!("Duplicate key in set, were events dropped?");
                }

                scan_chords(&current_keys, &config);

            }
            KeyEdge::Released => {
                if !current_keys.remove(&event.key()) {
                    log::debug!("Tried to remove absent key, were events dropped?");
                }
            }
            _ => {}
        }
    }
    log::error!("Polling thread gave up, giving up as well.");
}
