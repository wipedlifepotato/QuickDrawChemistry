use macroquad::prelude::*;
use crate::elements::Element;

pub struct Menu {
    pub show_menu: bool,
    pub button_width: f32,
    pub button_height: f32,
    pub buttons_per_row: usize,
    pub elements: Vec<String>,
}

impl Menu {
    pub fn new(elements: Vec<String>) -> Self {
        Menu {
            show_menu: false,
            button_width: 50.0, // Increased for longer formulas
            button_height: 30.0,
            buttons_per_row: 12, // Reduced columns to fit longer text
            elements,
        }
    }

    pub fn toggle(&mut self) {
        self.show_menu = !self.show_menu;
    }

    pub fn handle_click(&mut self, mouse_x: f32, mouse_y: f32) -> Option<Element> {
        if !self.show_menu {
            return None;
        }

        let screen_height = screen_height();
        let menu_height = ((self.elements.len() as f32 / self.buttons_per_row as f32).ceil()) * self.button_height;

        for (i, element) in self.elements.iter().enumerate() {
            let row = (i / self.buttons_per_row) as f32;
            let col = (i % self.buttons_per_row) as f32;
            let x = col * self.button_width;
            let y = screen_height - menu_height + row * self.button_height; // Position at bottom

            if mouse_x >= x && mouse_x <= x + self.button_width &&
               mouse_y >= y && mouse_y <= y + self.button_height {
                self.show_menu = false; // Hide menu after selection
                return Some(Element::with_value(element));
            }
        }
        None
    }

    pub fn draw(&self, font: &Font) {
        if !self.show_menu {
            return;
        }

        let screen_height = screen_height();
        let screen_width = screen_width();
        let menu_height = ((self.elements.len() as f32 / self.buttons_per_row as f32).ceil()) * self.button_height;

        // Draw semi-transparent background for the entire menu area
        draw_rectangle(0.0, screen_height - menu_height, screen_width, menu_height, Color::new(0.0, 0.0, 0.0, 0.7));

        for (i, element) in self.elements.iter().enumerate() {
            let row = (i / self.buttons_per_row) as f32;
            let col = (i % self.buttons_per_row) as f32;
            let x = col * self.button_width;
            let y = screen_height - menu_height + row * self.button_height; // Position at bottom

            // Draw button background with better contrast
            draw_rectangle(x, y, self.button_width, self.button_height, Color::new(0.8, 0.8, 0.8, 0.9));

            // Draw button border
            draw_rectangle_lines(x, y, self.button_width, self.button_height, 1.0, DARKGRAY);

            // Draw button text with smaller font for longer formulas
            let font_size = if element.len() > 3 { 14 } else { 18 };
            draw_text_ex(
                element,
                x + 2.0,
                y + self.button_height - 2.0,
                TextParams {
                    font: Some(font),
                    font_size,
                    color: BLACK,
                    ..Default::default()
                },
            );
        }
    }
}