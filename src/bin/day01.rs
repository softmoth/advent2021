const WINDOW: usize = 3; // Part 1: WINDOW = 1

#[derive(Debug)]
struct DoubleWindow {
    vals: [u16; WINDOW + 1],
    i: usize,
    count: usize,
}

impl Default for DoubleWindow {
    fn default() -> Self {
        DoubleWindow {
            vals: [0; WINDOW + 1],
            i: WINDOW + 1 - 1,
            count: 0,
        }
    }
}

impl DoubleWindow {
    fn next_index(i: usize) -> usize {
        (i + 1) % (WINDOW + 1)
    }

    fn push(&mut self, val: u16) {
        self.count += 1;
        self.i = Self::next_index(self.i);
        self.vals[self.i] = val;
    }

    fn sum_increased(&self) -> bool {
        // If the latest value is greater than the oldest, then the sum has increased; no need to
        // actually compute the sum
        self.count > WINDOW && self.vals[self.i] > self.vals[Self::next_index(self.i)]
    }
}

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let mut dw = DoubleWindow::default();
    let mut count = 0;
    for depth in std::io::stdin()
        .lines()
        .map(|s| s.unwrap().parse::<u16>().unwrap())
    {
        dw.push(depth);
        //dbg!(&dw);
        if dw.sum_increased() {
            count += 1;
        }
    }

    println!("Increases: {}", count);
}
