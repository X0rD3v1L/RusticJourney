fn main(){
    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    heap_procedure(&heap_f64);
    println!("In main heap {}", heap_f64);

}

fn stack_procedure(mut param: f64){
    param += 9.;
    println!("In stack procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>){
    println!("In heap procedure with param {}", param);
}