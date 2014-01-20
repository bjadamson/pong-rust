extern mod native;  // TO start a native thread
extern mod rsfml;   // Multimedia library

use rsfml::window::{ContextSettings, VideoMode, event, keyboard, Close};
use rsfml::graphics::{RenderWindow, Color, Texture, Sprite};

use std::hashmap::HashMap;
use std::option;

#[cfg(target_os="macos")]
#[start]
// OSX Prevents creating a window on the main thread.
fn start(argc: int, argv: **u8) -> int {
  return native::start(argc, argv, main);
}  // fn start

// Create the window that will be used by the rest of the program.
fn create_window() -> (RenderWindow, Color) {
  let settings = ContextSettings::default();
  let video_mode = VideoMode::new_init(1024, 768, 32);
  let title = "RSFML Pong - Rust";
  let clear_color = Color::new_RGB(255, 255, 255);
  let window = match RenderWindow::new(video_mode, title, Close, &settings) {
    Some(window) => window,
    None         => fail!("Error creating RenderWindow.")
  };
  return (window, clear_color);
}  // fn create_window

// In the rsfml code, the Sprite structure has an explicit lifetime s.
// The Sprite structure has a private optional<&'s Texture>.
// This structure, ts, a logical grouping.. let's not do that.
// Instead, let's return only the textures from load_assets(), and create the
// sprites by passing in the textures.
#[deriving(Eq)]
#[deriving(Clone)]
#[deriving(IterBytes)]
enum AssetId {
  bluepaddle,
  greenpaddle
}  // enum AssetId

// Loads the different textures as pairs with their corresponding AssetId
fn load_assets() -> HashMap<AssetId, Texture> {
  let error_prefix = "Could not load asset.";
  let dir = "./assets";
  let blue_paddle_filename = "blue-paddle.png";
  let green_paddle_filename = "green-paddle.png";
  let blue_paddle_texture = Texture::new_from_file(dir + blue_paddle_filename)
    .expect(error_prefix);
  let green_paddle_texture = Texture::new_from_file(dir + green_paddle_filename)
    .expect(error_prefix);

  let mut hs = HashMap::new();
  hs.insert(bluepaddle, blue_paddle_texture);
  hs.insert(greenpaddle, green_paddle_texture);
  return hs;
} // fn load_assets

// Construct sprites for each pair in assets, where the sprites returned
// have been constructed with references to the textures in assets.
// This should work in theory, as the assets container already lives.
// How do I annotate the return value here (like I did on load_sprite)
// To get the compiler to understand my intent?
fn load_sprites<'r>(assets: &'r HashMap<AssetId, Texture>) -> HashMap<AssetId, Sprite> {
  let error_msg = "Failed to create sprite from texture.";
  let zz: HashMap<AssetId, Sprite> =  assets.iter()
    .map(|(asset_id, texture)| -> (AssetId, Sprite) {
      let sprite = Sprite::new_with_texture(texture).expect(error_msg);
      return (asset_id.clone(), sprite);
    }).collect();
  return zz;
}

// Construct a sprite, that is created from a texture stored inside assets.
// This works!!
fn load_sprite<'r>(assets: &'r HashMap<AssetId, Texture>) -> Sprite<'r> {
  let (assetid, texture) = assets.iter().next().expect("No elements in assets!");
  return Sprite::new_with_texture(texture).expect("F");
}

// Logic for keyboard events
fn loop_keyboard_events(window: &mut RenderWindow, code: keyboard::Key) {
  match code {
    keyboard::Escape => window.close(),
    _                => {}
  }
}  // fn loop_keyboard_events

// Loop forever polling events from the window, until there are nomore events.
fn loop_events(window: &mut RenderWindow) {
  loop {
    match window.poll_event() {
      event::Closed               => window.close(),
      event::KeyPressed{code, ..} => loop_keyboard_events(window, code),
      _                           => break  // Maybe have to do event::NoEvent
    }
  }
}  // fn loop_events

// Entry point for pong
fn main() {
  let (mut window, clear_color) = create_window();
  let assets = load_assets();
  //let sprites = load_sprites(&assets);

  while window.is_open() {
    loop_events(&mut window); 
    window.clear(&clear_color);
  }
}  // fn main
