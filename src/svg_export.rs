use svg::node::element::Text;
use svg::Document;
use crate::elements::Element;
use crate::config::window_conf;

pub fn export_to_svg(elements: &[Element], filename: &str) {
    let conf = window_conf();
    let width = conf.window_width as f64;
    let height = conf.window_height as f64;

    let mut doc = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("width", width)
        .set("height", height);

    for el in elements {
        let text = Text::new(el.val.clone())
            .set("x", el.x as f64)
            .set("y", el.y as f64)
            .set("fill", "black")
            .set("font-size", 40)
            .set("transform", format!("rotate({},{},{})", el.rotations, el.x as f64, el.y as f64));

        doc = doc.add(text);
    }

    svg::save(filename, &doc).expect("Cannot save SVG file");
}