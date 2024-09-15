use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut solver = Solver::new(n, ab);
    solver.solve();
    solver.ans();
}

struct Solver {
    n: usize,
    ab: Vec<(usize, usize)>,
    s: Vec<(usize, usize)>,
    cost: usize,
    ans: Vec<String>,
}

impl Solver {
    fn new(n: usize, ab: Vec<(usize, usize)>) -> Solver {
        let s = vec![(0, 0)];
        let cost = 0;
        let ans = Vec::new();
        Solver { n, ab, s, cost, ans }
    }

    fn dist(&self, before: &(usize, usize), after: &(usize, usize)) -> usize {
        assert!(after.0 >= before.0 && after.1 >= before.1, "invalid before: {:?}, after: {:?}", before, after);

        after.0-before.0 + after.1-before.1
    }

    fn solve(&mut self) {
        let mut heap: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
        for &(a, b) in self.ab.iter() {
            heap.push((Reverse(a + b), a, b));
        }
        let mut ab = self.ab.clone();
        ab.sort();
        for _ in 0..self.n {
            let after = heap.pop().unwrap();
            let after = (after.1, after.2);
            let mut opt_before = (0, 0);
            let mut opt_dist = after.0 + after.1;
            for before in self.s.iter() {
                if after.0 < before.0 || after.1 < before.1 { continue; }
                if opt_dist > self.dist(before, &after) {
                    opt_dist = self.dist(before, &after);
                    opt_before = *before;
                }
            }
            self.make(opt_before, (after.0, opt_before.1));
            self.make((after.0, opt_before.1), after);
        }
        /*
        let mut pre_a = 0;
        for (a, b) in ab.iter() {
            self.make((pre_a, 0), (*a, 0));
            pre_a = *a;
        }
        for (a, b) in ab.iter() {
            self.make((*a, 0), (*a, *b));
        }*/
    }

    fn make(&mut self, before: (usize, usize), after: (usize, usize)) {
        self.ans.push(format!("{} {} {} {}", before.0, before.1, after.0, after.1));
        self.s.push(after);
        self.cost += after.0-before.0 + after.1-before.1;
    }

    fn ans(&self) {
        println!("{}", self.ans.len());
        for a in self.ans.iter() {
            println!("{}", a);
        }

        // json
        let max_ab = self.ab.iter().flat_map(|&(a, b)| vec![a, b]).max().unwrap();
        let mut opt_cost = 0;
        for i in 0..self.n {
            let (ai, bi) = self.ab[i];
            let mut tmp_a_cost = ai;
            let mut tmp_b_cost = bi;
            for j in 0..self.n {
                let (aj, bj) = self.ab[j];
                if ai > aj {
                    tmp_a_cost = tmp_a_cost.min(ai-aj);
                }
                if bi > bj {
                    tmp_b_cost = tmp_b_cost.min(bi-bj);
                }
            }
            opt_cost += tmp_a_cost + tmp_b_cost;
        }
        eprintln!("{{ \"N\": {}, \"cost\": {}, \"score\": {}, \"opt_cost\": {}, \"opt_score\": {} }}", self.n, self.cost, 10_usize.pow(6)*self.n*max_ab/(1+self.cost), opt_cost, 10_usize.pow(6)*self.n*max_ab/(1+opt_cost));
    }
}
