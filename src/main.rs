use std::rc::Rc;
use std::cell::RefCell;
use qt_widgets::qt_core::*;
use qt_widgets::*;

mod qt;
mod app;

fn main() {
    unsafe {
        QApplication::init(|_app| {
            let window = QWidget::new_0a();
            window.set_window_title(&Qstr!("Title"));
            window.resize_2a(300, 200);

            build!(window);

            window.show();
            QApplication::exec()
        });
    }
}
