use crate::{backend::Event, key::Key};
use crossbeam_channel::{Receiver, Sender, TrySendError};
use evdev::{Device, InputEventKind, Key as EvKey};
use std::{
    fmt::{Display, Formatter},
    io,
    io::ErrorKind,
    path::{Path, PathBuf},
    thread,
    thread::JoinHandle,
    time::Duration,
};

const RETRY_PERIOD_MILLIS: u64 = 1000;

const RELEASED: i32 = 0;
const PRESSED: i32 = 1;
const HELD: i32 = 2;

pub fn start(dev_path: PathBuf, retry: bool) -> (Receiver<Event>, JoinHandle<()>) {
    log::trace!("Starting keyboard polling thread");
    let (sender, receiver) = crossbeam_channel::bounded(1024);
    let handle = thread::spawn(move || {
        if retry {
            let mut opened = false;
            loop {
                match open_and_poll(&dev_path, sender.clone(), Some(&mut opened)) {
                    Err(Error::NotFound | Error::Disconnected) => {
                        log::debug!(
                            "Device file not found, retrying in {} milliseconds",
                            RETRY_PERIOD_MILLIS
                        );
                        if opened {
                            log::info!("Keyboard device unavailable. Retrying periodically...");
                            if sender.send(Event::Stop).is_err() {
                                log::error!("{}", Error::Hangup);
                                break;
                            }
                            opened = false;
                        }
                        thread::sleep(Duration::from_millis(RETRY_PERIOD_MILLIS));
                    }
                    Err(other) => {
                        log::error!("An unrecoverable error occurred: {other}");
                        break;
                    }
                    Ok(_) => unreachable!(),
                }
            }
        } else {
            let err = open_and_poll(&dev_path, sender, None).unwrap_err();
            log::error!("An error occurred in the backend: {err}");
        }
    });
    (receiver, handle)
}

fn open_and_poll(
    dev_path: &Path,
    sender: Sender<Event>,
    opened: Option<&mut bool>,
) -> Result<Never, Error> {
    log::debug!("Opening device {}", dev_path.to_string_lossy());
    let mut dev = Device::open(dev_path)?;
    if let Some(opened) = opened {
        *opened = true;
    }
    loop {
        let events = dev.fetch_events().map_err(|_| Error::Disconnected)?;
        for (keycode, raw_event) in events.filter_map(|event| match event.kind() {
            InputEventKind::Key(key) => Some((key, event)),
            _ => None,
        }) {
            let Some(key) = map_key(keycode) else {
                log::debug!("Unsupported keycode {}", keycode.code());
                continue;
            };

            let event = match raw_event.value() {
                RELEASED => Event::Released(key),
                PRESSED => Event::Pressed(key),
                HELD => {
                    continue;
                }
                o => {
                    log::warn!("Unexpected event value `{o}` for key {}", keycode.code());
                    continue;
                }
            };
            match sender.try_send(event) {
                Err(TrySendError::Full(_)) => {
                    log::warn!("Overflowed capacity, events may be dropped.");
                }
                Err(TrySendError::Disconnected(_)) => {
                    return Err(Error::Hangup);
                }
                _ => {}
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => Self::NotFound,
            _ => Self::Io(value),
        }
    }
}

#[derive(Debug)]
enum Error {
    NotFound,
    Disconnected,
    Hangup,
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Device file not found"),
            Error::Disconnected => write!(f, "The device was disconnected"),
            Error::Hangup => write!(f, "The receiving thread hung up"),
            Error::Io(io) => write!(f, "{io}"),
        }
    }
}

#[derive(Debug)]
enum Never {}

