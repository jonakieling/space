use std::f32::consts::{PI, FRAC_PI_2};

use ggez::GameResult;
use ggez::Context;
use ggez::graphics;

use constants::{LEVEL_SIZE, GRID_SIZE};
use misc::{Direction};
use storage::{SelectionStorage, Tree};
use dialog::DialogItem;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum WallType {
	Wall,
	Corner,
	T,
	Cross
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Wall {
	pub wall_type: WallType,
	pub face: Direction
}

impl Wall {
	pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE * GRID_SIZE;
	    let y = pos / LEVEL_SIZE * GRID_SIZE;

		let image_src;
		match self.wall_type {
			WallType::Corner => {
				image_src = "/corner.png";
			},
			_ => {
				image_src = "/wall.png";
			}
		}

		let dst = graphics::Point2::new(x as f32, y as f32);

        draw_tile(ctx, image_src, dst, Some(self.face))
	}
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Floor { }

impl Floor {
	pub fn draw(pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE * GRID_SIZE;
	    let y = pos / LEVEL_SIZE * GRID_SIZE;

		let dst = graphics::Point2::new(x as f32, y as f32);
		
        draw_tile(ctx, "/floor.png", dst, None)
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
    pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE * GRID_SIZE;
	    let y = pos / LEVEL_SIZE * GRID_SIZE;

		graphics::set_color(ctx, graphics::WHITE)?;
		let image_src;
		match self.status {
			DoorStatus::Open => {
				image_src = "/floor.png";
			},
			DoorStatus::Closed => {
				image_src = "/door.png";
			}
		}

		let dst = graphics::Point2::new(x as f32, y as f32);
		
        draw_tile(ctx, &image_src, dst, Some(self.face))
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Terminal {
    pub text: Box<String>,
    pub front: Direction,
}

impl Terminal {
    pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE * GRID_SIZE;
	    let y = pos / LEVEL_SIZE * GRID_SIZE;

		let dst = graphics::Point2::new(x as f32, y as f32);
		
        draw_tile(ctx, "/terminal.png", dst, Some(self.front))
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PilotSeat {
    pub front: Direction,
}

impl PilotSeat {
    pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE * GRID_SIZE;
	    let y = pos / LEVEL_SIZE * GRID_SIZE;

		let dst = graphics::Point2::new(x as f32, y as f32);
		
        draw_tile(ctx, "/pilot-seat.png", dst, Some(self.front))
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
	    graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32 + 3.0, (y * GRID_SIZE) as f32 + 3.0, 19.0, 19.0))?;
	    if self.powered {
	        graphics::set_color(ctx, graphics::Color{r: 0.5, g: 0.8, b: 0.5, a: 0.8,})?;
	    }
	    graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new((x * GRID_SIZE) as f32 + 5.0, (y * GRID_SIZE) as f32 + 5.0, 15.0, 15.0))?;

	    Ok(())
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Generator {
	pub face: Direction
}

impl Generator {
    pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE * GRID_SIZE;
	    let y = pos / LEVEL_SIZE * GRID_SIZE;
		
		let dst = graphics::Point2::new(x as f32, y as f32);
		
        draw_tile(ctx, "/generator.png", dst, Some(self.face))
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
    pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE;
	    let y = pos / LEVEL_SIZE;
        graphics::set_color(ctx, graphics::BLACK)?;
        let npc = graphics::Rect::new((x * GRID_SIZE) as f32, (y * GRID_SIZE) as f32, 20.0, 20.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, npc)?;

        graphics::set_color(ctx, graphics::WHITE)?;
        let face = graphics::Rect::new((x * GRID_SIZE) as f32 + 5.0 + (self.direction.value().viewport_x() * 0.2), (y * GRID_SIZE) as f32 + 5.0 + (self.direction.value().viewport_y() * 0.2), 10.0, 10.0);
        graphics::rectangle(ctx, graphics::DrawMode::Line(2.0), face)?;
        Ok(())
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Storage {
	pub content: SelectionStorage<Item>,
	pub face:Direction
}

impl Storage {
    pub fn draw(&self, pos: i32, ctx: &mut Context) -> GameResult<()> {
	    let x = pos % LEVEL_SIZE * GRID_SIZE;
	    let y = pos / LEVEL_SIZE * GRID_SIZE;
		
		let dst = graphics::Point2::new(x as f32, y as f32);

        draw_tile(ctx, "/storage.png", dst, Some(self.face))
	}
}

pub fn draw_tile(ctx: &mut Context, tile_src: &str, tile_dst: graphics::Point2, direction: Option<Direction>) -> GameResult<()> {
		graphics::set_color(ctx, graphics::WHITE)?;
		let mut storage_image = graphics::Image::new(ctx, tile_src)?;
		storage_image.set_filter(graphics::FilterMode::Nearest);
		let mut tile_dst = tile_dst;
		let rotation;
		match direction {
			Some(Direction::Up) => {
                rotation = PI;
				tile_dst = graphics::Point2::new(tile_dst.x + 24.0, tile_dst.y + 24.0);
			},
			Some(Direction::Down) => {
                rotation = 0.0;
			},
			Some(Direction::Left) => {
                rotation = FRAC_PI_2;
				tile_dst = graphics::Point2::new(tile_dst.x + 24.0, tile_dst.y);
			},
			Some(Direction::Right) => {
                rotation = 3.0 * FRAC_PI_2;
				tile_dst = graphics::Point2::new(tile_dst.x, tile_dst.y + 24.0);
			},
			_ => {
                rotation = 0.0;
			}
		}
		
		graphics::draw_ex(
			ctx,
			&storage_image,
			graphics::DrawParam {
				dest: tile_dst,
				rotation: rotation,
				scale: graphics::Point2::new(3.0, 3.0),
				..Default::default()
			},
		)?;

	    Ok(())
}