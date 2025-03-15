fn main() {
    let a;
    {
        let b = String::from("Ben");
        a = b; // &b does not live long enough
    }
    println!("{}", a);
}

// fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }

// fn main() {
//     let str1 = String::from("Rust");
//     let str2 = String::from("Programming");

//     let result = longest(&str1, &str2);
//     println!("The longest string is: {}", result);
// }

// #[allow(dead_code)]
// fn test_1(param_1: Vec<f64>) -> Vec<f64> { //Lifetimes don't apply because there are no reference inputs or output
//     param_1
// }

// #[allow(dead_code)]
// fn test_2<'a>(param_1: &'a Vec<f64>) -> Vec<f64> { //Lifetimes aren't an issue because there is no reference output
//     param_1.clone()
// }

// #[allow(dead_code)]
// fn test_3<'a>(param_1: Vec<f64>) -> &'a Vec<f64> { //Lifetimes don't apply because there are no reference input
//     &param_1
// }