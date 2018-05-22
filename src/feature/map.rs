use ggez::{Context, GameResult, graphics};
use ggez::graphics::get_screen_coordinates;
use ggez::event::{Keycode, Mod};

use world::WorldData;
use misc::{Position, Direction};
use game::{InputState, GameState};
use constants::GRID_SIZE;
use app::*;
use storage::SelectionStorage;
use objects::Location;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum MapFeature {
    View,
    Navigate
}

pub struct Handler {
    cursor: Position,
    change_state: Option<InputState>,
    map_selection: SelectionStorage<String>,
    feature: MapFeature
}

impl Handler {
    pub fn new(feature: MapFeature, data: &mut WorldData) -> Handler {
        data.overlay = true;
        let mut cursor = Position {x: 0, y: 0};

        match data.player_location {
            Location::Ship(ref ship_id) => {
                for ship in data.ships.iter_mut() {
                    if &ship.id == ship_id {
                        cursor = ship.position;
                    }
                }
            },
            Location::Station(ref station_id) => {
                for station in data.stations.iter_mut() {
                    if &station.id == station_id {
                        cursor = station.position;
                    }
                }
            }
        }

    	let mut handler = Handler {
            cursor,
            change_state: None,
            map_selection: SelectionStorage::new(),
            feature
        };

        let map_selection = handler.get_edit_selection(data);
        handler.map_selection = map_selection;

        handler
    }

    fn get_edit_selection(&mut self, data: &mut WorldData) -> SelectionStorage<String> {
        let mut selection_storage: SelectionStorage<String> = SelectionStorage::new();
        selection_storage.insert(self.cursor.to_string());
        selection_storage.insert("".to_string());
        for sector in data.sectors.iter() {
            if sector.position == self.cursor {
                selection_storage.insert(sector.id.clone());
            }
        }
        for station in data.stations.iter() {
            if station.position == self.cursor {
                selection_storage.insert(station.id.clone());
                if let Location::Station(ref station_id) = data.player_location {
                    if &station.id == station_id {
                        selection_storage.insert("Player".to_string());
                    }
                }
            }
        }
        for ship in data.ships.iter() {
            if ship.position == self.cursor {
                selection_storage.insert(ship.id.clone());
                if let Location::Ship(ref ship_id) = data.player_location {
                    if &ship.id == ship_id {
                        selection_storage.insert("Player".to_string());
                    }
                }
            }
        }

        selection_storage
    }
}

impl GameState for Handler {

    fn change_state(&mut self, _ctx: &mut Context, data: &mut WorldData) -> Option<Box<GameState>> {
        match self.change_state {
            Some(InputState::World) => {
                self.change_state = None;
                data.overlay = false;
                Some(Box::new(super::world::Handler::new()))
            },
            _ => None,
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, scene_data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => {
                self.change_state = Some(InputState::World);
            },
            Keycode::Left => {
                self.cursor = &self.cursor + &Direction::Left.value();
                self.map_selection = self.get_edit_selection(scene_data);
            },
            Keycode::Right => {
                self.cursor = &self.cursor + &Direction::Right.value();
                self.map_selection = self.get_edit_selection(scene_data);
            },
            Keycode::Up => {
                self.cursor = &self.cursor + &Direction::Up.value();
                self.map_selection = self.get_edit_selection(scene_data);
            },
            Keycode::Down => {
                self.cursor = &self.cursor + &Direction::Down.value();
                self.map_selection = self.get_edit_selection(scene_data);
            },
            Keycode::Return => {
                if self.feature == MapFeature::Navigate {
                    if let Location::Ship(ref ship_id) = scene_data.player_location {
                        for ship in scene_data.ships.iter_mut() {
                            if &ship.id == ship_id {
                                ship.position = self.cursor;
                            }
                        }
                    }
                    self.map_selection = self.get_edit_selection(scene_data);
                }
            }
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        data.camera = self.cursor;

        match self.feature {
            MapFeature::View => draw_input_state("Map", ctx)?,
            MapFeature::Navigate => draw_input_state("Navigation", ctx)?,
        }

        graphics::set_color(ctx, graphics::WHITE)?;

        draw_selection(&self.map_selection, ctx, false, false)?;

        for sector in data.sectors.iter() {
            let p = get_tile_params(ctx, sector.position, data.camera, None);
            add_sprite(&mut data.sprites, SpriteId::MapSector, p);
        }
        for station in data.stations.iter() {
            let p = get_tile_params(ctx, station.position, data.camera, None);
            add_sprite(&mut data.sprites, SpriteId::MapStation, p);
        }
        for ship in data.ships.iter() {
            let p = get_tile_params(ctx, ship.position, data.camera, None);
            add_sprite(&mut data.sprites, SpriteId::MapShip, p);
        }
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::MapSector)?;
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::MapShip)?;
        draw_spritebatch(ctx, &mut data.sprites, SpriteId::MapStation)?;

        graphics::set_color(ctx, graphics::Color{r: 0.2, g: 0.8, b: 0.2, a: 1.0,})?;
        let viewport_pos = self.cursor.viewport(data.camera);
        let sceen_horizontal_center = get_screen_coordinates(ctx).w / 2.0 - (GRID_SIZE / 2) as f32;
        let sceen_vertical_center = get_screen_coordinates(ctx).h / 2.0 - (GRID_SIZE / 2) as f32;
        let cursor = graphics::Rect::new(
            viewport_pos.x as f32 + sceen_horizontal_center,
            viewport_pos.y as f32 + sceen_vertical_center,
            GRID_SIZE as f32,
            GRID_SIZE as f32
        );
        graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), cursor)?;

        Ok(())
    }
}