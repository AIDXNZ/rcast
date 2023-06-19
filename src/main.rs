use chromecast::channels::media::{Media, StreamType};
use chromecast::channels::receiver::CastDeviceApp;
use chromecast::CastDevice;
use crossbeam_channel::unbounded;
use iced::futures::executor::block_on;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Sandbox, Settings};
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::spawn;
use std::time::Duration;
use std::{fs, future, thread, time};
use tokio::task::spawn_blocking;

pub fn main() -> iced::Result {
    //upload_imgs();
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
    halfmin: i32,
    secs: i32,
    urls: Vec<String>,
    status: String,
    sender: crossbeam_channel::Sender<String>,
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
    Stop,
    Upload,
}

fn upload_imgs() {
    //println!("Upload?");
    use std::process::Command;
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("cd")
            .arg("\\dist")
            .arg("imageuploader.exe")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("python3")
            .arg("imageuploader.py")
            .output()
            .expect("failed to execute process")
    };

    
}

fn get_img_urls() -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    let contents = fs::File::open("dist/config/links.txt").unwrap();
    let lines = BufReader::new(contents).lines();
    for line in lines {
        match line {
            Ok(val) => urls.push(val),
            Err(_) => {}
        }
    }
    return urls;
}

fn stop() {
    let mut urls: Vec<String> = Vec::new();
    let contents = fs::File::open("dist/config/address.txt").unwrap();
    let lines = BufReader::new(contents).lines();
    for line in lines {
        match line {
            Ok(val) => urls.push(val),
            Err(_) => {}
        }
    }
    for url in urls {
        Command::new(".\\rust_caster.exe")
            .args(["-a", &url.to_string(), "--stop-current"])
            .spawn()
            .expect("Could not stop Application");
    }
}

fn start_worker(r: crossbeam_channel::Receiver<String>) {
    thread::spawn(move || loop {
        let v: Vec<_> = r.try_iter().collect();

        match v.iter().next() {
            Some(s) => {
                println!("{}", s);
                if s == "STOP" {
                    stop()
                } else if s == "UPLOAD" {
                    upload_imgs()
                } else if s.parse::<i32>().is_ok() {
                    let mut urls: Vec<String> = Vec::new();
                    let contents = fs::File::open("dist/config/address.txt").unwrap();
                    let lines = BufReader::new(contents).lines();
                    for line in lines {
                        match line {
                            Ok(val) => urls.push(val),
                            Err(_) => {}
                        }
                    }

                    let links = get_img_urls();

                    for link in links.clone() {
                        let guess = mime_guess::from_path(link.clone());

                        for dev in urls.clone() {
                            //print!("{}", dev.clone());
                            use std::process::Command;
                            Command::new(".\\rust_caster.exe")
                                .arg("-a")
                                .arg(dev.clone())
                                .arg("-m")
                                .arg(link.clone())
                                .spawn()
                                .expect("failed to execute process");
                        }
                        std::thread::sleep(time::Duration::from_secs(s.parse::<u64>().unwrap()));
                    }
                }
            }
            None => {}
        }
    });
}

fn start_slideshow(dur: i32) {
    //stop();

    stop();
    let mut urls: Vec<String> = Vec::new();
    let contents = fs::File::open("dist/config/address.txt").unwrap();
    let lines = BufReader::new(contents).lines();
    for line in lines {
        match line {
            Ok(val) => urls.push(val),
            Err(_) => {}
        }
    }

    let links = get_img_urls();

    for link in links.clone() {
        let guess = mime_guess::from_path(link.clone());

        for dev in urls.clone() {
            //print!("{}", dev.clone());
            use std::process::Command;
            Command::new(".\\rust_caster.exe")
                .arg("-a")
                .arg(dev.clone())
                .arg("-m")
                .arg(link.clone())
                .spawn()
                .expect("failed to execute process");
        }
        std::thread::sleep(time::Duration::from_secs(dur as u64));
    }
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        let urls = get_img_urls();
        let (s, r) = unbounded();
        start_worker(r);
        Self {
            value: 0,
            halfmin: 0,
            secs: 0,
            urls,
            status: "Not Running".to_string(),
            sender: s,
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

                self.sender.send(dur.to_string()).unwrap();

                self.status = "Running".to_string();
            }
            Message::Stop => {
                self.sender.send("STOP".to_string()).unwrap();
                self.status = "Stopped".to_string();
            },
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
            Message::Upload => {
                self.sender.send("Upload".to_string()).unwrap();
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
                text("Status").size(30),
                column![row![text(format!("{:?}", self.status)),],],
                text("Images").size(30),
                column![row![text(format!("Num of Slides {:?}", self.urls.len())),],].padding(10),
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
                row![button(text("Stop").size(25)).on_press(Message::Stop)].padding(5),
                row![button(text("Upload").size(25)).on_press(Message::Upload)].padding(5),
                row![button(text("Start Slideshow").size(45)).on_press(Message::Start)].padding(5),
            ]
            .padding(20),
        ])
        .padding(20)
        .into()
    }
}
