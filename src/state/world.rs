use std::time::Duration;
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
use input::*;
use GameState;
use level;
use dialog::*;

#[derive(Debug, Clone)]
pub enum MenuOption {
    Save,
    Quit,
    Menu
}

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

pub struct Scene {
    pub movement_timer: Duration,
    pub player: Player,
    pub walls: PositionLevelStorage<Wall>,
    pub doors: PositionLevelStorage<Door>,
    pub terminals: PositionLevelStorage<Terminal>,
    pub circuitry: PositionLevelStorage<Circuitry>,
    pub generators: PositionLevelStorage<Generator>,
    pub npc: PositionLevelStorage<Npc>,
    pub terminal_text: graphics::Text,
    pub dialog: Node<DialogItem>,
    pub backdrop: String,
    pub input: InputState,
    pub edit_cursor: Position,
    pub edit_selection: SelectionStorage<String>,
    pub menu: SelectionStorage<MenuOption>,
    pub insight_view: bool,
    pub main_menu: bool
}

impl GameState for Scene {
    fn change_state(&self, ctx: &mut Context) -> Option<Box<GameState>> {
        if self.main_menu {
            let menu = super::menu::Scene::new(ctx).unwrap();
            Some(Box::new(menu))
        } else {
            None
        }
    }
}

impl Scene {
    pub fn new(ctx: &mut Context) -> GameResult<Scene> {

        let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
        
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


        let mut menu = SelectionStorage::new();
        menu.insert(MenuOption::Save);
        menu.insert(MenuOption::Menu);
        menu.insert(MenuOption::Quit);
        
        let mut scene = Scene {
            movement_timer: Duration::from_millis(0),
            player,
            walls,
            doors,
            terminals,
            circuitry,
            generators,
            npc,
            terminal_text: graphics::Text::new(ctx, "", &font)?,
            dialog: Node {
                value: DialogItem {
                    text: "".to_string(),
                    response: "".to_string(),
                    action: None
                },
                children: SelectionStorage::new()
            },
            backdrop: String::from("/none.png"),
            input: InputState::World,
            edit_cursor: Position {x: 0, y: 0},
            edit_selection: SelectionStorage::new(),
            menu,
            insight_view: false,
            main_menu: false
        };

        scene.update_power();

        Ok(scene)
    }

    pub fn check_player_collision(&self) -> bool {
        let mut found_collision = false;

        if let Some(&Some(_)) = self.walls.get(self.player.front_tile.x, self.player.front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(_)) = self.terminals.get(self.player.front_tile.x, self.player.front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(_)) = self.generators.get(self.player.front_tile.x, self.player.front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(_)) = self.npc.get(self.player.front_tile.x, self.player.front_tile.y) {
            found_collision = true;
        }

        if let Some(&Some(ref door)) = self.doors.get(self.player.front_tile.x, self.player.front_tile.y) {
            if let DoorStatus::Closed = door.status {
                found_collision = true;
            }
        }

        found_collision
    }

    pub fn get_edit_selection(&mut self) -> SelectionStorage<String> {
        let mut selection_storage: SelectionStorage<String> = SelectionStorage::new();
        if let Some(&Some(_)) = self.walls.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert("Wall".to_string());
        }
        
