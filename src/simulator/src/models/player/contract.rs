
use crate::models::player::player::Player;
use crate::core::context::SimulationContext;

pub use chrono::prelude::{NaiveDate, DateTime, Utc};

pub struct PlayerClubContract {
      player: Player,
      expired: NaiveDate,
}

impl PlayerClubContract {
      pub fn new(player: Player, expired: NaiveDate) -> Self {
            PlayerClubContract {
                  player: player,
                  expired: expired,
            }
      }

      pub fn is_expired(&self) -> bool {
            let now = Utc::now();

            return false;

            // let naive_now = NaiveDate::from_ymd(
            //       now.year(), now.month(), now.day()
            // );

            // self.expired. >= naive_now
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            self.player.simulate(context);
      }
}