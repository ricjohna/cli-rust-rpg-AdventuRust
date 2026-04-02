mod enemy;
mod player;
mod inventory;
use player::Player;
use enemy::Enemy;
use crate::inventory::Inventory;
use std::io;
use rand::Rng;
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
    let mut enemy: Enemy = Enemy {hp:20, damage:5};

    println!("Player HP: {}",player.hp);
    println!("Player Damage: {}",player.damage);

    println!("Enemy HP: {}", enemy.hp);
    println!("Enemy Damage: {}",enemy.damage);

    player.inventory.items.push(inventory::Item { name: "Potion".to_string(), heal: 20, });

    println!("Items: {}", player.inventory.items.len());

//Encounter
loop {
    
        match state {
            
    GameState::Encounter => {
    while player.hp > 0 && enemy.hp > 0 {
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

    println!("After the turn:");
    println!("Player HP: {}, Enemy HP: {}", player.hp, enemy.hp);
}
}
    GameState::BossEncounter => { println!("Boss Goes Here!");
    state = GameState::Victory; },

    GameState::Shop => { println!("Shop Goes Here!");
    println!("Shopkeeper: would you like to buy my wares?"); },
    GameState::Rest => {
        println!("1) Rest and go to next encounter");
        println!("2) check inventory");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        match input.trim(){
            "1" => {
                println!("You have rested!");
                player.hp +=10;
                level += 1;
                state = match level {
                    6 | 11 => GameState::Shop,
                    12 => GameState::BossEncounter,
                    _ => GameState::Encounter,  // go to rest, not directly to encounter
                    
                };
                enemy.hp = 20 + (level*2);
                enemy.damage = 5 + level;
    

            }
            "2" => {
                println!("Your inventory contains {} potion(s)", player.inventory.items.len());
                println!("Your Gold: {}", player.gold);
            }
            _ => println!("Invalid choice"),
        }


        },
    GameState::Victory => { println!("You Win!"); },
    GameState::GameOver => { println!("You Lose!"); },

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
                let gold_gen = rng.gen_range(1..=5);
                player.gold += gold_gen;
            }
            
            // Then determine next state
            if level == 12 {
                state = GameState::Victory;
            } else {
                state = GameState::Rest;
            }
}

}
}