        if let Some(&Some(_)) = self.doors.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert("Door".to_string());
        }
        
        if let Some(&Some(_)) = self.terminals.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert("Terminal".to_string());
        }
        
        if let Some(&Some(_)) = self.circuitry.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert("Circuitry".to_string());
        }
        
        if let Some(&Some(_)) = self.generators.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert("Generator".to_string());
        }
        
        if let Some(&Some(ref npc)) = self.npc.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert(npc.name.clone());
        }

        if self.edit_cursor.x == self.player.position.x && self.edit_cursor.y == self.player.position.y {
            selection_storage.insert("Player".to_string());
        }

        selection_storage
    }

    pub fn interact_with_door(&mut self) {
        if let Some(&mut Some(ref mut door)) = self.doors.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
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
        for (generator_pos, generator) in self.generators.iter().enumerate() {
            if let &Some(_) = generator {
                for (circuitry_pos, circuitry) in self.circuitry.iter_mut().enumerate() {
                    if let &mut Some(ref mut circuitry) = circuitry {
                        if Position::from_int(circuitry_pos as i32).dist(&Position::from_int(generator_pos as i32)) <= 10.0 {
                            circuitry.powered = true;
                        }
                    }
                }
            }
        }
    }

    pub fn interact_with_circuitry(&mut self) {
        if let Some(&mut Some(_)) = self.circuitry.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            self.input = InputState::Circuitry;
        }
    }

    pub fn current_circuitry(&mut self) -> Option<&mut Circuitry>{
        if let Some(&mut Some(ref mut current_circuitry)) = self.circuitry.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            Some(current_circuitry)
        } else {
            None
        }
    }

    pub fn interact_with_npc(&mut self) {
        if let Some(&mut Some(ref mut npc)) = self.npc.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            match self.player.direction {
                Direction::Down => npc.direction = Direction::Up,
                Direction::Left => npc.direction = Direction::Right,
                Direction::Up => npc.direction = Direction::Down,
                Direction::Right => npc.direction = Direction::Left,
            }
            self.dialog = npc.dialog.root.clone();
            self.input = InputState::Npc;
        }
    }

    pub fn current_npc(&mut self) -> Option<&mut Npc>{
        if let Some(&mut Some(ref mut current_npc)) = self.npc.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            Some(current_npc)
        } else {
            None
        }
    }

    pub fn interact_with_terminal(&mut self, ctx: &mut Context) {
        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            let terminal_front_tile = &self.player.front_tile + &current_terminal.front.value();
            if terminal_front_tile == self.player.position {
                self.input = InputState::Terminal;
                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                self.terminal_text = graphics::Text::new(ctx, &current_terminal.text, &font).unwrap();
            }
        }
    }

    pub fn clear_terminal(&mut self, ctx: &mut Context) {
        let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
        self.terminal_text = graphics::Text::new(ctx, "", &font).unwrap();
        self.input = InputState::World;
    }

    pub fn terminal_remove_character(&mut self, ctx: &mut Context) {
        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            if current_terminal.text.len() > 0 {
                let text_len = current_terminal.text.len();
                current_terminal.text.split_off(text_len - 1);

                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                self.terminal_text = graphics::Text::new(ctx, &current_terminal.text, &font).unwrap();
            }
        }
    }

    pub fn terminal_add_character(&mut self, ctx: &mut Context, text: String) {
        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            if current_terminal.text.len() <= TERMINAL_LIMIT {
                let new_terminal_text = format!("{}{}", current_terminal.text, text);
                current_terminal.text = Box::new(new_terminal_text);

                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                self.terminal_text = graphics::Text::new(ctx, &current_terminal.text, &font).unwrap();
            }
        }
    }
}

