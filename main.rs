extern mod native;  // TO start a native thread
extern mod rsfml;   // Multimedia library

use rsfml::window::{ContextSettings, VideoMode, event, keyboard, Close};
use rsfml::graphics::{RenderWindow, Color, Texture, Sprite, RenderStates,
    RenderTexture, IntRect};
use rsfml::system::vector2::Vector2f;
use rsfml::traits::drawable::Drawable;

use std::hashmap::HashMap;
use std::num::FromPrimitive;
//use std::option;

// Window Defaults
static window_width:  uint = 1024;
static window_height: uint = 768;
static pixels:        uint = 32;
static paddle_width:  i32 = 20;  // tyeps are weird due to RSFML binding.
static paddle_height: i32 = 50;  // tyeps are weird due to RSFML binding.

// Game Option Defaults
static paddle_padding:     f32 = 30.;
static lhs_start_pos_x:    f32 = 0. + paddle_padding;
static rhs_start_pos_x:    f32 = (window_width as f32) - paddle_padding
  - (paddle_width as f32);
static bottom_start_pos_y: f32 = (window_height as f32) - paddle_padding
  - (paddle_height as f32);

static paddle_velocity: f32 = 5.;
static up_vector:   Vector2f  = Vector2f { x:  0., y:  1. * paddle_velocity };
static down_vector: Vector2f  = Vector2f { x:  0., y: -1. * paddle_velocity };

static start_positions: [Vector2f, ..4] = [
  Vector2f { x: lhs_start_pos_x, y: 0. + paddle_padding }, // Player 1
  Vector2f { x: rhs_start_pos_x, y: 0. + paddle_padding }, // Player 2
  Vector2f { x: lhs_start_pos_x, y: bottom_start_pos_y },  // Player 3
  Vector2f { x: rhs_start_pos_x, y: bottom_start_pos_y },  // Player 4
];

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
  let video_mode  = VideoMode::new_init(window_width, window_height, pixels);
  let clear_color = Color::new_RGB(255, 255, 255);

  let window = match RenderWindow::new(video_mode, title, Close, &settings) {
    Some(window) => window,
    None         => fail!("Error creating RenderWindow.")
  };
  return (window, clear_color);
}  // fn create_window()

#[deriving(Eq, Clone, IterBytes, FromPrimitive)]
enum PlayerId {
  bluepaddle,
  greenpaddle
}  // enum PlayerId

struct Paddle<'r> {
  sprite: Sprite<'r>
}  // struct Paddle

impl<'r> Drawable for Paddle<'r> {
  fn draw_in_render_window(&self, render_window : &RenderWindow) -> () {
    render_window.draw_sprite(&self.sprite)
  }

  fn draw_in_render_window_rs(&self, render_window : &RenderWindow,
      render_states : &mut RenderStates) -> () {
    render_window.draw_sprite_rs(&self.sprite, render_states)
  }

  fn draw_in_render_texture(&self, render_texture : &RenderTexture) -> () {
    render_texture.draw_sprite(&self.sprite)
  }

  fn draw_in_render_texture_rs(&self, render_texture : &RenderTexture,
      render_states : &mut RenderStates) -> () {
    render_texture.draw_sprite_rs(&self.sprite, render_states)
  }
} // impl Drawable for Paddle

struct PongGameState<'r> {
  window:  &'r mut RenderWindow,
  paddles: ~[Paddle<'r>],
  player_id: PlayerId,
  //controlled: &'r Paddle<'r>,
  keys: ~[keyboard::Key]
}  // struct PongGameState

impl<'r> PongGameState<'r> {
  fn new_default(paddles_param: ~[Paddle<'r>], window_param: &'r mut RenderWindow,
      player_id_param: PlayerId) -> PongGameState<'r> {
    assert!((player_id_param as uint) < paddles_param.len());
    return PongGameState { keys: ~[], paddles: paddles_param,
      player_id: player_id_param, window: window_param };
  }  // fn new_default()
  
