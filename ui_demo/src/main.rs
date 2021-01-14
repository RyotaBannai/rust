use iced::{executor, Application, Command, Element, Settings, Text};

fn main() {
    GUI::run(Settings::default());
}

struct GUI;

// https://docs.rs/iced/0.2.0/iced/trait.Application.html
impl Application for GUI {
    type Executor = executor::Default; // iced_futures::executor::AsyncStd
    type Message = ();
    type Flags = ();

    // Application を run した時に実行した時に呼ばれるメソッド
    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (GUI, Command::none())
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, World!").into()
    }
}
