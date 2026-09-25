use bevy::prelude::*;

// Composants du jeu
#[derive(Component)]
struct Player {
    speed: f32,
    size: Vec2,
}

#[derive(Component)]
struct Wall {
    size: Vec2,
}

#[derive(Component)]
struct EndZone {
    size: Vec2,
}

#[derive(Component)]
struct EndText;

// Ressource pour suivre l'état de fin de partie
#[derive(Resource, Default)]
struct GameState {
    is_finished: bool,
}

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JDR - Labyrinthe & Guerrier".into(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, check_win_condition))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Caméra 2D
    commands.spawn(Camera2d);

    let tile_size = 40.0;

    // Définition de la carte (1 = Mur, 0 = Couloir, 2 = Départ, 3 = Arrivée)
    let map = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 2, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 3, 1],
        [1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let offset_x = -(15.0 * tile_size) / 2.0 + tile_size / 2.0;
    let offset_y = (9.0 * tile_size) / 2.0 - tile_size / 2.0;

    let player_scale = 0.075;
    let player_size = Vec2::new(300.0 * player_scale, 400.0 * player_scale);

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            let x = offset_x + col_idx as f32 * tile_size;
            let y = offset_y - row_idx as f32 * tile_size;

            match cell {
                1 => {
                    // Murs (gris)
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.3, 0.3, 0.35),
                            custom_size: Some(Vec2::splat(tile_size)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, 0.0),
                        Wall {
                            size: Vec2::splat(tile_size),
                        },
                    ));
                }
                2 => {
                    // Zone de départ (bleue)
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.2, 0.4, 0.8),
                            custom_size: Some(Vec2::splat(tile_size)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, 0.0),
                    ));

                    // Spawner le joueur au point de départ
                    commands.spawn((
                        Sprite::from_image(asset_server.load("sprites/perso.png")),
                        Transform {
                            translation: Vec3::new(x, y, 1.0),
                            scale: Vec3::splat(player_scale),
                            ..default()
                        },
                        Player {
                            speed: 150.0,
                            size: player_size,
                        },
                    ));
                }
                3 => {
                    // Zone d'arrivée (dorée)
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.9, 0.7, 0.1),
                            custom_size: Some(Vec2::splat(tile_size)),
                            ..default()
                        },
                        Transform::from_xyz(x, y, 0.0),
                        EndZone {
                            size: Vec2::splat(tile_size),
                        },
                    ));
                }
                _ => {}
            }
        }
    }
}

// Système de déplacement
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<(&Wall, &Transform), Without<Player>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if game_state.is_finished {
        return;
    }

    if let Ok((player, mut transform)) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let move_delta = direction * player.speed * time.delta_secs();

        // Déplacement Axe X
        let new_x = transform.translation.x + move_delta.x;
        if !check_collision(
            Vec2::new(new_x, transform.translation.y),
            player.size,
            &wall_query,
        ) {
            transform.translation.x = new_x;
        }

        // Déplacement Axe Y
        let new_y = transform.translation.y + move_delta.y;
        if !check_collision(
            Vec2::new(transform.translation.x, new_y),
            player.size,
            &wall_query,
        ) {
            transform.translation.y = new_y;
        }
    }
}

// Vérification de l'arrivée et affichage du message "THE END"
fn check_win_condition(
    mut commands: Commands,
    player_query: Query<(&Player, &Transform)>,
    end_zone_query: Query<(&EndZone, &Transform), Without<Player>>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.is_finished {
        return;
    }

    if let Ok((player, player_transform)) = player_query.single() {
        let player_pos = player_transform.translation.truncate();

        for (end_zone, end_transform) in end_zone_query.iter() {
            let end_pos = end_transform.translation.truncate();

            // Collision AABB avec la zone de fin
            let reached = player_pos.x - player.size.x / 2.0 < end_pos.x + end_zone.size.x / 2.0
                && player_pos.x + player.size.x / 2.0 > end_pos.x - end_zone.size.x / 2.0
                && player_pos.y - player.size.y / 2.0 < end_pos.y + end_zone.size.y / 2.0
                && player_pos.y + player.size.y / 2.0 > end_pos.y - end_zone.size.y / 2.0;

            if reached {
                game_state.is_finished = true;

                // Afficher l'encadré "THE END" au centre de l'écran
                commands
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        EndText,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Node {
                                    padding: UiRect::all(Val::Px(20.0)),
                                    border: UiRect::all(Val::Px(4.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
                                BorderColor::all(Color::srgb(0.9, 0.7, 0.1)),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("THE END"),
                                    TextFont::from_font_size(40.0),
                                    TextColor(Color::WHITE),
                                ));
                            });
                    });
            }
        }
    }
}

// Détection AABB pour les murs
fn check_collision(
    player_pos: Vec2,
    player_size: Vec2,
    wall_query: &Query<(&Wall, &Transform), Without<Player>>,
) -> bool {
    for (wall, wall_transform) in wall_query.iter() {
        let wall_pos = wall_transform.translation.truncate();

        let collision = player_pos.x - player_size.x / 2.0 < wall_pos.x + wall.size.x / 2.0
            && player_pos.x + player_size.x / 2.0 > wall_pos.x - wall.size.x / 2.0
            && player_pos.y - player_size.y / 2.0 < wall_pos.y + wall.size.y / 2.0
            && player_pos.y + player_size.y / 2.0 > wall_pos.y - wall.size.y / 2.0;

        if collision {
            return true;
        }
    }
    false
}
