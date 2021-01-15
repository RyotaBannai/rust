use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};

use iced_futures::{self, futures};
use std::time::{Duration, Instant};

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

const FPS: u64 = 30; // Frame per second: 描画フレーム数
const MILLISECOND: u64 = 1000;
const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings);
}

#[derive(Debug, Clone)]
pub enum Message {
    Start, // start msg
    Stop,  // stop msg
    Reset, // reset msg
    Update,
}

pub enum TickState {
    Stopped,
    Ticking,
}

struct GUI {
    last_update: Instant,
    total_duration: Duration,
    tick_state: TickState,
    start_stop_button_state: button::State, // ボタンの状態を管理するための変数
    reset_button_state: button::State,
}

// https://docs.rs/iced/0.2.0/iced/trait.Application.html
impl Application for GUI {
    type Executor = executor::Default; // iced_futures::executor::AsyncStd
    type Message = Message;
    type Flags = ();

    // Application を run した時に実行した時に呼ばれるメソッド
    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                last_update: Instant::now(),
                total_duration: Duration::default(),
                tick_state: TickState::Stopped,
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    // Receives messages.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Start => self.tick_state = TickState::Ticking,
            Message::Stop => self.tick_state = TickState::Stopped,
            Message::Reset => {
                self.last_update = Instant::now();
                self.total_duration = Duration::default();
            }
            Message::Update => match self.tick_state {
                TickState::Ticking => {
                    let now_update = Instant::now();
                    self.total_duration += now_update - self.last_update;
                    self.last_update = now_update;
                }
                _ => {}
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        // Text::new("Hello, World!").into() // 全ての wedged は Into<Element> が実装されているため、into() メソッドで Element に変換できる

        let seconds = self.total_duration.as_secs();
        let duration_text = format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,                           // hours
            (seconds % HOUR) / MINUTE,                // minutes
            seconds % MINUTE,                         // seconds
            self.total_duration.subsec_millis() / 10  // milliseconds
        );
        let start_stop_text = match self.tick_state {
            TickState::Stopped => Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
            TickState::Ticking => Text::new("Stop")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        };
        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop,
        };

        let tick_text = Text::new(duration_text).font(FONT).size(60);
        let start_stop_button = Button::new(&mut self.start_stop_button_state, start_stop_text)
            .min_width(80)
            .on_press(start_stop_message); // ボタンを押下したら message を飛ばす

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80)
        .on_press(Message::Reset);

        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center) // 中央寄せ
            .into()
    }
    fn subscription(&self) -> Subscription<Message> {
        let timer = Timer::new(Duration::from_millis(MILLISECOND / FPS));
        iced::Subscription::from_recipe(timer).map(|_| Message::Update)
    }
}

pub struct Timer {
    duration: Duration,
}

impl Timer {
    fn new(duration: Duration) -> Timer {
        Timer { duration: duration }
    }
}

impl<H, E> iced_native::subscription::Recipe<H, E> for Timer
where
    H: std::hash::Hasher,
{
    type Output = Instant;
    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        self.duration.hash(state);
    }

    // Recipe 実行し、Subscription のイベントを出力する Stream を実装する必要がある（handler のようなもの）
    // stream は Iterator の非同期版
    // Iterator は next で次のデータを取得. 対して、stream は poll_next
    // Stream は非同期でデータを出し続けるが、一旦 None を出すと全てのデータを出し切ったことを意味し完全に停止する
    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, E>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;
        async_std::stream::interval(self.duration) // 一定間隔で現在の時刻を返す
            .map(|_| Instant::now())
            .boxed()
    }
}