  // Construct a new state from an existing one.
  fn from_previous(prev: PongGameState<'r>) -> PongGameState<'r> {
    let mut state = prev;
    let player_index: uint = state.player_id as uint;
    { 
      let player_paddle = &mut state.paddles[player_index];
      for key in state.keys.iter() {
        match *key {
          keyboard::Escape => state.window.close(),
          keyboard::K => { player_paddle.sprite.move(&up_vector); },
          keyboard::J => { player_paddle.sprite.move(&down_vector); },
          _ => {}
        }
      }
    }
    state.keys.clear();
    return state;
  }  // fn from_previous()
}  // impl PongGameState

// Loads the different textures as pairs with their corresponding PlayerId
fn load_assets() -> HashMap<PlayerId, Texture> {
  let dir               = "./assets/";
  let blue_paddle_path  = dir + "blue-paddle.png";
  let green_paddle_path = dir + "green-paddle.png";
  let error_prefix      = "Could not load asset: ";

  let texture_rect = IntRect { left: 0, top: 0, width: paddle_width,
    height: paddle_height };
  let blue_paddle_texture = Texture::new_from_file_with_rect(
    blue_paddle_path, &texture_rect).expect(error_prefix + blue_paddle_path);

  let green_paddle_texture = Texture::new_from_file_with_rect(
    green_paddle_path, &texture_rect).expect(error_prefix + green_paddle_path);

  let mut hs = HashMap::new();
  hs.insert(bluepaddle, blue_paddle_texture);
  hs.insert(greenpaddle, green_paddle_texture);
  return hs;
} // fn load_assets

// Constructs (PlayerId, Sprite) corresponding 1:1 with pairs from the input
// HashMap (PlayerId, Texture). The sprites returned have been construted with
// references to the textures corresponding 1:1 using the PlayerId. The lifetime
// annotations are worth noting, it ties the lifetime of the (input) asset
// HashMap, to the Sprites returned inside the HashMap. This tells the compiler
// that the Sprites returned will live as long as the (input) HashMap. Since the
// sprite's are constructed with borrowed pointers to to the textures, this
// lifetime annotation is necessary to compile.
fn create_sprites<'r>(assets: &'r HashMap<PlayerId, Texture>)
    -> HashMap<PlayerId, Sprite<'r>> {
  let error_msg = "Could not create sprite from texture.";
  let sprite_map = assets.iter()
    .map(|(asset_id, texture)| { // asset_id, texture by reference/borrowed ptr.
      // Create the sprite with a borrowed ptr to the texture
      let sprite = Sprite::new_with_texture(texture).expect(error_msg);
      return (asset_id.clone(), sprite);
    }).collect();
  return sprite_map;
}  // fn create_sprites()

// The scan fn below returns an iterator that will iterator over the sprite's'
  // in assets HashMap.
  // The initial state of scan() is the sprites iterator.
  // The first parameter to scan is a lambda fn which the first parameter will
  // be the mutable state threaded through each invocation that scan does. We
  // don't need mutable state here, so use _. The second parameter of the lambda
  // is the reference to the current value (in our case a pair of values from
  // the HashMap (sprite_iter is a scan::iter)

  /*let mut sprite_iter = sprites.iter()
    .scan(sprites.iter(), |_, (_, sprite)| {
      return sprite.clone();
    }); */

fn create_paddles(sprites: HashMap<PlayerId, Sprite>) -> ~[Paddle] {
  // Zip the iterators together, so they can be iterated together. :)
  let zipped = sprites.iter().zip(start_positions.iter());

  return zipped
    .map(|((_, sprite_item), start_pos_item)| {
      let error_msg = "Error cloning sprite.";
      let mut paddle = Paddle {
        sprite: sprite_item.clone().expect(error_msg)
      };
      paddle.sprite.set_position(start_pos_item);
      return paddle; 
    }).collect();
}  // fn create_paddles()

// Loop forever polling events from the window, until there are no more events.
// When there are no more events, break out of the loop.
fn loop_events<'r>(prev: PongGameState<'r>) -> PongGameState<'r> {
  let mut state = PongGameState::from_previous(prev);
  loop {
    match state.window.poll_event() {
      event::Closed               => state.window.close(), 
      event::KeyPressed{code, ..} => { state.keys.push(code); },
      _                           => break  // Maybe have to do event::NoEvent
    }
  }
  return state;
}  // fn loop_events()

// Entry point for pong
fn main() {
  let (mut window, clear_color) = create_window();
  let assets = load_assets();

  let sprites = create_sprites(&assets);
  let paddles = create_paddles(sprites);

  // Each state needs a reference to the items it needs to update..
  // For example, it needs a reference to the paddle array.
  let player_id = FromPrimitive::from_int(1).expect("PlayerId");
  let mut state = PongGameState::new_default(paddles, &mut window, player_id);

  // when I press the 'hjkl' keys, move the first paddle..
  while state.window.is_open() {
    state.window.clear(&clear_color);
    state = loop_events(state); 
    // update_state

    for paddle in state.paddles.iter() {
      state.window.draw(paddle);
    }
    state.window.display();
  }
}  // fn main()
