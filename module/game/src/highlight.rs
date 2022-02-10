use bevy::prelude::*;

#[derive(Debug)]
enum HighlightCommand
{
  Highlight
  {
    pos : (u8, u8),
    color : Color,
  },
  Clear
  {
    pos : (u8, u8),
  },
  ClearAll,
}

///
/// Resource responsible for highlighting cells
///

#[derive(Debug)]
pub struct Highlight
{
  data : Vec<(Entity, Option<Color>)>,
  commands : Vec<HighlightCommand>,
}

impl Highlight
{
  pub fn highlight(&mut self, pos : (u8, u8), color : Color) { self.commands.push(HighlightCommand::Highlight { pos, color }); }

  pub fn clear(&mut self, pos : (u8, u8)) { self.commands.push(HighlightCommand::Clear { pos }); }

  pub fn clear_all(&mut self)
  {
    self.commands.clear();
    self.commands.push(HighlightCommand::ClearAll);
  }
}

struct ClearOnEachFrame(bool);

#[derive(Default, Debug)]
struct HighlightEntity;


#[derive(Debug)]
pub struct HighlightPlugin
{
  /// If true, highlighing is removed on each frame
  pub clear_on_each_frame : bool,
}
impl Plugin for HighlightPlugin
{
  fn name(&self) -> &str { "chess_game_highlight" }

  fn build(&self, app : &mut AppBuilder)
  {
    let highlight_data = Highlight {
      data : Vec::with_capacity(8 * 8),
      commands : Vec::new(),
    };

    app.insert_resource(highlight_data);
    app.insert_resource(ClearOnEachFrame(self.clear_on_each_frame));
    app.add_startup_system(setup_highlight.system());
    app.add_system(apply_requests.system());
  }
}

fn setup_highlight(mut cmd : Commands, mut highlight : ResMut<Highlight>, mut materials : ResMut<Assets<ColorMaterial>>)
{
  let size = 2.0 / 8.0;
  let delta = 1.0 - size / 2.0;

  for x in 0 .. 8
  {
    for y in 0 .. 8
    {
      let sprite = Sprite {
        size : Vec2::splat(size),
        ..Default::default()
      };

      let material = materials.add(ColorMaterial::color(Color::rgb(0.9, 0.0, 0.0)));

      let transform = Transform {
        translation : Vec3::new((x as f32) * size - delta, (y as f32) * size - delta, 0.5),
        ..Default::default()
      };

      //let material = materials.add(ColorMaterial::color(Color::rgb(1.0, 0.0, 0.0)));

      let ent = cmd
        .spawn()
        .insert_bundle(SpriteBundle {
          sprite,
          transform,
          material,
          ..Default::default()
        })
        .insert(Visible {
          is_visible : false,
          is_transparent : true,
        })
        .id();

      highlight.data.push((ent, None));
    }
  }
}

fn pos_to_index((x, y) : (u8, u8)) -> usize { x as usize * 8 + y as usize }

fn apply_requests(
  clear_on_each_frame : Res<ClearOnEachFrame>,
  mut highlight : ResMut<Highlight>,
  mut query : Query<(&mut Handle<ColorMaterial>, &mut Visible)>,
  mut materials : ResMut<Assets<ColorMaterial>>,
)
{
  let Highlight { commands, data } = &mut *highlight;

  for command in commands.drain(..)
  {
    match command
    {
      HighlightCommand::Highlight { pos, color } =>
      {
        let idx = pos_to_index(pos);
        if data[idx].1 == Some(color)
        {
          continue;
        }
        data[idx].1 = Some(color);

        let (mut material, mut visible) = query.get_mut(data[idx].0).unwrap();
        visible.is_visible = true;
        *material = materials.add(ColorMaterial::color(color));
      }

      HighlightCommand::Clear { pos } =>
      {
        let idx = pos_to_index(pos);
        if data[idx].1 == None
        {
          continue;
        }
        data[idx].1 = None;

        let (_, mut visible) = query.get_mut(data[idx].0).unwrap();
        visible.is_visible = false;
      }

      HighlightCommand::ClearAll =>
      {
        for (ent, color) in &mut *data
        {
          if *color == None
          {
            continue;
          }
          *color = None;

          let (_, mut visible) = query.get_mut(*ent).unwrap();
          visible.is_visible = false;
        }
      }
    }
  }

  if clear_on_each_frame.0
  {
    highlight.commands.push(HighlightCommand::ClearAll);
  }
}
