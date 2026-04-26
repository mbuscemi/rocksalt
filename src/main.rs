mod event_handler;
mod inline_code;
mod rpc;

use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    http::{header::CONTENT_TYPE, Response},
    webview::WebViewBuilder,
};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::<Vec<String>>::with_user_event();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Rocksalt")
        .with_inner_size(wry::application::dpi::LogicalSize::new(2048.0_f64, 1236.0_f64))
        .build(&event_loop)
        .unwrap();

    let webview = WebViewBuilder::new(window)
        .unwrap()
        .with_custom_protocol("rocksalt".into(), |request| {
            let path = request.uri().path().to_string();
            match inline_code::serve(&path) {
                Some((bytes, mime)) => Response::builder()
                    .header(CONTENT_TYPE, mime)
                    .body(bytes)
                    .map_err(Into::into),
                None => Response::builder()
                    .status(404)
                    .body(std::borrow::Cow::Borrowed(&[] as &[u8]))
                    .map_err(Into::into),
            }
        })
        .with_url("rocksalt://localhost/")?
        .with_ipc_handler(move |_window, message| {
            let commands = event_handler::handle(&message);
            if !commands.is_empty() {
                let _ = proxy.send_event(commands);
            }
        })
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(commands) => {
                for cmd in commands {
                    let _ = webview.evaluate_script(&cmd);
                }
            }
            _ => {}
        }
    });
}
