use crossbeam_channel::Receiver;
use serde::Deserialize;

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        mod linux;
        pub type CurrentBackend = linux::LinuxBackend;
        pub type BackendConfig = linux::BackendConfig;
    }
}

pub trait Backend {
    type Config: Deserialize<'static> + Default + 'static;
    fn poll_events(config: Self::Config) -> anyhow::Result<Receiver<KeyEvent>>;
}

pub struct KeyEvent {
    edge: KeyEdge,
    key: Key,
}

impl KeyEvent {
    #[allow(unused)]
    pub fn is_pressed(&self) -> bool {
        matches!(self.edge, KeyEdge::Pressed)
    }

    #[allow(unused)]
    pub fn is_released(&self) -> bool {
        matches!(self.edge, KeyEdge::Released)
    }
    pub fn edge(&self) -> KeyEdge {
        self.edge
    }
    pub fn key(&self) -> Key {
        self.key
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum KeyEdge {
    Pressed,
    Held,
    Released,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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