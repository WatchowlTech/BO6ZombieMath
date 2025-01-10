use std::io;

fn main() {
    println!("This guide is to get the beamsmasher Wonder Weapon.");
    println!("Step 1: Find the 3 computers");
    println!("1. Communications");
    println!("2. Storage Area");
    println!("3. Docks");
    println!("Step 2: Use deadwire to shoot the electrical boxes");
    println!("Step 3: Kill the zombie in the research office and collect his keycard");
    println!("Step 4: Bring the keycard to the hand/briefcase in Sea tower");
    println!("Step 5: Bring the reasonator back to the computer in the research office");
    println!("Step 6: Input the code into the computer");
    println!("Step 7: After one round, collect the reasonator and find the orbs");
    let mut input = String::new();

    println!("Enter value for x:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: i32 = input.trim().parse().expect("Please type a number!");

    input.clear();
    println!("Enter value for y:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y: i32 = input.trim().parse().expect("Please type a number!");

    input.clear();
    println!("Enter value for z:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let z: i32 = input.trim().parse().expect("Please type a number!");

    let a = (x * 2) + 11;
    let b = ((z * 2) + y) - 5;
    let c = ((y + z) - x).abs();

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);

    println!("Press Enter to exit...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
