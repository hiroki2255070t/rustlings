struct Point {
    x: u64,
    y: u64,
}

enum Message {
    // TODO: 以下で使う列挙子を記載してください。
    Resize{ width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGBはredとgreen、blueの数値で構成されています。
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        // TODO: 上記で定義したメソッドを使って異なるMessage列挙型に対するmatch処理を実装してください。
        match message {
            Message::Resize{width, height} => self.resize(width, height),
            Message::Move(point) => self.move_position(Point { x: point.x, y: point.y }),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(red, green, blue) => self.change_color(red, green, blue),
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
