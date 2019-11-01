use crate::models::player::player::FullName;
use crate::core::{EventType, SimulationContext };

use chrono::prelude::*;

pub struct Staff {
      pub id: u32,
      full_name: FullName,
      birth_date: NaiveDate,
      //behaviour: Behavior
}

impl Staff {
      pub fn new(id: u32, full_name: FullName, birth_date: NaiveDate) -> Staff {
            Staff {
                  id: id,
                  full_name: full_name,
                  birth_date: birth_date,
                  // behaviour: Behavior::new()
            }
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            let current_date = context.date;

            if self.birth_date.month() == current_date.month()
                  && self.birth_date.day() == current_date.day()
            {
                  context.send(EventType::Birthday(self.id));
            }
      }
}