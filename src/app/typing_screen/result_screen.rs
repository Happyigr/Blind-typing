#[derive(Clone, Copy)]
pub struct Accuracy {
    // amount of letters in the text
    pub amount: usize,
    // attempts of typing the letter
    pub attempts: usize,
}

impl Accuracy {
    pub fn new(amount: usize, attempts: usize) -> Accuracy {
        Accuracy { amount, attempts }
    }

    pub fn get_percent(&self) -> f64 {
        ((self.amount as f64 / self.attempts as f64) * 1000.0).round() / 10.0
    }
}

// pub struct Results {
//     q: usize,
//     w: usize,
//     e: usize,
//     r: usize,
//     t: usize,
//     y: usize,
//     u: usize,
//     i: usize,
//     o: usize,
//     p: usize,
//     a: usize,
//     s: usize,
//     d: usize,
//     f: usize,
//     g: usize,
//     h: usize,
//     j: usize,
//     k: usize,
//     l: usize,
//     z: usize,
//     x: usize,
//     c: usize,
//     v: usize,
//     b: usize,
//     n: usize,
//     m: usize,
// }
//
// impl Results {
//     pub fn new() -> Results {
//         Results {
//             q: 0,
//             w: 0,
//             e: 0,
//             r: 0,
//             t: 0,
//             y: 0,
//             u: 0,
//             i: 0,
//             o: 0,
//             p: 0,
//             a: 0,
//             s: 0,
//             d: 0,
//             f: 0,
//             g: 0,
//             h: 0,
//             j: 0,
//             k: 0,
//             l: 0,
//             z: 0,
//             x: 0,
//             c: 0,
//             v: 0,
//             b: 0,
//             n: 0,
//             m: 0,
//         }
//     }
// }
