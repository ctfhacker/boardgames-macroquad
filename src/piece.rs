use macroquad::*;
use crate::Resizeable;
use crate::assets::ASSETS;

/// Indiviual piece with potential children pieces that are drawn in relation to this `Piece`s 
/// location
#[derive(Debug, Clone)]
pub struct Piece {
    /// Texture for the current piece
    texture: u32,

    /// Vec of children that are drawn in relation to this `Piece`
    /// (Piece, Relation to parent, Relation to self)
    ///
    /// If the piece wanted to be 100% x of the parent, 40% of the y of the parent, and then
    /// centered on the child's texture itself
    ///
    /// (Parent)
    /// .-----------------.
    /// |                 | (Child)
    /// |               .---.
    /// |               |   |
    /// |               .---.
    /// |                 |
    /// .-----------------.
    ///
    /// ```
    /// let child = Piece::new(child_texture)
    /// parent.add_child(child, vec2(100., 40.), vec2(-.5, 0));
    /// ```
    ///
    ///
    children: Vec<(Piece, Vec2, Vec2)>,
}

impl Piece {
    pub fn new(texture: u32) -> Self {
        Piece {
            texture,
            children: Vec::new()
        }
    }

    /// Add a child `Piece` to the current `Piece` that will be drawn on top of the current `Piece`
    /// by percentages given by `offset_x` and `offset_y`
    pub fn add_child(&mut self, piece: Piece, rel_parent: Vec2, rel_self: Vec2) {
        self.children.push((piece, rel_parent, rel_self));
    }

    /// Get the `Texture2D` of this `Piece`
    pub fn texture(&self) -> Texture2D {
        *ASSETS.get().expect("ASSETS not set")
               .get(&self.texture).expect("Texture not set in child")
    }

    /// Get the width of the `Texture2D` of this piece. 
    ///
    /// Since it's possible for children's textures can extend past the bounds of the parent 
    /// texture, the calculation must be done to know how far the children extend in order to
    /// return a true width of this `Piece`.
    pub fn width(&self) -> f32 {
        let mut left  = 0.0;
        let mut right = self.texture().width();

        for (child, _rel_parent, rel_self) in self.children.iter() {
            // Get the child texture from the texture ID
            let child_texture = child.texture();

            // Check if left edge extends past top of parent texture
            let child_x_offset = child_texture.width()  * rel_self.x();
            if child_x_offset < left {
                left = child_x_offset;
            }

            // Check if right edge extends past top of parent texture
            let curr_right = child_x_offset + child_texture.width();
            if curr_right > right {
                right = curr_right;
            }
        }

        // Return true width of this piece
        right - left
    }

    /// Get the height of the `Texture2D` of this piece. 
    ///
    /// Since it's possible for children's textures can extend past the bounds of the parent 
    /// texture, the calculation must be done to know how far the children extend in order to
    /// return a true height of this `Piece`.
    pub fn height(&self) -> f32 {
        let mut top  = 0.0;
        let mut bottom = self.texture().height();

        for (child, _rel_parent, rel_self) in self.children.iter() {
            // Get the child texture from the texture ID
            let child_texture = child.texture();

            // Check if top edge extends past top of parent texture
            let child_y_offset = child_texture.height()  * rel_self.y();
            if child_y_offset < top {
                top = child_y_offset;
            }

            // Check if bottom edge extends past top of parent texture
            let curr_bottom = child_y_offset + child_texture.height();
            if curr_bottom > bottom {
                bottom = curr_bottom;
            }
        }

        // Return true height of this piece
        bottom - top
    }
}

impl Resizeable for Piece {
    fn draw(&self, location: Vec2, adjustment: f32) {
        let x_coord = location.x();
        let y_coord = location.y();

        // Get the texture from the texture ID
        let texture = self.texture();

        let parent_width = texture.width() * adjustment;
        let parent_height = texture.height() * adjustment;

        // Resize the image to fit the screen width
        let params = DrawTextureParams {
            dest_size: Some(vec2(parent_width, parent_height)),
            ..Default::default()
        };

        // Draw the texture at the calculated location
        draw_texture_ex(texture, x_coord, y_coord, WHITE, params);

        for (child, rel_parent, rel_self) in self.children.iter() {
            // Draw the texture for the child at the calculated location based on the size of the
            // parent texture
            let mut x_offset = x_coord + parent_width  * rel_parent.x();
            let mut y_offset = y_coord + parent_height * rel_parent.y();

            // Get the child texture from the texture ID
            let child_texture = child.texture();

            // Calculate x,y offset relative to the child itself
            x_offset += child_texture.width()  * adjustment * rel_self.x();
            y_offset += child_texture.height() * adjustment * rel_self.y();

           // Resize the image to fit the screen width
            let params = DrawTextureParams {
                dest_size: Some(vec2(child_texture.width()  * adjustment, 
                                     child_texture.height() * adjustment)),
                ..Default::default()
            };

            draw_texture_ex(child_texture, x_offset, y_offset, WHITE, params);
        }
    }
}
