use std::time::Duration;
use std::collections::{VecDeque, BTreeSet, HashMap};

use ggez::Context;
use ggez::graphics::{spritebatch::SpriteBatch, Image};

use player::*;
use storage::*;
use objects::*;
use misc::*;
use app::SpriteId;
use savegame;

#[derive(Serialize, Deserialize)]
pub struct Station {
    pub id: String,
    pub position: Position
}

#[derive(Serialize, Deserialize)]
pub struct Ship {
    pub id: String,
    pub position: Position
}

#[derive(Serialize, Deserialize)]
pub struct Sector {
    pub id: String,
    pub position: Position
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Level {
    pub backdrop: String,
    pub location: Location,
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
    pub storages: PositionLevelStorage<Storage>
}

#[derive(Serialize, Deserialize)]
pub struct Universe {
    pub sectors: Vec<Sector>,
    pub stations: Vec<Station>,
    pub ships: Vec<Ship>,
    pub player_location: Location
}

pub struct WorldData {
    pub movement_timer: Duration,
    pub level: Level,
    pub universe: Universe,
    pub receipes: Vec<Receipe>,
    pub insight_view: bool,
    pub overlay: bool,
    pub sprites: HashMap<SpriteId, SpriteBatch>,
    pub levels: HashMap<Location, Level>,
    pub camera: Position
}

impl WorldData {
    pub fn new(ctx: &mut Context) -> WorldData {
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
        sprites.insert(SpriteId::Wall, SpriteBatch::new(Image::new(ctx, "/wall.png").unwrap()));
        sprites.insert(SpriteId::Corner, SpriteBatch::new(Image::new(ctx, "/corner.png").unwrap()));
        sprites.insert(SpriteId::Edge, SpriteBatch::new(Image::new(ctx, "/edge.png").unwrap()));
        sprites.insert(SpriteId::Window, SpriteBatch::new(Image::new(ctx, "/window.png").unwrap()));
        sprites.insert(SpriteId::Floor(FloorType::Regular), SpriteBatch::new(Image::new(ctx, "/floor.png").unwrap()));
        sprites.insert(SpriteId::Floor(FloorType::Light), SpriteBatch::new(Image::new(ctx, "/floor-light.png").unwrap()));
        sprites.insert(SpriteId::Circuitry(CircuitryType::Powered), SpriteBatch::new(Image::new(ctx, "/circuitry.png").unwrap()));
        sprites.insert(SpriteId::Circuitry(CircuitryType::Inactive), SpriteBatch::new(Image::new(ctx, "/circuitry-inactive.png").unwrap()));
        sprites.insert(SpriteId::Door(DoorStatus::Closed), SpriteBatch::new(Image::new(ctx, "/door.png").unwrap()));
        sprites.insert(SpriteId::Door(DoorStatus::Open), SpriteBatch::new(Image::new(ctx, "/door-open.png").unwrap()));
        sprites.insert(SpriteId::Terminal(TerminalType::Intercomm), SpriteBatch::new(Image::new(ctx, "/terminal.png").unwrap()));
        sprites.insert(SpriteId::Terminal(TerminalType::ShipConsole), SpriteBatch::new(Image::new(ctx, "/ship-console.png").unwrap()));
        sprites.insert(SpriteId::PilotSeat, SpriteBatch::new(Image::new(ctx, "/pilot-seat.png").unwrap()));
        sprites.insert(SpriteId::Storage, SpriteBatch::new(Image::new(ctx, "/storage.png").unwrap()));
        sprites.insert(SpriteId::Generator, SpriteBatch::new(Image::new(ctx, "/generator.png").unwrap()));
        sprites.insert(SpriteId::MapSector, SpriteBatch::new(Image::new(ctx, "/map-sector.png").unwrap()));
        sprites.insert(SpriteId::MapStation, SpriteBatch::new(Image::new(ctx, "/map-station.png").unwrap()));
        sprites.insert(SpriteId::MapShip, SpriteBatch::new(Image::new(ctx, "/map-ship.png").unwrap()));
        sprites.insert(SpriteId::Decoration(DecorationType::Display), SpriteBatch::new(Image::new(ctx, "/display.png").unwrap()));
        sprites.insert(SpriteId::Decoration(DecorationType::Panel), SpriteBatch::new(Image::new(ctx, "/panel.png").unwrap()));

        WorldData {
            movement_timer: Duration::from_millis(0),
            level: Level {
                backdrop: String::from(""),
                location: Location::Space,
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
                storages
            },
            universe: savegame::static_levels::default_universe(),
            receipes,
            insight_view: false,
            overlay: false,
            sprites,
            levels: HashMap::new(),
            camera: Position { x: 0, y: 0}
        }
    }
}

impl Level {
    pub fn clear(&mut self) {
        self.walls.clear();
        self.floor.clear();
        self.doors.clear();
        self.terminals.clear();
        self.circuitry.clear();
        self.generators.clear();
        self.storages.clear();
        self.npc.clear();
        self.pilot_seats.clear();
        self.decorations.clear();
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

    pub fn reset_powert(&mut self) {
        for circuitry in self.circuitry.iter_mut() {
            if let &mut Some(ref mut circuitry) = circuitry {
                circuitry.variant = CircuitryType::Inactive;
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

                    if let Some(circuitry) = self.circuitry.get(root) {
                        if circuitry.contains(Item::PowerConductor) {
                            open_set.push_back(root);
                        }
                    }

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
                                if let Some(circuitry) = self.circuitry.get(neighbor) {
                                    if circuitry.contains(Item::PowerConductor) {
                                        open_set.push_back(neighbor);
                                    }
                                }
                            }
                        }

                        if let Some(circuitry) = self.circuitry.get(subtree_root_position) {
                            if circuitry.contains(Item::PowerConductor) {
                                closed_set.insert(subtree_root);
                            }
                        }
                    }
                }