impl event::EventHandler for Scene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.movement_timer += timer::get_delta(ctx);

        if self.movement_timer > Duration::from_millis(MOVEMENT_SPEED) {
            self.movement_timer = Duration::from_millis(0);
            if let Some(&current_movement) = self.player.movement.last() {
                if !self.check_player_collision() {
                    self.player.position = &self.player.position + &current_movement.value();
                }
            };
        }

        self.player.front_tile = &self.player.direction.value() + &self.player.position;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::LCtrl => {
                self.insight_view = true;
            },
            _ => ()
        }

        if self.input == InputState::World {
            world::key_down_event(self, _ctx, keycode, _keymod, repeat);
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::LCtrl => {
                self.insight_view = false;
            },
            _ => ()
        }

        match self.input {
            InputState::World => {
                world::key_up_event(self, ctx, keycode, _keymod, _repeat);
            },
            InputState::Terminal => {
                terminal::key_up_event(self, ctx, keycode, _keymod, _repeat);
            },
            InputState::Edit => {
                edit::key_up_event(self, ctx, keycode, _keymod, _repeat);
            },
            InputState::Inventory => {
                inventory::key_up_event(self, ctx, keycode, _keymod, _repeat);
            },
            InputState::Circuitry => {
                circuitry::key_up_event(self, ctx, keycode, _keymod, _repeat);
            },
            InputState::Menu => {
                menu::key_up_event(self, ctx, keycode, _keymod, _repeat);
            },
            InputState::Npc => {
                npc::key_up_event(self, ctx, keycode, _keymod, _repeat);
            },
            InputState::NpcTrade => {
                if keycode == Keycode::Escape {
                    self.input = InputState::World;
                    if let Some(npc) = self.current_npc() {
                        npc.direction = npc.look_at;
                    }
                }
            },
        }
    }

    fn text_input_event(&mut self, ctx: &mut Context, text: String) {
        if self.input == InputState::Terminal {
            self.terminal_add_character(ctx, text);
        }
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        level::save_scene(self, "saves/auto-save.tar");

        false
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);


        let mut backdrop = graphics::Image::new(ctx, &self.backdrop)?;
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

        for (pos, wall) in self.walls.iter().enumerate() {
            if let &Some(_) = wall {
                Wall::draw(pos as i32, ctx)?;
            }
        }

        for (pos, terminal) in self.terminals.iter().enumerate() {
            if let &Some(ref current_terminal) = terminal {
                current_terminal.draw(pos as i32, ctx)?;
            }
        }

        for (pos, item) in self.doors.iter().enumerate() {
            if let &Some(ref door) = item {
                door.draw(pos as i32, ctx)?;
            }
        }

        for (pos, generator) in self.generators.iter().enumerate() {
            if let &Some(_) = generator {
                Generator::draw(pos as i32, ctx)?;
            }
        }

        if self.insight_view {
            for (pos, circuitry) in self.circuitry.iter().enumerate() {
                if let &Some(ref circuitry) = circuitry {
                    circuitry.draw(pos as i32, ctx)?;
                }
            }
        } else if self.input == InputState::Circuitry {
            let front_index = self.player.front_tile.to_int();
            if let Some(ref circuitry) = self.current_circuitry() {
                circuitry.draw(front_index as i32, ctx)?;
            }
        }

        for (pos, npc) in self.npc.iter().enumerate() {
            if let &Some(ref npc) = npc {
                npc.draw(pos as i32, ctx)?;
            }
        }

        if self.input == InputState::Npc {
            super::draw_selection(&self.current_npc().unwrap().inventory, ctx, false)?;
            super::draw_dialog(&self.dialog, ctx)?;
        }

        self.player.draw(ctx)?;

        if let InputState::Terminal = self.input {
            super::draw_text(ctx, &self.terminal_text)?;
        }

        if self.input == InputState::Inventory {
            super::draw_selection(&self.player.inventory, ctx, true)?;
        }

        if self.input == InputState::Circuitry {
            super::draw_selection(&self.current_circuitry().unwrap().parts, ctx, true)?;
        }

        if self.input == InputState::Menu {
            super::draw_selection(&self.menu, ctx, true)?;
        }

        if self.input == InputState::Edit {
            super::draw_selection(&self.edit_selection, ctx, false)?;

            graphics::set_color(ctx, graphics::Color{r: 0.2, g: 0.8, b: 0.2, a: 1.0,})?;
            let edit_cursor = graphics::Rect::new(self.edit_cursor.viewport_x(), self.edit_cursor.viewport_y(), 21.0, 21.0);
            graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), edit_cursor)?;
        }

        match self.input {
            InputState::World => {
                if self.insight_view {
                    super::draw_input_state("World insight", ctx)?;
                } else {
                    super::draw_input_state("World", ctx)?;
                }
            },
            InputState::Terminal => {
                super::draw_input_state("Terminal", ctx)?;
            },
            InputState::Edit => {
                super::draw_input_state("Edit", ctx)?;
            },
            InputState::Inventory => {
                super::draw_input_state("Inventory", ctx)?;
            },
            InputState::Circuitry => {
                super::draw_input_state("Circuitry", ctx)?;
            },
            InputState::Menu => {
                super::draw_input_state("Menu", ctx)?;
            },
            InputState::Npc => {
                let current_npc = self.current_npc().unwrap();
                super::draw_input_state(&current_npc.name, ctx)?;
            },
            InputState::NpcTrade => {
                super::draw_input_state("Trade", ctx)?;
            },
        }

        graphics::present(ctx);

        Ok(())
    }
}