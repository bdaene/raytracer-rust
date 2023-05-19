use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, color: Color) {
        *self = Color {
            red: self.red + color.red,
            green: self.green + color.green,
            blue: self.blue + color.blue,
        }
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, scale: f64) -> Color {
        Color {
            red: self.red / scale,
            green: self.green / scale,
            blue: self.blue / scale,
        }
    }
}
