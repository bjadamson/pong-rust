extern mod native;  // TO start a native thread
extern mod rsfml;   // Multimedia library

use rsfml::window::{ContextSettings, VideoMode, event, keyboard, Close};
use rsfml::graphics::{RenderWindow, Color, Texture, Sprite};

use std::hashmap::HashMap;
//use std::option;

// OSX Prevents creating a window on the main thread, so start up a new thread
// and launch the window.
#[start]
#[cfg(target_os="macos")]
fn start(argc: int, argv: **u8) -> int {
  return native::start(argc, argv, main);
}  // fn start()

// Create the window that will be used by the rest of the program.
fn create_window() -> (RenderWindow, Color) {
  let title       = "RSFML Pong - Rust";
  let settings    = ContextSettings::default();
  let video_mode  = VideoMode::new_init(1024, 768, 32);
  let clear_color = Color::new_RGB(255, 255, 255);

  let window = match RenderWindow::new(video_mode, title, Close, &settings) {
    Some(window) => window,
    None         => fail!("Error creating RenderWindow.")
  };
  return (window, clear_color);
}  // fn create_window()

#[deriving(Eq, Clone, IterBytes)]
enum AssetId {
  bluepaddle,
  greenpaddle
}  // enum AssetId

// Loads the different textures as pairs with their corresponding AssetId
fn load_assets() -> HashMap<AssetId, Texture> {
  let blue_paddle_filename  = "blue-paddle.png";
  let green_paddle_filename = "green-paddle.png";
  let error_prefix          = "Could not load asset: ";
  let dir                   = "./assets/";

  let blue_paddle_texture = Texture::new_from_file(dir + blue_paddle_filename)
    .expect(error_prefix + blue_paddle_filename);
  let green_paddle_texture = Texture::new_from_file(dir + green_paddle_filename)
    .expect(error_prefix + green_paddle_filename);

  let mut hs = HashMap::new();
  hs.insert(bluepaddle, blue_paddle_texture);
  hs.insert(greenpaddle, green_paddle_texture);
  return hs;
} // fn load_assets

// Constructs (AssetId, Sprite) corresponding 1:1 with pairs from the input
// HashMap (AssetId, Texture). The sprites returned have been construted with
// references to the textures corresponding 1:1 using the AssetId. The lifetime
// annotations are worth noting, it ties the lifetime of the (input) asset
// HashMap, to the Sprites returned inside the HashMap. This tells the compiler
// that the Sprites returned will live as long as the (input) HashMap. Since the
// sprite's are constructed with borrowed pointers to to the textures, this
// lifetime annotation is necessary to compile.
fn create_sprites<'r>(assets: &'r HashMap<AssetId, Texture>)
    -> HashMap<AssetId, Sprite<'r>> {
  let error_msg = "Could not create sprite from texture.";
  let sprite_map = assets.iter()
    .map(|(asset_id, texture)| { // asset_id, texture by reference/borrowed ptr.
      // Create the sprite with a borrowed ptr to the texture
      let sprite = Sprite::new_with_texture(texture).expect(error_msg);
      return (asset_id.clone(), sprite);
    }).collect();
  return sprite_map;
}  // fn create_sprites()

// Logic for keyboard events
fn loop_keyboard_events(window: &mut RenderWindow, code: keyboard::Key) {
  match code {
    keyboard::Escape => window.close(),
    _                => {}
  }
}  // fn loop_keyboard_events()

// Loop forever polling events from the window, until there are no more events.
// When there are no more events, break out of the loop.
fn loop_events(window: &mut RenderWindow) {
  loop {
    match window.poll_event() {
      event::Closed               => window.close(),
      event::KeyPressed{code, ..} => loop_keyboard_events(window, code),
      _                           => break  // Maybe have to do event::NoEvent
    }
  }
}  // fn loop_events()

// Entry point for pong
fn main() {
  let (mut window, clear_color) = create_window();
  let assets = load_assets();

  // works!
  let sprites = create_sprites(&assets);

  while window.is_open() {
    loop_events(&mut window); 
    window.clear(&clear_color);
  }
}  // fn main()
