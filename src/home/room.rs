#[derive(Debug)]
pub struct Room {
  pub name: String,
  pub people_inside: usize,
}

impl Room {
  pub fn did_enter(&mut self) {
    self.people_inside += 1
  }
  pub fn did_leave(&mut self) {
    self.people_inside -= 1
  }
}
