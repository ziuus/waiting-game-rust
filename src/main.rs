use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "waiting-game-rust".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_speed = 8.0;
    let mut score = 0;
    let mut is_game_over = false;
    
    let mut dino_y = 900.0;
    let mut dino_dy = 0.0;
    let gravity = 0.7;
    let jump_force = 15.0;
    let mut is_grounded = true;
    let ground_level = 900.0;
    
    let mut obstacles: Vec<(f32, f32, f32, f32)> = Vec::new();
    
    loop {
        // Clear with 0 alpha (requires compositor support for true transparency)
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
        
        if is_key_pressed(KeyCode::Space) {
            if is_game_over {
                // Reset
                is_game_over = false;
                score = 0;
                game_speed = 8.0;
                obstacles.clear();
                dino_y = ground_level;
                dino_dy = 0.0;
                is_grounded = true;
            } else if is_grounded {
                dino_dy = -jump_force;
                is_grounded = false;
            }
        }
        
        if !is_game_over {
            // Physics
            if !is_grounded {
                dino_dy += gravity;
                dino_y += dino_dy;
            }
            if dino_y > ground_level {
                dino_y = ground_level;
                dino_dy = 0.0;
                is_grounded = true;
            }
            
            // Obstacles
            if rand::gen_range(0, 100) < 2 {
                if obstacles.is_empty() || obstacles.last().unwrap().0 < screen_width() - 400.0 {
                    let height = rand::gen_range(30.0, 80.0);
                    obstacles.push((screen_width(), ground_level + 60.0 - height, 30.0, height));
                }
            }
            
            for i in (0..obstacles.len()).rev() {
                obstacles[i].0 -= game_speed;
                
                // Collision
                let obs = obstacles[i];
                if 480.0 < obs.0 + obs.2 && 480.0 + 30.0 > obs.0 && dino_y < obs.1 + obs.3 && dino_y + 60.0 > obs.1 {
                    is_game_over = true;
                }
                
                if obs.0 + obs.2 < 0.0 {
                    obstacles.remove(i);
                    score += 1;
                    if score % 5 == 0 {
                        game_speed += 0.5;
                    }
                }
            }
        }
        
        // Draw
        let dino_color = Color::new(0.407, 0.729, 0.498, 1.0); // #68BA7F
        let dino_x = 480.0;
        
        // Dino Leg Animation
        let time = get_time() as f32;
        let leg_offset = if !is_grounded { 0.0 } else { (time * 20.0).sin() * 8.0 };
        
        // Draw Dino
        draw_rectangle(dino_x, dino_y + 15.0, 30.0, 30.0, dino_color); // Body
        draw_rectangle(dino_x + 15.0, dino_y, 20.0, 18.0, dino_color); // Head
        draw_rectangle(dino_x + 28.0, dino_y + 5.0, 4.0, 4.0, BLACK); // Eye
        draw_rectangle(dino_x + 4.0, dino_y + 45.0, 8.0, 15.0 + leg_offset, dino_color); // Left Leg
        draw_rectangle(dino_x + 20.0, dino_y + 45.0, 8.0, 15.0 - leg_offset, dino_color); // Right Leg
        
        // Draw Obstacles (Cacti)
        let obs_color = Color::new(1.0, 0.294, 0.168, 1.0); // #ff4b2b
        for obs in &obstacles {
            let (x, y, w, h) = (obs.0, obs.1, obs.2, obs.3);
            draw_rectangle(x + w/4.0, y, w/2.0, h, obs_color);
            draw_rectangle(x, y + h/3.0, w, h/6.0, obs_color);
            draw_rectangle(x, y + h/6.0, w/6.0, h/6.0, obs_color);
            draw_rectangle(x + w - w/6.0, y + h/6.0, w/6.0, h/6.0, obs_color);
        }
        
        // Score
        draw_text(&format!("{:05}", score), screen_width() / 2.0 - 50.0, 100.0, 60.0, dino_color);
        
        if is_game_over {
            draw_text("TERMINATED", screen_width() / 2.0 - 150.0, screen_height() / 2.0, 80.0, WHITE);
            draw_text("PRESS SPACE TO RESTART", screen_width() / 2.0 - 180.0, screen_height() / 2.0 + 50.0, 40.0, LIGHTGRAY);
        }
        
        next_frame().await
    }
}
