use derive_more::Display;
use std::{collections::HashMap, sync::OnceLock};

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
pub enum Key {
    Esc,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
    LeftShift,
    RightShift,
    LeftMeta,
    RightMeta,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Minus,
    Equal,
    Backspace,
    Tab,
    LeftBracket,
    RightBracket,
    Enter,
    Semicolon,
    Apostrophe,
    Grave,
    Backslash,
    Comma,
    Dot,
    Slash,
    Space,
    CapsLock,
    NumLock,
    ScrollLock,
    KP0,
    KP1,
    KP2,
    KP3,
    KP4,
    KP5,
    KP6,
    KP7,
    KP8,
    KP9,
    KPEnter,
    KPDot,
    KPMinus,
    KPPlus,
    KPAsterisk,
    KPSlash,
    KPJPComma,
    Zenkakuhankaku,
    Ro,
    Katakana,
    Hiragana,
    Henkan,
    Katakanahiragana,
    Muhenkan,
    SysRq,
    Linefeed,
    Home,
    Up,
    Pageup,
    Left,
    Right,
    End,
    Down,
    Pagedown,
    Insert,
    Delete,
    Macro,
    Mute,
    VolumeDown,
    VolumeUp,
    Power,
    KPEqual,
    KPPlusMinus,
    Pause,
    Scale,
    KPComma,
    Hangeul,
    Hanja,
    Yen,
    Compose,
    Again,
    Props,
    Undo,
    Front,
    Copy,
    Open,
    Paste,
    Find,
    Cut,
    Help,
    Menu,
    Calc,
    Setup,
    Sleep,
    Wakeup,
    File,
    SendFile,
    DeleteFile,
    Xfer,
    Prog1,
    Prog2,
    WWW,
    MSDOS,
    ScreenLock,
    RotateDisplay,
    CycleWindows,
    Mail,
    Bookmarks,
    Computer,
    Back,
    Forward,
    CloseCD,
    EjectCD,
    EjectCloseCD,
    NextSong,
    PlayPause,
    PreviousSong,
    StopCD,
    Record,
    Rewind,
    Phone,
    Iso,
    Config,
    Homepage,
    Refresh,
    Exit,
    Move,
    Edit,
    ScrollUp,
    ScrollDown,
    KPLeftParen,
    KPRightParen,
    New,
    Redo,
    PlayCD,
    PauseCD,
    Prog3,
    Prog4,
    AllApplications,
    Suspend,
    Close,
    Play,
    FastForward,
    BassBoost,
    Print,
    Hp,
    Camera,
    Sound,
    Question,
    Email,
    Chat,
    Search,
    Connect,
    Finance,
    Sport,
    Shop,
    AltErase,
    Cancel,
    BrightnessDown,
    BrightnessUp,
    Media,
    SwitchVideoMode,
    Send,
    Reply,
    ForwardMail,
    Save,
    Documents,
    Battery,
    Bluetooth,
    WLAN,
    UWB,
    VideoNext,
    VideoPrev,
    BrightnessCycle,
    BrightnessAuto,
    DisplayOff,
    WWAN,
    RFKill,
    MicMute,
}

static KEY_NAMES: OnceLock<HashMap<&'static str, Key>> = OnceLock::new();

static KEY_MULTI_OVERRIDE: OnceLock<HashMap<&'static str, Vec<Key>>> = OnceLock::new();

pub fn key_override(name: &str) -> Option<Vec<Key>> {
    KEY_MULTI_OVERRIDE
        .get_or_init(|| {
            let mut map = HashMap::new();
            map.insert("ctrl", vec![Key::LeftCtrl, Key::RightCtrl]);
            map.insert("alt", vec![Key::LeftAlt, Key::RightAlt]);
            map.insert("shift", vec![Key::LeftShift, Key::RightShift]);
            map.insert("meta", vec![Key::LeftMeta, Key::RightMeta]);
            map
        })
        .get(name)
        .cloned()
}

