use plotters::prelude::*;


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize SpringMass system
    let spring_mass_ptr = unsafe {
        SpringMass_create(200.0, 0.0, 161.0, 0.0)
    };

    // Generate SpringMass trajectory
    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    for t in 0..501 {
        unsafe {
            SpringMass_step(spring_mass_ptr);
            let mut x: f64 = 0.0;
            let mut y: f64 = 0.0;
            SpringMass_getConfiguration(spring_mass_ptr, t as i32, &mut x as *mut f64, &mut y as *mut f64);
            positions.push(x);
            velocities.push(y);
        }
    }

    // Destroy SpringMass system
    unsafe {
        SpringMass_destroy(spring_mass_ptr);
    }

    // Plotting code
    let root = BitMapBackend::new("mass.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("SpringMass Trajectory", ("sans-serif", 10).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..500f32, -200f32..200f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            positions.iter().enumerate().map(|(t, &x)| (t as f32, x as f32)),
            &RED,
        ))?
        .label("Position")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            velocities.iter().enumerate().map(|(t, &v)| (t as f32, v as f32)),
            &BLUE,
        ))?
        .label("Velocity")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    let spring_mass_ptr = unsafe {
        SpringDamperMass_create(200.0, 0.0, 161.0, 0.0,1.0)
    };

    // Generate SpringMass trajectory
    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    for t in 0..501 {
        unsafe {
            SpringDamperMass_step(spring_mass_ptr);
            let mut x: f64 = 0.0;
            let mut y: f64 = 0.0;
            SpringMass_getConfiguration(spring_mass_ptr, t as i32, &mut x as *mut f64, &mut y as *mut f64);
            positions.push(x);
            velocities.push(y);
        }
    }

    unsafe {
        SpringDamperMass_destroy(spring_mass_ptr);
    }

    // Plotting code
    let root = BitMapBackend::new("mass_damper.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("SpringMassDamp Trajectory", ("sans-serif", 10).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..500f32, -200f32..200f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            positions.iter().enumerate().map(|(t, &x)| (t as f32, x as f32)),
            &RED,
        ))?
        .label("Position")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            velocities.iter().enumerate().map(|(t, &v)| (t as f32, v as f32)),
            &BLUE,
        ))?
        .label("Velocity")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}