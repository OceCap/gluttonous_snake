/*
 * @Author: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @Date: 2022-05-24 19:55:24
 * @LastEditors: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @LastEditTime: 2022-05-25 01:31:39
 * @FilePath: \gluttonous_snake\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use bevy::{prelude::*};
use bevy_prototype_debug_lines::*;

#[derive(Component)]
struct SnakeHead;
/* #[derive(Component)]
struct Position {
    x :f64,
    y :f64
} */
/* pub struct Materials {
    snake_head_color :Handle<ColorMaterial>,
} */
pub const WINDOW_COLOR : ClearColor = ClearColor(Color::BLACK);
fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            width : 1200.0,
            height : 800.0,
            title : "gluttonous snake".to_string(),
            //不允许改变窗口大小
            resizable : false,
            ..Default::default()
        })
        .insert_resource(WINDOW_COLOR)
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spwan_snake.system()))
        .add_system(draw_grid.system())
        .add_system(move_snake.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .run();
}

fn setup(mut commands :Commands, /* mut materials :ResMut<Assets<ColorMaterial>> */) {
    let mut camera = OrthographicCameraBundle::new_2d();
        camera.transform = Transform::from_translation(Vec3::new(0.0,0.0,5.0));
        commands.spawn_bundle(camera);

        /* commands.insert_resource(Materials{
            snake_head_color :materials.add(Color::LIME_GREEN.into()),
        }); */
}

fn spwan_snake(mut commands :Commands) {
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite{    
            color :Color::Rgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 0.5 },
            custom_size :Some(Vec2::new(30.0, 30.0)),
            flip_x :false,
            flip_y :false,          
        },
        ..Default::default()
    })
    .insert(SnakeHead);
}

fn draw_grid(
    mut lines :ResMut<DebugLines>,
    windows : Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();
    lines.line_colored(
        Vec3::new(1.0 * width, 0.0, 0.0),
        Vec3::new(-1.0 * width, 0.0, 0.0),
        0.0,
        Color::RED
    );

    lines.line_colored(
        Vec3::new(0.0, 1.0 * height, 0.0), 
        Vec3::new(0.0, -1.0 * height, 0.0), 
        0.0, 
        Color::RED
    ) 
}


fn move_snake(
    windows :Res<Windows>,
    mut snake_head_position : Query<(&SnakeHead, &mut Transform)>,
    keyboard_input :ResMut<Input<KeyCode>>
) {
    let window  = windows.get_primary().unwrap();
    let half_width = 0.5 * window.width();
    let half_height = 0.5 * window.height();
   
    for (_head, mut transform) in snake_head_position.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            println!("({:?},{:?})",transform.translation.x,transform.translation.y);
            transform.translation.y +=5.0;
            if transform.translation.y > half_height {
                transform.translation.y = -1.0 * half_height + 1.0;
            }
            //snake_head_position.y += 1.0;
        }else if keyboard_input.pressed(KeyCode::Down) {
            println!("({:?},{:?})",transform.translation.x,transform.translation.y);
            transform.translation.y -=5.0;
            if transform.translation.y < -1.0 * half_height {
                transform.translation.y = half_height - 1.0;
            }
            //snake_head_position.y -= 1.0;
        }else if keyboard_input.pressed(KeyCode::Left) {
            println!("({:?},{:?})",transform.translation.x,transform.translation.y);
            transform.translation.x -=5.0;
            if transform.translation.x < -1.0 * half_width {
                transform.translation.x = half_width - 1.0;
            }
            //snake_head_position.x -=1.0;
        }else if keyboard_input.pressed(KeyCode::Right) {
            println!("({:?},{:?})",transform.translation.x,transform.translation.y);
            transform.translation.x +=5.0;
            if transform.translation.x > half_width {
                transform.translation.x = -1.0 * half_width + 1.0;
            }
            //snake_head_position.x +=1.0;
        }
    }
    //let pos = snake_head_position.iter_mut();
    
}