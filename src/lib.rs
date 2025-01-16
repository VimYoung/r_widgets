use chunks_rs::{
    position::{Edge, EdgeConfig, Layer},
    utils::{tag_button, tag_container, tag_label, tag_revealer},
    widgets::Chunk,
    Builder, GtkApp, Internal, Orientation, RevealerState, RevealerTransitionType,
};
use std::rc::Rc;
use std::sync::Mutex;

pub fn scroller_text(factory: &GtkApp) {
    let margins = vec![(Edge::Top, 20), (Edge::Right, 160)];
    let anchors = EdgeConfig::TOP_RIGHT.to_vec();

    let text1 = tag_label("storage");
    let text2 = tag_label("storage");
    let text3 = tag_label("storage");
    let text4 = tag_label("storage");
    let text5 = tag_label("storage");
    let text6 = tag_label("storage");
    let text7 = tag_label("storage");
    let text8 = tag_label("storage");

    Internal::static_widget(&text1, "THis is my first sentence.");
    Internal::static_widget(&text2, "THis is my first sentence.");
    Internal::static_widget(&text3, "THis is my first sentence.");
    Internal::static_widget(&text4, "THis is my first sentence.");
    Internal::static_widget(&text5, "THis is my first sentence.");
    Internal::static_widget(&text6, "THis is my first sentence.");
    Internal::static_widget(&text7, "THis is my first sentence.");
    Internal::static_widget(&text8, "THis is my first sentence.");

    let boxx = tag_container(
        "storage",
        Orientation::Vertical,
        20,
        vec![text1, text2, text3, text4, text5, text6, text7, text8],
    );

    Chunk::new(
        factory.clone(),
        "Storage",
        boxx, // tag_scroller(boxx)
        margins,
        anchors,
        Layer::Top,
        true,
    )
    .build();
}
pub fn show_hello(factory: &GtkApp) {
    let state = Rc::new(Mutex::new(RevealerState { open: false }));

    let button = tag_button("storage");
    let text = tag_label("storage");

    let revealbox = tag_revealer(
        "storage",
        text.clone(),
        2,
        RevealerTransitionType::SlideDown,
    );

    Internal::static_widget(&text, "Hello World");

    let rev_clone = revealbox.clone();
    Internal::static_button(&button, move || {
        Internal::update_revealer(rev_clone.clone(), state.clone())
    });

    Internal::static_widget(&button, "Open hello world");

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
