use std::time::Duration;
use std::collections::{VecDeque, BTreeSet, HashMap};
use std::f32::consts::{PI, FRAC_PI_2};

use ggez::timer::get_delta;
use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::graphics::{spritebatch::SpriteBatch, get_screen_coordinates};
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
    NpcTrade,
    Storage
}

pub struct SceneData {
    pub movement_timer: Duration,
    pub backdrop: String,
    pub player: Player,
    pub walls: PositionLevelStorage<Wall>,
    pub floor: PositionLevelStorage<Floor>,
    pub doors: PositionLevelStorage<Door>,
    pub terminals: PositionLevelStorage<Terminal>,
    pub decorations: PositionLevelStorage<Decoration>,
    pub circuitry: PositionLevelStorage<Circuitry>,
    pub generators: PositionLevelStorage<Generator>,
    pub pilot_seats: PositionLevelStorage<PilotSeat>,
    pub npc: PositionLevelStorage<Npc>,
    pub storages: PositionLevelStorage<Storage>,
    pub receipes: Vec<Receipe>,
    pub dialog: Node<DialogItem>,
    pub insight_view: bool,
    pub main_menu: bool,
    pub sprites: HashMap<SpriteId, SpriteBatch>,
    pub camera: Position
}

pub struct Scene {
    pub current_ingame_state: Box<GameState>,
    pub data: SceneData
}

