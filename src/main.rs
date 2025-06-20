use nalgebra::{self as na, Vector2};
use piston_window::*;
use std::{
    f64::{self, consts::PI},
    usize,
};

/*
* NOTE: designspec - graphical look at how an analemma is plotted
*       looking at sun position and plot points from change across time.
*
* NOTE: Calculations:
*       - Earth orbital mechanics, elliptical orbit, changing speed
*       - Axial Tilt 23.44
*       - Equation of time solar time diff
*       - Sun relative position
*       Maths:
*       - Coordinate system for conversions. celestial to screen
*       - 3D to 2D projection
*       Rendering strategy:
*       - Track year of sun positions
*       - Piston drawing primitives to render path and sol pos
*
*
* TODO: 1. change background to day sky
*       2. animate current sun movement throughout year.
*
*
*
*/
const WIDTH: f64 = 540.0;
const HEIGHT: f64 = 960.0;

#[derive(Clone)]
struct SunPosition {
    x: f64,
    y: f64,
    day: u32,
}

struct AnnalemmaSimulation {
    sun_positions: Vec<SunPosition>,
    current_day: u32,
    animation_speed: f64,
}

impl AnnalemmaSimulation {
    fn new() -> Self {
        let mut positions = Vec::new();

        for day in 0..365 {
            let pos = Self::calculate_sun_position(day as f64);
            positions.push(SunPosition {
                x: pos.x,
                y: pos.y,
                day: day as u32,
            });
        }

        Self {
            sun_positions: positions,
            current_day: 136, //TODO: Vary based on actual day? Animate from tday?
            animation_speed: 1.0,
        }
    }

    fn calculate_sun_position(day_of_year: f64) -> Vector2<f64> {
        let equation_of_time = Self::equation_of_time(day_of_year);

        let declination = Self::solar_declination(day_of_year);

        Vector2::new(equation_of_time, declination)
    }

    fn equation_of_time(day_of_year: f64) -> f64 {
        let day_angle = (day_of_year / 365.35) * 2.0 * PI;

        //orbital eccentricity effects
        let eccentricity_term = 9.655 * (2.0 * day_angle).sin();
        let obliquity_term = 7.873 * (day_angle + 3.588).sin();

        //in minutes, push to degrees for display
        let equation_minutes = eccentricity_term + obliquity_term;
        equation_minutes / 4.0
    }

    fn solar_declination(day_of_year: f64) -> f64 {
        let day_angle = (day_of_year / 365.25) * 2.0 * PI;

        let axial_tilt = 23.44;

        //sinusdial variation
        let solstice_offset = day_angle - (172.0 / 365.25) * 2.0 * PI;
        axial_tilt * solstice_offset.cos()
    }

    fn update(&mut self, dt: f64) {
        self.current_day =
            ((self.current_day as f64 + self.animation_speed * dt * 60.0) % 365.0) as u32;
    }

    fn screen_position(&self, sun_pos: &SunPosition) -> [f64; 2] {
        //conv astronomical coordinates to screen coordinates
        let center_x = WIDTH / 2.0;
        let center_y = HEIGHT / 2.0;

        //scaling
        let x_scale = 15.0; //eot (horizontal)
        let y_scale = 8.0; //declination (vertical)

        [
            center_x + sun_pos.x * x_scale,
            center_y - sun_pos.y * y_scale, //flip y
        ]
    }

    fn render(&mut self, c: Context, g: &mut G2d) {
        clear([0.502, 0.788, 0.909, 0.9], g); //sky blue?

        self.draw_sky_gradient(c, g);

        self.draw_analemma_path(c, g);

        self.draw_current_sun(c, g);

        // self.draw_date_info(c, g);
    }

    fn draw_sky_gradient(&self, c: Context, g: &mut G2d) {
        let steps = 100;
        let step_height = HEIGHT / steps as f64;

        for i in 0..steps {
            let t = i as f32 / (steps - 1) as f32;

            let colour = [
                0.53 + (0.87 - 0.53) * t,
                0.81 + (0.95 - 0.81) * t,
                0.92 + (1.0 - 0.92) * t,
                1.0,
            ];

            let y = i as f64 * step_height;

            rectangle(colour, [0.0, y, WIDTH, step_height], c.transform, g);
        }
    }

    fn draw_analemma_path(&self, c: Context, g: &mut G2d) {
        for i in 0..self.sun_positions.len() {
            let current_pos = self.screen_position(&self.sun_positions[i]);
            let next_pos =
                self.screen_position(&self.sun_positions[(i + 1) % self.sun_positions.len()]);

            let season_colour = self.season_colour(self.sun_positions[i].day);

            line(
                season_colour,
                2.0,
                [current_pos[0], current_pos[1], next_pos[0], next_pos[1]],
                c.transform,
                g,
            );
        }
    }

