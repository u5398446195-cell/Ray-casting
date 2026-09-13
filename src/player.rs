use raylib::prelude::*;

pub struct Player {
    pub pos: Vector2,
    pub angle: f32,
    pub speed: f32,
    pub rot_speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            angle: 0.0,
            speed: 3.5,
            rot_speed: 2.5, 
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        rl: &RaylibHandle,
        map: &mut [[i32; 12]; 12],
        score: &mut i32,
        won: &mut bool,
    ) {
        
        let mut rot_dir = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
            rot_dir -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
            rot_dir += 1.0;
        }
        self.angle += rot_dir * self.rot_speed * dt;


        let mut move_dir = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
            move_dir += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
            move_dir -= 1.0;
        }

        if move_dir != 0.0 {
            let step = self.speed * dt * move_dir;
            let dx = self.angle.cos() * step;
            let dy = self.angle.sin() * step;

            let new_x = self.pos.x + dx;
            if self.can_move_to(new_x, self.pos.y, map, score, won) {
                self.pos.x = new_x;
            }

            
            let new_y = self.pos.y + dy;
            if self.can_move_to(self.pos.x, new_y, map, score, won) {
                self.pos.y = new_y;
            }
        }
    }

  
    #[inline]
    fn can_move_to(
        &self,
        x: f32,
        y: f32,
        map: &mut [[i32; 12]; 12],
        score: &mut i32,
        won: &mut bool,
    ) -> bool {
        if x < 0.0 || y < 0.0 {
            return false;
        }

        let mx = x as usize;
        let my = y as usize;

        if my < map.len() && mx < map[0].len() {
            let tile = map[my][mx];
            if tile == 1 {
                return false;
            }

            if tile == 3 {
                map[my][mx] = 0; 
                *score += 1;
            } else if tile == 2 {
                *won = true; 
            }
            return true;
        }

        false
    }
}