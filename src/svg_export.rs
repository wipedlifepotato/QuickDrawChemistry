use svg::node::element::Text;
use svg::Document;
use crate::elements::Element;
use crate::config::window_conf;

fn to_superscript(c: char) -> char {
    match c {
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        '+' => '⁺',
        '-' => '⁻',
        _ => c,
    }
}

fn to_subscript(c: char) -> char {
    match c {
        '0' => '₀',
        '1' => '₁',
        '2' => '₂',
        '3' => '₃',
        '4' => '₄',
        '5' => '₅',
        '6' => '₆',
        '7' => '₇',
        '8' => '₈',
        '9' => '₉',
        '+' => '₊',
        '-' => '₋',
        _ => c,
    }
}

pub fn export_to_svg(elements: &[Element], filename: &str) {
    let conf = window_conf();
    let width = conf.window_width as f64;
    let height = conf.window_height as f64;

    let mut doc = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("width", width)
        .set("height", height);

    for el in elements {
        let e = el.val.clone();
        let mut out_str = String::new();
        let mut is_upper = false;
        let mut is_lower = false;

        for chr in e.chars() {
            if chr == '_' {
                is_lower = true;
                is_upper = false;
                continue;
            }
            if chr == '^' {
                is_upper = true;
                is_lower = false;
                continue;
            }

            if is_upper {
                out_str.push(to_superscript(chr));
            } else if is_lower {
                out_str.push(to_subscript(chr));
            } else {
                out_str.push(chr);
            }

        }

        let text = Text::new(out_str)
            .set("x", el.x as f64)
            .set("y", el.y as f64)
            .set("fill", "black")
            .set("font-size", 40)
            .set(
                "transform",
                format!("rotate({},{},{})", el.rotations, el.x as f64, el.y as f64),
            );

        doc = doc.add(text);
    }

    svg::save(filename, &doc).expect("Cannot save SVG file");
}

