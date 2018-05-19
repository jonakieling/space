use misc::{Direction};
use storage::{SelectionStorage, Tree};
use dialog::DialogItem;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum WallType {
	Wall,
	Corner,
	Edge
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Wall {
	pub wall_type: WallType,
	pub face: Direction
}

impl Wall {
    pub fn tile(&self) -> &'static str {
		let image_src;
		match self.wall_type {
			WallType::Corner => {
				image_src = "/corner.png";
			},
			WallType::Edge => {
				image_src = "/edge.png";
			},
			_ => {
				image_src = "/wall.png";
			}
		}

		image_src
	}
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Floor { }

impl Floor {
    pub fn tile(&self) -> &'static str {
        "/floor.png"
	}
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum DoorStatus {
    Open,
    Closed
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Door {
    pub status: DoorStatus,
	pub face: Direction
}

impl Door {
    pub fn tile(&self) -> &'static str {
		let image_src;
		match self.status {
			DoorStatus::Open => {
				image_src = "/door-open.png";
			},
			DoorStatus::Closed => {
				image_src = "/door.png";
			}
		}

        image_src
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Terminal {
    pub text: Box<String>,
    pub front: Direction,
}

impl Terminal {
    pub fn tile(&self) -> &'static str {
        "/terminal.png"
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PilotSeat {
    pub front: Direction,
}

impl PilotSeat {
    pub fn tile(&self) -> &'static str {
		"/pilot-seat.png"
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Circuitry {
    pub parts: SelectionStorage<Item>,
    pub powered: bool
}

impl Circuitry {
    pub fn tile(&self) -> &'static str {
	    "/circuitry.png"
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Generator {
	pub face: Direction
}

impl Generator {
    pub fn tile(&self) -> &'static str {
	    "/generator.png"
	}
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Item {
    Log,
    Terminal,
    Communicator,
    Scanner,
    PowerConductor,
	DataChip,
	MicroController
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
pub struct Npc {
	pub name: String,
    pub direction: Direction,
    pub look_at: Direction,
    pub dialog: Tree<DialogItem>,
    pub inventory: SelectionStorage<Item>,
}

impl Npc {
    pub fn tile(&self) -> &'static str {
		let image_src;
		match self.direction {
			Direction::Up => {
				image_src = "/gnoerf-back.png";
			},
			Direction::Down => {
				image_src = "/gnoerf-front.png";
			},
			Direction::Left => {
				image_src = "/gnoerf-left.png";
			},
			Direction::Right => {
				image_src = "/gnoerf-right.png";
			}
		}

		image_src
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Storage {
	pub content: SelectionStorage<Item>,
	pub face:Direction
}

impl Storage {
    pub fn tile(&self) -> &'static str {
		let image_src;
		match self.face {
			Direction::Up => {
				image_src = "/storage.png";
			},
			Direction::Down => {
				image_src = "/storage-front.png";
			},
			Direction::Left => {
				image_src = "/storage.png";
			},
			Direction::Right => {
				image_src = "/storage.png";
			}
		}
		
		image_src
	}
}
