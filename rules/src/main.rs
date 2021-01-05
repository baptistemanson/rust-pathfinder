use pathfinder::main_loop;

// immutable
// try changing the API for something fully immutable? It would maybe be easier to deal with?
// would inform the ui on what needs redraw, but Ill probably redraw everything each frame...
// so I would have 1 state pointer?

fn main() {
    match main_loop(false) {
        Ok(()) => println!("Thank you for playing my game"),
        Err(e) => println!("Error: {}", e),
    }
}
