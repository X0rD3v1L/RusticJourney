// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Locker {
    student_name: String,
    locker: Option<i32>
}
fn main() {
    let student_details = Locker {
        student_name: "Ben".to_owned(),
        locker: Some(115),
    };

    println!("Student Name :: {:?}", student_details.student_name);

    match student_details.locker {
        Some(locker_number) => println!("Locker Number :: {:?}", locker_number),
        None => println!("No locker assigned"),
    }

}