pub(crate) struct Window<'a>(
  Option<&'a u64>,
  Option<&'a u64>,
  Option<&'a u64>,
  Option<&'a u64>,
);

impl<'a> Window<'a> {
  pub(crate) fn new() -> Window<'a> {
    Window(None, None, None, None)
  }
  pub(crate) fn is_ready(&self) -> bool {
    if self.0.is_none() || self.1.is_none() || self.2.is_none() || self.3.is_none() {
      return false;
    }

    true
  }

  pub(crate) fn slide(&mut self, new_element: &'a u64) {
    self.3 = self.2;
    self.2 = self.1;
    self.1 = self.0;
    self.0 = Some(new_element);
  }

  pub(crate) fn get_current_sum(&self) -> u64 {
    self.0.unwrap() + self.1.unwrap() + self.2.unwrap()
  }

  pub(crate) fn get_previous_sum(&self) -> u64 {
    self.1.unwrap() + self.2.unwrap() + self.3.unwrap()
  }
}
