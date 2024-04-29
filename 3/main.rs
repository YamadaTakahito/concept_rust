// fn myprint<T: std::fmt::Display>(msg: &T) {
//     println!("{}", msg);
// }
//
fn my_clear(x: &mut String) {
    (*x).clear();
}

// fn return_hello() -> &String {
//     let s= "Hello".to_string();
//     &s
// }

fn pick1(x: &[i32], end: usize)->&[i32] {
    &x[..end]
}

fn pick2<'a>(x: &'a [i32], y:&'a [i32], end: usize) -> (&'a [i32], &'a [i32]) {
    (&x[..end], &y[..end])
}


fn main() {
    // let s = 1;
    // let t = s;
    // println!("s: {}, t: {}", s, t);

    // let s = "hello".to_string();
    // let t = s;
    // println!("s: {}, t: {}", s, t);

    // let s = 1;
    // myprint(s);
    // myprint(s);

    // let s = "Hello".to_string();
    // let ss = s.clone();
    // myprint(s);
    // myprint(ss);

    // let s = "Hello".to_string();
    // myprint(&s);
    // myprint(&s);

    // let mut s = "Hello".to_string();
    // println!("s: {}", s);
    // s.clear();
    // println!("s: {}", s);

    // let mut s = "Hello".to_string();
    // println!("s: {}", s);
    // my_clear(&mut s);
    // println!("s: {}", s);

    // let mut s = "Hello".to_string();
    // let s_ref = &mut s;
    // my_clear(s_ref);
    // println!("s: {}", s);
    // let ss_ref = &mut s;
    // my_clear(ss_ref);
    // println!("s: {}", s);

    // let mut x = 1;
    // let x_ref = &x;
    // println!("x: {}", x_ref);
    // x = 2;

    // let s = return_hello();
    // println!("{}", s);

    // let v1 = [1, 2, 3, 4, 5];
    // let p = pick1(&v1, 2);
    // for ss in p {
    //     println!("{}", ss);
    // }

    let v1 = [1, 2, 3, 4, 5];
    let v2 = [6, 7, 8];

    let p = pick2(&v1, &v2, 2);
    for ss in p.0 {
        println!("{}", ss);
    }

    for ss in p.1 {
        println!("{}", ss);
    }
}