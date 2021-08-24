use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Color};
use ggez::input::keyboard;
use ggez::{Context, ContextBuilder, GameResult};
use glam::*;
use std::f32::consts::PI;

const RAD: f32 = PI / 180.0 / 4.;
const LAG_TELA: f32 = 1200.;
const ALT_TELA: f32 = 600.;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Ricola")
        .window_setup(ggez::conf::WindowSetup::default().title("raycaster"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(LAG_TELA, ALT_TELA))
        .build()
        .expect("janela não inicializada");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    p1: Player,
    map: [[u8; 8]; 8],
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            p1: Player {
                x: 200.,
                y: 200.,
                dx: 4.,
                dy: 0.,
                a: 0.,
            },
            map: [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 0, 1, 1, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        }
    }
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.p1.a += if self.p1.a >= 2. * PI { -2. * PI } else { 0.1 };
            self.p1.dx = self.p1.a.cos() * 4.;
            self.p1.dy = self.p1.a.sin() * 4.;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.p1.a += if self.p1.a <= 0. { 2. * PI } else { -0.1 };
            self.p1.dx = self.p1.a.cos() * 4.;
            self.p1.dy = self.p1.a.sin() * 4.;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.p1.x += self.p1.dx;
            self.p1.y += self.p1.dy;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.p1.x -= self.p1.dx;
            self.p1.y -= self.p1.dy;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.5, 0.5, 0.5, 0.5));
        // draw map
        let lado_square: f32 = 62.0;

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let cor = if self.map[y][x] == 1 {
                    Color::WHITE
                } else {
                    Color::BLACK
                };
                let square = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    [
                        (x as f32 * lado_square),
                        (y as f32 * lado_square),
                        lado_square - 2.,
                        lado_square - 2.,
                    ]
                    .into(),
                    cor,
                )?;
                graphics::draw(ctx, &square, graphics::DrawParam::default())?;
            }
        }

        // draw player
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(self.p1.x, self.p1.y),
            5.0,
            2.0,
            Color::YELLOW,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        let mut c: f32 = 500.;
        // draw sky
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [500.0, 0.0, 700.0, ALT_TELA / 2.].into(),
            Color::new(0., 191., 200., 1.0),
        )?;
        graphics::draw(ctx, &square, graphics::DrawParam::default())?;

        for i in -120..=120 {
            let ray = self.raios(lado_square, i as f32);
            // draw rays
            let line = graphics::Mesh::new_line(
                ctx,
                &[Vec2::new(self.p1.x, self.p1.y), ray.0],
                1.,
                Color::BLUE,
            )?;
            graphics::draw(ctx, &line, graphics::DrawParam::default())?;
            // draw walls
            let alt_wall = ALT_TELA * 62. / ray.2;
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(c, ALT_TELA / 2. - alt_wall / 2.),
                    Vec2::new(c, ALT_TELA / 2. + alt_wall / 2.),
                ],
                3.,
                ray.1,
            )?;
            c += 3.;
            graphics::draw(ctx, &line, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)
    }
}

impl MyGame {
    // dda
    fn raios(&self, quad: f32, ra: f32) -> (Vec2, Color, f32) {
        let mut ra = self.p1.a + ra * RAD;
        ra += if ra < 0. {
            2. * PI
        } else if ra > 2. * PI {
            -2. * PI
        } else {
            0.
        };
        let tan = if 2. * PI < ra || ra < 0.0 {
            0.0
        } else {
            ra.tan()
        };

        // Horizontal lines
        let (mut rh, dh) = if ra < PI { // up
            let ry = quad - self.p1.y % quad;
            ((ry / tan + self.p1.x, ry + self.p1.y), (quad / tan, quad))
        } else { // down
            let ry = self.p1.y % -quad;
            (
                (-ry / tan + self.p1.x, -ry + self.p1.y - 0.0001),
                (-quad / tan, -quad),
            )
        };

        while let (0..=7, 0..=7) = ((rh.0 / quad) as usize, (rh.1 / quad) as usize) {
            if self.map[(rh.1 / quad) as usize][(rh.0 / quad) as usize] == 1 {
                break;
            }
            rh.0 += dh.0;
            rh.1 += dh.1;
        }

        // Vertical lines

        let (mut rv, dv) = if !(ra > PI / 2. && ra < 3. * PI / 2.) {
            // right
            let rx = quad - self.p1.x % quad;
            ((rx + self.p1.x, rx * tan + self.p1.y), (quad, quad * tan))
        } else {
            // left
            let rx = self.p1.x % -quad;
            (
                (-rx + self.p1.x - 0.0001, -rx * tan + self.p1.y),
                (-quad, -quad * tan),
            )
        };

        while let (0..=7, 0..=7) = ((rv.0 / quad) as usize, (rv.1 / quad) as usize) {
            if self.map[(rv.1 / quad) as usize][(rv.0 / quad) as usize] == 1 {
                break;
            }
            rv.0 += dv.0;
            rv.1 += dv.1;
        }

        let mut ca = self.p1.a - ra;
        if ra < 0. {
            ca +=  2. * PI;
        } else if ra > 2. * PI {
            ca += -2. * PI;
        }
        let calc_dist =
            |x: (f32, f32), p: &Player| ((x.0 - p.x).powf(2.) + (x.1 - p.y).powf(2.)).sqrt();
        let mh = calc_dist(rh, &self.p1) * ca.cos();
        let mv = calc_dist(rv, &self.p1) * ca.cos();

        if mh < mv {
            (
                Vec2::new(rh.0 + 1., rh.1 + 1.),
                Color::new(0.0, 0.2, 1.0, 1.0),
                mh,
            )
        } else {
            (
                Vec2::new(rv.0 + 1., rv.1 + 1.),
                Color::new(0.0, 0.1, 0.7, 1.0),
                mv,
            )
        }
    }
}

struct Player {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    a: f32,
}