impl AppState for Scene {
    fn change_state(&self, _ctx: &mut Context) -> Option<Box<AppState>> {
        if self.data.main_menu {
            save_scene(&self.data, "saves/auto-save.tar");
            let menu = super::menu::Scene::new().unwrap();
            Some(Box::new(menu))
        } else {
            None
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum SpriteId {
    Wall,
    Corner,
    Edge,
    Window,
    Floor(FloorType),
    Circuitry,
    Door(DoorStatus),
    Terminal(TerminalType),
    PilotSeat,
    Storage,
    Generator,
    Decoration(DecorationType)
}

impl Scene {
    pub fn new(ctx: &mut Context) -> GameResult<Scene> {
        
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
                variant: TerminalType::Intercomm,
                dialog: Node::new(),
                front: Direction::Down
            }),
            log: SelectionStorage::new()
        };

        let walls = <PositionLevelStorage<Wall>>::new();
        let floor = <PositionLevelStorage<Floor>>::new();
        let doors = <PositionLevelStorage<Door>>::new();
        let terminals = <PositionLevelStorage<Terminal>>::new();
        let decorations = <PositionLevelStorage<Decoration>>::new();
        let circuitry = <PositionLevelStorage<Circuitry>>::new();
        let generators = <PositionLevelStorage<Generator>>::new();
        let pilot_seats = <PositionLevelStorage<PilotSeat>>::new();
        let npc = <PositionLevelStorage<Npc>>::new();
        let storages = <PositionLevelStorage<Storage>>::new();
        
        let receipes = Vec::new();

        let mut sprites = HashMap::new();
        sprites.insert(SpriteId::Wall, SpriteBatch::new(graphics::Image::new(ctx, "/wall.png").unwrap()));
        sprites.insert(SpriteId::Corner, SpriteBatch::new(graphics::Image::new(ctx, "/corner.png").unwrap()));
        sprites.insert(SpriteId::Edge, SpriteBatch::new(graphics::Image::new(ctx, "/edge.png").unwrap()));
        sprites.insert(SpriteId::Window, SpriteBatch::new(graphics::Image::new(ctx, "/window.png").unwrap()));
        sprites.insert(SpriteId::Floor(FloorType::Regular), SpriteBatch::new(graphics::Image::new(ctx, "/floor.png").unwrap()));
        sprites.insert(SpriteId::Floor(FloorType::Light), SpriteBatch::new(graphics::Image::new(ctx, "/floor-light.png").unwrap()));
        sprites.insert(SpriteId::Circuitry, SpriteBatch::new(graphics::Image::new(ctx, "/circuitry.png").unwrap()));
        sprites.insert(SpriteId::Door(DoorStatus::Closed), SpriteBatch::new(graphics::Image::new(ctx, "/door.png").unwrap()));
        sprites.insert(SpriteId::Door(DoorStatus::Open), SpriteBatch::new(graphics::Image::new(ctx, "/door-open.png").unwrap()));
        sprites.insert(SpriteId::Terminal(TerminalType::Intercomm), SpriteBatch::new(graphics::Image::new(ctx, "/terminal.png").unwrap()));
        sprites.insert(SpriteId::Terminal(TerminalType::ShipConsole), SpriteBatch::new(graphics::Image::new(ctx, "/ship-console.png").unwrap()));
        sprites.insert(SpriteId::PilotSeat, SpriteBatch::new(graphics::Image::new(ctx, "/pilot-seat.png").unwrap()));
        sprites.insert(SpriteId::Storage, SpriteBatch::new(graphics::Image::new(ctx, "/storage.png").unwrap()));
        sprites.insert(SpriteId::Generator, SpriteBatch::new(graphics::Image::new(ctx, "/generator.png").unwrap()));

        let mut data = SceneData {
            movement_timer: Duration::from_millis(0),
            backdrop: String::from(""),
            player,
            walls,
            floor,
            doors,
            terminals,
            decorations,
            circuitry,
            generators,
            pilot_seats,
            npc,
            storages,
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
            main_menu: false,
            sprites,
            camera: Position { x: 0, y: 0}
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
    pub fn clear(&mut self) {
        self.walls.clear();
        self.doors.clear();
        self.terminals.clear();
        self.circuitry.clear();
        self.generators.clear();
        self.storages.clear();
    }

    pub fn check_player_collision(&self, direction: &Direction) -> bool {
        let mut found_collision = false;
        let collision_tile = &self.player.position + &direction.value();

        if let Some(_) = self.walls.get(collision_tile) {
            found_collision = true;
        }

        if let Some(_) = self.terminals.get(collision_tile) {
            found_collision = true;
        }

        if let Some(_) = self.generators.get(collision_tile) {
            found_collision = true;
        }

        if let Some(_) = self.storages.get(collision_tile) {
            found_collision = true;
        }

        if let Some(_) = self.npc.get(collision_tile) {
            found_collision = true;
        }

        if let Some(door) = self.doors.get(collision_tile) {
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

    pub fn current_storage(&mut self) -> Option<&mut Storage>{
        if let Some(current_storage) = self.storages.get_mut(self.player.front_tile) {
            Some(current_storage)
        } else {
            None
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

        if let Some(state) = self.current_ingame_state.change_state(&mut self.data) {
            self.current_ingame_state = state;
        }

        self.data.movement_timer += get_delta(ctx);

        if self.data.movement_timer > Duration::from_millis(MOVEMENT_DURATION) {
            if let Some(&current_movement) = self.data.player.movement.last() {
                if !self.data.check_player_collision(&current_movement) {
                    self.data.movement_timer = Duration::from_millis(0);
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

        save_scene(&self.data, "saves/auto-save.tar");

        false
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.data.camera = self.data.player.position;

        graphics::clear(ctx);

        if self.data.backdrop != "" {
            graphics::set_color(ctx, graphics::Color{r: 1.0, g: 1.0, b: 1.0, a: 0.25})?;
            let mut backdrop = graphics::Image::new(ctx, &self.data.backdrop)?;
            backdrop.set_filter(graphics::FilterMode::Nearest);

            // this is a convention for levels now (got stuck when setting up static levels via functions)
            let backdrop_pos = Position {
                x: 1,
                y: 1
            };
            let mut p = get_tile_params(ctx, backdrop_pos.to_int(), self.data.camera, None);
            // override with grid size scaling since backdrops are smaller scale (1 pixel = 1 tile)
            p.scale = graphics::Point2::new(GRID_SIZE as f32, GRID_SIZE as f32);
            graphics::draw_ex(
                ctx,
                &backdrop,
                p,
            )?;
        }

        graphics::set_color(ctx, graphics::BLACK)?;

        for (pos, item) in self.data.floor.iter().enumerate() {
            if let Some(floor) = item {
                let p = get_tile_params(ctx, pos as i32, self.data.camera, None);
                match floor.variant {
                    FloorType::Regular => self.data.sprites.get(&SpriteId::Floor(FloorType::Regular)).unwrap().add(p),
                    FloorType::Light => self.data.sprites.get(&SpriteId::Floor(FloorType::Light)).unwrap().add(p)
                };
            }
        }
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Floor(FloorType::Regular)).unwrap())?;
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Floor(FloorType::Light)).unwrap())?;

        for (pos, item) in self.data.walls.iter().enumerate() {
            if let Some(wall) = item {
                let p = get_tile_params(ctx, pos as i32, self.data.camera, Some(wall.face));
                match wall.variant {
                    WallType::Wall => self.data.sprites.get(&SpriteId::Wall).unwrap().add(p),
                    WallType::Corner => self.data.sprites.get(&SpriteId::Corner).unwrap().add(p),
                    WallType::Edge => self.data.sprites.get(&SpriteId::Edge).unwrap().add(p),
                    WallType::Window => self.data.sprites.get(&SpriteId::Window).unwrap().add(p),
                };
            }
        }
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Wall).unwrap())?;
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Corner).unwrap())?;
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Edge).unwrap())?;
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Window).unwrap())?;

        for (pos, terminal) in self.data.terminals.iter().enumerate() {
            if let Some(current_terminal) = terminal {
                let p = get_tile_params(ctx, pos as i32, self.data.camera, Some(current_terminal.front));
                match current_terminal.variant {
                    TerminalType::Intercomm => {
                        self.data.sprites.get(&SpriteId::Terminal(TerminalType::Intercomm)).unwrap().add(p);
                    },
                    TerminalType::ShipConsole => {
                        self.data.sprites.get(&SpriteId::Terminal(TerminalType::ShipConsole)).unwrap().add(p);
                    },
                    TerminalType::Hud => ()
                };
            }
        }
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Terminal(TerminalType::Intercomm)).unwrap())?;
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Terminal(TerminalType::ShipConsole)).unwrap())?;

        for (pos, item) in self.data.pilot_seats.iter().enumerate() {
            if let Some(pilot_seat) = item {
                let p = get_tile_params(ctx, pos as i32, self.data.camera, Some(pilot_seat.front));
                self.data.sprites.get(&SpriteId::PilotSeat).unwrap().add(p);
            }
        }
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::PilotSeat).unwrap())?;

        for (pos, item) in self.data.doors.iter().enumerate() {
            if let Some(door) = item {
                let p = get_tile_params(ctx, pos as i32, self.data.camera, Some(door.face));
                match door.status {
                    DoorStatus::Open => self.data.sprites.get(&SpriteId::Door(DoorStatus::Open)).unwrap().add(p),
                    DoorStatus::Closed => self.data.sprites.get(&SpriteId::Door(DoorStatus::Closed)).unwrap().add(p)
                };
            }
        }
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Door(DoorStatus::Closed)).unwrap())?;
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Door(DoorStatus::Open)).unwrap())?;

        for (pos, item) in self.data.generators.iter().enumerate() {
            if item.is_some() {
                let params = get_tile_params(ctx, pos as i32, self.data.camera, None);
                self.data.sprites.get(&SpriteId::Generator).unwrap().add(params);
            }
        }
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Generator).unwrap())?;

        for (pos, item) in self.data.storages.iter().enumerate() {
            if item.is_some() {
                let params = get_tile_params(ctx, pos as i32, self.data.camera, None);
                self.data.sprites.get(&SpriteId::Storage).unwrap().add(params);
            }
        }
        draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Storage).unwrap())?;

        if self.data.insight_view {
            for (pos, item) in self.data.circuitry.iter().enumerate() {
                if item.is_some() {
                    let params = get_tile_params(ctx, pos as i32, self.data.camera, None);
                    self.data.sprites.get(&SpriteId::Circuitry).unwrap().add(params);
                }
            }
            draw_spritebatch(ctx, &mut self.data.sprites.get(&SpriteId::Circuitry).unwrap())?;
        }

        for (pos, npc) in self.data.npc.iter().enumerate() {
            if let Some(npc) = npc {
                draw_tile(ctx, npc.tile(), pos as i32, self.data.camera, None)?;
            }
        }

        draw_tile(ctx, self.data.player.tile(), self.data.player.position.to_int(), self.data.camera, None)?;

        self.current_ingame_state.draw(&mut self.data, ctx)?;

        graphics::present(ctx);

        Ok(())
    }
}

