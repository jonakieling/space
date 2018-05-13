space
=====

setup
-----

wip
developed on linux

* clone this
* run `cargo run` to compile
* levels are loaded statically from save.rs functions
* and stored in saves/auto-save.tar
* most objects and states are stored on exit

and here is what you can do
---------------------------

* move the black square with the white square face with `↑`,`→`,`↓` and `←`
* open and close *doors* by pressing `Return`
* interact with *terminals* by pressing `Return` when standing in front of the front-facing side, edit by typing `Anything` and `Backspace` the text and exit with `Escape`
* access the *inventory* with `i` and switch through the items with `↑` and `↓` (close it with `i`)
* while your *inventory* is open you can put *items* into the *crafting area* by pressing `Tab`, `Return` combines them into a product. The first matching *receipe* is used to determine the resulting *item*.
* hold `Left Ctrl` to show *circuitry*, you can list the items by pressing `Enter` (switch through with `↑` and `↓`, and close it with `Escape`)
* press `Insert` to switch to *edit mode*
* and there is *Gnoerf* you can talk to him and select dialog by pressing `Enter` (change dialog options with `↑` and `↓`)
* when *trading* with *Gnoerf* you can switch between the *inventories* and *trade areas* with `←` and `→`, change the item cursor of the current selection with `↑` and `↓` and move *items* from and to an *inventory* with `Tab`, `Return` does the trade.

edit mode
---------

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

outline
-------

thoughts before implmentation.

parts
-----

* stations (✓ just levels)
* objects (✓)
* ships (✓ just levels)
* npc (✓)

interaction
-----------

* conversations (✓)
* terminals, consoles (✓)
* space flight
* inventory (✓)

systems
-------

* object crafting (✓)
* energy, circuits (✓)
* scanners, navigation
* logs (✓)

thoughts after implmentation start.

engine tech
-----------

* gui for selections and lists (✓)
* ~~scene/storage management (how to deal with space, ship, level, objects)~~ kiss