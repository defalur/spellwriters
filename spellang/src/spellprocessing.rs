use crate::spellparser::{self, SpellSeq};
use crate::rune::{self, RuneMaterial};
use gdnative::prelude::*;

#[derive(Debug)]
struct SpellEffect {
    effect_type: String,
    material: String,
}

#[derive(Debug)]
struct SpellElement {
    shape: rune::ShapeRune,
    effects: Vec<SpellEffect>,
}

#[derive(Debug)]
pub struct Spell {
    seq: Vec<SpellElement>,
}

impl Spell {    
    pub fn new_empty() -> Spell {
        Spell{seq: Vec::new()}
    }
    
    pub fn to_gdseq(&self) -> Vec<GodotString> {
        Vec::new()
    }
}

fn reduce_runes(materials: &Vec<rune::MatRune>) -> Vec<rune::MatRune> {
    let mut materials_present = vec![true; materials.len()];
    let mut result = Vec::new();

    for i in 0..materials.len() {
        for j in 0..materials.len() {
            match rune::compute_combination(&materials[i].mat, &materials[j].mat) {
                (true, None) => {
                    materials_present[i] = false;
                    materials_present[j] = false;
                },
                (_, _) => {}
            }
        }

    }

    for i in 0..materials.len() {
        if !materials_present[i] {
            continue;
        }

        let mut combination: Option<String> = None;
        for j in 0..materials.len() {
            if !materials_present[j] {
                continue;
            }

            match rune::compute_combination(&materials[i].mat, &materials[j].mat) {
                (true, Some(m)) => {
                    if combination == None {
                        materials_present[i] = false;
                        materials_present[j] = false;
                        if let Some(m) = rune::find_mat(&m) {
                            result.push(m);
                        }
                        combination = Some(m);
                    }
                },
                (true, None) => {
                    panic!("reduce_runes is panicking!");
                },
                (_, _) => ()
            }
        }
    }

    for i in 0..materials.len() {
        if materials_present[i] {
            result.push(materials[i].clone());
        }
    }

    result
}

fn create_subspell(shape: rune::ShapeRune, materials: &Vec<rune::MatRune>) -> SpellElement {
    let reduced_mat = reduce_runes(materials);
    let mut result = SpellElement{shape: shape.clone(), effects: Vec::new()};

    for m in reduced_mat {
        match shape.effect {
            rune::ShapeEffect::Base => {
                let effect = SpellEffect{effect_type: m.main_effect, material: m.mat.clone()};
                result.effects.push(effect);
                if let Some(e) = m.secondary_effect {
                    let effect = SpellEffect{effect_type: e, material: m.mat.clone()};
                    result.effects.push(effect);
                }
            },
            rune::ShapeEffect::Invocation => {
                if m.building_mat {
                    let effect = SpellEffect{
                    effect_type: "invocation".to_string(),
                    material: m.mat
                    };
                    result.effects.push(effect);
                }
            },
            rune::ShapeEffect::Protection => {
                if m.protection {
                    let effect = SpellEffect{
                    effect_type: "protection".to_string(),
                    material: m.mat
                    };
                    result.effects.push(effect);
                }
                else {
                    let effect = SpellEffect{effect_type: m.main_effect, material: m.mat.clone()};
                    result.effects.push(effect);
                    if let Some(e) = m.secondary_effect {
                        let effect = SpellEffect{effect_type: e, material: m.mat.clone()};
                        result.effects.push(effect);
                    }
                }
            },
        }
    }

    result
}

fn create_subspell_seq(shape_seq: &Vec<rune::ShapeRune>,
                       materials: &Vec<rune::MatRune>) -> Vec<SpellElement> {
    let mut result = Vec::new();
    let mut last = &shape_seq[0];
    for i in 1..shape_seq.len() {
        let spell_element = SpellElement{
            shape: last.clone(),
            effects: Vec::new()
        };

        result.push(spell_element);

        last = &shape_seq[i];
    }

    result.push(create_subspell(last.clone(), materials));

    result
}

pub fn create_spell(seq: SpellSeq) -> Spell {
    let mut result = Spell{seq: Vec::new()};

    for s in seq.spells {
        let mut subseq = create_subspell_seq(&s.shape_seq, &s.mat_props.props);
        result.seq.append(&mut subseq);
    }

    result
}

#[cfg(test)]
mod processing_tests {
    use super::*;

    fn mat_seq(input: &str) -> Vec<rune::MatRune> {
        rune::translate_runes(input).iter().filter_map(
            |x| -> Option<rune::MatRune> {
                if let rune::Rune::Material(m) = x {
                Some(*m)
            }
            else {
                None
        }})
        .collect()
    }

    fn convert_out(out: Vec<rune::MatRune>) -> Vec<RuneMaterial> {
        out.iter().map(|x| x.mat).collect()
    }

    #[test]
    fn reduce_adds_all_materials() {
        //let mut input = vec![RuneMaterial::Fire, RuneMaterial::Life, RuneMaterial::Stone];
        let mut input = mat_seq("fls");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == convert_out(input));
    }

    #[test]
    fn reduce_combines_simple_cases() {
        //let mut input = vec![RuneMaterial::Water, RuneMaterial::Frost];
        let mut input = mat_seq("wr");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == vec![RuneMaterial::Ice]);
    }

    #[test]
    fn reduce_eliminates_incompatibility_simple() {
        //let mut input = vec![RuneMaterial::Fire, RuneMaterial::Frost];
        let mut input = mat_seq("fr");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == Vec::new());
    }

    #[test]
    fn reduce_eliminates_separated_incompatibility() {
        //let mut input = vec![RuneMaterial::Fire, RuneMaterial::Stone, RuneMaterial::Frost];
        let mut input = mat_seq("fsr");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == vec![RuneMaterial::Stone]);
    }

    #[test]
    fn reduce_combines_separated_mats() {
        //let mut input = vec![RuneMaterial::Water, RuneMaterial::Stone, RuneMaterial::Frost];
        let mut input = mat_seq("wsr");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == vec![RuneMaterial::Ice, RuneMaterial::Stone]);
    }

    #[test]
    fn reduce_combines_once() {
        //let mut input = vec![RuneMaterial::Water, RuneMaterial::Frost, RuneMaterial::Frost];
        let mut input = mat_seq("wrr");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == vec![RuneMaterial::Ice, RuneMaterial::Frost]);
    }

    #[test]
    fn reduce_eliminates_all() {
        //let mut input = vec![RuneMaterial::Fire, RuneMaterial::Frost, RuneMaterial::Frost];
        let mut input = mat_seq("frr");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == Vec::new());
    }

    #[test]
    fn reduce_elimination_has_priority() {
        //let mut input = vec![RuneMaterial::Frost, RuneMaterial::Water, RuneMaterial::Fire];
        let mut input = mat_seq("rwf");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == vec![RuneMaterial::Water]);
    }

    #[test]
    fn reduce_elimination_has_priority2() {
        //let mut input = vec![RuneMaterial::Fire, RuneMaterial::Frost, RuneMaterial::Water];
        let mut input = mat_seq("frw");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == vec![RuneMaterial::Water]);
    }

    #[test]
    fn reduce_elimination_has_priority3() {
        //let mut input = vec![RuneMaterial::Water, RuneMaterial::Frost, RuneMaterial::Fire];
        let mut input = mat_seq("wrf");
        let output = convert_out(reduce_runes(&mut input));

        println!("{:?} -> {:?}", input, output);
        assert!(output == vec![RuneMaterial::Water]);
    }
}
