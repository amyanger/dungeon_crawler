use rand::Rng;

pub const DUNGEON_WIDTH: usize = 80;
pub const DUNGEON_HEIGHT: usize = 24;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
    Stairs,
}

pub struct Room {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Room { x, y, width, height }
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn center(&self) -> (usize, usize) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }
}

pub struct Dungeon {
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Room>,
    pub player_start: (usize, usize),
}

impl Dungeon {
    pub fn new() -> Self {
        let mut dungeon = Dungeon {
            tiles: vec![vec![Tile::Wall; DUNGEON_WIDTH]; DUNGEON_HEIGHT],
            rooms: Vec::new(),
            player_start: (0, 0),
        };
        dungeon.generate();
        dungeon
    }

    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let max_rooms = 10;
        let min_room_size = 4;
        let max_room_size = 10;

        for _ in 0..max_rooms {
            let width = rng.gen_range(min_room_size..=max_room_size);
            let height = rng.gen_range(min_room_size..=max_room_size);
            let x = rng.gen_range(1..DUNGEON_WIDTH.saturating_sub(width + 1));
            let y = rng.gen_range(1..DUNGEON_HEIGHT.saturating_sub(height + 1));

            let new_room = Room::new(x, y, width, height);

            let mut intersects = false;
            for room in &self.rooms {
                if new_room.intersects(room) {
                    intersects = true;
                    break;
                }
            }

            if !intersects {
                self.create_room(&new_room);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();

                    if rng.gen_bool(0.5) {
                        self.create_horizontal_tunnel(prev_x, new_x, prev_y);
                        self.create_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        self.create_vertical_tunnel(prev_y, new_y, prev_x);
                        self.create_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                self.rooms.push(new_room);
            }
        }

        if !self.rooms.is_empty() {
            self.player_start = self.rooms[0].center();

            if self.rooms.len() > 1 {
                let last_room_center = self.rooms[self.rooms.len() - 1].center();
                self.tiles[last_room_center.1][last_room_center.0] = Tile::Stairs;
            }
        }
    }

    fn create_room(&mut self, room: &Room) {
        for y in room.y..room.y + room.height {
            for x in room.x..room.x + room.width {
                if y < DUNGEON_HEIGHT && x < DUNGEON_WIDTH {
                    self.tiles[y][x] = Tile::Floor;
                }
            }
        }
    }

    fn create_horizontal_tunnel(&mut self, x1: usize, x2: usize, y: usize) {
        let start = x1.min(x2);
        let end = x1.max(x2) + 1;
        for x in start..end {
            if y < DUNGEON_HEIGHT && x < DUNGEON_WIDTH {
                self.tiles[y][x] = Tile::Floor;
            }
        }
    }

    fn create_vertical_tunnel(&mut self, y1: usize, y2: usize, x: usize) {
        let start = y1.min(y2);
        let end = y1.max(y2) + 1;
        for y in start..end {
            if y < DUNGEON_HEIGHT && x < DUNGEON_WIDTH {
                self.tiles[y][x] = Tile::Floor;
            }
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= DUNGEON_WIDTH || y >= DUNGEON_HEIGHT {
            return false;
        }
        self.tiles[y][x] != Tile::Wall
    }
}