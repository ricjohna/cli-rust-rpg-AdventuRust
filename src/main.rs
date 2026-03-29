mod enemy;
mod player;
mod inventory;
use player::Player;
use enemy::Enemy;
use crate::inventory::Inventory;
use std::io;

fn main(){

    let mut player: Player = Player {hp:100, damage:10,
    inventory: Inventory{
        items: Vec::new(),
    },
    };
    let mut enemy: Enemy = Enemy {hp:20, damage:5};

    println!("Player HP: {}",player.hp);
    println!("Player Damage: {}",player.damage);

    println!("Enemy HP: {}", enemy.hp);
    println!("Enemy Damage: {}",enemy.damage);

    player.inventory.items.push(inventory::Item { name: "Potion".to_string(), heal: 20, });

    println!("Items: {}", player.inventory.items.len());

    

    while player.hp > 0 && enemy.hp > 0 {
        // Ask for action
    println!("Choose action: 1) Attack 2) Defend 3) Use Item");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    match input.trim() {
        "1" => {
            println!("You attack the enemy for {} damage!", player.damage);
            enemy.hp -= player.damage;
        }
        "2" => {
            println!("You defend! Enemy deals half damage next turn.");
            // For now we not implement half damage, just a placeholder
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