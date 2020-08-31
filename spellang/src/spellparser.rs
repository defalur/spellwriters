use crate::rune;
use std::iter::Peekable;

use gdnative::prelude::*;

#[derive(Debug)]
pub struct MatProps {
    pub props: Vec<rune::MatRune>,
}

#[derive(Debug)]
pub struct SpellProps {
    pub mat_props: MatProps,
    pub shape_seq: Vec<rune::ShapeRune>,
}

#[derive(Debug)]
pub struct SpellSeq {
    pub spells: Vec<SpellProps>,
}

pub fn spell_parse(rune_seq: Vec<rune::Rune>) -> Result<SpellSeq, ()> {
    //godot_print!("{:?}", rune_seq);
    dspell_parse(&mut rune_seq.iter().peekable())
}

fn dspell_parse<'a, It>(rune_seq: &mut Peekable<It>) -> Result<SpellSeq, ()> 
    where It: Iterator<Item = &'a rune::Rune> {
    let mut result = SpellSeq{spells: Vec::new()};
    while let Some(r) = rune_seq.peek() {
        match r {
            rune::Rune::Shape(s) => {
                //println!("dspell: calling dshape.");
                result.spells.push(dshape_parse(rune_seq)?);
                //rune_seq.next();
            },
            rune::Rune::Material(_) => {
                //println!("dspell: found mat, abort.");
                break;
            }
        }
    }

    if result.spells.len() > 0 {
        //println!("dspell: success");
        Ok(result)
    }
    else {
        //println!("dspell: failure");
        Err(())
    }
}

fn dshape_parse<'a, It>(rune_seq: &mut Peekable<It>) -> Result<SpellProps, ()>
    where It: Iterator<Item = &'a rune::Rune> {
    let shape = match rune_seq.next().unwrap() {
        rune::Rune::Shape(s) => Ok(s),
        _ => Err(())
    }?;
    //println!("dshape: read shape");
    let mut result = SpellProps{
        mat_props: MatProps{props: Vec::new()},
        shape_seq: vec![shape.clone()]
    };

    while let Some(r) = rune_seq.peek() {
        match r {
            rune::Rune::Shape(s) => {
                //println!("dshape: append new shape");
                result.shape_seq.push(s.clone());
                rune_seq.next();
            },
            rune::Rune::Material(_) => {
                //println!("dshape: calling dmat");
                result.mat_props = dmat_parse(rune_seq)?;
                break;
            }
        }
    }

    if result.mat_props.props.len() > 0 {
        //println!("dshape: success");
        Ok(result)
    }
    else {
        //println!("dshape: failure");
        Err(())
    }
}

fn dmat_parse<'a, It>(rune_seq: &mut Peekable<It>) -> Result<MatProps, ()>
    where It: Iterator<Item = &'a rune::Rune> {
    let mat = match rune_seq.next().unwrap() {
        rune::Rune::Material(m) => Ok(m),
        rune::Rune::Shape(_) => Err(())
    }?;
    //println!("dmat: read mat");
    let mut result = MatProps{props: vec![mat.clone()]}; 
    match rune_seq.peek() {
        Some(r) => {
            if let rune::Rune::Material(m) = r {
                //println!("dmat: calling dmat");
                result = dmat_parse(rune_seq)?;
                //result.props.push(m.clone());
                result.props.push(mat.clone());
            }
            else {
                //println!("dmat: found shape, end function");
                return if result.props.len() > 0 {
                    //println!("dmat: success");
                    Ok(result)
                }
                else {
                    //println!("dmat: failure");
                    Err(())
                };
            }
        },
        None => {
            //println!("dmat: found end, end function");
            return if result.props.len() > 0 {
                //println!("dmat: success");
                Ok(result)
            }
            else {
                //println!("dmat: failure");
                Err(())
            };
        }
    }

    //println!("dmat: success");
    Ok(result)
}
