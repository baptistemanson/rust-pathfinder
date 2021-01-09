# Rust Pathfinder

Rust pathfinder is a toy implementation of the basic rules of Pathfinder Second Edition.
My goal is to learn Rust.

MIT and everything. Author Baptiste Manson

Most interesting to code: benchmark, improve roll, 3 actions, position, visual

key takeaways on performances:

- references help
- clone on character is nothing, at least I dont see it
- println! is more costly on windows
- the performance I can reach with Rust I cannot with JS, by a factor x10-100.

## General

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
