
#[derive(Debug)]
enum ExampleGameMode {
    
    Survival,
    Creative,
    Adventure,
    Spectator,
}

struct ExamplePlayer {
        
    name: String,
    hp: u64,
    gamemode: ExampleGameMode,
}

fn main() {

    let player = ExamplePlayer {
        name: String::from("Coffdage"),
        hp: 20,
        gamemode: ExampleGameMode::Survival,
    };

    println!("Player name: {} \nPlayer HP: {} \nPlayer gamemode: {:?}",
        player.name,
        player.hp,
        player.gamemode
    );
}