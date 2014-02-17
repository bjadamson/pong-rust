extern mod native;  // TO start a native thread
extern mod rsfml;   // Multimedia library

use rsfml::window::{ContextSettings, VideoMode, event, keyboard, Close};
use rsfml::graphics::{RenderWindow, Color, Texture, Sprite, IntRect, FloatRect, CircleShape};
use rsfml::system::vector2::Vector2f;

use std::hashmap::HashMap;
use std::num::{FromPrimitive, abs};
use std::rand::{task_rng, Rng};

// Window Defaults
static FPS_LIMIT:     uint = 60;
static WINDOW_WIDTH:  uint = 800;
static WINDOW_HEIGHT: uint = 600;
static PIXEL_COUNT:   uint = 32;
static PADDLE_WIDTH:  i32 = 20;  // tyeps are weird due to RSFML binding.
static PADDLE_HEIGHT: i32 = 50;  // tyeps are weird due to RSFML binding.

// Game Option Defaults
static PADDLE_PADDING:     f32 = 30.;
static LHS_START_POS_X:    f32 = 0. + PADDLE_PADDING;
static RHS_START_POS_X:    f32 = (WINDOW_WIDTH as f32) - PADDLE_PADDING - (PADDLE_WIDTH as f32);
static bottom_start_pos_y: f32 = (WINDOW_HEIGHT as f32) - PADDLE_PADDING - (PADDLE_HEIGHT as f32);

static PADDLE_VELOCITY: f32 = 15.;
//static UP_VECTOR:       Vector2f  = Vector2f { x:  0., y:  1. * PADDLE_VELOCITY };
//static DOWN_VECTOR:     Vector2f  = Vector2f { x:  0., y: -1. * PADDLE_VELOCITY };

static BALL_RADIUS:            f32 = 10.;
static BALL_OUTLINE_THICKNESS: f32 = 3.;
static BALL_INITIAL_POSITION:  Vector2f = Vector2f {
    x: (WINDOW_WIDTH as f32)   / 2.,
    y: (WINDOW_HEIGHT as f32)  / 2.
};
static BALL_VELOCITY:      f32 = 1.25;
static BALL_FILL_COLOR:    Color = Color { red: 255, green: 0, blue: 0, alpha: 255 };
static BALL_OUTLINE_COLOR: Color = Color { red: 255, green: 0, blue: 255, alpha: 255 };

static START_POSITIONS: [Vector2f, ..4] = [
  Vector2f { x: LHS_START_POS_X, y: 0. + PADDLE_PADDING }, // Player 1
  Vector2f { x: RHS_START_POS_X, y: 0. + PADDLE_PADDING }, // Player 2
  Vector2f { x: LHS_START_POS_X, y: bottom_start_pos_y },  // Player 3
  Vector2f { x: RHS_START_POS_X, y: bottom_start_pos_y },  // Player 4
];

#[deriving(Eq, Clone, IterBytes, FromPrimitive)]
enum PlayerId {
  bluepaddle,
  greenpaddle
}  // enum PlayerId

struct Paddle<'r> {
  sprite: Sprite<'r>,
  velocity: Vector2f
}  // struct Paddle

struct Ball<'r> {
  drawable: CircleShape<'r>,
  velocity: Vector2f
}  // struct Ball

struct PlayerContext<'r> {
  player_id: PlayerId,
  paddle:    Paddle<'r>,
  keys:      ~[keyboard::Key]
}  // struct PlayerContext

struct PongGameState<'r> {
  player_contexts: ~[PlayerContext<'r>],
  ball: Ball<'r>//,
}  // struct PongGameState

