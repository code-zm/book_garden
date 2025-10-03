use ratatui::{
    style::Color,
    widgets::canvas::{Canvas, Line, Points},
};

pub fn create_plant_canvas(progress: f64) -> Canvas<'static, impl Fn(&mut ratatui::widgets::canvas::Context)> {
    Canvas::default()
        .marker(ratatui::symbols::Marker::Braille)
        .paint(move |ctx| {
            draw_plant(ctx, progress);
        })
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
}

fn draw_plant(ctx: &mut ratatui::widgets::canvas::Context, progress: f64) {
    let progress = progress.clamp(0.0, 1.0);

    // Pot at the bottom
    draw_pot(ctx);

    // Main stem - grows smoothly from the start
    let stem_height = 15.0 + (progress * 65.0);
    let stem_x = 50.0;

    // Only draw stem if there's any progress at all
    if progress > 0.0 {
        // Draw 4 parallel lines for a thicker stem
        for offset in [-0.75, -0.25, 0.25, 0.75] {
            ctx.draw(&Line {
                x1: stem_x + offset,
                y1: 15.0,
                x2: stem_x + offset,
                y2: stem_height,
                color: Color::Green,
            });
        }
    }

    // Add leaves at different heights - grow smoothly as stem passes
    let leaf_grow_distance = 8.0; // Distance needed for leaf to fully grow

    if stem_height >= 25.0 {
        let growth = ((stem_height - 25.0) / leaf_grow_distance).clamp(0.0, 1.0);
        draw_leaf(ctx, stem_x, 25.0, -1.0, growth); // Left leaf
        draw_leaf(ctx, stem_x, 25.0, 1.0, growth);  // Right leaf
    }

    if stem_height >= 35.0 {
        let growth = ((stem_height - 35.0) / leaf_grow_distance).clamp(0.0, 1.0);
        draw_leaf(ctx, stem_x, 35.0, 1.0, growth);
        draw_leaf(ctx, stem_x, 35.0, -1.0, growth);
    }

    if stem_height >= 45.0 {
        let growth = ((stem_height - 45.0) / leaf_grow_distance).clamp(0.0, 1.0);
        draw_leaf(ctx, stem_x, 45.0, -1.0, growth);
        draw_leaf(ctx, stem_x, 45.0, 1.0, growth);
    }

    if stem_height >= 55.0 {
        let growth = ((stem_height - 55.0) / leaf_grow_distance).clamp(0.0, 1.0);
        draw_leaf(ctx, stem_x, 55.0, 1.0, growth);
        draw_leaf(ctx, stem_x, 55.0, -1.0, growth);
    }

    if stem_height >= 65.0 {
        let growth = ((stem_height - 65.0) / leaf_grow_distance).clamp(0.0, 1.0);
        draw_leaf(ctx, stem_x, 65.0, -1.0, growth);
        draw_leaf(ctx, stem_x, 65.0, 1.0, growth);
    }

    // Flower grows from 85% to 100%
    if progress > 0.85 {
        let flower_growth = ((progress - 0.85) / 0.15).clamp(0.0, 1.0);
        draw_flower(ctx, stem_x, stem_height, flower_growth);
    }
}

fn draw_pot(ctx: &mut ratatui::widgets::canvas::Context) {
    // Pot outline
    ctx.draw(&Line {
        x1: 35.0,
        y1: 15.0,
        x2: 40.0,
        y2: 5.0,
        color: Color::Yellow,
    });
    ctx.draw(&Line {
        x1: 40.0,
        y1: 5.0,
        x2: 60.0,
        y2: 5.0,
        color: Color::Yellow,
    });
    ctx.draw(&Line {
        x1: 60.0,
        y1: 5.0,
        x2: 65.0,
        y2: 15.0,
        color: Color::Yellow,
    });
    ctx.draw(&Line {
        x1: 65.0,
        y1: 15.0,
        x2: 35.0,
        y2: 15.0,
        color: Color::Yellow,
    });

    // Soil
    ctx.draw(&Line {
        x1: 37.0,
        y1: 12.0,
        x2: 63.0,
        y2: 12.0,
        color: Color::Rgb(139, 69, 19),
    });
}

