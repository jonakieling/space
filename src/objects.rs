use ggez::GameResult;
use ggez::Context;
use ggez::graphics;

use constants::{LEVEL_SIZE, GRID_SIZE};
use misc::Direction;
use storage::SelectionStorage;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Wall { }

impl Wall {
	pub fn draw(pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE;
	    let y = pos / LEVEL_SIZE;
	    graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;

	    Ok(())
	}
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

impl Door {
    pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE;
	    let y = pos / LEVEL_SIZE;
	    match self.status {
	        DoorStatus::Open => {
	            graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
	            graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 21.0))?;
	        },
	        DoorStatus::Closed => {
	            graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,})?;
	            graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
	        },
	    }

	    Ok(())
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Terminal {
    pub text: Box<String>,
    pub front: Direction,
}

impl Terminal {
    pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE;
	    let y = pos / LEVEL_SIZE;
	    graphics::set_color(ctx, graphics::BLACK)?;
	    graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0))?;
	    graphics::set_color(ctx, graphics::Color{r: 0.5, g: 0.8, b: 0.5, a: 1.0,})?;
	    graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 21.0))?;
	    match self.front {
	        Direction::Up => {
	            let front = graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 21.0, 3.0);
	            graphics::rectangle(ctx, graphics::DrawMode::Fill, front)?;
	        },
	        Direction::Down => {
	            let front = graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32 + (self.front.value().y as f32 * 17.0), 21.0, 4.0);
	            graphics::rectangle(ctx, graphics::DrawMode::Fill, front)?;
	            
	        },
	        Direction::Right => {
	            let front = graphics::Rect::new((x * GRID_SIZE) as f32 + (self.front.value().x as f32 * 17.0), (y * GRID_SIZE) as f32 + (self.front.value().y as f32), 4.0, 21.0);
	            graphics::rectangle(ctx, graphics::DrawMode::Fill, front)?;
	            
	        },
	        Direction::Left => {
	            let front = graphics::Rect::new((x * GRID_SIZE) as f32 + (self.front.value().x as f32), (y * GRID_SIZE) as f32 + (self.front.value().y as f32), 4.0, 21.0);
	            graphics::rectangle(ctx, graphics::DrawMode::Fill, front)?;
	            
	        },
	    }

	    Ok(())
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Circuitry {
    pub parts: SelectionStorage<Item>,
    pub powered: bool
}

impl Circuitry {
    pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE;
	    let y = pos / LEVEL_SIZE;
	    graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 0.1,})?;
	    graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32 + 3.0, (y * GRID_SIZE) as f32 + 3.0, 15.0, 15.0))?;
	    if self.powered {
	        graphics::set_color(ctx, graphics::Color{r: 0.5, g: 0.8, b: 0.5, a: 0.8,})?;
	    }
	    graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32 + 5.0, (y * GRID_SIZE) as f32 + 5.0, 11.0, 11.0))?;

	    Ok(())
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Generator { }

impl Generator {
    pub fn draw(pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE;
	    let y = pos / LEVEL_SIZE;
	    graphics::set_color(ctx, graphics::Color{r: 0.8, g: 0.8, b: 0.8, a: 0.1,})?;
	    graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new((x * GRID_SIZE) as f32 + 3.0, (y * GRID_SIZE) as f32 + 3.0, 15.0, 15.0))?;

	    graphics::set_color(ctx, graphics::Color{r: 0.8, g: 1.0, b: 0.8, a: 1.0,})?;
	    graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32 + 5.0, (y * GRID_SIZE) as f32 + 5.0, 11.0, 11.0))?;

	    Ok(())
	}
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