    fn draw_current_sun(&mut self, c: Context, g: &mut G2d) {
        if let Some(current_pos) = self.sun_positions.get(self.current_day as usize) {
            for i in 0..=365 {
                self.current_day += 1;
            }

            let screen_pos = self.screen_position(current_pos);

            ellipse(
                [1.0, 1.0, 1.0, 1.0],
                [screen_pos[0] - 8.0, screen_pos[1] - 8.0, 16.0, 16.0],
                c.transform,
                g,
            );
        }
    }

    fn get_date_info(&self) -> Vec<([f64; 2], &'static str)> {
        let key_days = [
            (80, "Vernal Equinox"),
            (172, "Summer Solstice"),
            (266, "Autumnal Equinox"),
            (355, "Winter Solstice"),
        ];

        let mut positions = Vec::new();

        for (day, name) in key_days.iter() {
            if let Some(pos) = self.sun_positions.get(*day) {
                let screen_pos = self.screen_position(pos);
                positions.push((screen_pos, *name));
            }
        }

        positions
    }

    fn draw_date_markers(&self, positions: &[([f64; 2], &str)], c: Context, g: &mut G2d) {
        for (screen_pos, _name) in positions {
            ellipse(
                [1.0, 0.5, 0.0, 0.8],
                [screen_pos[0] - 3.0, screen_pos[1] - 3.0, 6.0, 6.0],
                c.transform,
                g,
            );
        }
    }

    fn season_colour(&self, day: u32) -> [f32; 4] {
        match day {
            0..=79 => [0.4, 0.4, 1.0, 1.0],    //b
            80..=171 => [0.5, 1.0, 0.5, 1.0],  //g
            172..=265 => [1.0, 0.8, 0.3, 1.0], //r
            266..=365 => [1.0, 0.5, 0.2, 1.0],
            _ => [1.0, 1.0, 1.0, 1.0],
        }
    }
}

pub struct TextRenderer {
    glyphs: Glyphs,
}

impl TextRenderer {
    pub fn new(window: &mut PistonWindow) -> Result<Self, Box<dyn std::error::Error>> {
        let factory = window.create_texture_context();
        let glyphs = Glyphs::new("assets/Iosevka-Bold.ttf", factory, TextureSettings::new())?;

        Ok(TextRenderer { glyphs })
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        x: f64,
        y: f64,
        size: u32,
        colour: [f32; 4],
        context: Context,
        graphics: &mut G2d,
    ) -> Result<(), Box<dyn std::error::Error>> {
        text::Text::new_color(colour, size).draw(
            text,
            &mut self.glyphs,
            &context.draw_state,
            context.transform.trans(x, y),
            graphics,
        )?;
        Ok(())
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("ANALEMMA", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut simulation = AnnalemmaSimulation::new();

    let mut text_renderer = TextRenderer::new(&mut window).unwrap();

    let date_positions = simulation.get_date_info();

    while let Some(event) = window.next() {
        match event {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&event, |c, g, device| {
                    simulation.render(c, g);

                    render_text(50.0, 50.0, "Earth's Analemma", &mut text_renderer, &c, g);

                    for (pos, name) in &date_positions {
                        let offset = "Vernal Equinox";
                        if name == &offset {
                            render_text(pos[0] - 190.0, pos[1], name, &mut text_renderer, &c, g);
                        } else {
                            render_text(pos[0], pos[1], name, &mut text_renderer, &c, g);
                        }
                        simulation.draw_date_markers(&[(*pos, name)], c, g);
                    }
                    // if let Some((pos, name)) = date_positions.get(0) {
                    //     render_text(pos[0], pos[1], name, &mut text_renderer, &c, g);
                    // }

                    text_renderer.glyphs.factory.encoder.flush(device);
                });
            }
            Event::Loop(Loop::Update(UpdateArgs { dt })) => {
                simulation.update(dt);
            }
            _ => {}
        }
    }
}

fn render_text(
    pos_x: f64,
    pos_y: f64,
    text: &str,
    text_renderer: &mut TextRenderer,
    context: &Context,
    graphics: &mut G2d,
) {
    text_renderer
        .draw_text(
            text,
            pos_x,
            pos_y,
            24,
            [1.0, 1.0, 1.0, 1.0],
            *context,
            graphics,
        )
        .unwrap();
}
