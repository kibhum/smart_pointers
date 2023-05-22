fn main() {
    let single_value = Box::new(0.625);
    let x = 0.625;

    println!("Are the values equal? {}", x == *single_value);

    let mut stack_var = 4;
    let stack_ref = &stack_var;

    let heap_var = Box::new(stack_var);

    stack_var = 5;
    println!(
        "The value of stack_var = {} and heap_var = {}",
        stack_var, heap_var
    );

    let point = Box::new((100, 125));
    println!("{} {}", point.0, point.1);

    let x = *point;
}
