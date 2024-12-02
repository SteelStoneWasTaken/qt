use qt_widgets::{qt_core::*, QPushButton};
use qt_widgets::*;

#[macro_export]
macro_rules! Qstr {
    ($input:expr) => {{
        QString::from_std_str($input)
    }};
}

pub fn bind_button<F>(window: &QBox<QWidget>, target: &QBox<QPushButton>, func: F) where F: Fn() + 'static,
{
    unsafe {
        let slot = SlotNoArgs::new(window, func);
        target.clicked().connect(&slot);
    }
}


