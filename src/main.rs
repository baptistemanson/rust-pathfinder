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
* - the absence of "this" is dope.
* - I dont like the dep resolution by hand on serde. Sucks butt.
* - had a very weird linker issue when I tried to init a vec with the result of a function
* - string slice are pretty cool for immutable strings, but it requires some lifetime specs to allow rust to know when it can be dropped.
* - lifetimes are hard. If you borrow something, you probably want it to have its own lifetime, otherwise you could have transfered ownership.
*
* Derive
* - derive is asking the compiler to implement the trait for us. derive(Debug) is calling the macro Debug
* To implement a custom one: https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
* That would be ultimate
*
* - the absence of NULL and option everywhere is somewhat similar to the guarantee I have with Typescript at the end of the day.
*
* can do 100 inserts in 1ms in a hashmap? (should be doing 1,000 tbh)
* - so many data structures, its gonna take a while to pick the ones I like. I will start everything with the default vec, hashmaps and structs I think...


Cool stuff: was able to create a struct and a trait by myself.
*/
// flush trait/method is in std::io::Write
mod abilities;
mod character;
mod check;
mod dice;
mod equipment;

use std::io::stdout;
use std::io::Write;

// Result is clear
// Box is because we cannot return a simple error straight, as errors can have different sizes
// so we put it in a box, and heap allocate the error.
// keyword dyn, I dont know why
type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn pause() {
    print!("Press enter to continue");
    stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

struct EncounterState<'a, 'b> {
    participants: Vec<&'a mut character::Character<'b>>,
}

impl<'a, 'b> EncounterState<'a, 'b> {
    fn is_encounter_done(&mut self) -> bool {
        for e in self.participants.iter_mut() {
            if e.hp > 0 {
                return false;
            }
        }
        return true;
    }

    fn roll_initiative(&mut self) {
        for character in self.participants.iter_mut() {
            character.roll_initiative();
            println!("{} rolled {}", character.name, character.initiative)
        }
        self.participants
            .sort_by(|a, b| b.initiative.cmp(&a.initiative));
    }
}
// we split it out of the main function, as main here returns an int, and if we want to use ? we need to return Result.
fn resolve_encounter() -> BoxResult<()> {
    println!("{:?}", abilities::get_default_abilities());
    // let equipment = equipment::get_default_equipment();
    // let backpack = equipment::get_backpack();
    // equipment::view_stash(&backpack);

    let mut kobold1 = character::Character::create("Kobold 1", 40);
    let mut kobold2 = character::Character::create("Kobold 2", 40);
    // played with borrowing.
    {
        let mut encounter_state = EncounterState {
            participants: vec![&mut kobold1, &mut kobold2],
        };
        encounter_state.roll_initiative();

        // Step 1 - Initiative check p468

        // Step 2 - Play a round p468
        while !encounter_state.is_encounter_done() {
            pause();
            for character in encounter_state.participants.iter_mut() {
                character.sub_hp(dice::d20());
                println!("remaining life: {}", character.hp)
            }
            // Step 3 - Finish the round p468
        }
    }
    println!("{:?}", kobold1.name);
    println!("Enemy dead");
    Ok(())
}

fn main() {
    match resolve_encounter() {
        Ok(()) => println!("Thank you for playing my game"),
        Err(e) => println!("Error: {}", e),
    }
}
