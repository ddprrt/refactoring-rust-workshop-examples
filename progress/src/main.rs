const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn expensive_calculation(_n: &i32) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

struct Progress<I> {
    iter: I,
    current: usize,
}

impl<I> Progress<I>
where
    I: Iterator,
{
    fn new(iter: I) -> Self {
        Self { iter, current: 1 }
    }
}

impl<I> Iterator for Progress<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        println!("{}", "*".repeat(self.current));
        self.current += 1;
        self.iter.next()
    }
}

// Extension Trait
trait ProgressIteratorExt {
    fn progress(self) -> Progress<Self>
    where
        Self: Sized;
}

impl<I> ProgressIteratorExt for I
where
    I: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn main() {
    let vec: Vec<i32> = (0..10).collect();

    (0..10).progress().enumerate().take(5).for_each(|(_, i)| {
        expensive_calculation(&i);
    });

    for (_index, elem) in vec.iter().take(5).enumerate().progress() {
        expensive_calculation(elem);
    }

    for i in (0..10).progress() {
        expensive_calculation(&i);
    }
}
