#![allow(non_snake_case)]

pub mod components;

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use image_ranker::{
    item::Item,
    pair::Pair,
    ranking_service::RankingService,
    ranking_session::{RankingSession, WithStrategy},
};
fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(app);
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct LocalImage {
    path: String,
}

impl LocalImage {
    pub fn local_path(&self) -> &str {
        &self.path.as_str()[2..] // TODO: hack.
    }
}

fn app() -> Element {
    let mut service_signal = use_signal(|| RankingService::new(vec![]));
    let mut session_signal: Signal<RankingSession<LocalImage, WithStrategy>> = use_signal(|| {
        service_signal
            .read()
            .new_session()
            .with_each_with_each_strategy(false)
    }); // TODO: use Option::None.
    let mut current_pair_signal = use_signal(|| None);

    let on_images_selected = move |event: FormEvent| async move {
        let file_engine = match event.files() {
            Some(file_engine) => file_engine,
            _ => return,
        };

        let images = file_engine
            .files()
            .iter()
            .map(|p| LocalImage { path: p.clone() })
            .collect();
        let service = RankingService::new(images);

        service_signal.set(service);
    };

    let on_session_start = move |event: MouseEvent| async move {
        session_signal.set(
            service_signal
                .read()
                .new_session()
                .with_each_against_random_n_strategy(true, 5),
        );

        let pair = session_signal.write().next().clone();

        current_pair_signal.set(pair);
    };

    let selected_left_callback = move |event: MouseEvent| {
        (*current_pair_signal.write().as_mut().unwrap().left)
            .borrow_mut()
            .add_one_point();

        let pair = session_signal.write().next().clone();
        current_pair_signal.set(pair);
    };
    let selected_right_callback = move |event: MouseEvent| {
        (*current_pair_signal.write().as_mut().unwrap().right)
            .borrow_mut()
            .add_one_point();

        let pair = session_signal.write().next().clone();
        current_pair_signal.set(pair);
    };

    rsx! {
        body {
            style { {include_str!("../assets/main.css")} }

            h1 { "Image Ranking Tool" }

            p { "Select images to compare and rank them" }

            // TODO: add drop box with strategies.
            // TODO: add shuffle checkbox.
            // TODO: add skip pair button.

            div {
                label { r#for: "imageviewer", "Upload images to rank them" }
                input {
                    r#type: "file",
                    accept: "image/*",
                    multiple: true,
                    name: "imageviewer",
                    onchange: on_images_selected
                }
            }

            button {
                disabled: service_signal.read().items().len() == 0,
                onclick: on_session_start,
                "Start/Restart session"
            }

            p { "Images selected: {service_signal.read().items().len()}" }

            {
                let current_round = (*session_signal.read()).current_pair_index() as u32;
                let total_rounds = (*session_signal.read()).total_pairs() as u32;
                let progress = f64::from(current_round) / f64::from(total_rounds) * 100_f64;
                let pair = current_pair_signal.read();
                let pair = pair.as_ref();

                rsx! {
                    p { "Round: {current_round}/{total_rounds} ({progress:.1}%)" }

                    if pair.is_some() {
                        DisplayPair {
                            current_pair_signal: current_pair_signal,
                            selected_left_callback: selected_left_callback,
                            selected_right_callback: selected_right_callback,
                        }
                    }
                    else {
                        DisplayResult {
                            session: session_signal,
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct PairProprs {
    current_pair_signal: Signal<Option<Pair<Rc<RefCell<Item<LocalImage>>>>>>,
    selected_left_callback: EventHandler<MouseEvent>,
    selected_right_callback: EventHandler<MouseEvent>,
}

fn DisplayPair(props: PairProprs) -> Element {
    rsx! {
        div { class: "container",
            {
                let left = (*props.current_pair_signal.read().borrow().as_ref().expect("should be some").left)
                .borrow()
                .value()
                .local_path()
                .to_string();

                let right = (*props.current_pair_signal.read().borrow().as_ref().expect("should be some").right)
                .borrow()
                .value()
                .local_path()
                .to_string();

                rsx! {
                    div { class: "container__image",
                    input {
                        onclick: move |evt| props.selected_left_callback.call(evt),
                        src: "{left}",
                        alt: "Left image",
                        r#type: "image",
                        id: "left_image"
                    }
                }
                    div { class: "container__image",
                    input {
                        onclick: move |evt| props.selected_right_callback.call(evt),
                        src: "{right}",
                        alt: "Right image",
                        r#type: "image",
                        id: "right_image"
                    }
                }
            }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct ResultProprs {
    session: Signal<RankingSession<LocalImage, WithStrategy>>,
}

fn DisplayResult(props: ResultProprs) -> Element {
    let mut items = props.session.read().items().clone();
    items.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let total_items = items.len();
    rsx! {
        ul {
            for (index , image) in items.into_iter().enumerate() {
                {
                    let image = (*image).borrow();

                    let score = image.score();
                    let path = image.value().local_path();

                    rsx! {
                        li { class: "container__image",
                            p {
                                "Position: {index + 1}/{total_items}. Score: {score}."
                                br{}
                                "Path: {path}."
                            }
                            img { class: "container__image",
                                src: path,
                            }
                        }
                    }
                    }
            }
        }
    }
}
