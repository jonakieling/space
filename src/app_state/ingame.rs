use std::time::Duration;
use std::collections::VecDeque;
use std::collections::BTreeSet;

use ggez::timer;
use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::event;
use ggez::event::*;

use player::*;
use storage::*;
use objects::*;
use misc::*;
use constants::*;
use ingame_state::*;
use AppState;
use savegame::save_scene;
use dialog::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputState {
    Terminal,
    World,
    Edit,
    Inventory,
    Circuitry,
    Menu,
    Npc,
    NpcTrade
}

pub struct SceneData {
    pub movement_timer: Duration,
    pub backdrop: String,
    pub player: Player,
    pub walls: PositionLevelStorage<Wall>,
    pub doors: PositionLevelStorage<Door>,
    pub terminals: PositionLevelStorage<Terminal>,
    pub circuitry: PositionLevelStorage<Circuitry>,
    pub generators: PositionLevelStorage<Generator>,
    pub npc: PositionLevelStorage<Npc>,
    pub receipes: Vec<Receipe>,
    pub dialog: Node<DialogItem>,
    pub insight_view: bool,
    pub main_menu: bool
}

pub struct Scene {
    pub current_ingame_state: Box<GameState>,
    pub data: SceneData
}

impl AppState for Scene {
    fn change_state(&self) -> Option<Box<AppState>> {
        if self.data.main_menu {
            let menu = super::menu::Scene::new().unwrap();
            Some(Box::new(menu))
        } else {
            None
        }
    }
}

impl Scene {
    pub fn new() -> GameResult<Scene> {
        
        // initialize player and level object storages
        // state and object are loaded seperatly

        let player_position = Position { x: 10, y: 10 };
        let player_direction = Direction::Down;
        let player_front_tile = &player_direction.value() + &player_position;
        let inventory = SelectionStorage::new();
        let player = Player {
            position: player_position,
            movement: vec![],
            direction: player_direction,
            front_tile: player_front_tile,
            inventory,
            terminal: Box::new(Terminal {
                text: Box::new(String::new()),
                front: Direction::Down
            })
        };

        let walls = <PositionLevelStorage<Wall>>::new();
        let doors = <PositionLevelStorage<Door>>::new();
        let terminals = <PositionLevelStorage<Terminal>>::new();
        let circuitry = <PositionLevelStorage<Circuitry>>::new();
        let generators = <PositionLevelStorage<Generator>>::new();
        let npc = <PositionLevelStorage<Npc>>::new();
        
        let mut receipes = Vec::new();
        receipes.push(
            Receipe {
                result: Item::Log,
                incredients: vec![Item::MicroController, Item::DataChip]
            }
        );

        let mut data = SceneData {
            movement_timer: Duration::from_millis(0),
            backdrop: String::from("/none.png"),
            player,
            walls,
            doors,
            terminals,
            circuitry,
            generators,
            npc,
            receipes,
            dialog: Node {
                value: DialogItem {
                    text: "".to_string(),
                    response: "".to_string(),
                    action: None
                },
                children: SelectionStorage::new()
            },
            insight_view: false,
            main_menu: false
        };

        data.update_power();

        let scene = Scene {
            current_ingame_state: Box::new(world::State::new()),
            data
        };


        Ok(scene)
    }
}

impl SceneData {
    pub fn check_player_collision(&self) -> bool {
        let mut found_collision = false;

        if let Some(_) = self.walls.get(self.player.front_tile) {
            found_collision = true;
        }

        if let Some(_) = self.terminals.get(self.player.front_tile) {
            found_collision = true;
        }

        if let Some(_) = self.generators.get(self.player.front_tile) {
            found_collision = true;
        }

        if let Some(_) = self.npc.get(self.player.front_tile) {
            found_collision = true;
        }

        if let Some(door) = self.doors.get(self.player.front_tile) {
            if let DoorStatus::Closed = door.status {
                found_collision = true;
            }
        }

        found_collision
    }

    pub fn interact_with_door(&mut self) {
        if let Some(door) = self.doors.get_mut(self.player.front_tile) {
            match door.status {
                DoorStatus::Closed => {
                    door.status = DoorStatus::Open;
                },
                DoorStatus::Open => {
                    door.status = DoorStatus::Closed;
                },
            }
        }
    }

    pub fn reset_powert(&mut self) {
        for circuitry in self.circuitry.iter_mut() {
            if let &mut Some(ref mut circuitry) = circuitry {
                circuitry.powered = false;
            }
        }
    }

