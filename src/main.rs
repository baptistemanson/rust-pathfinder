/**
 * Findings:
 * - everything is a trait, no method per say.
 * - the trait implementations are not necessarily in the same package as the type definition. It allows to import only the code we need and extend existing types to conform to our traits.
 * - I can have method/trait implementation across several files / several times in a file.
 * - I like the absence of parenthesis and the implicit last line return in code blocks
 * - I cannot replace all unwrap by ?, it only works if all the errors are of the same type.
 * - We can find some types with the same name, in different namespaces, bringing confusion (std::error::Error and std::io::Error)
 * - I dont understand dyn, Box,
 * - not fully understand how ::<>() works
 * - structs are cool
 * - I like the snake case convention, just weird that types are camelCase.
 * - For what I am trying to achieve, I think I know enough Rust to just code it like I code in TS.
 * - Return types are mandatory
 * - debug is very cool, no reason to do C anymore when you have this
 * - modules are very powerful as well, they may turn messy later
 * - build size is small (300KB with serde, 115KB gzipped), because it doesnt include all chrome/v8
 * - ? wonder what would be the size with webgpu in it
 * - build speed is good, compared to Typescript (3s release build, 1.80 dev build for a microproject). Seems to have 1.2s overhead... I wonder how people deal with larger project build time... relying more on the compiler?
 * -
 * - I dont like the dep resolution by hand on serde. Sucks butt.
 * - had a very weird linker issue when I tried to init a vec with the result of a function
 * Derive
 * - derive is asking the compiler to implement the trait for us. derive(Debug) is calling the macro Debug
 * To implement a custom one: https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
 * That would be ultimate
 *
 * can do 100 inserts in 1ms in a hashmap? (should be doing 1,000 tbh)
 * - so many data structures, its gonna take a while to pick the ones I like. I will start everything with the default vec, hashmaps and structs I think...
 */
// flush trait/method is in std::io::Write
mod abilities;
mod equipment;

use abilities::*;
use std::io::stdout;
use std::io::Write;

use std::cmp::*;

struct Enemy {
    max_hp: u64,
    hp: u64,
    name: String,
}

impl Enemy {
    fn create(name: String, hp: u64) -> Enemy {
        Enemy {
            max_hp: hp,
            hp,
            name,
        }
    }
}
// Result is clear
// Box is because we cannot return a simple error straight, as errors can have different sizes
// so we put it in a box, and heap allocate the error.
// keyword dyn, I dont know why
type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// we split it out of the main function, as main here returns an int, and if we want to use ? we need to return Result.
fn fight_loop() -> BoxResult<()> {
    // let mut enemy = get_kobold_from_file()?;
    println!("{:?}", get_default_abilities());
    let equipment = equipment::get_default_equipment();
    let backpack = equipment::get_backpack();
    println!("{:?} {:?}", equipment, backpack);
    equipment::view_stash(&backpack);
    let mut enemy = Enemy::create(String::from("Kobold"), 12);

    while enemy.hp > 0 {
        print!("Enter damage:");
        stdout().flush()?;
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        let damage = line.trim().parse::<u64>()?;
        enemy.hp = max(enemy.hp - damage, 0);
        println!("Remaining life for {}: {} hp", enemy.name, enemy.hp);
    }
    println!("Enemy dead");
    Ok(())
}

fn main() {
    match fight_loop() {
        Ok(()) => println!("Thank you for playing my game"),
        Err(e) => println!("Error: {}", e),
    }
}
