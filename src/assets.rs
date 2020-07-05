use macroquad::*;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

pub static ASSETS: OnceCell<HashMap<u32, Texture2D>> = OnceCell::new();
