/*
Problem Statement:
You have an Employee struct with the following fields:

id (u32): Unique identifier

name (String): Employee's name

department (String): Department name

salary (u32): Employee's salary

Requirements:
Employees are considered equal if they have the same id and name, regardless of department or salary.

The hash code should be calculated only based on the id and name, ignoring department and salary.
*/

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// Define the Employee struct
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Employee {
    id: u32,
    name: String,
    department: String,
    salary: u32,
}

// Implement PartialEq and Eq for custom comparison
impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Eq for Employee {}

// Implement Hash for custom hashing
impl Hash for Employee {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only include id and name in the hash calculation
        self.id.hash(state);
        self.name.hash(state);
    }
}

fn main() {
    // Create a HashMap to store Employee objects as keys
    let mut employees = HashMap::new();

    // Create Employee objects
    let emp1 = Employee {
        id: 101,
        name: "Alice".to_string(),
        department: "Engineering".to_string(),
        salary: 75000,
    };

    let emp2 = Employee {
        id: 102,
        name: "Bob".to_string(),
        department: "Marketing".to_string(),
        salary: 60000,
    };

    let emp3 = Employee {
        id: 101,
        name: "Alice".to_string(),
        department: "HR".to_string(),  // Different department
        salary: 80000,                // Different salary
    };

    // Insert employee data into the HashMap
    employees.insert(emp1.clone(), "Full-time");
    employees.insert(emp2.clone(), "Contract");
    employees.insert(emp3.clone(), "Part-time"); // Overwrites emp1

    // Print the HashMap to verify the result
    for (employee, status) in &employees {
        println!("{:?} is {}", employee, status);
    }

    // Check the status of an employee using emp3 as the key
    if let Some(status) = employees.get(&emp3) {
        println!("Status of {:?} is {}", emp3, status);
    } else {
        println!("Employee not found!");
    }
}
