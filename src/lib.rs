use chunks_rs::position::{Edge, EdgeConfig, Layer};
use chunks_rs::utils::{tag_button, tag_container, tag_label, tag_revealer};
use chunks_rs::widgets::Chunk;
use chunks_rs::GtkApp;
use chunks_rs::Internal;
use chunks_rs::Orientation;
use chunks_rs::RevealerTransitionType;

// use std::process::Command;

static mut HELLO: bool = false;

fn change() {
    unsafe {
        HELLO = !HELLO;
    }
    // Command::new("notify-send").arg("'hello'").spawn().ok();
}

pub fn show_hello(factory: &GtkApp) {
    let button = tag_button("storage");
    let text = tag_label("storage");
    Internal::static_widget(&button, "Open hello world");
    Internal::static_widget(&text, "Hello World");
    Internal::static_button(&button, change);

    let revealbox = tag_revealer("storage", text, 2, RevealerTransitionType::SlideDown);
    unsafe {
        Internal::update_revealer(&revealbox, HELLO);
    }
    let boxx = tag_container("storage", Orientation::Vertical, 2, vec![button, revealbox]);

    let margins = vec![(Edge::Top, 20), (Edge::Right, 160)];
    let anchors = EdgeConfig::TOP_RIGHT.to_vec();

    Chunk::new(
        factory.clone(),
        "Storage",
        boxx,
        margins,
        anchors,
        Layer::Top,
        true,
    )
    .build();
}

pub fn storage(factory: &GtkApp) {
    let tag = tag_label("storage");
    let margins = vec![(Edge::Top, 20), (Edge::Right, 160)];
    let anchors = EdgeConfig::TOP_RIGHT.to_vec();

    let storage_closure = || {
        let text = format!(
            "<span foreground='#FFFFFF'>{:.0}%</span>",
            Internal::get_storage(),
        );
        text
    };

    Internal::update_storage(&tag, storage_closure);

    Chunk::new(
        factory.clone(),
        "Storage",
        tag,
        margins,
        anchors,
        Layer::Top,
        false,
    )
    .build();
}
