#[macro_export]
macro_rules! build {
    ($window:expr) => {{ ///////////////////////////
        
        let a = push!(0);
        let b = push!("World");
        
        let layout = QVBoxLayout::new_0a();

        let button = QPushButton::new();
        button.set_text(&Qstr!("Click Here!"));

        QBindButton!(&$window, &button, {
            let mut a = pull!(a);
            let b = pull!(b);
            *a += 1;
            println!("Hello, {}! ({})", *b, *a);
        });

        layout.add_widget(button.as_mut_raw_ptr());
        $window.set_layout(layout.as_mut_raw_ptr());
        
    }}; ////////////////////////////////////////////
}