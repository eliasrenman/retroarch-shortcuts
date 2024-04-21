use flate2::read::GzDecoder;
use protobuf::Message;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

use crate::generated::game::{GameDB, GameDBEntry};
pub struct GameDatabase {
    games: HashMap<i32, GameDBEntry>,
    crc_index: HashMap<u32, i32>,
}

impl GameDatabase {
    pub fn new(resource: &str, is_compressed: bool) -> Result<Self, io::Error> {
        let mut db = GameDatabase {
            games: HashMap::new(),
            crc_index: HashMap::new(),
        };

        let games = db.read_games(resource, is_compressed)?;

        for game in games {
            db.games.insert(game.id, game.clone());
            db.crc_index
                .insert(game.get_crcHash() as u32, game.get_id());
        }

        Ok(db)
    }

    fn read_games(
        &self,
        resource: &str,
        is_compressed: bool,
    ) -> Result<Vec<GameDBEntry>, io::Error> {
        let mut file = File::open(resource)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let bytes = if is_compressed {
            self.decompress(&buffer)?
        } else {
            buffer
        };
        let mut games_db = GameDB::new();
        games_db.merge_from_bytes(&bytes).unwrap();
        Ok(games_db.get_games().to_vec())
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, io::Error> {
        let mut decoder = GzDecoder::new(data);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        Ok(decompressed_data)
    }

    pub fn get_by_crc(&self, crc: u32) -> Option<&GameDBEntry> {
        let id = self.crc_index.get(&crc)?;
        self.games.get(id)
    }
}
