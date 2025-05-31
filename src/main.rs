use nalgebra::{self as na, ComplexField, Vector2};
use piston_window::*;
use std::f64::consts::PI;

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
*       - Store in hashmap
*       - Piston drawing primitives to render path and sol pos
*
*
* TODO: 1. PISTON window
*       1a. - Coordinate system
*       2. Equation of time implementation [Horizontal component]
*       3. Solar Declination calculations [Vertical component]
*       4. Combine both effects to create figure eight
*       5. Step through states in order to animate.
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
            current_day: 0,
            animation_speed: 1.0,
        }
    }

    fn calculate_sun_position(day_of_year: f64) -> Vector2<f64> {
        let day_angle = (day_of_year / 365.25) * 2.0 * PI;

        let equation_of_time = Self::equation_of_time(day_of_year);

        let declination = Self::solar_declination(day_of_year);

        Vector2::new(equation_of_time, declination)
    }

    fn equation_of_time(day_of_year: f64) -> f64 {
        let day_angle = (day_of_year / 365.35) * 2.0 * PI;

        //orbital eccentricity effects
        let eccentricity_term = 7.655 * (2.0 * day_angle).sin();
        let obliquity_term = 9.873 * (day_angle + 3.588).sin();

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
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("ANALEMMA", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [WIDTH / 2.0, HEIGHT / 2.0, 100.0, 100.0],
                c.transform,
                g,
            );
        });
    }
}
