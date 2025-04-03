use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};
use std::thread;
use std::time::Duration;

fn main() {
    let app = Application::new(Some("com.example.call_of_duty_warzone_multi"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Call of Duty Warzone Multi");
        window.set_default_size(300, 200);

        let label = Label::new(Some("Welcome to Call of Duty Warzone Multi"));
        let button = Button::with_label("Activate Features");
        button.connect_clicked(|_| {
            thread::spawn(|| {
                activate_features();
            });
        });

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&button, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
}

fn activate_features() {
    log::info!("Activating features...");
    thread::sleep(Duration::from_secs(2));
    log::info!("Features activated successfully.");
}