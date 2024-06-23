use crate::config::ChordAction;
use std::process::Command;

fn action_to_command(chord_action: &ChordAction, shell: Option<&Vec<String>>) -> Option<Command> {
    Some(match chord_action {
        ChordAction::Shell(chord_command) => {
            let Some(shell_conf) = shell else {
                log::error!("Cannot execute shell command without shell configured");
                return None;
            };
            let Some(shell) = shell_conf.first() else {
                log::error!("Configured shell is empty");
                return None;
            };
            let mut cmd = Command::new(shell);
            cmd.args(&shell_conf[1..]);
            cmd.arg(chord_command);
            cmd
        }
        ChordAction::Command(command) => {
            let Some(bin) = command.first() else {
                log::error!("Action command is empty");
                return None;
            };
            let mut cmd = Command::new(bin);
            cmd.args(&command[1..]);
            cmd
        }
    })
}

pub fn exec_action(chord_action: &ChordAction, shell: Option<&Vec<String>>) {
    let Some(mut cmd) = action_to_command(chord_action, shell) else {
        return;
    };
    std::thread::spawn(move || {
        let Ok(mut child) = cmd.spawn() else {
            log::error!("Failed to spawn child command: {:?}", cmd);
            return;
        };
        let Ok(_) = child.wait() else {
            log::error!("Failed to wait child process");
            return;
        };
    });
}
