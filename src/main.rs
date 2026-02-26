slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();

    let main_window_weak = main_window.as_weak();
    main_window.on_text_changed(move |new_text| {
        main_window_weak.unwrap().set_text(new_text);
    });

    main_window.run().unwrap();
}
