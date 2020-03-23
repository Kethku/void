use std::error;
use std::fmt;
use std::convert::TryInto;

use skulpin::skia_safe::Color4f;

use crate::editor::{Colors, Style, CursorMode, CursorShape};

#[derive(Debug)]
pub struct GridLineCell {
    pub text: String,
    pub highlight_id: Option<u64>,
    pub repeat: Option<u64>
}

pub type StyledContent = Vec<(u64, String)>;

#[derive(Debug)]
pub enum MessageKind {
    Unknown,
    Confirm,
    ConfirmSubstitute,
    Error,
    Echo,
    EchoMessage,
    EchoError,
    LuaError,
    RpcError,
    ReturnPrompt,
    QuickFix,
    SearchCount,
    Warning
}

impl MessageKind {
    pub fn parse(kind: &str) -> MessageKind {
        match kind {
            "confirm" => MessageKind::Confirm,
            "confirm_sub" => MessageKind::ConfirmSubstitute,
            "emsg" => MessageKind::Error,
            "echo" => MessageKind::Echo,
            "echomsg" => MessageKind::EchoMessage,
            "echoerr" => MessageKind::EchoError,
            "lua_error" => MessageKind::LuaError,
            "rpc_error" => MessageKind::RpcError,
            "return_prompt" => MessageKind::ReturnPrompt,
            "quickfix" => MessageKind::QuickFix,
            "search_count" => MessageKind::SearchCount,
            "wmsg" => MessageKind::Warning,
            _ => MessageKind::Unknown
        }
    }
}

#[derive(Debug)]
pub enum GuiOption {
    ArabicShape(bool),
    AmbiWidth(String),
    Emoji(bool),
    GuiFont(String),
    GuiFontSet(String),
    GuiFontWide(String),
    LineSpace(u64),
    Pumblend(u64),
    ShowTabLine(u64),
    TermGuiColors(bool),
    Unknown(String, Value)
}

#[derive(Debug)]
pub enum WindowAnchor {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast
}

#[derive(Debug)]
pub enum RedrawEvent {
    SetTitle { title: String },
    ModeInfoSet { cursor_modes: Vec<CursorMode> },
    OptionSet { gui_option: GuiOption },
    ModeChange { mode_index: u64 },
    BusyStart,
    BusyStop,
    Flush,
    Resize { grid: u64, width: u64, height: u64 },
    DefaultColorsSet { colors: Colors },
    HighlightAttributesDefine { id: u64, style: Style },
    GridLine { grid: u64, row: u64, column_start: u64, cells: Vec<GridLineCell> },
    Clear { grid: u64 },
    CursorGoto { grid: u64, row: u64, column: u64 },
    Scroll { grid: u64, top: u64, bottom: u64, left: u64, right: u64, rows: i64, columns: i64 },
}
