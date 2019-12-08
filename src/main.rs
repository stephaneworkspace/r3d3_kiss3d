extern crate astro;
extern crate kiss3d;
extern crate nalgebra as na;

use astro::*;
use kiss3d::window::Window;
use na::{Translation2, UnitComplex};

fn main() {
    // Init Date 01-01-2020
    let day_of_month = time::DayOfMonth {
        day: 1,
        hr: 0,
        min: 0,
        sec: 0.0,
        time_zone: 0.0,
    };
    let date = time::Date {
        year: 2020,
        month: 1,
        decimal_day: time::decimal_day(&day_of_month),
        cal_type: time::CalType::Gregorian,
    };
    let julian_day = time::julian_day(&date);

    let delta_t = time::delta_t(date.year as i32, date.month);
    let julian_ephem_day = time::julian_ephemeris_day(julian_day, delta_t);

    // 2 animation
    let mut window = Window::new("Kiss3d: rectangle");
    let mut rect = window.add_rectangle(50.0, 150.0);
    let mut circ = window.add_circle(50.0);
    // Translation2 200.0 -> distance from center 300.0 = max ~
    circ.append_translation(&Translation2::new(50.0, 000.0));

    rect.set_color(0.0, 1.0, 0.0);
    circ.set_color(0.0, 0.0, 1.0);

    // Speed rotation
    let rot_rect = UnitComplex::new(0.014);
    // let rot_circ = UnitComplex::new(-0.014);
    let rot_circ = UnitComplex::new(-0.101);

    while window.render() {
        rect.prepend_to_local_rotation(&rot_rect);
        circ.append_rotation(&rot_circ);
    }
}
