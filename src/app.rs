#[macro_export]
macro_rules! build {
    ($window:expr) => {{ ///////////////////////////
        
        let a = Rc::clone(&Rc::new(RefCell::new(0)));
        
        let layout = QVBoxLayout::new_0a();

        let button = QPushButton::new();
        button.set_text(&Qstr!("Click Here!"));

        qt::bind_button(&$window, &button, move || {
            let mut a = a.borrow_mut();
            *a += 1;

            println!("{}", *a);
        });

        layout.add_widget(button.as_mut_raw_ptr());
        $window.set_layout(layout.as_mut_raw_ptr());
        
    }}; ////////////////////////////////////////////
}