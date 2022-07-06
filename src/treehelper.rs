// TODO get location
//
//
use crate::snippets::MESSAGE_STORAGE;
use lsp_types::Position;
use lsp_types::Range;
use tree_sitter::{Node, Point};
#[inline]
pub fn point_to_position(input: Point) -> Position {
    Position {
        line: input.row as u32,
        character: input.column as u32,
    }
}
#[inline]
fn position_to_point(input: Position) -> Point {
    Point {
        row: input.line as usize,
        column: input.character as usize,
    }
}
pub fn get_cmake_doc(location: Position, root: Node, source: &str) -> Option<String> {
    match get_positon_string(location, root, source) {
        Some(message) => MESSAGE_STORAGE.get(&message).map(|context| context.to_string()),
        None => None,
    }
}
pub fn get_positon_string(location: Position, root: Node, source: &str) -> Option<String> {
    let neolocation = position_to_point(location);
    let newsource: Vec<&str> = source.lines().collect();
    let mut course = root.walk();
    for child in root.children(&mut course) {
        // if is inside same line
        if neolocation.row <= child.end_position().row
            && neolocation.row >= child.start_position().row
        {
            if child.child_count() != 0 {
                return get_positon_string(location, child, source);
            }
            // if is the same line
            else if child.start_position().row == child.end_position().row
                && neolocation.column <= child.end_position().column
                && neolocation.column >= child.start_position().column
            {
                let h = child.start_position().row;
                let x = child.start_position().column;
                let y = child.end_position().column;

                let message = &newsource[h][x..y];
                //crate::notify_send(message, crate::Type::Info);
                return Some(message.to_string());
            }
        }
    }
    None
}
pub fn get_positon_range(location: Position, root: Node, source: &str) -> Option<Range> {
    let neolocation = position_to_point(location);
    //let newsource: Vec<&str> = source.lines().collect();
    let mut course = root.walk();
    for child in root.children(&mut course) {
        // if is inside same line
        if neolocation.row <= child.end_position().row
            && neolocation.row >= child.start_position().row
        {
            if child.child_count() != 0 {
                return get_positon_range(location, child, source);
            }
            // if is the same line
            else if child.start_position().row == child.end_position().row
                && neolocation.column <= child.end_position().column
                && neolocation.column >= child.start_position().column
            {
                //let h = child.start_position().row;
                //let x = child.start_position().column;
                //let y = child.end_position().column;

                //let message = &newsource[h][x..y];
                //crate::notify_send(message, crate::Type::Info);
                return Some(Range {
                    start: point_to_position(child.start_position()),
                    end: point_to_position(child.end_position()),
                });
            }
        }
    }
    None
}