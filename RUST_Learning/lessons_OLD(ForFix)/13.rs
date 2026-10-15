/* Program description: 
Info!
    Array -!- fixed-size table.
    List  -!- dynamic-size table.

Attention!
    If something says Err! with [&str] you can try '&[&str]'.
    And: '&[<type>]', example: '&[i32]'.

For list (Vector / vec! / Vec) is like the same but!:
    Insted of [...] we use Vec<<type>> = vec!<array>
    Example:
    let mut fruits: Vec<&str> = vec!["item1", "item2", "item3"];

We see that there is a array. (array/list because there 3x not 1x item).
Because of that we put them inside "["<item1>", "<item2>", "<item3>"]".
Because its "String" and array, we need to add "(...): [&str] = (...)".
To print all items, we need to add ":?" inside "{}".
To print selected item like "<item1>" we need to use "<variable>[<item_id]".
Where <item1> = [0], <item2> = [1], (...), because we counting from 0 not 1.
'&str' you can't extend or shorten it, but 'String' can be extended or shortened.
Because its 3 items we add "[<type>; <count>]" (no id but count).
We cant left alone just '[&str]' because there is 3 items.
*/

fn list_example() {
    let mut dynamic_fruits: Vec<&str> = vec!["dyn_apple", "dyn_banana"];
    println!("Dynamic Fruits before push: {:?}", dynamic_fruits);
    let count_fruits_before = dynamic_fruits.len();
    println!("Amount of Fruits now: {}.", count_fruits_before);

    dynamic_fruits.push("dyn_orange");

    println!("Dynamic Fruits after push: {:?}", dynamic_fruits);
    let count_fruits_after = dynamic_fruits.len();
    println!("Amount of Fruits now: {}.", count_fruits_after);

    println!("Dynamic Fruit 1: {}", dynamic_fruits[0]);
    println!("Dynamic Fruit 2: {}", dynamic_fruits[1]);
    println!("Dynamic Fruit 3: {}", dynamic_fruits[2]);
}

fn main() {
    let fruits: [&str; 3] = ["apple", "orange", "banana"];

    println!("All fruits list: {:?}", fruits);
    println!("Fruit 1: {}", fruits[0]);
    println!("Fruit 2: {}", fruits[1]);
    println!("Fruit 3: {}", fruits[2]);

    list_example();
}

/* Output:
All fruits list: ["apple", "orange", "banana"]
Fruit 1: apple
Fruit 2: orange
Fruit 3: banana
*/