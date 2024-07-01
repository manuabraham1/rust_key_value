mod bst; // Import the bst module

fn main() {
    let mut bst = bst::BST::new();

    bst.insert(6, "Manu");
    bst.insert(5, "23");
    bst.insert(7, "Kochi");

    bst.print_in_order();

    if let Some(value) = bst.get(&5) {
        println!("Found: {}", value);
    } else {
        println!("Not Found");
    }

    bst.update(6, "Manu Abraham");

    bst.update(20, "IT");

    if let Some(value) = bst.get(&7) {
        println!("Found: {}", value);
    } else {
        println!("Not Found");
    }

    // Print all key-value pairs in the BST
    println!("All key-value pairs:");
    bst.print_in_order();
}
