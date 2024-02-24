use macroquad::prelude::*;

const CELL_SIZE: (f32, f32) = (20., 20.);
const GRID_SIZE: (f32, f32) = (60., 30.);
const WINDOW_SIZE: (f32, f32) = ((CELL_SIZE.0 * GRID_SIZE.0), (CELL_SIZE.1 * GRID_SIZE.1));
const FPS: f32 = 12.;

enum STATE {
    RUN,
    PAUSE,
}

struct Game {
    grid: Vec<Vec<bool>>,
    running: bool,
    fps: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize],
            running: false,
            fps: FPS,
        }
    }
    pub fn draw(&self) {
        for x in 0..GRID_SIZE.0 as usize {
            for y in 0..GRID_SIZE.1 as usize {
                if self.grid[x][y] {
                    draw_rectangle(
                        x as f32 * CELL_SIZE.0,
                        y as f32 * CELL_SIZE.1,
                        CELL_SIZE.0,
                        CELL_SIZE.1,
                        BLACK,
                    )
                }
            }
        }
    }
    pub fn update(&mut self) {
        let mut temp_grid = self.grid.clone();
        for x in 0..(GRID_SIZE.0) as usize {
            for y in 0..(GRID_SIZE.1) as usize {
                let neighbours = self.count_neighbours((x, y));
                temp_grid[x][y] = self.check_state(self.grid[x][y], neighbours);
            }
        }
        self.grid = temp_grid;
    }
    pub fn clear_grid(&mut self) {
        self.grid = vec![vec![false; GRID_SIZE.1 as usize]; GRID_SIZE.0 as usize]
    }
    fn check_state(&self, curr_cell: bool, neighbours: f32) -> bool {
        let mut is_alive = false;
        if neighbours == 2. {
            is_alive = curr_cell;
        } else if neighbours == 3. {
            is_alive = true;
        }
        return is_alive;
    }
    fn count_neighbours(&self, pos: (usize, usize)) -> f32 {
        let mut neighbours: i8 = 0;
        for i in -1..2 {
            for j in -1..2 {
                let x = (pos.0 as f32 + i as f32 + GRID_SIZE.0) % GRID_SIZE.0;
                let y = (pos.1 as f32 + j as f32 + GRID_SIZE.1) % GRID_SIZE.1;
                neighbours += self.grid[x as usize][y as usize] as i8
            }
        }
        return (neighbours - self.grid[pos.0][pos.1] as i8) as f32;
    }
}

#[macroquad::main("Conway's Game Of Life")]
async fn main() {
    let mut timer: f32 = 0.;
    request_new_screen_size(WINDOW_SIZE.0, WINDOW_SIZE.1);
    let mut game = Game::new();

    loop {
        let delta = game.fps * get_frame_time();
        set_camera(&Camera2D {
            zoom: vec2(2. / screen_width(), 2. / screen_height()),
            target: vec2(WINDOW_SIZE.0 * 0.5, WINDOW_SIZE.1 * 0.5),
            ..Default::default()
        });

        clear_background(WHITE);

        if is_key_pressed(KeyCode::Backspace) {
            game.clear_grid();
        }
        if is_key_pressed(KeyCode::Space) {
            game.running ^= true;
        }

        let game_state = if game.running {
            STATE::RUN
        } else {
            STATE::PAUSE
        };

        match game_state {
            STATE::PAUSE => {
                draw_text("Paused", WINDOW_SIZE.0 / 2. - 120., 100., 100., BLACK);
                let pos = calculate_pos(mouse_position());
                if is_mouse_button_down(MouseButton::Left) {
                    game.grid[pos.0][pos.1] = true;
                }
                if is_mouse_button_down(MouseButton::Right) {
                    game.grid[pos.0][pos.1] = false;
                }
            }
            STATE::RUN => {
                timer += delta;
                if timer >= 1. {
                    game.update();
                    timer -= 1.;
                } else if timer >= 5. {
                    timer = 0.;
                }
            }
        }
        game.draw();
        next_frame().await;
    }
}
fn calculate_pos(pos: (f32, f32)) -> (usize, usize) {
    let calculated_pos: (usize, usize) = (
        ((pos.0 - pos.0 % CELL_SIZE.0) / CELL_SIZE.0) as usize,
        ((pos.1 - pos.1 % CELL_SIZE.1) / CELL_SIZE.1) as usize,
    );
    return calculated_pos;
}
