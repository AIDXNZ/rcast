use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;

use chromecast::channels::media::{Media, StreamType};
use chromecast::channels::receiver::CastDeviceApp;
use chromecast::CastDevice;
use iced::futures::executor::block_on;
use iced::widget::{button, column, container, image, pick_list, row, text};
use iced::{Alignment, Element, Sandbox, Settings};
use serde_json::Value;
use std::fs::read_dir;
use tokio::runtime;

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
    halfmin: i32,
    secs: i32,
    urls: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    Incrementhalfmin,
    Decrementhalfmin,
    IncrementSec,
    DecrementSec,
    Start,
}

async fn get_img_urls() -> Vec<String> {
    let mut paths = Vec::new();
    let mut urls: Vec<String> = Vec::new();
    for i in read_dir("/home/zdroid/Documents/Programming/rcast/src/images").unwrap() {
        let path = i.unwrap().path();
        paths.push(path.clone());
        let client = reqwest::Client::new();
        let contents = fs::read(path).unwrap();
        let mut target = String::new();
        for c in contents {
            target.push_str(&c.to_string())
        }

        let mut map = HashMap::new();
        map.insert("key", "6d207e02198a847aa98d0a2a901485a5");
        map.insert("action", "upload");
        map.insert("source", &target);

        let post_url =
            format!("https://freeimage.host/api/1/upload/?key=6d207e02198a847aa98d0a2a901485a5");
        let mut req = client.post(post_url).form(&map).send().await.unwrap();
        //Read Response
        let resp = req.text().await.unwrap();
        print!("{}", resp);
        let v: Value = serde_json::from_str(&resp).unwrap();
        let url = v["image"]["url"].to_string();
        urls.push(url.clone());
        println!("{}", url.clone());
    }

    return urls;
}

fn start_slideshow() {
    let device = CastDevice::connect("192.168.1.2", 8009).unwrap();
    let guess = mime_guess::from_path("some.png");

    let rec = device
        .receiver
        .launch_app(&CastDeviceApp::DefaultMediaReceiver)
        .unwrap();
    let session_id = rec.session_id;
    device
        .media
        .load(
            "destination",
            &session_id,
            &Media {
                content_id: "id".to_string(),
                stream_type: StreamType::None,
                content_type: format!("{:?}", guess),
                metadata: None,
                duration: None,
            },
        )
        .unwrap();
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let urls = rt.block_on(get_img_urls());
        Self { value: 0,halfmin: 0,secs: 0, urls }
    }

    fn title(&self) -> String {
        String::from("ColdWell Display")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                if self.value == 0 {
                } else {
                    self.value -= 1;
                }
            }
            Message::Start => {
                get_img_urls();
            }
            Message::Incrementhalfmin => {
                self.halfmin += 1;
            },
            Message::Decrementhalfmin => {
                if self.halfmin == 0 {
                } else {
                    self.halfmin -= 1;
                }
            },
            Message::IncrementSec => {
                self.secs += 1;
            },
            Message::DecrementSec => {
                if self.secs == 0 {
                } else {
                    self.secs -= 1;
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        //let path = Path::new("images/");
        container(column![
            row![text("Coldwell Displays").size(90),].padding(20),
            //pick_list(),
            column![
                text("Intervals").size(40),
                row![
                    column![
                        text("Minutes"),
                        row![
                            button("-").on_press(Message::DecrementPressed),
                            text(self.value).size(30),
                            button("+").on_press(Message::IncrementPressed),
                        ],
                    ].padding(10),
                    column![
                        text("1/2 Min"),
                        row![
                            button("-").on_press(Message::Decrementhalfmin),
                            text(self.halfmin).size(30),
                            button("+").on_press(Message::Incrementhalfmin),
                        ],
                    ].padding(10),
                    column![
                        text("Seconds"),
                        row![
                            button("-").on_press(Message::DecrementSec),
                            text(self.secs).size(30),
                            button("+").on_press(Message::IncrementSec),
                        ],
                    ].padding(10),
                ].padding(10),
                button(text("Start Slideshow").size(45)).on_press(Message::Start),
            ].padding(10),
        ])
        .padding(20)
        .into()
    }
}
