const ENEMY_NAMES: &[&str] = &["Goblin", "Slime", "Lesser Demon", "Orc", "Skeleton"];
pub struct Enemy {
    pub name:String,
    pub hp:i32,
    pub damage:i32,
}

pub fn create_enemy(level: i32) -> Enemy {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let name = ENEMY_NAMES[rng.gen_range(0..ENEMY_NAMES.len())];
    
    Enemy {
        name: name.to_string(),
        hp: 20 + (level * 2),
        damage: 5 + level,
    }
}
