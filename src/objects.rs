use misc::{Direction};
use storage::{SelectionStorage, Node};
use dialog::DialogItem;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Object {
	Terminal,
	Door
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WallType {
	Wall,
	Corner,
	Edge,
	Window
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Wall {
	pub variant: WallType,
	pub face: Direction
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DecorationType {
	Display,
	Panel
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Decoration {
	pub variant: DecorationType,
	pub face: Direction
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FloorType {
	Regular,
	Light
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Floor {
	pub variant: FloorType
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum DoorStatus {
    Open,
    Closed
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub enum Location {
	Ship(String),
	Station(String),
	Planet(String),
	Space
}

impl ToString for Location {
    fn to_string(&self) -> String {
		match self {
			Location::Ship(id) => id.clone(),
			Location::Station(id) => id.clone(),
			Location::Planet(id) => id.clone(),
			Location::Space => "Space".to_string()
		}
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum DoorType {
	Passage,
	Exit(Location)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Door {
    pub status: DoorStatus,
	pub variant: DoorType,
	pub face: Direction
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum TerminalType {
	ShipConsole,
	Intercomm,
	Hud
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Terminal {
	pub variant: TerminalType,
    pub dialog: Node<DialogItem>,
    pub front: Direction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PilotSeat {
    pub front: Direction,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum CircuitryType {
	Powered,
	Inactive
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Circuitry {
    pub parts: SelectionStorage<Item>,
    pub variant: CircuitryType
}

impl Circuitry {
    pub fn tile(&self) -> &'static str {
		match self.variant {
			CircuitryType::Inactive => "/circuitry-inactive.png",
			CircuitryType::Powered => "/circuitry.png",
		}
	}

	pub fn contains(&self, needle: Item) -> bool {
		self.parts.iter().find(|&&item| (item == needle)).is_some()
	}

	pub fn powered(&self) -> bool {
		self.variant == CircuitryType::Powered
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Generator {
	pub face: Direction
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Item {
	PowerConductor,
	Navcomp
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Receipe {
	pub result: Item,
	pub incredients: Vec<Item>
}

impl Receipe {
	pub fn _receipes_as_incredient<'a>(item: &Item, receipes: &'a Vec<Receipe>) -> Vec<&'a Receipe> {
		receipes.iter().filter(|receipe| receipe.incredients.contains(item)).collect()
	}
	
	pub fn _from_item<'a>(item: &Item, receipes: &'a Vec<Receipe>) -> Option<&'a Receipe> {
		receipes.iter().filter(|receipe| receipe.result == *item).next()
	}

	pub fn receipe_match<'a>(items: &'a Vec<Item>, receipes: &'a Vec<Receipe>) -> Vec<&'a Receipe> {
		receipes.iter().filter(|receipe| {
			receipe.incredients.iter().all(|incredient| {
				items.contains(incredient)
			})
		}).collect()
	}
}

impl ToString for Item {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NpcType {
	Gnoerf,
	Guard
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Npc {
	pub name: String,
	pub variant: NpcType,
    pub direction: Direction,
    pub look_at: Direction,
    pub dialog: Node<DialogItem>,
    pub inventory: SelectionStorage<Item>,
}

impl Npc {
    pub fn tile(&self) -> &'static str {
		let image_src;
		match self.variant {
			NpcType::Gnoerf => {
				match self.direction {
					Direction::Up => image_src = "/gnoerf-back.png",
					Direction::Down => image_src = "/gnoerf-front.png",
					Direction::Left => image_src = "/gnoerf-left.png",
					Direction::Right => image_src = "/gnoerf-right.png"
				}
			},
			NpcType::Guard => {
				match self.direction {
					Direction::Up => image_src = "/guard-back.png",
					Direction::Down => image_src = "/guard-front.png",
					Direction::Left => image_src = "/guard-left.png",
					Direction::Right => image_src = "/guard-right.png"
				}
			},
		}

		image_src
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Storage {
	pub content: SelectionStorage<Item>,
	pub face:Direction
}