fn map_key(input: EvKey) -> Option<Key> {
    Some(match input {
        EvKey::KEY_ESC => Key::Esc,
        EvKey::KEY_F1 => Key::F1,
        EvKey::KEY_F2 => Key::F2,
        EvKey::KEY_F3 => Key::F3,
        EvKey::KEY_F4 => Key::F4,
        EvKey::KEY_F5 => Key::F5,
        EvKey::KEY_F6 => Key::F6,
        EvKey::KEY_F7 => Key::F7,
        EvKey::KEY_F8 => Key::F8,
        EvKey::KEY_F9 => Key::F9,
        EvKey::KEY_F10 => Key::F10,
        EvKey::KEY_F11 => Key::F11,
        EvKey::KEY_F12 => Key::F12,
        EvKey::KEY_F13 => Key::F13,
        EvKey::KEY_F14 => Key::F14,
        EvKey::KEY_F15 => Key::F15,
        EvKey::KEY_F16 => Key::F16,
        EvKey::KEY_F17 => Key::F17,
        EvKey::KEY_F18 => Key::F18,
        EvKey::KEY_F19 => Key::F19,
        EvKey::KEY_F20 => Key::F20,
        EvKey::KEY_F21 => Key::F21,
        EvKey::KEY_F22 => Key::F22,
        EvKey::KEY_F23 => Key::F23,
        EvKey::KEY_F24 => Key::F24,
        EvKey::KEY_LEFTCTRL => Key::LeftCtrl,
        EvKey::KEY_RIGHTCTRL => Key::RightCtrl,
        EvKey::KEY_LEFTALT => Key::LeftAlt,
        EvKey::KEY_RIGHTALT => Key::RightAlt,
        EvKey::KEY_LEFTSHIFT => Key::LeftShift,
        EvKey::KEY_RIGHTSHIFT => Key::RightShift,
        EvKey::KEY_LEFTMETA => Key::LeftMeta,
        EvKey::KEY_RIGHTMETA => Key::RightMeta,
        EvKey::KEY_1 => Key::N1,
        EvKey::KEY_2 => Key::N2,
        EvKey::KEY_3 => Key::N3,
        EvKey::KEY_4 => Key::N4,
        EvKey::KEY_5 => Key::N5,
        EvKey::KEY_6 => Key::N6,
        EvKey::KEY_7 => Key::N7,
        EvKey::KEY_8 => Key::N8,
        EvKey::KEY_9 => Key::N9,
        EvKey::KEY_0 => Key::N0,
        EvKey::KEY_Q => Key::Q,
        EvKey::KEY_W => Key::W,
        EvKey::KEY_E => Key::E,
        EvKey::KEY_R => Key::R,
        EvKey::KEY_T => Key::T,
        EvKey::KEY_Y => Key::Y,
        EvKey::KEY_U => Key::U,
        EvKey::KEY_I => Key::I,
        EvKey::KEY_O => Key::O,
        EvKey::KEY_P => Key::P,
        EvKey::KEY_A => Key::A,
        EvKey::KEY_S => Key::S,
        EvKey::KEY_D => Key::D,
        EvKey::KEY_F => Key::F,
        EvKey::KEY_G => Key::G,
        EvKey::KEY_H => Key::H,
        EvKey::KEY_J => Key::J,
        EvKey::KEY_K => Key::K,
        EvKey::KEY_L => Key::L,
        EvKey::KEY_Z => Key::Z,
        EvKey::KEY_X => Key::X,
        EvKey::KEY_C => Key::C,
        EvKey::KEY_V => Key::V,
        EvKey::KEY_B => Key::B,
        EvKey::KEY_N => Key::N,
        EvKey::KEY_M => Key::M,
        EvKey::KEY_MINUS => Key::Minus,
        EvKey::KEY_EQUAL => Key::Equal,
        EvKey::KEY_BACKSPACE => Key::Backspace,
        EvKey::KEY_TAB => Key::Tab,
        EvKey::KEY_LEFTBRACE => Key::LeftBracket,
        EvKey::KEY_RIGHTBRACE => Key::RightBracket,
        EvKey::KEY_ENTER => Key::Enter,
        EvKey::KEY_SEMICOLON => Key::Semicolon,
        EvKey::KEY_APOSTROPHE => Key::Apostrophe,
        EvKey::KEY_GRAVE => Key::Grave,
        EvKey::KEY_BACKSLASH => Key::Backslash,
        EvKey::KEY_COMMA => Key::Comma,
        EvKey::KEY_DOT => Key::Dot,
        EvKey::KEY_SLASH => Key::Slash,
        EvKey::KEY_SPACE => Key::Space,
        EvKey::KEY_CAPSLOCK => Key::CapsLock,
        EvKey::KEY_NUMLOCK => Key::NumLock,
        EvKey::KEY_SCROLLLOCK => Key::ScrollLock,
        EvKey::KEY_KP0 => Key::KP0,
        EvKey::KEY_KP1 => Key::KP1,
        EvKey::KEY_KP2 => Key::KP2,
        EvKey::KEY_KP3 => Key::KP3,
        EvKey::KEY_KP4 => Key::KP4,
        EvKey::KEY_KP5 => Key::KP5,
        EvKey::KEY_KP6 => Key::KP6,
        EvKey::KEY_KP7 => Key::KP7,
        EvKey::KEY_KP8 => Key::KP8,
        EvKey::KEY_KP9 => Key::KP9,
        EvKey::KEY_KPENTER => Key::KPEnter,
        EvKey::KEY_KPDOT => Key::KPDot,
        EvKey::KEY_KPMINUS => Key::KPMinus,
        EvKey::KEY_KPPLUS => Key::KPPlus,
        EvKey::KEY_KPASTERISK => Key::KPAsterisk,
        EvKey::KEY_KPSLASH => Key::KPSlash,
        EvKey::KEY_KPJPCOMMA => Key::KPJPComma,
        EvKey::KEY_ZENKAKUHANKAKU => Key::Zenkakuhankaku,
        EvKey::KEY_RO => Key::Ro,
        EvKey::KEY_KATAKANA => Key::Katakana,
        EvKey::KEY_HIRAGANA => Key::Hiragana,
        EvKey::KEY_HENKAN => Key::Henkan,
        EvKey::KEY_KATAKANAHIRAGANA => Key::Katakanahiragana,
        EvKey::KEY_MUHENKAN => Key::Muhenkan,
        EvKey::KEY_SYSRQ => Key::SysRq,
        EvKey::KEY_LINEFEED => Key::Linefeed,
        EvKey::KEY_HOME => Key::Home,
        EvKey::KEY_UP => Key::Up,
        EvKey::KEY_PAGEUP => Key::Pageup,
        EvKey::KEY_LEFT => Key::Left,
        EvKey::KEY_RIGHT => Key::Right,
        EvKey::KEY_END => Key::End,
        EvKey::KEY_DOWN => Key::Down,
        EvKey::KEY_PAGEDOWN => Key::Pagedown,
        EvKey::KEY_INSERT => Key::Insert,
        EvKey::KEY_DELETE => Key::Delete,
        EvKey::KEY_MACRO => Key::Macro,
        EvKey::KEY_MUTE => Key::Mute,
        EvKey::KEY_VOLUMEDOWN => Key::VolumeDown,
        EvKey::KEY_VOLUMEUP => Key::VolumeUp,
        EvKey::KEY_POWER => Key::Power,
        EvKey::KEY_KPEQUAL => Key::KPEqual,
        EvKey::KEY_KPPLUSMINUS => Key::KPPlusMinus,
        EvKey::KEY_PAUSE => Key::Pause,
        EvKey::KEY_SCALE => Key::Scale,
        EvKey::KEY_KPCOMMA => Key::KPComma,
        EvKey::KEY_HANGEUL => Key::Hangeul,
        EvKey::KEY_HANJA => Key::Hanja,
        EvKey::KEY_YEN => Key::Yen,
        EvKey::KEY_COMPOSE => Key::Compose,
        EvKey::KEY_AGAIN => Key::Again,
        EvKey::KEY_PROPS => Key::Props,
        EvKey::KEY_UNDO => Key::Undo,
        EvKey::KEY_FRONT => Key::Front,
        EvKey::KEY_COPY => Key::Copy,
        EvKey::KEY_OPEN => Key::Open,
        EvKey::KEY_PASTE => Key::Paste,
        EvKey::KEY_FIND => Key::Find,
        EvKey::KEY_CUT => Key::Cut,
        EvKey::KEY_HELP => Key::Help,
        EvKey::KEY_MENU => Key::Menu,
        EvKey::KEY_CALC => Key::Calc,
        EvKey::KEY_SETUP => Key::Setup,
        EvKey::KEY_SLEEP => Key::Sleep,
        EvKey::KEY_WAKEUP => Key::Wakeup,
        EvKey::KEY_FILE => Key::File,
        EvKey::KEY_SENDFILE => Key::SendFile,
        EvKey::KEY_DELETEFILE => Key::DeleteFile,
        EvKey::KEY_XFER => Key::Xfer,
        EvKey::KEY_PROG1 => Key::Prog1,
        EvKey::KEY_PROG2 => Key::Prog2,
        EvKey::KEY_WWW => Key::WWW,
        EvKey::KEY_MSDOS => Key::MSDOS,
        EvKey::KEY_COFFEE => Key::ScreenLock,
        EvKey::KEY_ROTATE_DISPLAY => Key::RotateDisplay,
        EvKey::KEY_CYCLEWINDOWS => Key::CycleWindows,
        EvKey::KEY_MAIL => Key::Mail,
        EvKey::KEY_BOOKMARKS => Key::Bookmarks,
        EvKey::KEY_COMPUTER => Key::Computer,
        EvKey::KEY_BACK => Key::Back,
        EvKey::KEY_FORWARD => Key::Forward,
        EvKey::KEY_CLOSECD => Key::CloseCD,
        EvKey::KEY_EJECTCD => Key::EjectCD,
        EvKey::KEY_EJECTCLOSECD => Key::EjectCloseCD,
        EvKey::KEY_NEXTSONG => Key::NextSong,
        EvKey::KEY_PLAYPAUSE => Key::PlayPause,
        EvKey::KEY_PREVIOUSSONG => Key::PreviousSong,
        EvKey::KEY_STOPCD => Key::StopCD,
        EvKey::KEY_RECORD => Key::Record,
        EvKey::KEY_REWIND => Key::Rewind,
        EvKey::KEY_PHONE => Key::Phone,
        EvKey::KEY_ISO => Key::Iso,
        EvKey::KEY_CONFIG => Key::Config,
        EvKey::KEY_HOMEPAGE => Key::Homepage,
        EvKey::KEY_REFRESH => Key::Refresh,
        EvKey::KEY_EXIT => Key::Exit,
        EvKey::KEY_MOVE => Key::Move,
        EvKey::KEY_EDIT => Key::Edit,
        EvKey::KEY_SCROLLUP => Key::ScrollUp,
        EvKey::KEY_SCROLLDOWN => Key::ScrollDown,
        EvKey::KEY_KPLEFTPAREN => Key::KPLeftParen,
        EvKey::KEY_KPRIGHTPAREN => Key::KPRightParen,
        EvKey::KEY_NEW => Key::New,
        EvKey::KEY_REDO => Key::Redo,
        EvKey::KEY_PLAYCD => Key::PlayCD,
        EvKey::KEY_PAUSECD => Key::PauseCD,
        EvKey::KEY_PROG3 => Key::Prog3,
        EvKey::KEY_PROG4 => Key::Prog4,
        EvKey::KEY_DASHBOARD => Key::AllApplications,
        EvKey::KEY_SUSPEND => Key::Suspend,
        EvKey::KEY_CLOSE => Key::Close,
        EvKey::KEY_PLAY => Key::Play,
        EvKey::KEY_FASTFORWARD => Key::FastForward,
        EvKey::KEY_BASSBOOST => Key::BassBoost,
        EvKey::KEY_PRINT => Key::Print,
        EvKey::KEY_HP => Key::Hp,
        EvKey::KEY_CAMERA => Key::Camera,
        EvKey::KEY_SOUND => Key::Sound,
        EvKey::KEY_QUESTION => Key::Question,
        EvKey::KEY_EMAIL => Key::Email,
        EvKey::KEY_CHAT => Key::Chat,
        EvKey::KEY_SEARCH => Key::Search,
        EvKey::KEY_CONNECT => Key::Connect,
        EvKey::KEY_FINANCE => Key::Finance,
        EvKey::KEY_SPORT => Key::Sport,
        EvKey::KEY_SHOP => Key::Shop,
        EvKey::KEY_ALTERASE => Key::AltErase,
        EvKey::KEY_CANCEL => Key::Cancel,
        EvKey::KEY_BRIGHTNESSDOWN => Key::BrightnessDown,
        EvKey::KEY_BRIGHTNESSUP => Key::BrightnessUp,
        EvKey::KEY_MEDIA => Key::Media,
        EvKey::KEY_SWITCHVIDEOMODE => Key::SwitchVideoMode,
        EvKey::KEY_SEND => Key::Send,
        EvKey::KEY_REPLY => Key::Reply,
        EvKey::KEY_FORWARDMAIL => Key::ForwardMail,
        EvKey::KEY_SAVE => Key::Save,
        EvKey::KEY_DOCUMENTS => Key::Documents,
        EvKey::KEY_BATTERY => Key::Battery,
        EvKey::KEY_BLUETOOTH => Key::Bluetooth,
        EvKey::KEY_WLAN => Key::WLAN,
        EvKey::KEY_UWB => Key::UWB,
        EvKey::KEY_VIDEO_NEXT => Key::VideoNext,
        EvKey::KEY_VIDEO_PREV => Key::VideoPrev,
        EvKey::KEY_BRIGHTNESS_CYCLE => Key::BrightnessCycle,
        EvKey::KEY_BRIGHTNESS_AUTO => Key::BrightnessAuto,
        EvKey::KEY_DISPLAY_OFF => Key::DisplayOff,
        EvKey::KEY_WWAN => Key::WWAN,
        EvKey::KEY_RFKILL => Key::RFKill,
        EvKey::KEY_MICMUTE => Key::MicMute,
        _ => {
            return None;
        }
    })
}
