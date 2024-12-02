#[macro_export]
macro_rules! Qstr {
    ($input:expr) => {{
        QString::from_std_str($input)
    }};
}

#[macro_export]
macro_rules! QBindButton {
    ($window:expr, $button:expr, $($input:tt)*) => {{
        let slot = SlotNoArgs::new($window, move || {
            $($input)*
        });
        $button.clicked().connect(&slot);
    }};
}

#[macro_export]
macro_rules! push {
    ($input:expr) => {{
        let a = Rc::new(RefCell::new($input));
        a
    }};
}

#[macro_export]
macro_rules! pull {
    ($a:expr) => {{
        let a = $a.borrow_mut();
        a
    }};
}