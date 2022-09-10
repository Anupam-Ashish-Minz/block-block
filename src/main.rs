use rand::Rng;
use raylib::prelude::*;

struct Window {
    width: i32,
    height: i32,
}

struct Ball {
    x: i32,
    y: i32,
    r: f32,
}

#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn check_player_collision(player: &Ball, object: &Rect) -> bool {
    // object condition, rect cicle collision
    if player.x >= object.x
        && player.x <= object.x + object.w
        && player.y >= object.y
        && player.y <= object.y + object.h
    {
        return true;
    }
    return false;
}

fn main() {
    let window = Window {
        width: 640,
        height: 480,
    };
    let mut ball: Ball = Ball {
        x: 50,
        y: 50,
        r: 10.0,
    };
    let end: Rect = Rect {
        x: 400,
        y: 400,
        w: 10,
        h: 50,
    };

    let mut rng = rand::thread_rng();

    let mut obsticles: Vec<Rect> = Vec::new();
    for _ in 0..30 {
        obsticles.push(Rect {
            x: rng.gen_range(0..window.width),
            y: rng.gen_range(0..window.height),
            w: 20,
            h: 20,
        });
    }

    let speed = 10;

    let mut lose = false;
    let mut win = false;

    let (mut rl, thread) = raylib::init()
        .size(window.width, window.height)
        .title("Hello, World")
        .build();

    rl.set_target_fps(30);

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            ball.y -= speed;
        } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            ball.y += speed;
        } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            ball.x -= speed;
        } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            ball.x += speed;
        }


        if !lose {
            for o in obsticles.iter() {
                lose = check_player_collision(&ball, &o);
                if lose {
                    break;
                }
            }
        }

        if !win {
            win = check_player_collision(&ball, &end);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        obsticles = obsticles.iter().map(|o| {
			let k = Rect {
				x: o.x,
				y: o.y + 1,
				w: o.w,
				h: o.h,
			};
            return k;
        }).collect();

        obsticles.iter().for_each(|o| {
            d.draw_rectangle(o.x, o.y, o.w, o.h, Color::RED);
        });

        if !win && lose {
            d.draw_text("your lose, try again", 50, 50, 32, Color::RED);
        }
        if win && !lose {
            d.draw_text("game over, you won", 50, 50, 32, Color::GOLD);
        }
        d.draw_circle(ball.x, ball.y, ball.r, Color::WHITE);
        d.draw_rectangle(end.x, end.y, end.w, end.h, Color::GREEN);
    }
}
