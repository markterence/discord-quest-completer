use std::env;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowAttributes;

#[derive(Debug)]
struct Config {
    title: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            title: "Discord Quest Completer".to_string(),
        }
    }
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                if i + 1 < args.len() {
                    config.title = args[i + 1].clone();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    config
}

fn main() {
    let config = parse_args();
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let _window = event_loop
        .create_window(
            WindowAttributes::default()
                .with_title(config.title)
                .with_visible(true)
                .with_resizable(true),
        )
        .expect("Failed to create window");

    event_loop
        .run(move |event, event_loop| {
            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } = event
            {
                event_loop.exit();
            }
        })
        .expect("Failed to run event loop");
}