impl<'r> PongGameState<'r> {
// Returns the initial state of the game.
fn new_default(paddles_param: ~[Paddle], ball_param: Ball<'r>) -> PongGameState<'r> {
  let player_context = paddles_param.move_iter()
    .enumerate() // generate (index, item) pairs ...
    .map(|(index, paddle_item)| {
      return PlayerContext {
        player_id: FromPrimitive::from_uint(index).unwrap(),
        paddle: paddle_item,
        keys: ~[]
      };
    }).collect();
  return PongGameState {
    ball: ball_param,
    player_contexts: player_context
  };
}  // fn new_default()
  
// Construct a new state from an existing one.
fn from_previous(prev: PongGameState<'r>) -> PongGameState<'r> {
  let mut state = prev;
  /*
  let player_index: uint = state.player_id as uint;
  { 
    let player_paddle = &mut state.paddles[player_index];
    for key in state.keys.iter() {
      match *key {
        keyboard::Escape => state.window.close(),
        keyboard::K => player_paddle.velocity.y -= BALL_VELOCITY,
        keyboard::J => player_paddle.velocity.y += BALL_VELOCITY,
        _ => {}
      }
    }
    if (player_paddle.velocity.y > 5.) {
      player_paddle.velocity.y = 5.;
    } else if (player_paddle.velocity.y < -5.) {
      player_paddle.velocity.y = -5.;
    }
    player_paddle.sprite.move(&player_paddle.velocity);
  }
  {
    state.ball.drawable.move(&state.ball.velocity);
  }
  {
    // test if ball intersects with the paddles
    let intersecting_rect = FloatRect::new(0.,0.,0.,0.);
    let ref paddle = state.paddles[player_index];
    let paddle_bounds = paddle.sprite.get_global_bounds();
    let ball_bounds = state.ball.drawable.get_global_bounds();
    let intersects = FloatRect::intersects(&ball_bounds, &paddle_bounds, &intersecting_rect);

    if intersects {
      if intersecting_rect.width > intersecting_rect.height {
        state.ball.velocity.y *= -1.;
      }
      else if intersecting_rect.height > intersecting_rect.width {
        state.ball.velocity.x *= -1.;
      }
    }
  }
  {
    let ball_bounds = state.ball.drawable.get_global_bounds();
    state.ball.velocity.y = match ball_bounds.top {
      y_pos if (y_pos <= 0. || y_pos >= (WINDOW_HEIGHT as f32)) => -1. * state.ball.velocity.y,
      _ => state.ball.velocity.y
    };
    let ball_bounds_right = ball_bounds.left + ball_bounds.width;
    if (ball_bounds.left <= 0. || ball_bounds_right > (WINDOW_WIDTH as f32)) {
      state.ball = create_ball();
    }
  }
  state.keys.clear();
  */
  return state;
}  // fn from_previous()

}  // impl PongGameState

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
  let video_mode  = VideoMode::new_init(WINDOW_WIDTH, WINDOW_HEIGHT, PIXEL_COUNT);
  let clear_color = Color::new_RGB(255, 255, 255);

  let mut window = match RenderWindow::new(video_mode, title, Close, &settings) {
    Some(window) => window,
    None         => fail!("Error creating RenderWindow.")
  };
  window.set_framerate_limit(FPS_LIMIT);
  return (window, clear_color);
}  // fn create_window()

// Create a ball that's initialized with the values we declare statically.
fn create_ball() -> Ball {
  static x_values: [f32, ..4] = [-1., -0.75, 0.75, 1.];
  let mut rng = task_rng();
  let mut ball = Ball {
      drawable: CircleShape::new().expect("Could not instantiate ball"),
      velocity: Vector2f {
          x: *rng.choose_option(x_values).unwrap(),
          y: rng.gen_range::<f32>(-1., 1.)
      }
  };
  ball.drawable.set_radius(BALL_RADIUS);
  ball.drawable.set_outline_thickness(BALL_OUTLINE_THICKNESS);
  ball.drawable.set_fill_color(&BALL_FILL_COLOR);
  ball.drawable.set_outline_color(&BALL_OUTLINE_COLOR);
  ball.drawable.set_position(&BALL_INITIAL_POSITION);

  return ball;
}  // fn create_ball()

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
      let sprite: Sprite = Sprite::new_with_texture(texture).expect(error_msg);
      return (asset_id.clone(), sprite);
    }).collect();
  return sprite_map;
}  // fn create_sprites()

// Constructs a vector of Padddles, one paddle for each item in the sprites hashmap
// parameter. The Paddles returned own the sprite. The interesting bit of implementation
// is zipping the sprites with the START_POSITIONS vector. This creates an iterator we
// can iterate over, yielding pairs of ((PlayerId, Sprite), Vector2f).
fn create_paddles(sprites: HashMap<PlayerId, Sprite>) -> ~[Paddle] {
  // Zip the iterators together, so they can be iterated together. :)
  let zipped = sprites.iter().zip(START_POSITIONS.iter());

  return zipped
    .map(|((_, sprite_item), start_pos_item)| {
      let error_msg = "Error cloning sprite.";
      let mut paddle = Paddle {
        // todo: do I really have to clone? I just want to move the sprite's ...
        sprite: sprite_item.clone().expect(error_msg),
        velocity: Vector2f::new(0., 0.)
      };
      paddle.sprite.set_position(start_pos_item);
      return paddle; 
    }).collect();
}  // fn create_paddles()

// Loads the different textures as pairs with their corresponding PlayerId
fn load_assets() -> HashMap<PlayerId, Texture> {
  let dir               = "./assets/";
  let blue_paddle_path  = dir + "blue-paddle.png";
  let green_paddle_path = dir + "green-paddle.png";
  let error_prefix      = "Could not load asset: ";

  let texture_rect = IntRect { left: 0, top: 0, width: PADDLE_WIDTH,
    height: PADDLE_HEIGHT };
  let blue_paddle_texture = Texture::new_from_file_with_rect(
    blue_paddle_path, &texture_rect).expect(error_prefix + blue_paddle_path);

  let green_paddle_texture = Texture::new_from_file_with_rect(
    green_paddle_path, &texture_rect).expect(error_prefix + green_paddle_path);

  let mut hs = HashMap::new();
  hs.insert(bluepaddle, blue_paddle_texture);
  hs.insert(greenpaddle, green_paddle_texture);
  return hs;
} // fn load_assets

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

// Loop forever polling events from the window, until there are no more events.
// When there are no more events, break out of the loop.
fn loop_events<'r>(player_context: PlayerContext<'r>, window: &'r mut RenderWindow)
    -> PlayerContext<'r> {
  let mut context = player_context;
  loop {
    match window.poll_event() {
      event::Closed               => { window.close(); fail!("fail") },
      event::KeyPressed{code, ..} => { context.keys.push(code); },
      _                           => break  // Maybe have to do event::NoEvent
    }
  }
  return context;
}  // fn loop_events()

// Entry point for pong
fn main() {
  let (mut window, clear_color) = create_window();
  let assets = load_assets();

  let sprites = create_sprites(&assets);
  let paddles = create_paddles(sprites);
  let ball = create_ball();

  // Each state needs a reference to the items it needs to update..
  // For example, it needs a reference to the paddle array.
  let state = PongGameState::new_default(paddles, ball);
  let ref ctx = state.player_contexts[0];
  // when I press the 'j | k' keys, move the first paddle..
  while window.is_open() {
    window.clear(&clear_color);
    let context = loop_events((*ctx).clone(), &mut window); 
    //state = PongGameState::from_previous(state);
    // update_state

    //for player_context in state.player_contexts.iter() {
     // state.window.draw(&player_context.paddle.sprite);
    //for paddle in state.paddles.iter() {
      //state.window.draw(&state.ball.drawable);
      //state.window.draw(&paddle.sprite);
    //}
    window.display();
  }
}  // fn main()