    pub fn update_power(&mut self) {
        self.reset_powert();
        for (generator_index, generator) in self.generators.iter().enumerate() {
            let generator_pos = Position::from_int(generator_index as i32);
            
            if let &Some(_) = generator {
                let mut open_set = VecDeque::new();
                let mut closed_set: BTreeSet<Option<Position>> = BTreeSet::new();

                {
                    let mut root = generator_pos;
                    open_set.push_back(root);

                    while open_set.len() != 0 {
                        let subtree_root = open_set.pop_front();

                        if subtree_root == None {
                            break;
                        }
                        let subtree_root_position = subtree_root.unwrap();
                        for neighbor in self.circuitry.get_neighbors_at(subtree_root_position) {
                            if closed_set.contains(&Some(neighbor)) {
                                continue;
                            }
                            
                            if let None = open_set.iter().find(|&&visited| (neighbor == visited)) {
                                open_set.push_back(neighbor);
                            }
                        }

                        closed_set.insert(subtree_root);
                    }
                }

                for pos in closed_set {
                    if let Some(ref mut circuitry) = self.circuitry.get_mut(pos.unwrap()) {
                        circuitry.powered = true;
                    }
                }
            }
        }
    }

    pub fn current_circuitry(&mut self) -> Option<&mut Circuitry>{
        if let Some(current_circuitry) = self.circuitry.get_mut(self.player.front_tile) {
            Some(current_circuitry)
        } else {
            None
        }
    }

    pub fn current_terminal(&mut self) -> Option<&mut Terminal>{
        if let Some(current_terminal) = self.terminals.get_mut(self.player.front_tile) {
            Some(current_terminal)
        } else {
            None
        }
    }

    pub fn current_npc(&mut self) -> Option<&mut Npc>{
        if let Some(current_npc) = self.npc.get_mut(self.player.front_tile) {
            Some(current_npc)
        } else {
            None
        }
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        if let Some(state) = self.current_ingame_state.change_state() {
            self.current_ingame_state = state;
        }

        self.data.movement_timer += timer::get_delta(ctx);

        if self.data.movement_timer > Duration::from_millis(MOVEMENT_SPEED) {
            self.data.movement_timer = Duration::from_millis(0);
            if let Some(&current_movement) = self.data.player.movement.last() {
                if !self.data.check_player_collision() {
                    self.data.player.position = &self.data.player.position + &current_movement.value();
                }
            };
        }

        self.data.player.front_tile = &self.data.player.direction.value() + &self.data.player.position;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::LCtrl => {
                self.data.insight_view = true;
            },
            _ => ()
        }

        self.current_ingame_state.key_down_event(&mut self.data, _ctx, keycode, _keymod, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::LCtrl => {
                self.data.insight_view = false;
            },
            _ => ()
        }

        self.current_ingame_state.key_up_event(&mut self.data, ctx, keycode, _keymod, _repeat);
    }

    fn text_input_event(&mut self, ctx: &mut Context, text: String) {
        self.current_ingame_state.text_input_event(&mut self.data, ctx, text);
    }

    fn quit_event(&mut self, ctx: &mut Context) -> bool {
        self.current_ingame_state.quit_event(&mut self.data, ctx);

        save_scene(self, "saves/auto-save.tar");

        false
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, graphics::WHITE)?;
        let mut backdrop = graphics::Image::new(ctx, &self.data.backdrop)?;
        backdrop.set_filter(graphics::FilterMode::Nearest);

        let dst = graphics::Point2::new(20.0, 20.0);
        graphics::draw_ex(
            ctx,
            &backdrop,
            graphics::DrawParam {
                // src: src,
                dest: dst,
                rotation: 0.0,
                // offset: Point2::new(-16.0, 0.0),
                scale: graphics::Point2::new(GRID_SIZE as f32, GRID_SIZE as f32),
                // shear: shear,
                ..Default::default()
            },
        )?;

        graphics::set_color(ctx, graphics::BLACK)?;

        for (pos, wall) in self.data.walls.iter().enumerate() {
            if let &Some(_) = wall {
                Wall::draw(pos as i32, ctx)?;
            }
        }

        for (pos, terminal) in self.data.terminals.iter().enumerate() {
            if let &Some(ref current_terminal) = terminal {
                current_terminal.draw(pos as i32, ctx)?;
            }
        }

        for (pos, item) in self.data.doors.iter().enumerate() {
            if let &Some(ref door) = item {
                door.draw(pos as i32, ctx)?;
            }
        }

        for (pos, generator) in self.data.generators.iter().enumerate() {
            if let &Some(_) = generator {
                Generator::draw(pos as i32, ctx)?;
            }
        }

        if self.data.insight_view {
            for (pos, circuitry) in self.data.circuitry.iter().enumerate() {
                if let &Some(ref circuitry) = circuitry {
                    circuitry.draw(pos as i32, ctx)?;
                }
            }
        }

        for (pos, npc) in self.data.npc.iter().enumerate() {
            if let &Some(ref npc) = npc {
                npc.draw(pos as i32, ctx)?;
            }
        }

        self.data.player.draw(ctx)?;

        self.current_ingame_state.draw(&mut self.data, ctx)?;

        graphics::present(ctx);

        Ok(())
    }
}