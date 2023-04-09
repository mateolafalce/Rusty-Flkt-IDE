use fltk::{
    prelude::*,
    window::Window,
    enums::Color,
    app::{
        App,
        event_x_root,
        event_y_root
    },
    text::{
        TextBuffer,
        TextEditor
    },
    tree::Tree,
    image::PngImage,
};
#[path="./btn/btn_add_folder.rs"]
mod btn_add_folder;

use std::path::Path;

pub fn options_windows(
    app: App,
    folders: &mut Tree,
    text_buffer: TextBuffer,
    text_editor: TextEditor,
) -> Window {
    let icon: PngImage = PngImage::load(&Path::new("src/options.png")).unwrap();
    let mut options_windows: Window = Window::new(
        event_x_root(),
        event_y_root(),
        300,
        200,
        "Options"
    );
    options_windows.set_icon(Some(icon));
    options_windows.set_border(true);
    options_windows.set_color(Color::White);
    btn_add_folder::btn_add_folder(
        app.clone(),
        folders.clone(),
        text_buffer.clone(),
        text_editor.clone(),
        options_windows.clone(),
    );
    options_windows.end();
    options_windows.show();
    options_windows
}
