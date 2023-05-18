use chromecast::channels::media::{Media, StreamType};
use chromecast::channels::receiver::CastDeviceApp;
use chromecast::CastDevice;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Sandbox, Settings};
use std::time::Duration;
use std::{fs, time, thread};
use std::io::{BufRead, BufReader};
use std::thread::spawn;

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
            .arg("cd")
            .arg("\\dist")
            .arg("imageuploader.exe")
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

fn get_img_urls() -> Vec<String> {
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

fn start_slideshow(dur: i32) {
    let mut urls: Vec<String> = Vec::new();
    let contents = fs::File::open("config/address.txt").unwrap();
    let lines = BufReader::new(contents).lines();
    for line in lines {
        match line {
            Ok(val) => urls.push(val),
            Err(_) => {}
        }
    }

    let links = get_img_urls();

    for link in links {
        let guess = mime_guess::from_path(link.clone());

        for dev in urls.clone() {
            let device = CastDevice::connect(dev, 8009).unwrap();
            let rec = device
                .receiver
                .launch_app(&CastDeviceApp::DefaultMediaReceiver)
                .unwrap();
            let session_id = rec.session_id;
            device
                .media
                .load(
                    link.clone(),
                    session_id,
                    &Media {
                        content_id: link.clone(),
                        stream_type: StreamType::None,
                        content_type: format!("{:?}", guess),
                        metadata: None,
                        duration: None,
                    },
                )
                .unwrap();
        }
        //Wait for intervals
        std::thread::sleep(Duration::new(dur as u64, 0));
    }
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        let urls = get_img_urls();
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
                let min = self.value.clone() * 60;
                let half_min = self.halfmin.clone() * 30;
                let seconds = self.secs.clone();
                let dur = min + half_min + seconds;
                thread::spawn(move|| {
                    start_slideshow(dur.clone());
                });
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
                ]
                .padding(10),
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
