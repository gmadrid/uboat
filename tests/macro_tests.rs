#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/*.rs")
}

fn main() {}
