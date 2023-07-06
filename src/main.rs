use game_of_life::game::Game;

fn main() -> Result<(), String> {
    let mut game = Game::new();
    game.start()?;

    Ok(())
}
