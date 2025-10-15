mod config; 
use macroquad::prelude::*;
use config::window_conf;

#[derive(Debug, Clone)]
struct Element {
    x: f32,
    y: f32,
    rotations: f32,
    val: String,
}

macro_rules! BindKey {
    ($key_code:expr, $formula:expr, $current:expr, $turn:expr) => {
        if is_key_pressed($key_code) {
            $current = Element{x: 0.0, y: 0.0, rotations: 0.0, val: String::from($formula)};
            $turn = true;
            dbg!($key_code);
        }        
    }   
}
use svg::node::element::{Text, Rectangle, Line};
use svg::Document;

fn export_to_svg(elements: &[Element], filename: &str) {
    let mut doc = Document::new()
        .set("viewBox", (0, 0, 800, 600))
        .set("width", 800)
        .set("height", 600);

    for el in elements {
        let text = Text::new(el.val.clone())
            .set("x", el.x)
            .set("y", el.y)
            .set("fill", "black")
            .set("font-size", 40)
            .set("transform", format!("rotate({},{},{})", el.rotations, el.x, el.y));

        doc = doc.add(text);
    }

    svg::save(filename, &doc).expect("Cannot save SVG file");
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut formulas = Vec::<Element>::new();
    let mut is_form_create = false;
    let mut _current_formula = String::new();
    const METHAN:&str  = "CH4";
    const METHANRADICAL:&str  = "CH3";
    const MET2:&str  = "CH2";
    const MET:&str = "CH"; 
    const BOND1: &str = "-";
    const BOND2: &str = "=";
    const BOND3: &str = "≡";
    const BENZENE: &str = "⌬";
    const SPIRIT: &str = "OH";
    let mut _current_formula: Element = Element{x:0.0, y:0.0, rotations: 0.0, val: String::from("")};
    let font = load_ttf_font("FreeSerif.otf")
        .await
        .expect("Error to load font");
    loop {
        let mouse_x = mouse_position().0;
        let mouse_y = mouse_position().1;
        BindKey!(KeyCode::Key1, METHAN, _current_formula, is_form_create);
        BindKey!(KeyCode::Key2, METHANRADICAL, _current_formula, is_form_create);
        BindKey!(KeyCode::Key3, MET2, _current_formula, is_form_create);
        BindKey!(KeyCode::Key4, MET, _current_formula, is_form_create);
        BindKey!(KeyCode::Key5, BOND1, _current_formula, is_form_create);
        BindKey!(KeyCode::Key6, BOND2, _current_formula, is_form_create);
        BindKey!(KeyCode::Key7, BOND3, _current_formula, is_form_create);
        BindKey!(KeyCode::Key8, BENZENE, _current_formula, is_form_create);
        BindKey!(KeyCode::Key9, SPIRIT, _current_formula, is_form_create);
                
        dbg!(&_current_formula);
        if is_mouse_button_pressed(MouseButton::Left) {
            _current_formula.x = mouse_x;
            _current_formula.y = mouse_y;
            formulas.push(_current_formula);
            is_form_create = false;
            _current_formula = Element{x:0.0, y:0.0, rotations: 0.0, val: String::from("")};
        }
        let (_, scroll_y) = mouse_wheel();
        if scroll_y > 0.0 {
            _current_formula.rotations += 5.0;
        }
        if scroll_y < 0.0 {
            _current_formula.rotations -= 5.0;
        }
       // let h = screen_height();
        clear_background(WHITE);
        if is_form_create {
            dbg!(mouse_x);
            dbg!(mouse_y);
            dbg!(_current_formula.val.to_string());
            draw_text_ex(
                &_current_formula.val.to_string(),
                mouse_x,
                mouse_y,
                TextParams {
                    font: Some(&font),
                    font_size: 40,
                    rotation: _current_formula.rotations.to_radians(),
                    color: BLACK,
                    ..Default::default()
                },
            );
        }
        for formula in &formulas {
            draw_text_ex(
                &formula.val.to_string(),
                formula.x,
                formula.y,
                TextParams {
                    font: Some(&font),
                    font_size: 40,
                    rotation: formula.rotations.to_radians(),
                    color: BLACK,
                    ..Default::default()
                },
            );
        }
        //let w = screen_width();
        //
        if is_key_pressed(KeyCode::S) {
            export_to_svg(&formulas, "output.svg");
            println!("Saved SVG!");
        }


        next_frame().await
    }
}
