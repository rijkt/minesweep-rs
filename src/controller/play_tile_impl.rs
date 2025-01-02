use super::PlayTile;

impl PlayTile {
    pub(super) fn hidden() -> Self {
        Self {
            flagged: false,
            revealed: false,
            mine: false,
            mine_neighbors: 0,
        }
    }
}

impl Clone for PlayTile {
    fn clone(&self) -> Self {
        Self {
            flagged: self.flagged.clone(),
            revealed: self.revealed.clone(),
            mine_neighbors: self.mine_neighbors.clone(),
            mine: self.mine.clone(),
        }
    }
}
