use macroquad::*;
use crate::Resizeable;
use crate::piece::Piece;

#[derive(Default, Debug, Clone)]
/// Collections of items that will be displayed on the same Row on screen that is ready to be
/// resized based on the current screen size.
pub struct Row {
    /// Current items in the Row
    items: Vec<Piece>,

    /// Raw width of all items currently in the `Row` without resize adjustment.
    ///
    /// To calculate the resize adjustment, `screen_width()` / `raw_width` is calculated and then
    /// used each draw frame to account for the resize
    raw_width: f32,

    /// Raw height of all items currently in the `Row` without resize adjustment.
    ///
    /// To calculate the resize adjustment, `screen_height()` / `raw_height` is calculated and then
    /// used each draw frame to account for the resize
    raw_height: f32,

    /// Number of pixels to put between each element for even horizontal spacing
    spacing: f32
}

impl Row {
    /// Initialize Row to `0` spacing with no items
    pub fn new() -> Self {
        Row::default()
    }

    /// Set the new spacing and recalculating the raw dimensions using the new spacing
    pub fn spacing(&mut self, spacing: f32) {
        // Calculate the raw width of only the current items
        let items_width: f32 = self.items.iter().map(|x| x.width()).sum();

        // Re-calculate raw width with new spacing. Spacing on the left and right borders
        self.raw_width = spacing * (self.items.len() + 1) as f32 + items_width;

        // Re-calculate raw width with new spacing. Spacing on the top and bottom borders
        self.raw_height = spacing * 2.0;

        // Update raw height with found max height of the row
        let mut max_height = 0.0;
        for item in self.items.iter() {
            if item.height() > max_height {
                max_height = item.height();
            }
        }
        self.raw_height += max_height;

        // Set the new spacing
        self.spacing = spacing;
    }

    /// Add an element to the current `Row` and update the raw dimensions based on the new element
    pub fn add(&mut self, item: Piece) {
        self.raw_width += item.width() + self.spacing;
        if (item.height() + self.spacing) > self.raw_height {
            self.raw_height = item.height();
        }

        self.items.push(item);

        info!("{}: {}", self.items.len(), self.raw_width);
    }
    
    /// Get the current adjusted height of the `Row`
    pub fn height(&self) -> f32 {
        // Calculate the adjustment fraction to fill the entire screen
        let adjustment = screen_width() as f32 / self.raw_width;

        self.raw_height * adjustment
    }

    pub fn draw(&self, location: Vec2) {
        let adjustment = screen_width() / self.raw_width;

        // Initialize the current X position from the given starting X position
        let mut curr_x = location.x() + self.spacing * adjustment;

        // Initialize the current Y position
        let curr_y = location.y() + self.spacing * adjustment;

        // Draw each item in the row with the found adjustment
        for item in &self.items {
            // Draw the texture at the calculated location
            // draw_texture_ex(item.texture(), curr_x, curr_y, WHITE, params);
            item.draw(vec2(curr_x, curr_y), adjustment);

            // let extension = item.width() - item.texture().width();
            let extension = 0.0;

            // draw_rectangle(curr_x, curr_y, item.width(), item.height(), GREEN);

            // Update X position for the current item
            curr_x += item.width() * adjustment + self.spacing * adjustment + extension;
        }
    }
}

/*
impl Resizeable for Row {
    fn draw(&self, location: Vec2, adjustment: f32) {
        // Initialize the current X position from the given starting X position
        let mut curr_x = location.x() + self.spacing * adjustment;

        // Initialize the current Y position
        let curr_y = location.y();

        // Draw each item in the row with the found adjustment
        for item in &self.items {
            // Draw the texture at the calculated location
            // draw_texture_ex(item.texture(), curr_x, curr_y, WHITE, params);
            item.draw(vec2(curr_x, curr_y), adjustment);

            // Update X position for the current item
            curr_x += item.width() * adjustment + self.spacing * adjustment;
        }
    }
}
*/

