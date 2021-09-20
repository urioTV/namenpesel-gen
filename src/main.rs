use std::fs;
use rand::Rng;
use file_indexing::pust;
use scanrs::scann;
use iced::{Align, Button, Clipboard, Column, Command, Element, Sandbox, Settings, Text, TextInput, button, executor, text_input, ProgressBar};



pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}
#[derive(Default)]
struct Hello{
    input: text_input::State,
    input_value: String,
    button: button::State,
    status: bool,
    value: f32
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonPressed
}

impl Sandbox for Hello {
    type Message = Message;

    fn new() -> Self {
        Hello::default()
    }

    fn title(&self) -> String {
        String::from("Generator ludzi")
    }

    fn update(&mut self, message: Message){
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {
                let parsedstring = match self.input_value.as_str().parse::<usize>(){
                    Ok(i) => i,
                    Err(_e) => 1,
                };
                self.status = dopesel(parsedstring);
                self.value = parsedstring as f32;
            }
        }
        
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Generator ludzi"))
            .push(TextInput::new(&mut self.input, "How many people...", &self.input_value, Message::InputChanged))
            .push(Button::new(&mut self.button, Text::new("Submit")).on_press(Message::ButtonPressed))
            .push(ProgressBar::new(0.0..=(match self.input_value.as_str().parse::<f32>(){Ok(i) => i, Err(_e) => 1.0,}), self.value))
            .push(Text::new(format!("{}", self.status)))
            .into()
    }
}



fn dopesel(peopletimes: usize) -> bool {
    //println!("How many people do you want to gen?");
    let peopletimes = peopletimes;

    let namefile = "names.txt";
    let lastnamefile = "lastnames.txt";
    let writefile = "namenpesel.txt";

    fs::write(writefile, "").expect("File clean failure");
    let firstnames = fs::read_to_string(namefile)
                .expect("Something went wrong reading the file");
    let lastnames = fs::read_to_string(lastnamefile)
                .expect("Something went wrong reading the file");


    let mut firstnamelines: Vec<String> = firstnames.split_whitespace().map(|s| s.to_string()).collect();
    let mut lastnamelines: Vec<String> = lastnames.split_whitespace().map(|s| s.to_string()).collect();

    let line = connect_pesel(&mut firstnamelines, &mut lastnamelines, peopletimes);
        
    pust(writefile, line.as_str()).expect("Something went wrong writing the file");
    return true;
}

fn connect_pesel(names: &mut Vec<String>, lastnames: &mut Vec<String>, times: usize) -> String{
    let mut rng = rand::thread_rng();
    let mut namenpesel = "".to_string();
    let mut showing: String;

    for i in 0..times{
        showing = format!("{} {} {}\n", names[rng.gen_range(0..names.len())],lastnames[rng.gen_range(0..lastnames.len())] , rng.gen_range::<u64, _>(10000000000..99999999999));
        namenpesel += showing.as_str();
        //println!("{}/{}",i ,times);
        
    }
    return namenpesel;
}