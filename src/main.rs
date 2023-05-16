use std::collections::HashMap;
use std::fs;

use std::thread::spawn;

use chromecast::channels::media::{Media, StreamType};
use chromecast::channels::receiver::CastDeviceApp;
use chromecast::CastDevice;
use iced::futures::executor::block_on;
use iced::futures::io;

use iced::widget::{button, column, container, image, pick_list, row, svg, text};
use iced::{Alignment, Element, Sandbox, Settings};
use serde_json::Value;
use std::fs::read_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::runtime;

pub fn main() -> iced::Result {
    spawn(|| upload_imgs()).join().unwrap();
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

fn upload_imgs() {
    use std::process::Command;
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["python3", "imageuploader.py"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("python3")
            .arg("imageuploader.py")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
    println!("{}", String::from_utf8(hello).unwrap());
}

async fn get_img_urls() -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    let contents = fs::File::open("config/links.txt").unwrap();
    let lines = BufReader::new(contents).lines();
    for line in lines {
        match line {
            Ok(val) => urls.push(val),
            Err(_) => {}
        }
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
        Self {
            value: 0,
            halfmin: 0,
            secs: 0,
            urls,
        }
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
                upload_imgs();
            }
            Message::Incrementhalfmin => {
                self.halfmin += 1;
            }
            Message::Decrementhalfmin => {
                if self.halfmin == 0 {
                } else {
                    self.halfmin -= 1;
                }
            }
            Message::IncrementSec => {
                self.secs += 1;
            }
            Message::DecrementSec => {
                if self.secs == 0 {
                } else {
                    self.secs -= 1;
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        //let path = Path::new("images/");
        container(column![
            row![text("Coldwell Displays").size(90)].padding(20),
            //pick_list(),
            column![
                text("Settings").size(50),
                text("Images").size(30),
                column![
                    row![text(format!("Num of Slides {:?}", self.urls.len())),],
                    text(format!("Slide preivews: {:?}", self.urls))
                ].padding(10),
                text("Intervals").size(30),
                row![
                    column![
                        text("Minutes"),
                        row![
                            button("-").on_press(Message::DecrementPressed),
                            text(self.value).size(30),
                            button("+").on_press(Message::IncrementPressed),
                        ],
                    ]
                    .padding(10),
                    column![
                        text("1/2 Min"),
                        row![
                            button("-").on_press(Message::Decrementhalfmin),
                            text(self.halfmin).size(30),
                            button("+").on_press(Message::Incrementhalfmin),
                        ],
                    ]
                    .padding(10),
                    column![
                        text("Seconds"),
                        row![
                            button("-").on_press(Message::DecrementSec),
                            text(self.secs).size(30),
                            button("+").on_press(Message::IncrementSec),
                        ],
                    ]
                    .padding(10),
                ],
                button(text("Start Slideshow").size(45)).on_press(Message::Start),
            ]
            .padding(20),
        ])
        .padding(20)
        .into()
    }
}