pub fn draw_spritebatch(ctx: &mut Context, spritebatch: &mut SpriteBatch) -> GameResult<()> {
    graphics::set_color(ctx, graphics::WHITE)?;
    let params = graphics::DrawParam {
        dest: graphics::Point2::new(0.0, 0.0),
        ..Default::default()
    };
    graphics::draw_ex(ctx, spritebatch, params)?;
    spritebatch.clear();

    Ok(())
}

pub fn get_tile_params(ctx: &mut Context, pos: i32, camera: Position, direction: Option<Direction>) -> graphics::DrawParam {
    let pos = Position {
        x: pos % LEVEL_SIZE,
        y: pos / LEVEL_SIZE
    };

    let viewport_pos = pos.viewport(camera);

    let sceen_horizontal_center = get_screen_coordinates(ctx).w / 2.0 - (GRID_SIZE / 2) as f32;
    let sceen_vertical_center = get_screen_coordinates(ctx).h / 2.0 - (GRID_SIZE / 2) as f32;
    let dst = graphics::Point2::new(viewport_pos.x as f32 + sceen_horizontal_center, viewport_pos.y as f32 + sceen_vertical_center);

    let mut tile_dst = dst;
    let rotation;
    match direction {
        Some(Direction::Up) => {
            rotation = PI;
            tile_dst = graphics::Point2::new(dst.x + GRID_SIZE as f32, dst.y + GRID_SIZE as f32);
        },
        Some(Direction::Down) => {
            rotation = 0.0;
        },
        Some(Direction::Left) => {
            rotation = FRAC_PI_2;
            tile_dst = graphics::Point2::new(tile_dst.x + GRID_SIZE as f32, tile_dst.y);
        },
        Some(Direction::Right) => {
            rotation = 3.0 * FRAC_PI_2;
            tile_dst = graphics::Point2::new(tile_dst.x, tile_dst.y + GRID_SIZE as f32);
        },
        _ => {
            rotation = 0.0;
        }
    }

    graphics::DrawParam {
        dest: tile_dst,
        rotation: rotation,
        scale: graphics::Point2::new(PIXEL_SCALE as f32, PIXEL_SCALE as f32),
        ..Default::default()
    }
}

pub fn draw_tile(ctx: &mut Context, tile_src: &str, pos: i32, camera: Position, direction: Option<Direction>) -> GameResult<()> {
		
    graphics::set_color(ctx, graphics::WHITE)?;
    let mut storage_image = graphics::Image::new(ctx, tile_src)?;
    storage_image.set_filter(graphics::FilterMode::Nearest);

    let params = get_tile_params(ctx, pos, camera, direction);
    
    graphics::draw_ex(
        ctx,
        &storage_image,
        params,
    )?;

    Ok(())
}