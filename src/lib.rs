use rand::Rng;
use rand::thread_rng;
use std::fmt::format;
use std::fmt::Debug;
use std::str;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Size {
    width: f64,
    height: f64,
}

impl Size {
    fn new(width: f64) -> Size {
        Self {
            width,
            height: width,
        }
    }
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let pos = Point { x: 100.0, y: 200.0 };
    let size = Size::new(100.0);

    let pos_tri = Point { x: 300.0, y: 300.0 };
    let size_tri = Size::new(300.0);

    let pos_3 = Point { x: 100.0, y: 850.0 };
    let size_3 = Size::new(800.0);

    // draw_squre(&context, &pos, &size);

    // draw_triangle(&context, &pos_tri, &size_tri, "#000000");

    let color = rgb_to_hex();

    draw_sierpinski_triangle(&context, &pos_3, &size_3, &color, 20);

    Ok(())
}

fn rgb_to_hex() -> String {
    let mut rng = thread_rng();
    let color = (
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );

    format!("#{:02x}{:02x}{:02x}", color.0, color.1, color.2)
}

fn draw_triangle(context: &CanvasRenderingContext2d, cordinate: &Point, size: &Size, color: &str) {
    context.begin_path();
    context.move_to(cordinate.x, cordinate.y);

    context.line_to(cordinate.x + size.width, cordinate.y);
    context.line_to(cordinate.x + (size.width / 2.0), cordinate.y - size.height);

    context.close_path();
    context.stroke();
    context.set_fill_style(&color.into());
    context.fill();
}

fn draw_squre(context: &CanvasRenderingContext2d, cordinate: &Point, size: &Size) {
    context.move_to(cordinate.x, cordinate.y);
    context.begin_path();

    context.line_to(cordinate.x, cordinate.y + size.height);
    context.line_to(cordinate.x + size.width, cordinate.y + size.height);
    context.line_to(cordinate.x + size.width, cordinate.y);
    context.line_to(cordinate.x, cordinate.y);

    context.close_path();
    context.stroke();
    context.set_fill_style(&"#DDCC00".into());
    context.fill();
}

fn draw_sierpinski_triangle(
    context: &CanvasRenderingContext2d,
    cordinate: &Point,
    size: &Size,
    color: &str,
    levels: u8,
) {
    /* console::log_4(
        &JsValue::from_str("---Parent-->"),
        &JsValue::from_str(&format!("{:#?}", cordinate)),
        &JsValue::from_str(&format!("{:#?}", size)),
        &JsValue::from_str(&format!("{:#?}", levels)),
    ); */

    draw_triangle(context, cordinate, &size, color);

    if levels > 0 {
        let innerSize = Size {
            height: size.height / 2.0,
            width: size.width / 2.0,
        };

        let inner_color =rgb_to_hex();

        for inner in 0..3 {
            /*   console::log_3(
                &JsValue::from_str("Inner Level-->"),
                &JsValue::from_str(&format!("{}", inner)),
                &JsValue::from_str(&format!("{:#?}", innerSize)),
            ); */

            let posX = match inner {
                0 => cordinate.x,
                1 => cordinate.x + innerSize.width,
                _ => cordinate.x + (innerSize.width / 2.0),
            };

            let posY = match inner {
                0..=1 => cordinate.y,
                _ => cordinate.y - innerSize.height,
            };

            let pp = Point { x: posX, y: posY };

            /*  console::log_3(
                &JsValue::from_str("POS X , Y-->"),
                &JsValue::from_str(&format!("{}", pp.x)),
                &JsValue::from_str(&format!("{}", pp.y)),
            ); */

            draw_sierpinski_triangle(context, &pp, &innerSize, &inner_color, levels - 1);
        }
    }
}
