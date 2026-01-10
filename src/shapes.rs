use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

pub struct ShapeContext<'a, 'w, 's> {
    pub painter: &'a mut ShapePainter<'w, 's>,
}

impl<'a, 'w, 's> ShapeContext<'a, 'w, 's> {

    pub fn new(painter: &'a mut ShapePainter<'w, 's>) -> Self {
        Self { painter }
    }
    
    /// Draw a filled circle
    pub fn circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        self.painter.set_translation(Vec3::new(x, y, 0.0));
        self.painter.color = color;
        self.painter.circle(radius);
    }

    /// Draw a filled rectangle
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.painter.set_translation(Vec3::new(x, y, 0.0));
        self.painter.color = color;
        self.painter.rect(Vec2::new(w, h));
    }

    /// Draw a hollow ring (just to show versatility)
    pub fn ring(&mut self, x: f32, y: f32, radius: f32, thickness: f32, color: Color) {
        self.painter.set_translation(Vec3::new(x, y, 0.0));
        self.painter.color = color;
        self.painter.hollow = true;
        self.painter.thickness = thickness;
        self.painter.circle(radius);
        self.painter.hollow = false;
    }

}