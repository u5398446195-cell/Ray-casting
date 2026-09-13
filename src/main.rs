use raylib::prelude::*;

mod map;
mod player;

use map::MAP;
use player::Player;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Cactus labyrinthe")
        .build();

    rl.set_target_fps(60);

    let mut player = Player::new(1.5, 1.5);
    let mut score = 0;
    let mut won = false;
    let mut current_map = MAP;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        if !won {
            player.update(dt, &rl, &mut current_map, &mut score, &mut won);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT / 2, Color::DARKBLUE);
        d.draw_rectangle(0, SCREEN_HEIGHT / 2, SCREEN_WIDTH, SCREEN_HEIGHT / 2, Color::DARKGRAY);

        render_rays(&mut d, &player, &current_map);

        render_hud(&mut d, score);
        render_minimap(&mut d, &current_map, &player);

        if won {
            render_victory_overlay(&mut d);
        }
    }
}

fn render_rays(d: &mut RaylibDrawHandle, player: &Player, map: &[[i32; 12]; 12]) {
    let fov = std::f32::consts::FRAC_PI_3;
    let mw = map[0].len() as i32;
    let mh = map.len() as i32;

    for i in 0..SCREEN_WIDTH {
        let ra = player.angle - (fov / 2.0) + (i as f32 / SCREEN_WIDTH as f32) * fov;
        let (cos_a, sin_a) = (ra.cos(), ra.sin());

        let mut dist = 0.0;
        let mut hit = 1;

        while dist < 20.0 {
            dist += 0.05;
            let tx = (player.pos.x + cos_a * dist) as i32;
            let ty = (player.pos.y + sin_a * dist) as i32;

            if tx < 0 || tx >= mw || ty < 0 || ty >= mh {
                hit = 1;
                dist = 20.0;
                break;
            }

            let t = map[ty as usize][tx as usize];
            if t != 0 {
                hit = t;
                break;
            }
        }

        let corrected = dist * (ra - player.angle).cos();
        let ceil = (SCREEN_HEIGHT as f32 / 2.0) - ((SCREEN_HEIGHT as f32) / corrected.max(0.1));
        let floor = (SCREEN_HEIGHT as f32) - ceil;

        let shade = (255.0 / (1.0 + corrected * corrected * 0.15)).clamp(15.0, 255.0) as u8;

        let color = match hit {
            2 => Color::new(0, shade, 0, 255),
            3 => Color::new(shade, shade, 0, 255),
            _ => Color::new(shade, shade / 2, shade / 3, 255),
        };

        d.draw_line(i, ceil as i32, i, floor as i32, color);
    }
}

fn render_hud(d: &mut RaylibDrawHandle, score: i32) {
    d.draw_rectangle(8, 8, 220, 65, Color::new(0, 0, 0, 180));
    d.draw_text("Flèches: Déplacement", 15, 15, 14, Color::LIGHTGRAY);
    d.draw_text(&format!("Score: {} / 3", score), 15, 40, 20, Color::GOLD);
}

fn render_minimap(d: &mut RaylibDrawHandle, map: &[[i32; 12]; 12], player: &Player) {
    let scale = 10;
    let offset_x = SCREEN_WIDTH - (map[0].len() as i32 * scale) - 15;
    let offset_y = 15;

    d.draw_rectangle(
        offset_x - 4,
        offset_y - 4,
        (map[0].len() as i32 * scale) + 8,
        (map.len() as i32 * scale) + 8,
        Color::new(0, 0, 0, 200),
    );

    for (y, row) in map.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            let color = match tile {
                1 => Color::GRAY,
                2 => Color::GREEN,
                3 => Color::GOLD,
                _ => Color::DARKGRAY,
            };
            d.draw_rectangle(
                offset_x + (x as i32 * scale),
                offset_y + (y as i32 * scale),
                scale - 1,
                scale - 1,
                color,
            );
        }
    }

    let px = offset_x + (player.pos.x * scale as f32) as i32;
    let py = offset_y + (player.pos.y * scale as f32) as i32;
    let dir_x = px + (player.angle.cos() * 8.0) as i32;
    let dir_y = py + (player.angle.sin() * 8.0) as i32;

    d.draw_circle(px, py, 3.0, Color::RED);
    d.draw_line(px, py, dir_x, dir_y, Color::RED);
}

fn render_victory_overlay(d: &mut RaylibDrawHandle) {
    let box_w = 400;
    let box_h = 180;
    let box_x = (SCREEN_WIDTH - box_w) / 2;
    let box_y = (SCREEN_HEIGHT - box_h) / 2;

    d.draw_rectangle(box_x, box_y, box_w, box_h, Color::new(10, 10, 20, 230));
    d.draw_rectangle_lines(box_x, box_y, box_w, box_h, Color::GOLD);
    d.draw_text("Terminé !", box_x + 120, box_y + 40, 40, Color::GREEN);
    d.draw_text("GG !", box_x + 125, box_y + 100, 18, Color::WHITE);
}