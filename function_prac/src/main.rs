fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");

    println!("expression a: {}", expression_test());
}

fn expression_test() -> i32 {
    let y = {
        let x = 3;
        x + 1
    };
    return y;
}
