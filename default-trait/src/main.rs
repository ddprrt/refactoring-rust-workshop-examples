#[derive(Default)]
struct Clock {
    hours: usize,
    minutes: usize,
}

fn anything<T: Default>() -> T {
    Default::default()
}

trait X {
    fn x() -> Self;
}

impl X for Clock {
    fn x() -> Self {
        Clock::default()
    }
}

enum MyOption<T> {
    Some(T),
    None,
}

impl<T> From<T> for MyOption<T> {
    fn from(value: T) -> Self {
        MyOption::Some(value)
    }
}

fn main() {
    let clock = Clock::default();
    let clock: Clock = Default::default();
    let clock = Clock {
        ..Default::default()
    };
    let clock = Clock {
        minutes: 50,
        ..Default::default()
    };
    let clock: Clock = X::x();
    let z: Option<Clock> = Clock::default().into();
    let clock = anything::<Clock>();
    println!("Hello, world!");
}
