use nalgebra::{self as na, ComplexField};
use piston_window::*;
use std::collections::HashMap;

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
const GRAVITY: f64 = 0.9;

struct Body {
    position: na::Vector2<f64>,
    velocity: na::Vector2<f64>,
    acceleration: na::Vector2<f64>,
    radius: f64,
    mass: f64,
}

impl Body {
    fn new(&mut self) -> Self {
        let x = WIDTH / 2.0;
        let y = HEIGHT / 2.0;

        let angle = 1.0;
        let speed = 1.0;
        let vx = angle.cos() * speed;
        let vy = angle.sin() * speed;

        Self {
            position: na::Vector2::new(x, y),
            velocity: na::Vector2::new(vx, vy),
            acceleration: na::Vector2::new(0.0, 0.0),
            radius: (10.0),
            mass: (10.0),
        }
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