pub fn get_key_for_name(name: &str) -> Option<Key> {
    KEY_NAMES
        .get_or_init(|| {
            let mut map = HashMap::new();
            map.insert("esc", Key::Esc);
            map.insert("f1", Key::F1);
            map.insert("f2", Key::F2);
            map.insert("f3", Key::F3);
            map.insert("f4", Key::F4);
            map.insert("f5", Key::F5);
            map.insert("f6", Key::F6);
            map.insert("f7", Key::F7);
            map.insert("f8", Key::F8);
            map.insert("f9", Key::F9);
            map.insert("f10", Key::F10);
            map.insert("f11", Key::F11);
            map.insert("f12", Key::F12);
            map.insert("f13", Key::F13);
            map.insert("f14", Key::F14);
            map.insert("f15", Key::F15);
            map.insert("f16", Key::F16);
            map.insert("f17", Key::F17);
            map.insert("f18", Key::F18);
            map.insert("f19", Key::F19);
            map.insert("f20", Key::F20);
            map.insert("f21", Key::F21);
            map.insert("f22", Key::F22);
            map.insert("f23", Key::F23);
            map.insert("f24", Key::F24);
            map.insert("leftctrl", Key::LeftCtrl);
            map.insert("rightctrl", Key::RightCtrl);
            map.insert("leftalt", Key::LeftAlt);
            map.insert("rightalt", Key::RightAlt);
            map.insert("leftshift", Key::LeftShift);
            map.insert("rightshift", Key::RightShift);
            map.insert("leftmeta", Key::LeftMeta);
            map.insert("rightmeta", Key::RightMeta);
            map.insert("n1", Key::N1);
            map.insert("n2", Key::N2);
            map.insert("n3", Key::N3);
            map.insert("n4", Key::N4);
            map.insert("n5", Key::N5);
            map.insert("n6", Key::N6);
            map.insert("n7", Key::N7);
            map.insert("n8", Key::N8);
            map.insert("n9", Key::N9);
            map.insert("n0", Key::N0);
            map.insert("1", Key::N1);
            map.insert("2", Key::N2);
            map.insert("3", Key::N3);
            map.insert("4", Key::N4);
            map.insert("5", Key::N5);
            map.insert("6", Key::N6);
            map.insert("7", Key::N7);
            map.insert("8", Key::N8);
            map.insert("9", Key::N9);
            map.insert("0", Key::N0);
            map.insert("q", Key::Q);
            map.insert("w", Key::W);
            map.insert("e", Key::E);
            map.insert("r", Key::R);
            map.insert("t", Key::T);
            map.insert("y", Key::Y);
            map.insert("u", Key::U);
            map.insert("i", Key::I);
            map.insert("o", Key::O);
            map.insert("p", Key::P);
            map.insert("a", Key::A);
            map.insert("s", Key::S);
            map.insert("d", Key::D);
            map.insert("f", Key::F);
            map.insert("g", Key::G);
            map.insert("h", Key::H);
            map.insert("j", Key::J);
            map.insert("k", Key::K);
            map.insert("l", Key::L);
            map.insert("z", Key::Z);
            map.insert("x", Key::X);
            map.insert("c", Key::C);
            map.insert("v", Key::V);
            map.insert("b", Key::B);
            map.insert("n", Key::N);
            map.insert("m", Key::M);
            map.insert("minus", Key::Minus);
            map.insert("dash", Key::Minus);
            map.insert("equal", Key::Equal);
            map.insert("plus", Key::Equal);
            map.insert("backspace", Key::Backspace);
            map.insert("tab", Key::Tab);
            map.insert("leftbracket", Key::LeftBracket);
            map.insert("leftbrace", Key::LeftBracket);
            map.insert("rightbracket", Key::RightBracket);
            map.insert("rightbrace", Key::RightBracket);
            map.insert("enter", Key::Enter);
            map.insert("semicolon", Key::Semicolon);
            map.insert("apostrophe", Key::Apostrophe);
            map.insert("grave", Key::Grave);
            map.insert("tilde", Key::Grave);
            map.insert("backslash", Key::Backslash);
            map.insert("comma", Key::Comma);
            map.insert("dot", Key::Dot);
            map.insert("slash", Key::Slash);
            map.insert("space", Key::Space);
            map.insert("capslock", Key::CapsLock);
            map.insert("numlock", Key::NumLock);
            map.insert("scrolllock", Key::ScrollLock);
            map.insert("kp0", Key::KP0);
            map.insert("kp1", Key::KP1);
            map.insert("kp2", Key::KP2);
            map.insert("kp3", Key::KP3);
            map.insert("kp4", Key::KP4);
            map.insert("kp5", Key::KP5);
            map.insert("kp6", Key::KP6);
            map.insert("kp7", Key::KP7);
            map.insert("kp8", Key::KP8);
            map.insert("kp9", Key::KP9);
            map.insert("np0", Key::KP0);
            map.insert("np1", Key::KP1);
            map.insert("np2", Key::KP2);
            map.insert("np3", Key::KP3);
            map.insert("np4", Key::KP4);
            map.insert("np5", Key::KP5);
            map.insert("np6", Key::KP6);
            map.insert("np7", Key::KP7);
            map.insert("np8", Key::KP8);
            map.insert("np9", Key::KP9);
            map.insert("numpad0", Key::KP0);
            map.insert("numpad1", Key::KP1);
            map.insert("numpad2", Key::KP2);
            map.insert("numpad3", Key::KP3);
            map.insert("numpad4", Key::KP4);
            map.insert("numpad5", Key::KP5);
            map.insert("numpad6", Key::KP6);
            map.insert("numpad7", Key::KP7);
            map.insert("numpad8", Key::KP8);
            map.insert("numpad9", Key::KP9);
            map.insert("kpenter", Key::KPEnter);
            map.insert("npenter", Key::KPEnter);
            map.insert("numpadenter", Key::KPEnter);
            map.insert("kpdot", Key::KPDot);
            map.insert("kpminus", Key::KPMinus);
            map.insert("kpplus", Key::KPPlus);
            map.insert("kpasterisk", Key::KPAsterisk);
            map.insert("kpslash", Key::KPSlash);
            map.insert("kpjpcomma", Key::KPJPComma);
            map.insert("npdot", Key::KPDot);
            map.insert("npminus", Key::KPMinus);
            map.insert("npplus", Key::KPPlus);
            map.insert("npasterisk", Key::KPAsterisk);
            map.insert("npslash", Key::KPSlash);
            map.insert("npjpcomma", Key::KPJPComma);
            map.insert("numpaddot", Key::KPDot);
            map.insert("numpadminus", Key::KPMinus);
            map.insert("numpadplus", Key::KPPlus);
            map.insert("numpadasterisk", Key::KPAsterisk);
            map.insert("numpadslash", Key::KPSlash);
            map.insert("numpadjpcomma", Key::KPJPComma);
            map.insert("zenkakuhankaku", Key::Zenkakuhankaku);
            map.insert("ro", Key::Ro);
            map.insert("katakana", Key::Katakana);
            map.insert("hiragana", Key::Hiragana);
            map.insert("henkan", Key::Henkan);
            map.insert("katakanahiragana", Key::Katakanahiragana);
            map.insert("muhenkan", Key::Muhenkan);
            map.insert("sysrq", Key::SysRq);
            map.insert("linefeed", Key::Linefeed);
            map.insert("home", Key::Home);
            map.insert("up", Key::Up);
            map.insert("uparrow", Key::Up);
            map.insert("pageup", Key::Pageup);
            map.insert("left", Key::Left);
            map.insert("leftarrow", Key::Left);
            map.insert("right", Key::Right);
            map.insert("rightarrow", Key::Right);
            map.insert("end", Key::End);
            map.insert("down", Key::Down);
            map.insert("downarrow", Key::Down);
            map.insert("pagedown", Key::Pagedown);
            map.insert("insert", Key::Insert);
            map.insert("delete", Key::Delete);
            map.insert("macro", Key::Macro);
            map.insert("mute", Key::Mute);
            map.insert("volumedown", Key::VolumeDown);
            map.insert("volumeup", Key::VolumeUp);
            map.insert("power", Key::Power);
            map.insert("kpequal", Key::KPEqual);
            map.insert("kpplusminus", Key::KPPlusMinus);
            map.insert("pause", Key::Pause);
            map.insert("scale", Key::Scale);
            map.insert("kpcomma", Key::KPComma);
            map.insert("hangeul", Key::Hangeul);
            map.insert("hanja", Key::Hanja);
            map.insert("yen", Key::Yen);
            map.insert("compose", Key::Compose);
            map.insert("again", Key::Again);
            map.insert("props", Key::Props);
            map.insert("undo", Key::Undo);
            map.insert("front", Key::Front);
            map.insert("copy", Key::Copy);
            map.insert("open", Key::Open);
            map.insert("paste", Key::Paste);
            map.insert("find", Key::Find);
            map.insert("cut", Key::Cut);
            map.insert("help", Key::Help);
            map.insert("menu", Key::Menu);
            map.insert("calc", Key::Calc);
            map.insert("calculator", Key::Calc);
            map.insert("setup", Key::Setup);
            map.insert("sleep", Key::Sleep);
            map.insert("wakeup", Key::Wakeup);
            map.insert("file", Key::File);
            map.insert("sendfile", Key::SendFile);
            map.insert("deletefile", Key::DeleteFile);
            map.insert("xfer", Key::Xfer);
            map.insert("prog1", Key::Prog1);
            map.insert("prog2", Key::Prog2);
            map.insert("www", Key::WWW);
            map.insert("msdos", Key::MSDOS);
            map.insert("screenlock", Key::ScreenLock);
            map.insert("rotatedisplay", Key::RotateDisplay);
            map.insert("cyclewindows", Key::CycleWindows);
            map.insert("mail", Key::Mail);
            map.insert("bookmarks", Key::Bookmarks);
            map.insert("computer", Key::Computer);
            map.insert("back", Key::Back);
            map.insert("forward", Key::Forward);
            map.insert("closecd", Key::CloseCD);
            map.insert("ejectcd", Key::EjectCD);
            map.insert("ejectclosecd", Key::EjectCloseCD);
            map.insert("nextsong", Key::NextSong);
            map.insert("playpause", Key::PlayPause);
            map.insert("previoussong", Key::PreviousSong);
            map.insert("stopcd", Key::StopCD);
            map.insert("record", Key::Record);
            map.insert("rewind", Key::Rewind);
            map.insert("phone", Key::Phone);
            map.insert("iso", Key::Iso);
            map.insert("config", Key::Config);
            map.insert("homepage", Key::Homepage);
            map.insert("refresh", Key::Refresh);
            map.insert("exit", Key::Exit);
            map.insert("move", Key::Move);
            map.insert("edit", Key::Edit);
            map.insert("scrollup", Key::ScrollUp);
            map.insert("scrolldown", Key::ScrollDown);
            map.insert("kpleftparen", Key::KPLeftParen);
            map.insert("kprightparen", Key::KPRightParen);
            map.insert("new", Key::New);
            map.insert("redo", Key::Redo);
            map.insert("playcd", Key::PlayCD);
            map.insert("pausecd", Key::PauseCD);
            map.insert("prog3", Key::Prog3);
            map.insert("prog4", Key::Prog4);
            map.insert("allapplications", Key::AllApplications);
            map.insert("suspend", Key::Suspend);
            map.insert("close", Key::Close);
            map.insert("play", Key::Play);
            map.insert("fastforward", Key::FastForward);
            map.insert("bassboost", Key::BassBoost);
            map.insert("print", Key::Print);
            map.insert("hp", Key::Hp);
            map.insert("camera", Key::Camera);
            map.insert("sound", Key::Sound);
            map.insert("question", Key::Question);
            map.insert("email", Key::Email);
            map.insert("chat", Key::Chat);
            map.insert("search", Key::Search);
            map.insert("connect", Key::Connect);
            map.insert("finance", Key::Finance);
            map.insert("sport", Key::Sport);
            map.insert("shop", Key::Shop);
            map.insert("alterase", Key::AltErase);
            map.insert("cancel", Key::Cancel);
            map.insert("brightnessdown", Key::BrightnessDown);
            map.insert("brightnessup", Key::BrightnessUp);
            map.insert("media", Key::Media);
            map.insert("switchvideomode", Key::SwitchVideoMode);
            map.insert("send", Key::Send);
            map.insert("reply", Key::Reply);
            map.insert("forwardmail", Key::ForwardMail);
            map.insert("save", Key::Save);
            map.insert("documents", Key::Documents);
            map.insert("battery", Key::Battery);
            map.insert("bluetooth", Key::Bluetooth);
            map.insert("wlan", Key::WLAN);
            map.insert("uwb", Key::UWB);
            map.insert("videonext", Key::VideoNext);
            map.insert("videoprev", Key::VideoPrev);
            map.insert("brightnesscycle", Key::BrightnessCycle);
            map.insert("brightnessauto", Key::BrightnessAuto);
            map.insert("displayoff", Key::DisplayOff);
            map.insert("wwan", Key::WWAN);
            map.insert("rfkill", Key::RFKill);
            map.insert("micmute", Key::MicMute);
            map
        })
        .get(name)
        .cloned()
}
