use std::time::Duration;
use std::fmt::Debug;
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
use mode::*;

pub struct Scene {
    pub movement_timer: Duration,
    pub player: Player,
    pub walls: PositionLevelStorage<Wall>,
    pub doors: PositionLevelStorage<Door>,
    pub terminals: PositionLevelStorage<Terminal>,
    pub circuitry: PositionLevelStorage<Circuitry>,
    pub generators: PositionLevelStorage<Generator>,
    pub terminal_text: graphics::Text,
    pub input: InputState,
    pub edit_cursor: Position,
    pub edit_selection: SelectionStorage<String>,
    pub insight_view: bool,
}

impl Scene {
    pub fn new(_ctx: &mut Context) -> GameResult<Scene> {

        let font = graphics::Font::new(_ctx, "/04B_03.TTF", 12).unwrap();
        
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
        
        let mut scene = Scene {
            movement_timer: Duration::from_millis(0),
            player,
            walls,
            doors,
            terminals,
            circuitry,
            generators,
            terminal_text: graphics::Text::new(_ctx, "", &font)?,
            input: InputState::World,
            edit_cursor: Position {x: 0, y: 0},
            edit_selection: SelectionStorage::new(),
            insight_view: false,
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
            selection_storage.insert(String::from("Wall"));
        }
        
        if let Some(&Some(_)) = self.doors.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert(String::from("Door"));
        }
        
        if let Some(&Some(_)) = self.terminals.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert(String::from("Terminal"));
        }
        
        if let Some(&Some(_)) = self.circuitry.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert(String::from("Circuitry"));
        }
        
        if let Some(&Some(_)) = self.generators.get(self.edit_cursor.x, self.edit_cursor.y) {
            selection_storage.insert(String::from("Generator"));
        }

        selection_storage
    }

    pub fn interact_with_door(&mut self) {
        if let Some(&mut Some(ref mut door)) = self.doors.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            match door.status {
                DoorStatus::Closed => {
                    door.status = DoorStatus::Open;
                    println!("door opened");
                },
                DoorStatus::Open => {
                    door.status = DoorStatus::Closed;
                    println!("door closed");
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
                        if Position::to_two_d(circuitry_pos as i32).dist(&Position::to_two_d(generator_pos as i32)) <= 10.0 {
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

    pub fn interact_with_terminal(&mut self, ctx: &mut Context) {
        if let Some(&mut Some(ref mut current_terminal)) = self.terminals.get_mut(self.player.front_tile.x, self.player.front_tile.y) {
            let terminal_front_tile = &self.player.front_tile + &current_terminal.front.value();
            if terminal_front_tile == self.player.position {
                self.input = InputState::Terminal;
                let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
                self.terminal_text = graphics::Text::new(ctx, &current_terminal.text, &font).unwrap();
            } else {
                println!("this is not the front of the terminal");
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
            }
        }
    }

    fn text_input_event(&mut self, ctx: &mut Context, text: String) {
        if self.input == InputState::Terminal {
            self.terminal_add_character(ctx, text);
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let mut ship = graphics::Image::new(ctx, "/realm_of_sol__0000s_0001_2.1.png")?;
        ship.set_filter(graphics::FilterMode::Nearest);
        let dst = graphics::Point2::new(20.0, 20.0);
        graphics::draw_ex(
            ctx,
            &ship,
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
            let front_index = self.player.front_tile.to_one_d();
            if let Some(ref circuitry) = self.current_circuitry() {
                circuitry.draw(front_index as i32, ctx)?;
            }
        }

        graphics::set_color(ctx, graphics::BLACK)?;
        let player = graphics::Rect::new(self.player.position.viewport_x(), self.player.position.viewport_y(), 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, player)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new(self.player.position.viewport_x() + 5.0 + (self.player.direction.value().viewport_x() * 0.2), self.player.position.viewport_y() + 5.0 + (self.player.direction.value().viewport_y() * 0.2), 10.0, 10.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, face)?;

        if let InputState::Terminal = self.input {
            graphics::set_color(ctx, graphics::BLACK)?;
            let console = graphics::Rect::new(740.0 - self.terminal_text.width() as f32 + 20.0, 20.0, self.terminal_text.width() as f32 + 20.0, 20.0);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, console)?;
            graphics::set_color(ctx, graphics::WHITE)?;
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), console)?;
            graphics::draw(ctx, &self.terminal_text, graphics::Point2::new(750.0 - self.terminal_text.width() as f32 + 20.0, 20.0), 0.0)?;
        }

        if self.input == InputState::Inventory {
            draw_selection(&self.player.inventory, ctx, true)?;
        }

        if self.input == InputState::Circuitry {
            draw_selection(&self.current_circuitry().unwrap().parts, ctx, true)?;
        }

        if self.input == InputState::Edit {
            draw_selection(&self.edit_selection, ctx, false)?;
        }

        if self.input == InputState::Edit {
            graphics::set_color(ctx, graphics::Color{r: 0.2, g: 0.8, b: 0.2, a: 1.0,})?;
            let edit_cursor = graphics::Rect::new(self.edit_cursor.viewport_x(), self.edit_cursor.viewport_y(), 21.0, 21.0);
            graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), edit_cursor)?;
        }

        match self.input {
            InputState::World => {
                if self.insight_view {
                    draw_input_state("World insight", ctx)?;
                } else {
                    draw_input_state("World", ctx)?;
                }
            },
            InputState::Terminal => {
                draw_input_state("Terminal", ctx)?;
            },
            InputState::Edit => {
                draw_input_state("Edit", ctx)?;
            },
            InputState::Inventory => {
                draw_input_state("Inventory", ctx)?;
            }
            InputState::Circuitry => {
                draw_input_state("Circuitry", ctx)?;
            },
        }

        graphics::present(ctx);

        Ok(())
    }
}

