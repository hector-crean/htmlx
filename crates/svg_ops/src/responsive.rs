use std::borrow::BorrowMut;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::{self, fs::File};
use usvg::{fontdb, AspectRatio, Node, NonZeroRect, Options, Rect, Transform, Tree};

fn main<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let svg_data = fs::read_to_string("input.svg").expect("Unable to read file");

    // Parse the SVG file using usvg
    let opt = Options::default();

    let font_db = fontdb::Database::new();
    let mut tree = Tree::from_str(&svg_data, &opt, &font_db).expect("Unable to parse SVG");

    // for node in tree.root().children() {
    //     if let Node::Group(box svg) = &mut *node.borrow_mut() {

    //         svg.view_box = Some(usvg::ViewBox {
    //             rect: NonZeroRect::from_xywh(0.0, 0.0, 500.0, 500.0)
    //                 .expect("Invalid viewBox values"),
    //             aspect: AspectRatio {
    //                 defer: false,
    //                 align: usvg::Align::XMidYMid,
    //                 slice: false,
    //             },
    //         });
    //         break;
    //     }
    // }

    Ok(())
}
