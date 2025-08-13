use super::Style;

pub trait Theme: Clone + std::fmt::Debug {
  fn get_style(&self, tokens: Vec<&str>) -> Option<Style>;
}
