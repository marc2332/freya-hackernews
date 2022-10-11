#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod client;

use client::{Client, Story};
use dioxus::prelude::*;
use freya::{dioxus_elements, *};

fn main() {
    launch_with_title(app, "Freya client for Hacker News ");
}

fn app(cx: Scope) -> Element {
    let stories = use_state(&cx, Vec::new);
    let stories_setter = stories.setter();

    use_effect(&cx, (), move |_| async move {
        let client = Client::new();
        let client_stories = client.get_stop_stories().await;
        if let Ok(client_stories) = client_stories {
            stories_setter(client_stories);
        }
    });

    render!(
        container {
            background: "rgb(35, 35, 35)",
            width: "100%",
            height: "100%",
            if stories.is_empty() {
                rsx!{
                    label {
                        color: "white",
                        "Loading..."
                    }
                }
            } else {
                rsx!{
                    ScrollView {
                        show_scrollbar: true,
                        stories.get().iter().map(|story| {
                            rsx! {
                                Card {
                                    story: story
                                }
                            }
                        })
                    }
                }
            }
        }
    )
}

#[inline_props]
#[allow(non_snake_case)]
fn Card<'a>(cx: Scope<'a>, story: &'a Story) -> Element<'a> {
    render!(
        container {
            width: "100%",
            height: "150",
            padding: "10",
            container {
                background: "rgb(255, 102, 0)",
                padding: "25",
                width: "100%",
                height: "100%",
                radius: "7",
                label {
                    color: "rgb(20, 20, 20)",
                    font_size: "20",
                    "{story.title}"
                }
                rect {
                    width: "100%",
                    height: "100%",
                    padding: "15",
                    story.url.as_ref().map(|url| {
                        Some(rsx!{
                            label {
                                font_size: "17",
                                color: "rgb(35, 35, 35)",
                                "{url}"
                            }
                        })
                    })
                    story.text.as_ref().map(|text| {
                        Some(rsx!{
                            ScrollView {
                                show_scrollbar: true,
                                label {
                                    color: "rgb(20, 20, 20)",
                                    "{text}"
                                }
                            }
                        })
                    })
                }
            }
        }
    )
}
