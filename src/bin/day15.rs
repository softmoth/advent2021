#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

#![feature(mixed_integer_ops)]
#![feature(anonymous_lifetime_in_impl_trait)]

use itertools::Itertools;

use petgraph::{
    algo::astar,
    data::{Element, FromElements},
    graph::UnGraph,
    visit::{EdgeRef, NodeIndexable},
};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!(
        "Result: {:?}",
        process(&std::fs::read_to_string("inputs/day15.txt").unwrap())
    );
}

fn process(input: &str) -> u64 {
    let grid = Grid::new(input, 5, 5);
    //eprintln!("{:?}", &grid);

    let graph = UnGraph::<u8, ()>::from_elements(grid.nodes().chain(grid.edges()));
    //eprintln!("{:?}", &graph);

    let start = graph.from_index(0);
    let end = graph.from_index(graph.node_count() - 1);
    let path = astar(
        &graph,
        start,
        |finish| finish == end,
        |e| u64::from(graph[e.target()]),
        |_n| 0 /* {
            // Manhattan distance is guaranteed to be <= real distance since no diagonal
            // movement is allowed, so this is a valid heuristic
            let cur = grid.position_of(graph.to_index(n));
            let end = grid.position_of(graph.to_index(end));
            (cur.0.abs_diff(end.0) + cur.1.abs_diff(end.1))
                .try_into()
                .unwrap()
        }*/,
    );

    //eprintln!("{:?}", &path);

    path.unwrap().0
}

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
    x_repeat: usize,
    y_repeat: usize,
}

type GraphElement = Element<u8, ()>;

impl Grid {
    fn new(input: &str, x_repeat: usize, y_repeat: usize) -> Self {
        let width = input.lines().next().unwrap().len();

        let data: Vec<u8> = input
            .lines()
            .flat_map(|line| {
                assert_eq!(line.len(), width);
                line.chars().map(|c| {
                    let c = c.to_digit(10).unwrap().try_into().unwrap();
                    assert!((1..=9).contains(&c)); // Guaranteed & required by problem defn
                    c
                })
            })
            .collect();

        let height = data.len() / width;

        Self {
            data,
            width: width * x_repeat,
            height: height * y_repeat,
            x_repeat,
            y_repeat,
        }
    }

    const fn index_of(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    const fn position_of(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn at(&self, (x, y): (usize, usize)) -> u8 {
        let w = self.width / self.x_repeat;
        let xr = x / w;
        let x = x % w;
        let h = self.height / self.y_repeat;
        let yr = y / h;
        let y = y % h;

        let cost: u8 = self.data[y * w + x] + u8::try_from(xr).unwrap() + u8::try_from(yr).unwrap();
        (cost - 1) % 9 + 1
    }

    fn nodes(&self) -> impl Iterator<Item = GraphElement> + '_ {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|pos| GraphElement::Node {
                weight: self.at(pos),
            } as GraphElement)
    }

    fn edges(&self) -> impl Iterator<Item = GraphElement> + '_ {
        let horizontal = (1..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| ((x - 1, y), (x, y)));
        let vertical = (0..self.width)
            .cartesian_product(1..self.height)
            .map(|(x, y)| ((x, y - 1), (x, y)));

        horizontal.chain(vertical).flat_map(|(apos, bpos)| {
            let a = self.index_of(apos);
            let b = self.index_of(bpos);
            [
                GraphElement::Edge {
                    source: a,
                    target: b,
                    weight: (),
                },
                GraphElement::Edge {
                    source: b,
                    target: a,
                    weight: (),
                },
            ]
        })
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.data.chunks(self.width / self.x_repeat) {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

/*
Part 1: using petgraph astar
Result: 673
dhat: Total:     4,340,851 bytes in 95 blocks
dhat: At t-gmax: 2,313,140 bytes in 9 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 3876maxresident)k
0inputs+0outputs (0major+628minor)pagefaults 0swaps

Part 2:
Result: 2893
dhat: Total:     85,013,859 bytes in 122 blocks
dhat: At t-gmax: 46,963,636 bytes in 9 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

0.10user 0.00system 0:00.10elapsed 100%CPU (0avgtext+0avgdata 47104maxresident)k
0inputs+0outputs (0major+8568minor)pagefaults 0swaps


Part 2, 15x15 repeat, constant 0 heuristic:
Result: 8393
dhat: Total:     1,099,109,875 bytes in 143 blocks
dhat: At t-gmax: 585,194,420 bytes in 9 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

1.01user 0.05system 0:01.07elapsed 99%CPU (0avgtext+0avgdata 408132maxresident)k
0inputs+0outputs (0major+16825minor)pagefaults 0swaps


With manhattan distance heuristic:
Result: 8393
dhat: Total:     998,577,651 bytes in 144 blocks
dhat: At t-gmax: 534,928,308 bytes in 9 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

1.07user 0.06system 0:01.14elapsed 100%CPU (0avgtext+0avgdata 409372maxresident)k
0inputs+0outputs (0major+16632minor)pagefaults 0swaps

*/
