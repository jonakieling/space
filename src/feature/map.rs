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

#[derive(PartialEq, Eq)]
pub enum Mode {
    Universe,
    Sector,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum MapFeature {
    View,
    Navigate
}

pub struct Handler {
    cursor: Position,
    change_state: Option<InputState>,
    map_info_selection: SelectionStorage<String>,
    map_selection: SelectionStorage<Location>,
    feature: MapFeature,
    mode: Mode
}

impl Handler {
    pub fn new(feature: MapFeature, data: &mut WorldData) -> Handler {
        data.overlay = true;
        let mut cursor = Position {x: 0, y: 0};

        match data.universe.player_location {
            Location::Ship(ref ship_id) => {
                for ship in data.universe.ships.iter_mut() {
                    if &ship.id == ship_id {
                        cursor = ship.position;
                    }
                }
            },
            Location::Station(ref station_id) => {
                for station in data.universe.stations.iter_mut() {
                    if &station.id == station_id {
                        cursor = station.position;
                    }
                }
            },
            Location::Space => { },
            Location::Planet(_) => { }
        }

    	let mut handler = Handler {
            cursor,
            change_state: None,
            map_selection: SelectionStorage::new(),
            map_info_selection: SelectionStorage::new(),
            feature,
            mode: Mode::Universe
        };

        let map_selection = handler.get_map_selection(data);
        handler.map_selection = map_selection;
        let map_info_selection = handler.get_map_info_selection(data);
        handler.map_info_selection = map_info_selection;

        handler
    }

    fn update_selections(&mut self, data: &mut WorldData) {
        self.map_info_selection = self.get_map_info_selection(data);
        self.map_selection = self.get_map_selection(data);
    }

    fn get_map_info_selection(&mut self, data: &mut WorldData) -> SelectionStorage<String> {
        let mut selection_storage: SelectionStorage<String> = SelectionStorage::new();
        selection_storage.insert(self.cursor.to_string());
        selection_storage.insert("-".to_string());
        for sector in data.universe.sectors.iter() {
            if sector.position == self.cursor {
                selection_storage.insert(sector.id.clone());
                selection_storage.insert("-".to_string());
            }
        }
        let mut player_present = false;
        for station in data.universe.stations.iter() {
            if station.position == self.cursor {
                selection_storage.insert(station.id.clone());
                if let Location::Station(ref station_id) = data.universe.player_location {
                    if &station.id == station_id {
                        player_present = true;
                    }
                }
            }
        }
        for ship in data.universe.ships.iter() {
            if ship.position == self.cursor {
                selection_storage.insert(ship.id.clone());
                if let Location::Ship(ref ship_id) = data.universe.player_location {
                    if &ship.id == ship_id {
                        player_present = true;
                    }
                }
            }
        }

        if player_present {
            selection_storage.insert("-".to_string());
            selection_storage.insert("Player".to_string());
        }

        selection_storage
    }

