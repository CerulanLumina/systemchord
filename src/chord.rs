use crate::{
    backend::Event,
    config::{Chord, ChordAction, ChordOpts},
    exec,
    key::Key,
};
use crossbeam_channel::Receiver;
use itertools::Itertools;
use std::{collections::HashSet, thread, thread::JoinHandle};

fn update(state: &mut HashSet<Key>, event: Event) {
    match event {
        Event::Pressed(key) => {
            if !state.insert(key) {
                log::warn!("Duplicate press of {key}, were events dropped?");
            }
        }
        Event::Released(key) => {
            if !state.remove(&key) {
                log::warn!("Duplicate release of {key}, were events dropped?");
            }
        }
        Event::Stop => {
            log::debug!("Device disconnected, clearing cache.");
            state.clear();
        }
    }
}

fn match_chords<'a, 'b: 'a, 'c: 'a>(
    state: &'c HashSet<Key>,
    chords: impl Iterator<Item = &'a Chord>,
    chord_opts: &'b ChordOpts,
) -> impl Iterator<Item = &'a ChordAction> {
    chords
        .filter(move |chord| {
            // chords that match the state
            let use_exclusive = chord
                .options
                .and_then(|e| e.exclusive)
                .unwrap_or(chord_opts.exclusive);

            let matches_inclusive = chord.sequence.iter().all(|seq_key| {
                // everything in the configured sequence
                seq_key
                    .matching()
                    .any(|seq_key_opt| state.contains(seq_key_opt)) // at least one of the options for the named key matches
            });

            if matches_inclusive && use_exclusive {
                // if we had a match *and* we're using exclusive match, do more checks
                let mut only_matching = true;
                // check every key in the state is a match for the given chord
                for state_key in state {
                    if !chord
                        .sequence
                        .iter()
                        .any(|seq_key| seq_key.matches(state_key))
                    {
                        only_matching = false;
                        break;
                    }
                }
                only_matching // we have confirmed that we have an inclusive match, so only return whether we had an exclusive match too
            } else {
                // not using exclusive mode or no match
                // return if we had a match and not using exclusive mode
                matches_inclusive && !use_exclusive
            }
        })
        .map(|chord| {
            (
                &chord.action,
                chord
                    .options
                    .and_then(|opts| opts.passthrough)
                    .unwrap_or(chord_opts.passthrough),
            )
        })
        .take_while_inclusive(|(_, passthrough)| *passthrough)
        .map(|(action, _)| action)
}

pub fn chord_handler(
    recv: Receiver<Event>,
    chords: Vec<Chord>,
    chord_opts: ChordOpts,
    shell: Option<Vec<String>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut keyboard_state = HashSet::with_capacity(256);
        for event in recv {
            update(&mut keyboard_state, event);
            for action in match_chords(&keyboard_state, chords.iter(), &chord_opts) {
                exec::exec_action(action, shell.as_ref());
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        chord::match_chords,
        config::{Chord, ChordAction, ChordOpts, ChordOptsChild, ConfiguredKey},
        key::Key,
    };
    use std::str::FromStr;

    trait VParse {
        fn parse(self) -> Vec<ConfiguredKey>;
    }

    impl VParse for Vec<&str> {
        fn parse(self) -> Vec<ConfiguredKey> {
            self.into_iter()
                .map(ConfiguredKey::from_str)
                .collect::<Result<Vec<ConfiguredKey>, _>>()
                .expect("Parsing")
        }
    }

    #[test]
    fn match_non_exclusive_passthrough() {
        let chords = vec![
            Chord {
                sequence: vec!["ctrl", "a"].parse(),
                action: ChordAction::Shell("one".to_owned()),
                options: None,
            },
            Chord {
                sequence: vec!["ctrl", "a", "b"].parse(),
                action: ChordAction::Shell("two".to_owned()),
                options: None,
            },
            Chord {
                sequence: vec!["ctrl", "a", "b", "c"].parse(),
                action: ChordAction::Shell("three".to_owned()),
                options: None,
            },
            Chord {
                sequence: vec!["ctrl", "a", "b", "z"].parse(),
                action: ChordAction::Shell("not matching".to_owned()),
                options: None,
            },
        ];

        let state = maplit::hashset! { Key::LeftCtrl, Key::A, Key::B, Key::C, Key::D };
        let actions = match_chords(
            &state,
            chords.iter(),
            &ChordOpts {
                passthrough: true,
                exclusive: false,
            },
        )
        .collect::<Vec<_>>();

        cool_asserts::assert_matches!(actions[0], ChordAction::Shell(b) if b == "one");
        cool_asserts::assert_matches!(actions[1], ChordAction::Shell(b) if b == "two");
        cool_asserts::assert_matches!(actions[2], ChordAction::Shell(b) if b == "three");
        assert_eq!(actions.len(), 3);
    }

    #[test]
    fn match_non_exclusive_non_passthrough_part() {
        let chords = vec![
            Chord {
                sequence: vec!["ctrl", "a"].parse(),
                action: ChordAction::Shell("one".to_owned()),
                options: None,
            },
            Chord {
                sequence: vec!["ctrl", "a", "b"].parse(),
                action: ChordAction::Shell("two".to_owned()),
                options: Some(ChordOptsChild {
                    passthrough: Some(false),
                    exclusive: None,
                }),
            },
            Chord {
                sequence: vec!["ctrl", "a", "b", "c"].parse(),
                action: ChordAction::Shell("three".to_owned()),
                options: None,
            },
            Chord {
                sequence: vec!["ctrl", "a", "b", "z"].parse(),
                action: ChordAction::Shell("not matching".to_owned()),
                options: None,
            },
        ];

        let state = maplit::hashset! { Key::LeftCtrl, Key::A, Key::B, Key::C, Key::D };
        let actions = match_chords(
            &state,
            chords.iter(),
            &ChordOpts {
                passthrough: true,
                exclusive: false,
            },
        )
        .collect::<Vec<_>>();

        cool_asserts::assert_matches!(actions[0], ChordAction::Shell(b) if b == "one");
        cool_asserts::assert_matches!(actions[1], ChordAction::Shell(b) if b == "two");
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn match_exclusive() {
        let chords = vec![
            Chord {
                sequence: vec!["ctrl", "a"].parse(),
                action: ChordAction::Shell("one".to_owned()),
                options: None,
            },
            Chord {
                sequence: vec!["ctrl", "a", "b"].parse(),
                action: ChordAction::Shell("two".to_owned()),
                options: Some(ChordOptsChild {
                    passthrough: None,
                    exclusive: Some(true),
                }),
            },
            Chord {
                sequence: vec!["ctrl", "a", "b", "c"].parse(),
                action: ChordAction::Shell("three".to_owned()),
                options: None,
            },
            Chord {
                sequence: vec!["ctrl", "a", "b", "z"].parse(),
                action: ChordAction::Shell("not matching".to_owned()),
                options: None,
            },
        ];

        let state = maplit::hashset! { Key::LeftCtrl, Key::A, Key::B, Key::C, Key::D };
        let actions = match_chords(
            &state,
            chords.iter(),
            &ChordOpts {
                passthrough: true,
                exclusive: false,
            },
        )
        .collect::<Vec<_>>();

        cool_asserts::assert_matches!(actions[0], ChordAction::Shell(b) if b == "one");
        cool_asserts::assert_matches!(actions[1], ChordAction::Shell(b) if b == "three");
        assert_eq!(actions.len(), 2);
    }
}
