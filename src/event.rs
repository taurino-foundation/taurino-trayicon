use serde::Serialize;
use taurino_core::{
    dpi::{PhysicalPosition, Rect},
    serde,
};

use crate::TrayIconId;
/// Describes the mouse button state.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(crate = "taurino_core::serde")]
pub enum MouseButtonState {
    /// Mouse button released.
    #[default]
    Up,

    /// Mouse button pressed.
    Down,
}

impl From<taurino_core::tray_icon::MouseButtonState> for MouseButtonState {
    fn from(value: taurino_core::tray_icon::MouseButtonState) -> Self {
        match value {
            taurino_core::tray_icon::MouseButtonState::Up => Self::Up,
            taurino_core::tray_icon::MouseButtonState::Down => Self::Down,
        }
    }
}

/// Describes which mouse button triggered the event.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Default)]
#[serde(crate = "taurino_core::serde")]
pub enum MouseButton {
    #[default]
    Left,
    Right,
    Middle,
}

impl From<taurino_core::tray_icon::MouseButton> for MouseButton {
    fn from(value: taurino_core::tray_icon::MouseButton) -> Self {
        match value {
            taurino_core::tray_icon::MouseButton::Left => Self::Left,
            taurino_core::tray_icon::MouseButton::Right => Self::Right,
            taurino_core::tray_icon::MouseButton::Middle => Self::Middle,
        }
    }
}

/// Describes a tray icon event.
#[derive(Debug, Clone, Serialize)]
#[serde(crate = "taurino_core::serde")]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TrayIconEvent {
    #[serde(rename_all = "camelCase")]
    Click {
        id: TrayIconId,
        position: PhysicalPosition<f64>,
        rect: Rect,
        button: MouseButton,
        button_state: MouseButtonState,
    },

    DoubleClick {
        id: TrayIconId,
        position: PhysicalPosition<f64>,
        rect: Rect,
        button: MouseButton,
    },

    Enter {
        id: TrayIconId,
        position: PhysicalPosition<f64>,
        rect: Rect,
    },

    Move {
        id: TrayIconId,
        position: PhysicalPosition<f64>,
        rect: Rect,
    },

    Leave {
        id: TrayIconId,
        position: PhysicalPosition<f64>,
        rect: Rect,
    },
}

impl TrayIconEvent {
    pub fn id(&self) -> &TrayIconId {
        match self {
            Self::Click { id, .. }
            | Self::DoubleClick { id, .. }
            | Self::Enter { id, .. }
            | Self::Move { id, .. }
            | Self::Leave { id, .. } => id,
        }
    }
}

impl From<taurino_core::tray_icon::TrayIconEvent> for TrayIconEvent {
    fn from(value: taurino_core::tray_icon::TrayIconEvent) -> Self {
        match value {
            taurino_core::tray_icon::TrayIconEvent::Click {
                id,
                position,
                rect,
                button,
                button_state,
            } => Self::Click {
                id,
                position,
                rect: Rect {
                    position: rect.position.into(),
                    size: rect.size.into(),
                },
                button: button.into(),
                button_state: button_state.into(),
            },

            taurino_core::tray_icon::TrayIconEvent::DoubleClick {
                id,
                position,
                rect,
                button,
            } => Self::DoubleClick {
                id,
                position,
                rect: Rect {
                    position: rect.position.into(),
                    size: rect.size.into(),
                },
                button: button.into(),
            },

            taurino_core::tray_icon::TrayIconEvent::Enter { id, position, rect } => Self::Enter {
                id,
                position,
                rect: Rect {
                    position: rect.position.into(),
                    size: rect.size.into(),
                },
            },

            taurino_core::tray_icon::TrayIconEvent::Move { id, position, rect } => Self::Move {
                id,
                position,
                rect: Rect {
                    position: rect.position.into(),
                    size: rect.size.into(),
                },
            },

            taurino_core::tray_icon::TrayIconEvent::Leave { id, position, rect } => Self::Leave {
                id,
                position,
                rect: Rect {
                    position: rect.position.into(),
                    size: rect.size.into(),
                },
            },

            // taurino_core::tray_icon::TrayIconEvent is #[non_exhaustive].
            // With the currently supported tray-icon version these are all
            // known variants.
            _ => unreachable!("unsupported tray-icon event variant"),
        }
    }
}

// -----------------------------------------------------------------------------
// tray-icon -> application/Tao event bridge
//
// Must be installed exactly once.
// -----------------------------------------------------------------------------

pub fn install_tray_event_handler<F>(send: F)
where
    F: Fn(TrayIconEvent) + Send + Sync + 'static,
{
    taurino_core::tray_icon::TrayIconEvent::set_event_handler(Some(
        move |event: taurino_core::tray_icon::TrayIconEvent| {
            send(event.into());
        },
    ));
}
