fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 {
        None
    } else {
        Some(x / y)
    };
    ans
}

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x);
    } else {
        println!("None");
    }
}

fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

fn func_ex_print_result<T: std::fmt::Display, S: std::fmt::Display>(ans: Result<T, S>) {
    match ans {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}

fn add_i32(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test1() {
    assert_eq!(add_i32(1, 2), 3);
}

#[test]
fn test2() {
    assert_eq!(add_i32(1, 2), 4);
}

fn main() {
    func_ex_print_some(func_ex_div_some(10, 5));
    func_ex_print_some(func_ex_div_some(10, 0));
    func_ex_print_some_match(func_ex_div_some(10, 5));
    func_ex_print_some_match(func_ex_div_some(10, 0));
    func_ex_print_result(func_ex_div_result(10, 5));
    func_ex_print_result(func_ex_div_result(10, 0));
}
