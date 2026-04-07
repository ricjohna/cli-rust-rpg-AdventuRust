mod enemy;
mod player;
mod inventory;
use player::Player;
use crate::inventory::Inventory;
use crate::inventory::Item;
use std::io;
use rand::Rng;
use std::thread;
use std::time::Duration;
    enum GameState {
    Encounter,
    BossEncounter,
    Shop,
    Victory,
    GameOver,
    Rest,
}


fn main(){

    let mut level: i32 = 1;
    let mut state = GameState::Encounter;
    let mut player: Player = Player {hp:100, damage:10,
    inventory: Inventory{
        items: Vec::new(),
    },gold:0,
    };
    let mut enemy = enemy::create_enemy(level);



    player.inventory.items.push(inventory::Item { name: "Potion".to_string(), heal: 20, });

    println!("Items: {}", player.inventory.items.len());

//Encounter
loop {
    
        match state {
            
    GameState::Encounter => {
    println!("{} Approaches!", enemy.name);
    while player.hp > 0 && enemy.hp > 0 {
    //Initial Display
    println!("Player HP: {}",player.hp);
    println!("Player Damage: {}",player.damage);

    
    println!("{} HP: {}",enemy.name ,enemy.hp);
    println!("{} Damage: {}",enemy.name ,enemy.damage);

    
        // Ask for action
    println!("Choose action: 1) Attack 2) Defend 3) Use Potion");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    match input.trim() {
        "1" => {
            println!("You attack the enemy for {} damage!", player.damage);
            enemy.hp -= player.damage;
        }
        "2" => {
            println!("You defend! Enemy deals half damage next turn. (To be implemented)");
            //To be implemented for now just skips turn xd
        }
        "3" => {
            if let Some(item) = player.inventory.items.pop() {
                player.hp += item.heal;
                println!("You used {} and healed {} HP!", item.name, item.heal);
            } else {
                println!("No items left!");
            }
        }
        _ => println!("Invalid action!"),
    }

    // Enemy attacks
    println!("Enemy attacks for {} damage!", enemy.damage);
    player.hp -= enemy.damage;
    clear_screen();

}
}
    GameState::BossEncounter => { 
        clear_screen();
        println!("Boss Goes Here!");
    state = GameState::Victory; },

    GameState::Shop => {
    clear_screen();
    println!("\n=== SHOP ===");
    println!("You have {} potion(s) and {} gold", player.inventory.items.len(), player.gold);
    println!("------------------------");
    println!("1) Buy Potion (5 gold)");
    println!("2) Leave shop");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    match input.trim() {
        "1" => {
        if player.gold >= 5 {
            player.gold -= 5;
            player.inventory.items.push(Item { name: "Potion".to_string(), heal: 20 });
            println!("Bought! You now have {} potion(s)", player.inventory.items.len());
        } else {
            println!("Not enough gold! You need 5, you have {}", player.gold);
        }
        }
        "2" => {
            state = GameState::Rest;
        }
        _ => println!("Invalid choice"),
    }
}
    
    GameState::Rest => {
        println!("1) Rest and go to next encounter");
        println!("2) check inventory");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        match input.trim(){
            "1" => {
                println!("You have rested! Heading to next encounter. . .");
                clear_screen();
                //Player Scaling
                player.hp +=10+(level*2);
                player.damage += 1*level;

                //State Transitioner
                level += 1;
                state = match level {
                    6 | 11 => GameState::Shop,
                    12 => GameState::BossEncounter,
                    _ => GameState::Encounter,  // go to rest, not directly to encounter
                
                };
                //Enemy Scaling
                enemy.hp = 20 + (level*2);
                enemy.damage = 5 + level;
    

            }
            "2" => {
                println!("Your inventory contains {} potion(s)", player.inventory.items.len());
                println!("Your Gold: {}", player.gold);
                clear_screen();
            }
            _ => println!("Invalid choice"),
        }


        },

    GameState::Victory => { 
            clear_screen();
            println!("You Win!");
            break; },
    GameState::GameOver => { 
        clear_screen();
        println!("You Lose!");
        break; },

}
    //Progression
        if player.hp <= 0 {
            state = GameState::GameOver;
        } else if enemy.hp <= 0 {
            //Check from combat
            let was_combat = matches!(state, GameState::Encounter | GameState::BossEncounter);
            
            // Add gold if from combat
            if was_combat {
                let mut rng = rand::thread_rng();
                let gold_gen = rng.gen_range(1..=5)+level;
                player.gold += gold_gen;
            
            println!("You defeated the {}! You gained {} gold!", enemy.name ,gold_gen);
            println!("Heading to Rest area...");
            // Short Delay to display reward
            
            clear_screen();
            }
            
            // Boss Fight Checker
            if level == 12 {
                state = GameState::Victory;
            } else {
                state = GameState::Rest;
            }
}

}
}

fn clear_screen() {
    thread::sleep(Duration::from_secs(3));
    print!("\x1B[2J\x1B[H");
}
