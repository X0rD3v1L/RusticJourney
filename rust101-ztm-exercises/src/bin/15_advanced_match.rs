// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, f64),
    Standard(f64),
    VIP(String, f64)
}
fn main() {
    let tickets = vec![
        Ticket::Backstage("Ben10".to_owned(), 50.0),
        Ticket::Standard(3.0),
        Ticket::VIP("Ben".to_owned() , 100.0)
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(holder, price) => println!("Backstage Ticket Holder {:?}, Price {:?}", holder, price),
            Ticket::Standard(price) => println!("Standard Ticket Price {:?}", price),
            Ticket::VIP(holder, price) => println!("VIP Ticket Holder {:?}, Price {:?}", holder, price)
        }
    }
}