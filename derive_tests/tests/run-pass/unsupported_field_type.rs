use validify::Validate;

#[derive(Validate)]
struct Test {
    s: [u8; 11],
}

fn main() {}
