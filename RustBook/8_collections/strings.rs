fn main(){
    let data = "initial contents";

    let s = data.to_string();

    println!("{}",s);

    //Appending to a String with push_str and push

    let mut mutuable_string = String::from("foo");
    let s1 = "bar";
    mutuable_string.push(' '); //use to append single character
    mutuable_string.push_str(s1);
    println!("{}", mutuable_string);

    //Concatenation with the + Operator or the format! Macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{s}");

    let fs1 = String::from("tic");
    let fs2 = String::from("tac");
    let fs3 = String::from("toe");

    let fs = format!("{fs1}-{fs2}-{fs3}");

    println!("{fs}");

    //Iterating Over Strings

    for c in "Зд".chars() {
        println!("{c}");
    }
    


}