use std::path::PathBuf;
use std::thread;
use anyhow::Context;
use crossbeam_channel::{bounded, Receiver, TrySendError};
use evdev::{Device, InputEventKind, Key as EvKey};
use serde::Deserialize;
use crate::backend::{Backend, KeyEdge, KeyEvent};

#[derive(Deserialize)]
pub struct LinuxBackend;

const RELEASED: i32 = 0;
const PRESSED: i32 = 1;
const HELD: i32 = 2;

impl Backend for LinuxBackend {
    type Config = BackendConfig;

    fn poll_events(config: Self::Config) -> anyhow::Result<Receiver<KeyEvent>> {
        log::trace!("Starting keyboard polling thread");
        let Some(dev_path) = config.event_path else { log::error!("No device specified, cannot scan events."); return Ok(bounded(0).1) };
        log::debug!("Opening device {}", dev_path.to_string_lossy());
        let dev = Device::open(dev_path).context("Opening device");
        dev.is_err().then(|| log::error!("Failed to open device. Do you have the right permissions?"));
        let mut dev = dev?;
        let (sender, receiver) = crossbeam_channel::bounded(1024);
        thread::spawn(move || {
            'outer: loop {
                let res = dev.fetch_events().context("Reading events");
                log::trace!("Received events, processing...");
                let events = match res {
                    Err(err) => {
                        log::error!("Failed to read events. Error: {err}");
                        break;
                    }
                    Ok(events) => events,
                };
                'inner: for (keycode, raw_event) in events.filter_map(|event| match event.kind() {
                    InputEventKind::Key(key) => Some((key, event)),
                    _ => None
                }) {
                    let edge = match raw_event.value() {
                        RELEASED => KeyEdge::Released,
                        PRESSED => KeyEdge::Pressed,
                        HELD => KeyEdge::Held,
                        o => {
                            log::warn!("Unexpected event value `{o}` for key {}", keycode.code());
                            continue 'inner;
                        }
                    };
                    let Some(key) = map_key(keycode) else {
                        log::debug!("Unsupported keycode {}", keycode.code());
                        continue 'inner;
                    };
                    match sender.try_send(KeyEvent { key, edge }) {
                        Err(TrySendError::Full(_)) => { log::warn!("Overflowed capacity, events may be dropped."); }
                        Err(TrySendError::Disconnected(_)) => { break 'outer; }
                        _ => {}
                    }
                }
            }
        });
        Ok(receiver)
    }
}

#[derive(Deserialize, Default)]
pub struct BackendConfig {
    pub event_path: Option<PathBuf>,
}

