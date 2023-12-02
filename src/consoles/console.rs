

use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::{Point,Rect};
use crate::prelude::FontCharType;


#[derive(PartialEq,Copy,Clone,Debug)]
pub struct Tile{
    pub bg: RGBA,
    pub fg: RGBA,
    pub glyph: FontCharType,

}



pub enum CharacterTranslationMode{
    Codepage437,
    Unicode,
}



pub trait Console{

fn cls(&mut self);

fn print(&mut self, x:i32,y:i32,output:&str);


fn at(&self ,x:i32 ,y:i32) -> usize;

fn in_bounds(&self, x: i32, y:i32) -> bool{
    let bounds = self.get_char_size();
    if let Some(clip) = self.get_clipping(){
        clip.point_in_rect(Point::new(x,y))
            && x >= 0
            && x < bounds.0 as i32
            && y >= 0
            && y < bounds.1 as i32
    }else{
        x >= 0 && x < bounds.0 as i32 && y >= 0 && y < bounds.1 as i32
    }
}


fn get_clipping(&self) -> Option<Rect>;

fn get_char_size(&self) -> (u32,u32);



fn try_at(&self,x: i32,y: i32) -> Option<usize>{
    if self.in_bounds(x,y){
        Some(self.at(x,y))
    }else{
        None
    }
}


}
