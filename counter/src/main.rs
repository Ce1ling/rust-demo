fn main() {
    let mut counter = Counter::new();

    counter.add();
    counter.add();
    counter.add();
    println!("counter add: {}", counter.count);

    counter.minus();
    counter.minus();
    println!("counter minus: {}", counter.count);

    counter.multiply(2);
    counter.multiply(2);
    counter.multiply(2);
    println!("counter multiply: {}", counter.count);

    counter.divide(5);
    counter.divide(2);
    println!("counter divide: {}", counter.count);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

trait Math {
    fn add(&mut self) -> ();
    fn minus(&mut self) -> ();
    fn multiply(&mut self, n: u32) -> ();
    fn divide(&mut self, n: u32) -> ();
}

impl Math for Counter {
    fn add(&mut self) {
        self.count += 1;
    }

    fn minus(&mut self) {
        self.count -= 1;
    }

    fn multiply(&mut self, n: u32) {
        self.count *= n;
    }

    fn divide(&mut self, n: u32) {
        self.count /= n;
    }
}
