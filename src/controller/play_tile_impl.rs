use super::PlayTile;

impl PlayTile {
    pub(super) fn hidden(pos: (i32, i32)) -> Self {
        Self {
            pos,
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
            pos: self.pos,
            flagged: self.flagged,
            revealed: self.revealed,
            mine_neighbors: self.mine_neighbors,
            mine: self.mine,
        }
    }
}
