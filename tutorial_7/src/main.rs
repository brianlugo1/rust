// Conditions and Control Flow

fn main() {
    // Condition
    // Six core operators, evaluate to true or false
    // <
    // >
    // <=
    // >=
    // !=
    // ==
    // let cond = 2 < 3;
    // let cond = 2 <= 3;
    // Different types cannot be compared
    // let cond = 2 <= 2.2;
    // Convert to same types!
    let cond = (2 as f32) <= 2.2;
    // Compound Conditions
    // && and
    // || or
    // ! not
    // let cond2 = true && cond;
    // let cond2 = false || cond;
    // let cond2 = !(true || cond);
    let _cond2 = false || !cond;
    // Precedence, ! first, && second, || third
    // Adding parenthesis changes Precedence
    // println!("{}", cond);
    // println!("{}", cond2);

    let food = "bread";
    if food == "cookie" {
        println!("I like cookies too!");
    } else if food == "fruit" {
        println!("That sounds healthy!");
    } else if food == "bread" {
        println!("That sounds boring!");
    } else {
        println!("Oh, that's too bad!");
    }
}
