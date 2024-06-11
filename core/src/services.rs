use crate::{entities, interfaces};


type Repo<'rlt> = &'rlt dyn interfaces::FiveLettersRepo;

pub struct GameSessionService<'rlt> {
    repo: Repo<'rlt>
}

impl<'rlt> GameSessionService<'rlt> {
    pub fn new(repo: Repo) -> GameSessionService {
        GameSessionService { repo }
    }

    pub fn get_current_session(&self) -> entities::GameSession {
        let current_game_session = self.repo.get_actual_session();
        current_game_session
    }
}