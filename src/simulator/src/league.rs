use crate::chrono::Datelike;
use crate::club::Club;
use crate::core::SimulationContext;
use crate::play::{Match, MatchResult};
use crate::schedule::Schedule;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub struct League {
      pub name: String,
      pub clubs: HashMap<u32, Club>,
      pub schedule: Option<Schedule>,
      pub settings: LeagueSettings,
}

impl League {
      pub fn items_count(&self) -> usize {
            return self.clubs.iter().map(|club| club.1.items_count()).sum();
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            if self.schedule.is_none() || self.settings.is_time_for_new_schedule(context) {
                  let club_list = self.clubs.values().collect();
                  self.schedule = Some(Schedule::generate(club_list, context.date).unwrap());
            }

            for club in &mut self.clubs {
                  club.1.simulate(context);
            }

            let matches_to_play = self.schedule.as_ref().unwrap().get_matches(context.date);

            for m in matches_to_play {
                  let home_club = self.clubs[&m.home_club_id].clone();
                  let away_club = self.clubs[&m.guest_club_id].clone();

                  let club_match = Match::make(home_club, away_club);

                  let match_result = club_match.play();

                  //println!("{}", match_result);
            }
      }
}

pub struct LeagueSettings {
      pub season_starting: (u8, u8),
      pub season_ending: (u8, u8),
}

impl LeagueSettings {
      pub fn is_time_for_new_schedule(&self, context: &SimulationContext) -> bool {
            let current_day = context.date.day() as u8;
            let current_month = context.date.month() as u8;

            current_day == self.season_starting.0 && current_month == self.season_starting.1
      }
}

#[cfg(test)]
mod tests {
      use super::*;

      #[test]
      fn is_time_for_new_schedule_is_correct() {
            let mut settings = LeagueSettings {
                  season_starting: (1, 3),
                  season_ending: (4, 5),
            };

            let mut context = SimulationContext {
                  events: vec![],
                  date: NaiveDate::from_ymd(2020, 3, 1),
            };

            let result = settings.is_time_for_new_schedule(&mut context);

            assert_eq!(true, result);
      }
}