fn draw_input_state(state: &str, ctx: &mut Context) -> GameResult<()> {
    let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
    let input_state_text = String::from(state);
    let input_state_graphics = graphics::Text::new(ctx, &input_state_text, &font).unwrap();
    graphics::set_color(ctx, graphics::BLACK)?;
    let input_state_box = graphics::Rect::new(20.0, 20.0, input_state_graphics.width() as f32 + 20.0, 20.0);
    graphics::rectangle(ctx, graphics::DrawMode::Fill, input_state_box)?;
    graphics::set_color(ctx, graphics::WHITE)?;
    graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), input_state_box)?;
    graphics::draw(ctx, &input_state_graphics, graphics::Point2::new(30.0, 20.0), 0.0)?;

    Ok(())
}

fn draw_selection<T: Clone + Debug>(selection: &SelectionStorage<T>, ctx: &mut Context, cursor: bool) -> GameResult<()> {
    let font = graphics::Font::new(ctx, "/04B_03.TTF", 12).unwrap();
    let mut inventory_item_position = 0.0;
    let current_item = selection.current_index();
    for (pos, item) in selection.iter().enumerate() {
        let item_text = format!("{:?}", item);
        let item_graphics = graphics::Text::new(ctx, &item_text, &font).unwrap();
        let inventory_box = graphics::Rect::new(760.0 - (item_graphics.width() as f32), 20.0 + (inventory_item_position * 25.0), item_graphics.width() as f32 + 20.0, 20.0);
        graphics::set_color(ctx, graphics::BLACK)?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, inventory_box)?;
        graphics::set_color(ctx, graphics::WHITE)?;
        if pos == current_item && cursor {
            graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), inventory_box)?;
        }
        graphics::draw(ctx, &item_graphics, graphics::Point2::new(771.0 - item_graphics.width() as f32, 20.0 + (inventory_item_position * 25.0)), 0.0)?;
        inventory_item_position += 1.0;
    }

    Ok(())
}