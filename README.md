# Rust Pathfinder

Rust pathfinder is a toy implementation of the basic rules of Pathfinder Second Edition.
My goal is to learn Rust, Webgpu and text rendering.

MIT and everything. Author Baptiste Manson

What sounds the most fun for me right now is to improve the visual engine a tad more, in order to have moving stuff on screen, and then make it playable.
If I ever reach a state where I can start the game, select which action I want to do with my character, and finish the encounter, it will be mission accomplished.
I have no ambition whatsoever around this project. It's just for fun.

## Graphics

- [x] allow render dimension to be different from tile atlas
- [x] add notion of scroll
- [x] scroll back and forth in the render loop
- [x] scroll based on keyboard input
- [x] block max scroll to border of the blueprints
- [x] get better tiles (buy them?)
- [x] keyboard simple support
- [x] import a map from tiled
- [x] add notion of layers of different renderers
- [x] cleanup renderers, develop helpers etc

### Sprite subproject

- [x] finish Bindable trait, BindGroup for binding position, and refactor the whole thing.
- [x] autogenerate vertex layout from struct via a macro
- [x] create a struct to manage a sprite atlas
- [x] load a demo sprite atlas
- [x] position 2 sprites
- [ ] fix scaling issues in sprite engine

### Screen space project

- [ ] Try to emulate light one way or another, in pure fragment shader!

### State and Animation Subproject

- [ ] extract state of the app outside the renderers
- [ ] create a notion of scene + resource manager to switch resources (check other frameworks for inspiration)
- [ ] create an animation kernel
- [ ] check with Cheng how he would do it
- [ ] add smooth move
- [ ] plug to rule engine via action descriptors

### Vignette shader Subproject

- [ ] create a renderer that vignettes / tilde shift the viewport at the end

### Text renderer Subproject

- [ ] render text in boxes
- [ ] make them scrollable?

### Interactivity Subproject

- [ ] make a click detector with a button class

### GUI Subproject

- [ ] implement the true game UI, with a map, units, text log, and actions panel. Just take BG's layout, it proved it is good.

## General Rules

- [x] add unit tests
- [x] learn benchmarking
- [x] learn profiling
- [x] setup webgpu
- [x] optimize (went from 80ms to 0.064ms)
- [ ] test the weapon traits

### Rules engine

- [x] add a rule engine
- [x] add multiple actions per turn

### Conditions

- [x] develop condition engine
  - [x] add concept of timers in timeline
  - [x] add condition in character
  - [x] implement dying, bless as PoC

### Equipment

- [x] add equipment to characters
- [x] provide an ergonomic API to add characters and loadouts.

### Weapons p278 - p283

- [ ] Attack Rolls: Multiple Attack Penalty
- [x] Attack Rolls: melee
- [x] Attack Rolls: ranged
- [x] Attack Rolls: bonus malus
- [x] Damage Rolls: melee
- [x] Damage Rolls: ranged
- [x] Damage Rolls: damage type PBS
- [x] Critical Hits
- [x] Unarmed Attacks
- [x] Unarmed Attacks: bonus different body parts
- [x] Improvised Weapons (just cook the -2 in the weapon)
- [ ] Range penalty
- [ ] Reload
- [x] Bulk (no rule per say)
- [x] Hands: general
- [x] Hands: general
- [ ] Ammunitions
- [ ] Port table of weapons

## Weapon Traits

- [x] Weapons traits: model and a few of the 33 core traits I have 63 traits capability
- [ ] Weapons traits: agile
- [ ] Weapons traits: attached
- [ ] Weapons traits: backstabber
- [ ] Weapons traits: backswing
- [x] Weapons traits: deadly
- [ ] Weapons traits: disarm
- [ ] Weapons traits: dwarf
- [ ] Weapons traits: elf
- [ ] Weapons traits: fatal
- [x] Weapons traits: finesse
- [ ] Weapons traits: forceful
- [ ] Weapons traits: free-hand
- [ ] Weapons traits: gnome
- [ ] Weapons traits: goblin
- [ ] Weapons traits: grapple
- [ ] Weapons traits: halfling
- [ ] Weapons traits: jousting
- [ ] Weapons traits: monk
- [ ] Weapons traits: nonlethal
- [ ] Weapons traits: orc
- [ ] Weapons traits: parry
- [x] Weapons traits: propulsive
- [ ] Weapons traits: reach
- [ ] Weapons traits: shove
- [ ] Weapons traits: sweep
- [ ] Weapons traits: thrown
- [ ] Weapons traits: trip
- [ ] Weapons traits: twin
- [ ] Weapons traits: two-hand
- [x] Weapons traits: unarmed
- [ ] Weapons traits: versatile
- [ ] Weapons traits: volley
- [ ] Critical specialization: model p282
- [ ] Critical specialization: axe p282
- [ ] Critical specialization: bomb
- [ ] Critical specialization: bow
- [ ] Critical specialization: brawling
- [ ] Critical specialization: club
- [ ] Critical specialization: dart
- [ ] Critical specialization: flail
- [ ] Critical specialization: hammer
- [ ] Critical specialization: knife
- [ ] Critical specialization: pick
- [ ] Critical specialization: polearm
- [ ] Critical specialization: shield
- [ ] Critical specialization: sling
- [ ] Critical specialization: spear
- [ ] Critical specialization: sword

### Runes

- [x] Weapon Potency
- [x] Striking
- [ ] Armor Potency
- [ ] Resilient

### Performance

- [ ] avoid clone on active_character

### Completed ✓

- [x] action selector / AI
- [x] base project

Weapons that can be modelled with simple callback based hashmap of timings (I scanned 50% of those)

- bloodletting kukri (after attack, src, target, with critical info)
- caterwaul sling (freq)
- dagger of Venom (freq. after attack, activate w/ trigger on damage enemy)
- dwarven thrower
- fighter's fork (activate, position)
- flame tongue (activate)
- frost brand (activate, notion of fire)
- gloomblade (need notion of light, detection, special upgrade)
- holy avenger (at start of round, activate, freq, after attack, on action after successful attack)

- start of action - offer new actions / apply temp reaction status. Need to have previous action.
- end of attack roll - allow to modify results.
- end of damage - modify results.

To validate shaders with Naga
cargo run --example convert --features wgsl-in,spv-out -- ..\rust-pathfinder\app\src\shaders\fragment.wgsl ..\rust-pathfinder\app\src\shaders\hi.spv
