use crate::dungeon::Dungeon;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub level: u32,
    pub experience: u32,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Player {
            x,
            y,
            health: 100,
            max_health: 100,
            attack: 10,
            level: 1,
            experience: 0,
        }
    }

    pub fn move_up(&mut self, dungeon: &Dungeon) {
        if self.y > 0 && dungeon.is_walkable(self.x, self.y - 1) {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self, dungeon: &Dungeon) {
        if dungeon.is_walkable(self.x, self.y + 1) {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self, dungeon: &Dungeon) {
        if self.x > 0 && dungeon.is_walkable(self.x - 1, self.y) {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self, dungeon: &Dungeon) {
        if dungeon.is_walkable(self.x + 1, self.y) {
            self.x += 1;
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        while self.experience >= self.level * 100 {
            self.experience -= self.level * 100;
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.max_health += 20;
        self.health = self.max_health;
        self.attack += 5;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}