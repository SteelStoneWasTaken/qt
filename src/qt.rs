

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



