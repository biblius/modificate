use validify::Validate;

#[derive(Validate)]
struct Test {
    #[validate(length())]
    s: String,
}

fn main() {}
