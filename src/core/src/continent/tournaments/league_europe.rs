use crate::continent::{Tournament, TournamentContext};
use crate::context::GlobalContext;

pub struct LeagueEurope {}

impl LeagueEurope {}

impl Tournament for LeagueEurope {
    fn simulate(&mut self, tournament_ctx: &mut TournamentContext, ctx: GlobalContext<'_>) {
    }
}
