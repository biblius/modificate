use validify::Validate;

#[derive(Validate)]
struct Test {
    #[validate(length())]
    s: usize,
}

fn main() {}
