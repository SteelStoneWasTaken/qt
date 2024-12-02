use qt_widgets::*;
use std::rc::Rc;
use std::cell::RefCell;
mod qt;
mod app;

fn main() {
    unsafe {
        QApplication::init(|_app| {
            let window = QWidget::new_0a();
            window.set_window_title(&qt::str("Title"));
            window.resize_2a(300, 200);

            build!(window);

            window.show();
            QApplication::exec()
        });
    }
}
