#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//!
//! Chess game implemented on Bevy for educational purpose.
//!

use bevy::render::RenderSystem;
use bevy::render::camera::camera_system;
use game_chess_core as core;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::{EguiPlugin};
use bevy::input::system::exit_on_esc_system;

pub mod camera;
pub mod piece;
pub mod common;

use common::GameState;

/// mut material  359
/// My color change
/// 

pub fn color_change
(
  mut materials: ResMut<Assets<ColorMaterial>>,
   query_white: Query<&Handle<ColorMaterial>, With<CellWhite>>,
   query_black: Query<&Handle<ColorMaterial>, With<CellBlack>>, 
    color_schema: Res<CellColorSchema>) 
    {
 
  for handle in query_white.iter() {
    let mut material = materials.get_mut(handle).unwrap();
    material.color = Color::rgb(color_schema.white[0],color_schema.white[1], color_schema.white[2]);
  }

  for handle in query_black.iter() {
    let mut material = materials.get_mut(handle).unwrap();
    material.color = Color::rgb(color_schema.black[0],color_schema.black[1], color_schema.black[2]);
  }
  /*commands.insert_resource(Materials {
  _white : materials.add(ColorMaterial::color(Color::rgb(0.9, 0.0, 0.0)));
  _black : materials.add(ColorMaterial::color(Color::rgb(0.2, 0.8, 0.8)));
  });*/
}

///
/// Board setup.
///

pub fn board_setup(mut commands : Commands, mut materials : ResMut<Assets<ColorMaterial>>)
{
  /* camera */
  commands
    .spawn_bundle(camera::ChessCameraBundle::new())
    .insert(Timer::from_seconds(2.0, false));


  let size_in_cells = (8, 8);

  let white = materials.add(ColorMaterial::color(Color::rgb(0.9, 0.9, 0.7)));
  let black = materials.add(ColorMaterial::color(Color::rgb(0.2, 0.2, 0.1)));

  let size = 2.0 / 8.0;
  let delta = 1.0 - size / 2.0;

  for x in 0 .. size_in_cells.0
  {
    for y in 0 .. size_in_cells.1
    {
      let isBlack = (x + y) % 2 == 0;
      let  material = if isBlack { black.clone() } else { white.clone() };

      let sprite = Sprite {
        size : Vec2::new(size, size),
        ..Default::default()
      };

      let transform = Transform {
        translation : Vec3::new((x as f32) * size - delta, (y as f32) * size - delta, 0.0),
        ..Default::default()
      };

      let mut cell =
      commands.spawn_bundle(SpriteBundle {
        sprite,
        material,
        transform,
        ..Default::default()
      }); //.insert(Cell)

      cell.insert(Cell);
      if isBlack {
        cell.insert(CellBlack);
      }
      else {
        cell.insert(CellWhite);
      }
    }
  }

  // diagnostics_rect( &mut commands, &mut materials );
}

///
/// Add sprite of size 2x2 for diagnostics purpose. The sprite should cover central zone of window.
///

pub fn diagnostics_rect(commands : &mut Commands, materials : &mut ResMut<Assets<ColorMaterial>>)
{
  let red = materials.add(ColorMaterial::color(Color::rgb(0.9, 0.2, 0.2)));

  let sprite = Sprite {
    size : Vec2::new(2., 2.),
    ..Default::default()
  };

  let transform = Transform {
    translation : Vec3::new(0.0, 0.0, 0.0),
    ..Default::default()
  };

  commands.spawn_bundle(SpriteBundle {
    sprite,
    material : red,
    transform,
    ..Default::default()
  });
}

///
/// Startup system for the game.
///

pub fn core_setup(mut commands : Commands, mut game_state : ResMut<State<GameState>>)
{
  let mut game = core::Game::default();
  game.board_print();
  game.make_move("a2a4".into());
  game.board_print();
  commands.insert_resource(game);

  game_state.set(GameState::GameStart).unwrap();
}

fn timer_system(time : Res<Time>, mut query : Query<&mut Timer>, mut game_state : ResMut<State<GameState>>)
{
  let mut timer = query.single_mut().unwrap();
  timer.tick(time.delta());
  if timer.finished()
  {
    game_state.set(GameState::GameNew).unwrap();
  }
}

/// my struct

pub fn setup_egui(egui_context : Res<EguiContext>, mut color_schema: ResMut<CellColorSchema>)
{
  // add fixated panel
  egui::SidePanel::left("Menu")
    .resizable(false)
    //.default_width(SIDE_PANEL_WIDTH)
    .show(egui_context.ctx(), |ui| {
      ui.heading("1");
      ui.horizontal(|ui|{
        //let mut color_white = [0.,0.,0.,0.];
        if ui.color_edit_button_rgba_unmultiplied(&mut color_schema.white).changed() {
          //dbg!(color_white);
        }
      }); 
    });
}

pub struct Cell;

pub struct CellWhite;
pub struct CellBlack;

pub struct CellColorSchema {
  pub white : [f32; 4],
  pub black : [f32; 4] 
}

impl Default for CellColorSchema {
    fn default() -> Self {
        Self
        {
          white: [1.0, 0.5, 0.5, 1.],
          black: [4.0, 0.2, 0.2, 1.]
        }
    }
}

fn main()
{
  let mut app = App::build();
  /* default plugins */
  app.add_plugins(DefaultPlugins);

  app.add_plugin(EguiPlugin);
  app.add_system(setup_egui.system());

  /* background */
  app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));
  app.insert_resource(CellColorSchema::default());
  app.add_state(GameState::Init);
  app.add_system_set(SystemSet::on_update(GameState::Init).with_system(timer_system.system()));
  /* setup core */
  app.add_system_set(SystemSet::on_update(GameState::GameNew).with_system(core_setup.system()));
  app.add_system_set(SystemSet::on_update(GameState::GameStart).with_system(piece::pieces_setup.system()));
  /* setup board */
  app.add_startup_system(board_setup.system());

  /* escape on exit */
  app.add_system(exit_on_esc_system.system());

  app.add_system(color_change.system());

  app.add_system_to_stage(
    CoreStage::PostUpdate,
    camera_system::<camera::ChessProjection>
      .system()
      .before(RenderSystem::VisibleEntities),
  );
  /* for web target */
  #[cfg(target_arch = "wasm32")]
  app.add_plugin(bevy_webgl2::WebGL2Plugin);
  /* run */
  app.run();
}