fn map_key(input: EvKey) -> Option<super::Key> {
    Some(match input {
        EvKey::KEY_ESC => super::Key::Esc,
        EvKey::KEY_F1 => super::Key::F1,
        EvKey::KEY_F2 => super::Key::F2,
        EvKey::KEY_F3 => super::Key::F3,
        EvKey::KEY_F4 => super::Key::F4,
        EvKey::KEY_F5 => super::Key::F5,
        EvKey::KEY_F6 => super::Key::F6,
        EvKey::KEY_F7 => super::Key::F7,
        EvKey::KEY_F8 => super::Key::F8,
        EvKey::KEY_F9 => super::Key::F9,
        EvKey::KEY_F10 => super::Key::F10,
        EvKey::KEY_F11 => super::Key::F11,
        EvKey::KEY_F12 => super::Key::F12,
        EvKey::KEY_F13 => super::Key::F13,
        EvKey::KEY_F14 => super::Key::F14,
        EvKey::KEY_F15 => super::Key::F15,
        EvKey::KEY_F16 => super::Key::F16,
        EvKey::KEY_F17 => super::Key::F17,
        EvKey::KEY_F18 => super::Key::F18,
        EvKey::KEY_F19 => super::Key::F19,
        EvKey::KEY_F20 => super::Key::F20,
        EvKey::KEY_F21 => super::Key::F21,
        EvKey::KEY_F22 => super::Key::F22,
        EvKey::KEY_F23 => super::Key::F23,
        EvKey::KEY_F24 => super::Key::F24,
        EvKey::KEY_LEFTCTRL => super::Key::LeftCtrl,
        EvKey::KEY_RIGHTCTRL => super::Key::RightCtrl,
        EvKey::KEY_LEFTALT => super::Key::LeftAlt,
        EvKey::KEY_RIGHTALT => super::Key::RightAlt,
        EvKey::KEY_LEFTSHIFT => super::Key::LeftShift,
        EvKey::KEY_RIGHTSHIFT => super::Key::RightShift,
        EvKey::KEY_LEFTMETA => super::Key::LeftMeta,
        EvKey::KEY_RIGHTMETA => super::Key::RightMeta,
        EvKey::KEY_1 => super::Key::N1,
        EvKey::KEY_2 => super::Key::N2,
        EvKey::KEY_3 => super::Key::N3,
        EvKey::KEY_4 => super::Key::N4,
        EvKey::KEY_5 => super::Key::N5,
        EvKey::KEY_6 => super::Key::N6,
        EvKey::KEY_7 => super::Key::N7,
        EvKey::KEY_8 => super::Key::N8,
        EvKey::KEY_9 => super::Key::N9,
        EvKey::KEY_0 => super::Key::N0,
        EvKey::KEY_Q => super::Key::Q,
        EvKey::KEY_W => super::Key::W,
        EvKey::KEY_E => super::Key::E,
        EvKey::KEY_R => super::Key::R,
        EvKey::KEY_T => super::Key::T,
        EvKey::KEY_Y => super::Key::Y,
        EvKey::KEY_U => super::Key::U,
        EvKey::KEY_I => super::Key::I,
        EvKey::KEY_O => super::Key::O,
        EvKey::KEY_P => super::Key::P,
        EvKey::KEY_A => super::Key::A,
        EvKey::KEY_S => super::Key::S,
        EvKey::KEY_D => super::Key::D,
        EvKey::KEY_F => super::Key::F,
        EvKey::KEY_G => super::Key::G,
        EvKey::KEY_H => super::Key::H,
        EvKey::KEY_J => super::Key::J,
        EvKey::KEY_K => super::Key::K,
        EvKey::KEY_L => super::Key::L,
        EvKey::KEY_Z => super::Key::Z,
        EvKey::KEY_X => super::Key::X,
        EvKey::KEY_C => super::Key::C,
        EvKey::KEY_V => super::Key::V,
        EvKey::KEY_B => super::Key::B,
        EvKey::KEY_N => super::Key::N,
        EvKey::KEY_M => super::Key::M,
        EvKey::KEY_MINUS => super::Key::Minus,
        EvKey::KEY_EQUAL => super::Key::Equal,
        EvKey::KEY_BACKSPACE => super::Key::Backspace,
        EvKey::KEY_TAB => super::Key::Tab,
        EvKey::KEY_LEFTBRACE => super::Key::LeftBracket,
        EvKey::KEY_RIGHTBRACE => super::Key::RightBracket,
        EvKey::KEY_ENTER => super::Key::Enter,
        EvKey::KEY_SEMICOLON => super::Key::Semicolon,
        EvKey::KEY_APOSTROPHE => super::Key::Apostrophe,
        EvKey::KEY_GRAVE => super::Key::Grave,
        EvKey::KEY_BACKSLASH => super::Key::Backslash,
        EvKey::KEY_COMMA => super::Key::Comma,
        EvKey::KEY_DOT => super::Key::Dot,
        EvKey::KEY_SLASH => super::Key::Slash,
        EvKey::KEY_SPACE => super::Key::Space,
        EvKey::KEY_CAPSLOCK => super::Key::CapsLock,
        EvKey::KEY_NUMLOCK => super::Key::NumLock,
        EvKey::KEY_SCROLLLOCK => super::Key::ScrollLock,
        EvKey::KEY_KP0 => super::Key::KP0,
        EvKey::KEY_KP1 => super::Key::KP1,
        EvKey::KEY_KP2 => super::Key::KP2,
        EvKey::KEY_KP3 => super::Key::KP3,
        EvKey::KEY_KP4 => super::Key::KP4,
        EvKey::KEY_KP5 => super::Key::KP5,
        EvKey::KEY_KP6 => super::Key::KP6,
        EvKey::KEY_KP7 => super::Key::KP7,
        EvKey::KEY_KP8 => super::Key::KP8,
        EvKey::KEY_KP9 => super::Key::KP9,
        EvKey::KEY_KPENTER => super::Key::KPEnter,
        EvKey::KEY_KPDOT => super::Key::KPDot,
        EvKey::KEY_KPMINUS => super::Key::KPMinus,
        EvKey::KEY_KPPLUS => super::Key::KPPlus,
        EvKey::KEY_KPASTERISK => super::Key::KPAsterisk,
        EvKey::KEY_KPSLASH => super::Key::KPSlash,
        EvKey::KEY_KPJPCOMMA => super::Key::KPJPComma,
        EvKey::KEY_ZENKAKUHANKAKU => super::Key::Zenkakuhankaku,
        EvKey::KEY_RO => super::Key::Ro,
        EvKey::KEY_KATAKANA => super::Key::Katakana,
        EvKey::KEY_HIRAGANA => super::Key::Hiragana,
        EvKey::KEY_HENKAN => super::Key::Henkan,
        EvKey::KEY_KATAKANAHIRAGANA => super::Key::Katakanahiragana,
        EvKey::KEY_MUHENKAN => super::Key::Muhenkan,
        EvKey::KEY_SYSRQ => super::Key::SysRq,
        EvKey::KEY_LINEFEED => super::Key::Linefeed,
        EvKey::KEY_HOME => super::Key::Home,
        EvKey::KEY_UP => super::Key::Up,
        EvKey::KEY_PAGEUP => super::Key::Pageup,
        EvKey::KEY_LEFT => super::Key::Left,
        EvKey::KEY_RIGHT => super::Key::Right,
        EvKey::KEY_END => super::Key::End,
        EvKey::KEY_DOWN => super::Key::Down,
        EvKey::KEY_PAGEDOWN => super::Key::Pagedown,
        EvKey::KEY_INSERT => super::Key::Insert,
        EvKey::KEY_DELETE => super::Key::Delete,
        EvKey::KEY_MACRO => super::Key::Macro,
        EvKey::KEY_MUTE => super::Key::Mute,
        EvKey::KEY_VOLUMEDOWN => super::Key::VolumeDown,
        EvKey::KEY_VOLUMEUP => super::Key::VolumeUp,
        EvKey::KEY_POWER => super::Key::Power,
        EvKey::KEY_KPEQUAL => super::Key::KPEqual,
        EvKey::KEY_KPPLUSMINUS => super::Key::KPPlusMinus,
        EvKey::KEY_PAUSE => super::Key::Pause,
        EvKey::KEY_SCALE => super::Key::Scale,
        EvKey::KEY_KPCOMMA => super::Key::KPComma,
        EvKey::KEY_HANGEUL => super::Key::Hangeul,
        EvKey::KEY_HANJA => super::Key::Hanja,
        EvKey::KEY_YEN => super::Key::Yen,
        EvKey::KEY_COMPOSE => super::Key::Compose,
        EvKey::KEY_AGAIN => super::Key::Again,
        EvKey::KEY_PROPS => super::Key::Props,
        EvKey::KEY_UNDO => super::Key::Undo,
        EvKey::KEY_FRONT => super::Key::Front,
        EvKey::KEY_COPY => super::Key::Copy,
        EvKey::KEY_OPEN => super::Key::Open,
        EvKey::KEY_PASTE => super::Key::Paste,
        EvKey::KEY_FIND => super::Key::Find,
        EvKey::KEY_CUT => super::Key::Cut,
        EvKey::KEY_HELP => super::Key::Help,
        EvKey::KEY_MENU => super::Key::Menu,
        EvKey::KEY_CALC => super::Key::Calc,
        EvKey::KEY_SETUP => super::Key::Setup,
        EvKey::KEY_SLEEP => super::Key::Sleep,
        EvKey::KEY_WAKEUP => super::Key::Wakeup,
        EvKey::KEY_FILE => super::Key::File,
        EvKey::KEY_SENDFILE => super::Key::SendFile,
        EvKey::KEY_DELETEFILE => super::Key::DeleteFile,
        EvKey::KEY_XFER => super::Key::Xfer,
        EvKey::KEY_PROG1 => super::Key::Prog1,
        EvKey::KEY_PROG2 => super::Key::Prog2,
        EvKey::KEY_WWW => super::Key::WWW,
        EvKey::KEY_MSDOS => super::Key::MSDOS,
        EvKey::KEY_COFFEE => super::Key::ScreenLock,
        EvKey::KEY_ROTATE_DISPLAY => super::Key::RotateDisplay,
        EvKey::KEY_CYCLEWINDOWS => super::Key::CycleWindows,
        EvKey::KEY_MAIL => super::Key::Mail,
        EvKey::KEY_BOOKMARKS => super::Key::Bookmarks,
        EvKey::KEY_COMPUTER => super::Key::Computer,
        EvKey::KEY_BACK => super::Key::Back,
        EvKey::KEY_FORWARD => super::Key::Forward,
        EvKey::KEY_CLOSECD => super::Key::CloseCD,
        EvKey::KEY_EJECTCD => super::Key::EjectCD,
        EvKey::KEY_EJECTCLOSECD => super::Key::EjectCloseCD,
        EvKey::KEY_NEXTSONG => super::Key::NextSong,
        EvKey::KEY_PLAYPAUSE => super::Key::PlayPause,
        EvKey::KEY_PREVIOUSSONG => super::Key::PreviousSong,
        EvKey::KEY_STOPCD => super::Key::StopCD,
        EvKey::KEY_RECORD => super::Key::Record,
        EvKey::KEY_REWIND => super::Key::Rewind,
        EvKey::KEY_PHONE => super::Key::Phone,
        EvKey::KEY_ISO => super::Key::Iso,
        EvKey::KEY_CONFIG => super::Key::Config,
        EvKey::KEY_HOMEPAGE => super::Key::Homepage,
        EvKey::KEY_REFRESH => super::Key::Refresh,
        EvKey::KEY_EXIT => super::Key::Exit,
        EvKey::KEY_MOVE => super::Key::Move,
        EvKey::KEY_EDIT => super::Key::Edit,
        EvKey::KEY_SCROLLUP => super::Key::ScrollUp,
        EvKey::KEY_SCROLLDOWN => super::Key::ScrollDown,
        EvKey::KEY_KPLEFTPAREN => super::Key::KPLeftParen,
        EvKey::KEY_KPRIGHTPAREN => super::Key::KPRightParen,
        EvKey::KEY_NEW => super::Key::New,
        EvKey::KEY_REDO => super::Key::Redo,
        EvKey::KEY_PLAYCD => super::Key::PlayCD,
        EvKey::KEY_PAUSECD => super::Key::PauseCD,
        EvKey::KEY_PROG3 => super::Key::Prog3,
        EvKey::KEY_PROG4 => super::Key::Prog4,
        EvKey::KEY_DASHBOARD => super::Key::AllApplications,
        EvKey::KEY_SUSPEND => super::Key::Suspend,
        EvKey::KEY_CLOSE => super::Key::Close,
        EvKey::KEY_PLAY => super::Key::Play,
        EvKey::KEY_FASTFORWARD => super::Key::FastForward,
        EvKey::KEY_BASSBOOST => super::Key::BassBoost,
        EvKey::KEY_PRINT => super::Key::Print,
        EvKey::KEY_HP => super::Key::Hp,
        EvKey::KEY_CAMERA => super::Key::Camera,
        EvKey::KEY_SOUND => super::Key::Sound,
        EvKey::KEY_QUESTION => super::Key::Question,
        EvKey::KEY_EMAIL => super::Key::Email,
        EvKey::KEY_CHAT => super::Key::Chat,
        EvKey::KEY_SEARCH => super::Key::Search,
        EvKey::KEY_CONNECT => super::Key::Connect,
        EvKey::KEY_FINANCE => super::Key::Finance,
        EvKey::KEY_SPORT => super::Key::Sport,
        EvKey::KEY_SHOP => super::Key::Shop,
        EvKey::KEY_ALTERASE => super::Key::AltErase,
        EvKey::KEY_CANCEL => super::Key::Cancel,
        EvKey::KEY_BRIGHTNESSDOWN => super::Key::BrightnessDown,
        EvKey::KEY_BRIGHTNESSUP => super::Key::BrightnessUp,
        EvKey::KEY_MEDIA => super::Key::Media,
        EvKey::KEY_SWITCHVIDEOMODE => super::Key::SwitchVideoMode,
        EvKey::KEY_SEND => super::Key::Send,
        EvKey::KEY_REPLY => super::Key::Reply,
        EvKey::KEY_FORWARDMAIL => super::Key::ForwardMail,
        EvKey::KEY_SAVE => super::Key::Save,
        EvKey::KEY_DOCUMENTS => super::Key::Documents,
        EvKey::KEY_BATTERY => super::Key::Battery,
        EvKey::KEY_BLUETOOTH => super::Key::Bluetooth,
        EvKey::KEY_WLAN => super::Key::WLAN,
        EvKey::KEY_UWB => super::Key::UWB,
        EvKey::KEY_VIDEO_NEXT => super::Key::VideoNext,
        EvKey::KEY_VIDEO_PREV => super::Key::VideoPrev,
        EvKey::KEY_BRIGHTNESS_CYCLE => super::Key::BrightnessCycle,
        EvKey::KEY_BRIGHTNESS_AUTO => super::Key::BrightnessAuto,
        EvKey::KEY_DISPLAY_OFF => super::Key::DisplayOff,
        EvKey::KEY_WWAN => super::Key::WWAN,
        EvKey::KEY_RFKILL => super::Key::RFKill,
        EvKey::KEY_MICMUTE => super::Key::MicMute,
        _ => { return None; }
    })
}
