# setup
wip
developed on linux
* clone this
* run `cargo run` to compile
* levels are loaded statically from save.rs functions
* and stored in dev-level.tar
* most objects and states are stored on exit

# and here is what you can do
* move the black square with the white square face with `↑`,`→`,`↓` and `←`
* open and close *doorss* by pressing `enter`
* interact with *terminals* by pressing `enter` when standing in front of the front-facing side, edit by typing `Anything` and `Backspace` the text and exit with `Escape`
* access the *inventory* with `i` and switch through the items with `↑` and `↓` (close it with `i`)
* hold `left ctrl` to show *circuitry*, you can list the items by pressing `Enter` (switch through with `↑` and `↓`, and close it with `i`)
* press `Insert` to switch to *edit mode*

* and there is Gnoerf, to whom you may speak but beware, there is nothing but emptiness behind his words

# edit mode
pressing `Insert`
* move the *cursor* with `↑`,`→`,`↓` and `←`
* *object*s on the currently selected tile are listed on the top right 
* you can add things by pressing *key*s
* `w` for a *wall*
* `d` for a *door*
* `c` for *circuitry*
* `g` for a *generator*
* `t` for a *terminal*
* you can toggle a *doors* state and a *terminals* front facing direction by pressing `Tab` when having the *cursor* on the same tile

# outline

thoughts before implmentation.

## parts
* stations
* objects (✓)
* ships
* npc (✓)

## interaction
* conversations (✓)
* terminals, consoles (✓)
* space flight
* inventory (✓)

## systems
* object crafting
* energy, circuits
* scanners, navigation
* logs

thoughts after implmentation start.

## engine tech
* gui for selections and lists (✓)
* scene/storage management (how to deal with space, ship, level, objects)