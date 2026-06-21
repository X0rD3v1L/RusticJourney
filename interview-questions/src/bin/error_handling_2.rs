/*
Without ?

fn calculate_age(birth_year: &str) -> Result<i32, std::num::ParseIntError> {
    let year = match birth_year.parse::<i32>() {
        Ok(y) => y,
        Err(e) => return Err(e),
    };

    Ok(2026 - year)
}

fn main() {
    println!("{:?}", calculate_age("1998"));
}
*/

fn calculate_age(birth_year: &str) -> Result<i32, std::num::ParseIntError> {
    let year = birth_year.parse::<i32>()?;
    Ok(2026 - year)
}

fn main() {
    println!("{:?}", calculate_age("2000"));
}