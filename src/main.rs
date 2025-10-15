mod config;
mod elements;
mod menu;
mod svg_export;

use macroquad::prelude::*;
use macroquad::text::{measure_text, Font};
use macroquad::input::get_char_pressed;
use config::window_conf;
use elements::{Element, get_elements};
use menu::Menu;
use svg_export::export_to_svg;
fn draw_formula(formula: &str, x: f32, y: f32, rotation: f32, font: &Font) {
    let mut current_x = x;
    let normal_size = 40.0;
    let sub_super_size = 24.0;
    let sub_super_offset = 10.0;

    let chars: Vec<char> = formula.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        if ch == '_' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit() {
            let digit = chars[i + 1];
            draw_text_ex(
                &digit.to_string(),
                current_x,
                y + sub_super_offset,
                TextParams {
                    font: Some(font),
                    font_size: sub_super_size as u16,
                    rotation: rotation.to_radians(),
                    color: BLACK,
                    ..Default::default()
                },
            );
            let text_width = measure_text(&digit.to_string(), Some(font), sub_super_size as u16, 1.0).width;
            current_x += text_width;
            i += 2;
        } else if ch == '^' {
            let mut text = String::new();
            let mut chars_consumed = 1;
            if i + 1 < chars.len() {
                let next_ch = chars[i + 1];
                if next_ch == '+' || next_ch == '-' {
                    text.push(next_ch);
                    chars_consumed += 1;
                    if i + 2 < chars.len() && chars[i + 2].is_ascii_digit() {
                        text.push(chars[i + 2]);
                        chars_consumed += 1;
                    }
                } else if next_ch.is_ascii_digit() {
                    text.push(next_ch);
                    chars_consumed += 1;
                }
            }
            if !text.is_empty() {
                draw_text_ex(
                    &text,
                    current_x,
                    y - sub_super_offset,
                    TextParams {
                        font: Some(font),
                        font_size: sub_super_size as u16,
                        rotation: rotation.to_radians(),
                        color: BLACK,
                        ..Default::default()
                    },
                );
                let text_width = measure_text(&text, Some(font), sub_super_size as u16, 1.0).width;
                current_x += text_width;
            }
            i += chars_consumed;
        } else {
            draw_text_ex(
                &ch.to_string(),
                current_x,
                y,
                TextParams {
                    font: Some(font),
                    font_size: normal_size as u16,
                    rotation: rotation.to_radians(),
                    color: BLACK,
                    ..Default::default()
                },
            );
            let text_width = measure_text(&ch.to_string(), Some(font), normal_size as u16, 1.0).width;
            current_x += text_width;
            i += 1;
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut formulas = Vec::<Element>::new();
    let mut is_form_create = false;
    let mut current_formula: Element = Element::new();

    let elements = get_elements();
    let mut menu = Menu::new(elements);
    let font = load_ttf_font("FreeSerif.otf")
        .await
        .expect("Error to load font");
    loop {
        let mouse_x = mouse_position().0;
        let mouse_y = mouse_position().1;

        // Handle text input for subscripts and superscripts
        if is_key_pressed(KeyCode::Backspace) && !is_form_create && !formulas.is_empty() {
            formulas.pop();
        } else if is_key_pressed(KeyCode::X) && is_key_down(KeyCode::LeftControl) && !is_form_create {
            formulas.clear();
        } else if let Some(ch) = get_char_pressed() {
            if (ch.is_ascii_digit() && ch >= '1' && ch <= '9') || ch == '+' || ch == '-' || ch == '_' || ch == '^' {
                if ch.is_ascii_digit() && !current_formula.val.ends_with('_') && !current_formula.val.ends_with('^') {
                    current_formula.val.insert(0, ch);
                } else if ch == '+' || ch == '-' {
                    if current_formula.val.ends_with('^') {
                        current_formula.val.push(ch);
                    } else {
                        current_formula.val.insert(0, ch);
                    }
                } else {
                    current_formula.val.push(ch);
                }
            }
        }

        // Toggle menu with TAB
        if is_key_pressed(KeyCode::Tab) {
            menu.toggle();
        }

        // Handle menu button clicks
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(selected_element) = menu.handle_click(mouse_x, mouse_y) {
                current_formula = selected_element;
                is_form_create = true;
            } else if is_form_create && !menu.show_menu {
                // Place element on canvas when element is selected and menu is not shown
                current_formula.x = mouse_x;
                current_formula.y = mouse_y;
                formulas.push(current_formula.clone());
                is_form_create = false;
                current_formula = Element::new();
            }
        }

        // Confirm formula with Enter
        if is_key_pressed(KeyCode::Enter) && is_form_create {
            current_formula.x = mouse_x;
            current_formula.y = mouse_y;
            formulas.push(current_formula.clone());
            is_form_create = false;
            current_formula = Element::new();
        }

        // Keyboard shortcuts 1-9 for specific elements as requested (only when not creating a formula)
        if !is_form_create {
            if is_key_pressed(KeyCode::Key1) {
                current_formula = Element::with_value("CH4");
                is_form_create = true;
            }
            if is_key_pressed(KeyCode::Key2) {
                current_formula = Element::with_value("CH3");
                is_form_create = true;
            }
            if is_key_pressed(KeyCode::Key3) {
                current_formula = Element::with_value("CH2");
                is_form_create = true;
            }
            if is_key_pressed(KeyCode::Key4) {
                current_formula = Element::with_value("CH");
                is_form_create = true;
            }
            if is_key_pressed(KeyCode::Key5) {
                current_formula = Element::with_value("-");
                is_form_create = true;
            }
            if is_key_pressed(KeyCode::Key6) {
                current_formula = Element::with_value("=");
                is_form_create = true;
            }
            if is_key_pressed(KeyCode::Key7) {
                current_formula = Element::with_value("≡");
                is_form_create = true;
            }
            if is_key_pressed(KeyCode::Key8) {
                current_formula = Element::with_value("+");
                is_form_create = true;
            }
            if is_key_pressed(KeyCode::Key9) {
                current_formula = Element::with_value("⌬");
                is_form_create = true;
            }
        }
                
        let (_, scroll_y) = mouse_wheel();
        if scroll_y > 0.0 {
            current_formula.rotations += 5.0;
        }
        if scroll_y < 0.0 {
            current_formula.rotations -= 5.0;
        }
       // let h = screen_height();
        clear_background(WHITE);

        // Draw menu
        menu.draw(&font);

        if is_form_create {
            draw_formula(&current_formula.val, mouse_x, mouse_y, current_formula.rotations, &font);
        }
        for formula in &formulas {
            // Only draw formulas that are not in the menu area when menu is shown
            let should_draw = if menu.show_menu {
                let screen_height = screen_height();
                let menu_height = ((menu.elements.len() as f32 / menu.buttons_per_row as f32).ceil()) * menu.button_height;
                formula.y < screen_height - menu_height
            } else {
                true
            };

            if should_draw {
                draw_formula(&formula.val, formula.x, formula.y, formula.rotations, &font);
            }
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
