mod event_handler;
mod file;
mod inline_code;
mod rpc;

use web_view::*;

fn main() {
    web_view::builder()
        .title("Rocksalt")
        .content(Content::Html(inline_code::html()))
        .size(2048, 1236)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(event_handler::handle)
        .run()
        .unwrap();
}
