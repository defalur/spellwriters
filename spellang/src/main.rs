use spellang::rune::translate_runes;
use spellang::spellparser;
use spellang::spellprocessing::{self, create_spell};

fn compile_spell(input: &str) -> Option<spellprocessing::Spell> {
    let runes = translate_runes(input);
    //println!("RuneSeq: {:?}\n\n", runes);
    println!("Runes: {}\n\n", input);

    let spellseq = match spellparser::spell_parse(runes){
        Ok(seq) => seq,
        _ => {return None;}
    };

    Some(create_spell(spellseq))
}

fn main() {
    println!("{:?}\n\n====================", compile_spell("Prwa").unwrap());
    println!("{:?}\n\n====================", compile_spell("PSlrw").unwrap());
    println!("{:?}\n\n====================", compile_spell("Ffs").unwrap());
    println!("{:?}\n\n====================", compile_spell("BrAfa").unwrap());
}
