// struct Test {
//     csv: String,
// }

// impl Test {
//     pub fn new(a: String) -> Self {
//         Test { csv: a }
//     }

//     fn split(&mut self) -> std::str::Split {
//         self.csv.split(",")
//     }

//     fn print_b(&mut self) {
//         println!("{:?}", self.test_b);
//     }
// }

fn main() {
    // let mut test = Test::new("a,b,c".to_string());
    // test.print_a();
    // test.print_b();
    for s in "123,456,789".split(",") {
        println!("{}", s);
    }
}
