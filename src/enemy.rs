use rand::Rng;
use crate::dungeon::Dungeon;

#[derive(Clone)]
pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub enemy_type: EnemyType,
    pub symbol: char,
}

#[derive(Clone, Copy)]
pub enum EnemyType {
    Goblin,
    Orc,
    Troll,
}

impl Enemy {
    pub fn new(x: usize, y: usize, enemy_type: EnemyType) -> Self {
        let (health, attack, symbol) = match enemy_type {
            EnemyType::Goblin => (30, 5, 'g'),
            EnemyType::Orc => (50, 8, 'o'),
            EnemyType::Troll => (80, 12, 'T'),
        };

        Enemy {
            x,
            y,
            health,
            max_health: health,
            attack,
            enemy_type,
            symbol,
        }
    }

    pub fn move_towards(&mut self, target_x: usize, target_y: usize, dungeon: &Dungeon, enemies: &[Enemy]) {
        let dx = if self.x < target_x { 1i32 } else if self.x > target_x { -1 } else { 0 };
        let dy = if self.y < target_y { 1i32 } else if self.y > target_y { -1 } else { 0 };

        let new_x = (self.x as i32 + dx) as usize;
        let new_y = (self.y as i32 + dy) as usize;

        if dungeon.is_walkable(new_x, new_y) && !self.is_position_occupied(new_x, new_y, enemies) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    fn is_position_occupied(&self, x: usize, y: usize, enemies: &[Enemy]) -> bool {
        enemies.iter().any(|e| e.x == x && e.y == y && !std::ptr::eq(e, self))
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn distance_to(&self, x: usize, y: usize) -> f32 {
        let dx = self.x as f32 - x as f32;
        let dy = self.y as f32 - y as f32;
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn spawn_enemies(dungeon: &Dungeon) -> Vec<Enemy> {
    let mut enemies = Vec::new();
    let mut rng = rand::thread_rng();

    for room in &dungeon.rooms[1..] {
        let num_enemies = rng.gen_range(0..=2);
        for _ in 0..num_enemies {
            let x = rng.gen_range(room.x..room.x + room.width);
            let y = rng.gen_range(room.y..room.y + room.height);

            let enemy_type = match rng.gen_range(0..10) {
                0..=5 => EnemyType::Goblin,
                6..=8 => EnemyType::Orc,
                _ => EnemyType::Troll,
            };

            enemies.push(Enemy::new(x, y, enemy_type));
        }
    }

    enemies
}