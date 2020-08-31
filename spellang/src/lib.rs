#[macro_use]
extern crate lazy_static;

use gdnative::prelude::*;

pub mod spellparser;
pub mod rune;
pub mod spellprocessing;

#[derive(gdnative::NativeClass)]
#[inherit(Node)]
pub struct SpellCompiler;

#[methods]
impl SpellCompiler {
    fn new(_owner: &Node) -> SpellCompiler {
        SpellCompiler
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("SpellParser ready!");
        //godot_print!("{:?}", self.compile_spell(_owner, GodotString::from_str("Pfs")));
    }

    #[export]
    fn compile_spell(&self, _owner: &Node, input: GodotString) -> Vec<GodotString> {
        let input = &input.to_string();
        let runes = rune::translate_runes(input);
        //println!("RuneSeq: {:?}\n\n", runes);
        //println!("Runes: {}\n\n", input);

        let spellseq = match spellparser::spell_parse(runes){
            Ok(seq) => seq,
            _ => {return Vec::new();}
        };
        //godot_print!("{:?}", spellseq);

        spellprocessing::create_spell(spellseq).to_gdseq()
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<SpellCompiler>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