                for pos in closed_set {
                    if let Some(ref mut circuitry) = self.circuitry.get_mut(pos.unwrap()) {
                        circuitry.variant = CircuitryType::Powered;
                    }
                }
            }
        }
    }

    pub fn terminal_connected(&mut self) -> SelectionStorage<(Position, Object)> {
        let mut connected =  SelectionStorage::new();

        if let Some(_) = self.current_terminal() {
            let mut open_set = VecDeque::new();
            let mut closed_set: BTreeSet<Option<Position>> = BTreeSet::new();

            {
                let mut root = self.player.front_tile;
                
                if let Some(circuitry) = self.circuitry.get(root) {
                        if circuitry.contains(Item::PowerConductor) {
                            open_set.push_back(root);
                        }
                    }

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
                            if let Some(circuitry) = self.circuitry.get(neighbor) {
                                if circuitry.contains(Item::PowerConductor) {
                                    open_set.push_back(neighbor);
                                }
                            }
                        }
                    }

                    if let Some(circuitry) = self.circuitry.get(subtree_root_position) {
                        if circuitry.contains(Item::PowerConductor) {
                            closed_set.insert(subtree_root);
                            if let Some(pos) = subtree_root {
                                if pos != self.player.front_tile {
                                    if let Some(_) = self.terminals.get(pos) {
                                        connected.insert((pos, Object::Terminal));
                                    }
                                    if let Some(_) = self.doors.get(pos) {
                                        connected.insert((pos, Object::Door));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        connected
    }

    pub fn current_storage(&mut self) -> Option<&mut Storage> {
        if let Some(current_storage) = self.storages.get_mut(self.player.front_tile) {
            Some(current_storage)
        } else {
            None
        }
    }

    pub fn current_circuitry(&mut self) -> Option<&mut Circuitry> {
        if let Some(current_circuitry) = self.circuitry.get_mut(self.player.front_tile) {
            Some(current_circuitry)
        } else {
            None
        }
    }

    pub fn current_terminal(&mut self) -> Option<&mut Terminal> {
        if let Some(current_terminal) = self.terminals.get_mut(self.player.front_tile) {
            Some(current_terminal)
        } else {
            None
        }
    }

    pub fn current_npc(&mut self) -> Option<&mut Npc> {
        if let Some(current_npc) = self.npc.get_mut(self.player.front_tile) {
            Some(current_npc)
        } else {
            None
        }
    }
}