fn draw_leaf(ctx: &mut ratatui::widgets::canvas::Context, stem_x: f64, y: f64, direction: f64, growth: f64) {
    let leaf_length = 5.0 * growth;
    let leaf_end_x = stem_x + (direction * leaf_length);

    // Only draw if there's any growth
    if growth > 0.0 {
        // Main leaf line
        ctx.draw(&Line {
            x1: stem_x,
            y1: y,
            x2: leaf_end_x,
            y2: y + (2.0 * growth),
            color: Color::Green,
        });

        // Leaf tip - only appears when growth is substantial
        if growth > 0.3 {
            ctx.draw(&Line {
                x1: leaf_end_x,
                y1: y + (2.0 * growth),
                x2: leaf_end_x - (direction * 1.5 * growth),
                y2: y + (3.5 * growth),
                color: Color::Green,
            });

            ctx.draw(&Line {
                x1: leaf_end_x,
                y1: y + (2.0 * growth),
                x2: leaf_end_x - (direction * 1.5 * growth),
                y2: y + (0.5 * growth),
                color: Color::Green,
            });
        }
    }
}

fn draw_flower(ctx: &mut ratatui::widgets::canvas::Context, x: f64, y: f64, growth: f64) {
    // Flower center grows first (0-30% of growth)
    let center_growth = (growth / 0.3).clamp(0.0, 1.0);

    // Petals start growing after center is established (30-100% of growth)
    if growth > 0.3 {
        let petal_growth = ((growth - 0.3) / 0.7).clamp(0.0, 1.0);

        // Create 12 petals arranged in a circle (sunflower-like)
        let num_petals = 12;
        let petal_length = 10.0 * petal_growth;

        for i in 0..num_petals {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / (num_petals as f64);
            let petal_x = x + angle.cos() * petal_length;
            let petal_y = y + angle.sin() * petal_length;

            // Calculate perpendicular for thickness
            let dx = petal_x - x;
            let dy = petal_y - y;
            let len = (dx * dx + dy * dy).sqrt();

            let (perp_x, perp_y) = if len > 0.0 {
                (-dy / len, dx / len)
            } else {
                (0.0, 0.0)
            };

            // Draw thick petal lines
            if petal_growth > 0.1 && len > 0.0 {
                // Draw 4 parallel lines for thickness
                for offset in [-0.75, -0.25, 0.25, 0.75] {
                    ctx.draw(&Line {
                        x1: x + perp_x * offset,
                        y1: y + perp_y * offset,
                        x2: petal_x + perp_x * offset,
                        y2: petal_y + perp_y * offset,
                        color: Color::Magenta,
                    });
                }
            }

            // Draw petal tips
            if petal_growth > 0.3 {
                ctx.draw(&Points {
                    coords: &[
                        (petal_x, petal_y),
                        (petal_x + angle.cos() * 1.5, petal_y + angle.sin() * 1.5),
                        (petal_x + angle.cos() * 2.0, petal_y + angle.sin() * 2.0),
                        (petal_x + perp_x * 0.5, petal_y + perp_y * 0.5),
                        (petal_x - perp_x * 0.5, petal_y - perp_y * 0.5),
                        (petal_x + perp_x * 1.0, petal_y + perp_y * 1.0),
                        (petal_x - perp_x * 1.0, petal_y - perp_y * 1.0),
                    ],
                    color: Color::Magenta,
                });
            }
        }

        // Fill in the ring around the center with purple
        if petal_growth > 0.2 {
            let ring_radius = 4.0 * petal_growth;
            for angle_deg in (0..360).step_by(15) {
                let angle = (angle_deg as f64).to_radians();
                let ring_x = x + angle.cos() * ring_radius;
                let ring_y = y + angle.sin() * ring_radius;

                ctx.draw(&Points {
                    coords: &[
                        (ring_x, ring_y),
                        (ring_x + 0.5, ring_y),
                        (ring_x - 0.5, ring_y),
                        (ring_x, ring_y + 0.5),
                        (ring_x, ring_y - 0.5),
                    ],
                    color: Color::Magenta,
                });
            }
        }
    }

    // Draw center on top so it's always visible
    if center_growth > 0.0 {
        // Larger yellow center
        for radius in [0.0, 1.0, 2.0, 3.0] {
            for angle_deg in (0..360).step_by(30) {
                let angle = (angle_deg as f64).to_radians();
                let cx = x + angle.cos() * radius * center_growth;
                let cy = y + angle.sin() * radius * center_growth;

                ctx.draw(&Points {
                    coords: &[
                        (cx, cy),
                        (cx + 0.5, cy),
                        (cx - 0.5, cy),
                        (cx, cy + 0.5),
                        (cx, cy - 0.5),
                    ],
                    color: Color::Yellow,
                });
            }
        }
    }
}
