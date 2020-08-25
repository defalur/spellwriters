#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuneShape {
    Projectile,
    Beam,
    Statue,
    Area,
    Wall,
    Focus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuneMaterial {
    Fire = 0,
    Life = 1,
    Stone = 2,
    Air = 3,
    Water = 4,
    Frost = 5,
    Ice = 6,
}

#[derive(Debug, Clone, Copy)]
pub enum TriggerType {
    Contact,
    Cast,
}

#[derive(Debug, Copy, Clone)]
pub enum EffectType {
    Damage,
    Invocation,
    Protection,
    Poison,
    Burn,
    Freeze,
    Bleed,
    Heal,
    Amplify,
}

#[derive(Debug, Copy, Clone)]
pub struct RuneCombination {
    a: RuneMaterial,
    b: RuneMaterial,
    combined: Option<RuneMaterial>,
}

const RUNE_COMBINATIONS: [RuneCombination; 3] = [
    RuneCombination{a: "frost", b: "water", combined: Some("ice")},
    RuneCombination{a: "fire", b: "ice", combined: Some("water")},
    RuneCombination{a: "fire", b: "frost", combined: None},
];

pub fn compute_combination(a: &RuneMaterial, b: &RuneMaterial) -> (bool, Option<RuneMaterial>) {
    let mut result = None;
    let mut found = false;
    for c in RUNE_COMBINATIONS.iter() {
        if c.a == *a {
            if c.b == *b {
                result = c.combined;
                found = true;
            }
        }
        else if c.b == *a {
            if c.a == *b {
                result = c.combined;
                found = true;
            }
        }
    }

    (found, result)
}

#[derive(Debug, Clone)]
pub struct MatRune {
    pub mat: String,//material type
    pub main_effect: String,//main effect for the rune
    pub secondary_effect: Option<String>,//optional secondary effect
    pub building_mat: bool,//can be used for statue/wall
    pub protection: bool,//can be used for armor
}

#[derive(Debug, Clone, Copy)]
pub enum ShapeEffect {
    Base,
    Invocation,
    Protection,
}

#[derive(Debug, Clone)]
pub struct ShapeRune {
    pub shape: String,
    pub trigger: String,
    pub target_self: bool,
    pub target_ground: bool,
    pub target_entity: bool,
    pub effect: ShapeEffect,
}

#[derive(Debug, Clone)]
pub enum Rune {
    Shape(ShapeRune),
    Material(MatRune),
}

impl Rune {
    fn new_shape(
        shape: &str,
        trigger: &str,
        targets: &str,
        effect: ShapeEffect
        ) -> Rune {
        let target_self = targets.chars().any(|c| c == 's');
        let target_ground = targets.chars().any(|c| c == 'g');
        let target_entity = targets.chars().any(|c| c == 'e');
        Rune::Shape(ShapeRune{
            shape: shape.to_string(),
            trigger: trigger.to_string(),
            target_self,
            target_ground,
            target_entity,
            effect
        })
    }

    fn new_material(mat: &str,
                    main_effect: &str,
                    secondary_effect: Option<String>,
                    building_mat: bool,
                    protection: bool) -> Rune {
        Rune::Material(MatRune{
            mat: mat.to_string(),
            main_effect: main_effect.to_string(),
            secondary_effect,
            building_mat,
            protection})
    }
}

lazy_static! {
    static ref RUNES: [Rune; 12] = [
        Rune::new_shape("projectile", "contact", "ge", ShapeEffect::Base),
        Rune::new_shape("beam", "contact", "e", ShapeEffect::Base),
        Rune::new_shape("statue", "cast", "g", ShapeEffect::Invocation),
        Rune::new_shape("area", "cast", "g", ShapeEffect::Base),
        Rune::new_shape("focus", "cast", "se", ShapeEffect::Protection),
        //Rune::new_shape(RuneShape::Wall, TriggerType::Cast, "g", ShapeEffect::Invocation),
        Rune::new_material("fire", "damage", Some("burn".to_string()), true, true),
        Rune::new_material("life", "heal", None, true, false),
        Rune::new_material("stone", "damage", None, true, true),
        Rune::new_material("air", "damage", Some("bleed".to_string()), false, false),
        Rune::new_material("water", "amplify", None, true, true),
        Rune::new_material("frost", "freeze", None, false, false),
        Rune::new_material("ice", "damage", Some("freeze".to_string()), true, true),
    ];
}

pub fn find_mat(mat: String) -> Option<MatRune> {
    match RUNES.iter().find(
        |x|
        if let Rune::Material(m) = x {
            m.mat == mat
        }
        else {
            false
        }) {
        Some(m) => {
            if let Rune::Material(res) = m {
                Some(*res)
            }
            else {
                None
            }
        }
        _ => None
    }
}

pub fn translate_runes(input: &str) -> Vec<Rune> {
    let mut result = Vec::new();
    for c in input.chars() {
        match c {
            'P' => result.push(RUNES[0]),//projectile
            'B' => result.push(RUNES[1]),//beam
            'S' => result.push(RUNES[2]),//statue
            'A' => result.push(RUNES[3]),//area
            'F' => result.push(RUNES[4]),//focus
            'f' => result.push(RUNES[5]),//fire
            'l' => result.push(RUNES[6]),//life
            's' => result.push(RUNES[7]),//stone
            'a' => result.push(RUNES[8]),//air
            'w' => result.push(RUNES[9]),//water
            'r' => result.push(RUNES[10]),//frost
            'i' => result.push(RUNES[11]),//ice
            _ => (),
        };
    }

    result
}
