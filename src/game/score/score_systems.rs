use bevy::prelude::*;
use rusqlite::{Connection, Result};

use crate::game::score::score_resources::*;

pub fn update_score(
    score: Res<Score>,
) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default())
}

pub fn create_high_scores_table() -> Result<()> {
    let conn = Connection::open("data/scores.db")?;
    conn.execute(
        "create table if not exists scores (
             rank integer primary key,
             name text not null,
             score integer not null
         )",
        (),
    )?;
    Ok(())
}

pub fn db_error_handler(In(result): In<Result<()>>) {
    if let Err(err) = result {
        println!("DB encountered an error {:?}", err);
    }
}