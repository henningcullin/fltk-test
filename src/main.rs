use fltk::{app, button, prelude::{WidgetExt, GroupExt}, window};

fn game_move(label_data: String) {
    println!("This buttons content is: {}", label_data);
}

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300).with_label("My Window");
    let mut btn = button::Button::default().with_size(80, 30).center_of_parent().with_label("Click");
    win.end();
    win.show();
    btn.set_callback(move |b| {
        let content = b.label();
        game_move(content);
        b.set_label("Clicked");
        win.set_label("Button Clicked!");
    });
    a.run().unwrap();
}