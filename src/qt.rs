use qt_widgets::{qt_core::*, QPushButton};
//use qt_widgets::qt_core::*;
use qt_widgets::*;
use cpp_core::CppBox;

pub fn str(input: &str) -> CppBox<QString> {
    QString::from_std_str(input) // Yep, just because it's ugly.
}

pub fn bind_fn<F>(window: &QBox<QWidget>, target: &QBox<QPushButton>, func: F) where F: Fn() + 'static,
{
    unsafe {
        let slot = SlotNoArgs::new(window, func);
        target.clicked().connect(&slot);
    }
}