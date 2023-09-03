use deckspacer2023::*;

const PIN_DIAMETER: f64 = 0.25;
const HOLE_DIAMETER: f64 = 0.25;
const LEFT_MARGIN: f64 = 1.0;
const JOIST_WIDTH: f64 = 1.5;
const JIG_WIDTH: f64 = 5.0;
const JIG_LENGTH: f64 = 60.0;
const BOARD_WIDTH: f64 = 5.5;
const BOTTOM_MARGIN: f64 = 1.0;
const BOARD_SCREW_MARGIN: f64 = 1.0;
const BOARDS: i32 = 11;
const HALF: f64 = 0.5;
const ORIGIN_X: f64 = -20.0;
const ORIGIN_Y: f64 = -20.0;

fn main() {
    // create a closure to add a line

    let mut drawing = dxf::Drawing::new();

    // draw a rectangle 4" wide by 60" tall
    add_lines!(
        drawing,
        (ORIGIN_X, ORIGIN_Y),
        (ORIGIN_X + JIG_WIDTH, ORIGIN_Y),
        (ORIGIN_X + JIG_WIDTH, ORIGIN_Y + JIG_LENGTH),
        (ORIGIN_X, ORIGIN_Y + JIG_LENGTH),
        (ORIGIN_X, ORIGIN_Y),
    );

    // draw a line 1" from the left of the rectangle
    add_line!(
        drawing,
        (ORIGIN_X + LEFT_MARGIN, ORIGIN_Y),
        (ORIGIN_X + LEFT_MARGIN, ORIGIN_Y + JIG_LENGTH)
    );

    // draw a line 1.5" over from that to the right
    add_line!(
        drawing,
        (ORIGIN_X + LEFT_MARGIN + JOIST_WIDTH, ORIGIN_Y),
        (ORIGIN_X + LEFT_MARGIN + JOIST_WIDTH, ORIGIN_Y + JIG_LENGTH)
    );

    // Add the 1/4" pins with 5.75" spacing
    for i in 0..BOARDS {
        // yf is the y coordinate of the center of the pin
        let pin_y = ORIGIN_Y + BOTTOM_MARGIN + (i as f64 * (BOARD_WIDTH + PIN_DIAMETER));
        let pin_r = PIN_DIAMETER * HALF;
        let pin_x = ORIGIN_X + LEFT_MARGIN - pin_r;
        add_circle!(drawing, (pin_x, pin_y), pin_r);

        if i < BOARDS - 1 {
            let abs_board_start = pin_y + pin_r;
            let hole_x = ORIGIN_X + LEFT_MARGIN + JOIST_WIDTH * HALF;
            let hole_y = abs_board_start + BOARD_SCREW_MARGIN;
            let hole_r = HOLE_DIAMETER * HALF;
            add_circle!(drawing, (hole_x, hole_y), hole_r);

            let hole_y = abs_board_start + BOARD_WIDTH - BOARD_SCREW_MARGIN;
            add_circle!(drawing, (hole_x, hole_y), hole_r);

            add_line!(
                drawing,
                (ORIGIN_X - 20.0, abs_board_start),
                (ORIGIN_X + 20.0, abs_board_start)
            );
            add_line!(
                drawing,
                (ORIGIN_X - 20.0, abs_board_start + BOARD_WIDTH),
                (ORIGIN_X + 20.0, abs_board_start + BOARD_WIDTH)
            );
        }
    }

    // `added_entity_ref` is a reference to the newly added entity
    drawing.save_file("output.dxf").unwrap();
}