    fn get_map_selection(&mut self, data: &mut WorldData) -> SelectionStorage<Location> {
        let mut selection_storage: SelectionStorage<Location> = SelectionStorage::new();
        selection_storage.insert(Location::Space);
        for station in data.universe.stations.iter() {
            if station.position == self.cursor {
                selection_storage.insert(Location::Station(station.id.clone()));
            }
        }
        for ship in data.universe.ships.iter() {
            if ship.position == self.cursor {
                selection_storage.insert(Location::Ship(ship.id.clone()));
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

    fn key_up_event(&mut self, _ctx: &mut Context, data: &mut WorldData, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match self.mode {
            Mode::Sector => {
                match keycode {
                    Keycode::Escape => {
                        self.mode = Mode::Universe;
                    },
                    Keycode::Up => {
                        self.map_selection.prev();
                    },
                    Keycode::Down => {
                        self.map_selection.next();
                    },
                    Keycode::Return => {
                        if self.feature == MapFeature::Navigate {
                            if let Location::Ship(ref ship_id) = data.universe.player_location {
                                for ship in data.universe.ships.iter_mut() {
                                    if &ship.id == ship_id {
                                        ship.position = self.cursor;
                                    }
                                }
                            }
                            self.update_selections(data);
                        }
                    },
                    _ => ()
                }
            },
            Mode::Universe => {
                match keycode {
                    Keycode::Escape => {
                        self.change_state = Some(InputState::World);
                    },
                    Keycode::Left => {
                        self.cursor = &self.cursor + &Direction::Left.value();
                        self.update_selections(data);
                    },
                    Keycode::Right => {
                        self.cursor = &self.cursor + &Direction::Right.value();
                        self.update_selections(data);
                    },
                    Keycode::Up => {
                        self.cursor = &self.cursor + &Direction::Up.value();
                        self.update_selections(data);
                    },
                    Keycode::Down => {
                        self.cursor = &self.cursor + &Direction::Down.value();
                        self.update_selections(data);
                    },
                    Keycode::Return => {
                        match self.feature {
                            MapFeature::Navigate => {
                                let mut enter_sector = false;
                                for ref sector in data.universe.sectors.iter() {
                                    if sector.position == self.cursor {
                                        self.mode = Mode::Sector;
                                        enter_sector = true;
                                    }
                                }
                                if self.feature == MapFeature::Navigate && !enter_sector {
                                    if let Location::Ship(ref ship_id) = data.universe.player_location {
                                        for ship in data.universe.ships.iter_mut() {
                                            if &ship.id == ship_id {
                                                ship.position = self.cursor;
                                            }
                                        }
                                    }
                                    self.update_selections(data);
                                }
                            },
                            MapFeature::View => {
                                for ref sector in data.universe.sectors.iter() {
                                    if sector.position == self.cursor {
                                        self.mode = Mode::Sector;
                                    }
                                }
                            },
                        }
                    }
                    _ => ()
                }
            },
        }
    }

    fn draw(&mut self, ctx: &mut Context, data: &mut WorldData) -> GameResult<()> {
        data.camera = self.cursor;

        draw_backdrop(ctx, &data.backdrops, &BackdropId::MapSector)?;

        match self.mode {
            Mode::Sector => {
                draw_backdrop(ctx, &data.backdrops, &BackdropId::MapSector)?;

                if data.universe.has_stations(&self.cursor) {
                    draw_backdrop(ctx, &data.backdrops, &BackdropId::MapStation)?;
                }

                if data.universe.has_planets(&self.cursor) {
                    draw_backdrop(ctx, &data.backdrops, &BackdropId::MapPlanet)?;
                }
                match self.feature {
                    MapFeature::Navigate => {
                        let sector_description = format!("Navigation Sector {}", self.cursor.to_string());
                        draw_input_state(&sector_description, ctx)?
                    },
                    MapFeature::View => {
                        let sector_description = format!("Map Sector {}", self.cursor.to_string());
                        draw_input_state(&sector_description, ctx)?
                    },
                }
                draw_selection(&self.map_selection, ctx, false, false)?;
            },
            Mode::Universe => {
                match self.feature {
                    MapFeature::Navigate => {
                        draw_input_state("Navigation Universe", ctx)?;
                    },
                    MapFeature::View => {
                        draw_input_state("Map Universe", ctx)?;
                    },
                }

                graphics::set_color(ctx, graphics::WHITE)?;
                draw_selection(&self.map_info_selection, ctx, false, false)?;

                for sector in data.universe.sectors.iter() {
                    let p = get_tile_params(ctx, sector.position, data.camera, None);
                    add_sprite(&mut data.sprites, &SpriteId::MapSector, p);
                }
                for station in data.universe.stations.iter() {
                    let p = get_tile_params(ctx, station.position, data.camera, None);
                    add_sprite(&mut data.sprites, &SpriteId::MapStation, p);
                }
                for ship in data.universe.ships.iter() {
                    let p = get_tile_params(ctx, ship.position, data.camera, None);
                    add_sprite(&mut data.sprites, &SpriteId::MapShip, p);
                }
                draw_spritebatch(ctx, &mut data.sprites, &SpriteId::MapSector)?;
                draw_spritebatch(ctx, &mut data.sprites, &SpriteId::MapShip)?;
                draw_spritebatch(ctx, &mut data.sprites, &SpriteId::MapStation)?;

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
            }
        }

        Ok(())
    }
}