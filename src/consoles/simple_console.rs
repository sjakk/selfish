use bracket_geometry::prelude::Rect;
use bracket_color::prelude::*;

use crate::prelude::{string_to_cp437,Tile,CharacterTranslationMode,Console,FontCharType};

pub struct SimpleConsole{

pub width: u32,
pub height: u32,


pub tiles: Vec<Tile>,
pub is_dirty: bool,

pub offset_x: f32,
pub offset_y: f32,

pub scale: f32,
pub scale_center: (i32,i32),

pub extra_clipping: Option<Rect>,
pub translation: CharacterTranslationMode,
//pub(crate) needs_resize_internal: bool,

}





impl SimpleConsole{
    pub fn init(width: u32,height: u32) -> Box<SimpleConsole>{
        let num_tiles: usize = (width * height) as usize;
        let mut tiles: Vec<Tile> = Vec::with_capacity(num_tiles);

        for _ in 0..num_tiles{
            tiles.push(Tile{
                glyph: 0,
                fg: RGBA::from_u8(255,255,255,255),
                bg: RGBA::from_u8(0,0,0,255),
            });
        }

    let new_console = SimpleConsole{
        width,
        height,
        tiles,
        is_dirty: true,
        offset_x: 0.0,
        offset_y: 0.0,
        scale: 1.0,
        scale_center: (width as i32 / 2, height as i32 /2),
        extra_clipping: None,
        translation: CharacterTranslationMode::Codepage437,
        
    };

        Box::new(new_console)
    } // fn init



}// impl simple_console




impl Console for SimpleConsole{


fn at(&self, x:i32,y:i32) -> usize{
        (((self.height -1 - y as u32) * self.width) + x as u32) as usize
}

fn get_clipping(&self) -> Option<Rect>{
    self.extra_clipping
}


fn get_char_size(&self) -> (u32,u32){
    (self.width,self.height)
}



fn cls(&mut self){
    self.is_dirty = true;
    for tile in &mut self.tiles{
        tile.glyph = 32;
        tile.fg = RGBA::from_u8(255,255,255,255);
        tile.bg = RGBA::from_u8(0,0,0,255);
    }
}



fn print(&mut self,mut x: i32, y:i32,output: &str){
self.is_dirty = true;
let bytes = match self.translation{
    CharacterTranslationMode::Codepage437 => string_to_cp437(output),
    CharacterTranslationMode::Unicode => {
        output.chars().map(|c| c as FontCharType).collect()
    }
};

for glyph in bytes{
    if let Some(idx) = self.try_at(x,y){
        self.tiles[idx].glyph = glyph;
    }
    x+=1;
}
}




}








