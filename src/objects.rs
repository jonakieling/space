use storage::SelectionStorage;
use misc::Direction;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Wall {
    
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum DoorStatus {
    Open,
    Closed
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Door {
    pub status: DoorStatus
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Terminal {
    pub text: Box<String>,
    pub front: Direction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Circuitry {
	pub parts: SelectionStorage<Item>
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Item {
    Log,
    PilotLicense,
    Terminal,
    Communicator,
    Chip,
    Cable,
    Isolation,
    Adapter,
}