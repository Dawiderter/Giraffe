use bevy::prelude::*;

#[derive(Debug)]
pub struct LayerData {
    /// Relative speed of layer to the camera movement
    pub speed: f32,
    /// Path to layer texture file
    pub path: String,
    /// Size of a tile of the texture
    pub tile_size: Vec2,
    /// Columns in the texture file
    pub cols: usize,
    /// Rows in the texture file
    pub rows: usize,
    /// Scale of the texture
    pub scale: f32,
    /// Z position of the layer
    pub z: f32,
    /// Default initial position of the Entity container
    pub position: Vec2,
    /// Number used to determine when textures are moved to opposite side of camera
    pub transition_factor: f32,
}

pub struct ParallaxBG;

#[derive(Debug)]
pub struct ParallaxResource {
    /// Data to describe each layer of parallax
    pub layer_data: Vec<LayerData>,
    /// Parallax layer entities
    pub layer_entities: Vec<Entity>,
    /// Dimensions of window
    pub window_size: Vec2,
}

impl Default for ParallaxResource {
    fn default() -> Self {
        Self {
            layer_data: vec![],
            layer_entities: vec![],
            window_size: Vec2::ZERO,
        }
    }
}

impl ParallaxResource {
    /// Create a new parallax resource
    pub fn new(layer_data: Vec<LayerData>) -> Self {
        ParallaxResource {
            layer_data,
            layer_entities: vec![],
            window_size: Vec2::ZERO,
        }
    }

    /// Delete all layer entities in parallax resource and empty Vec
    pub fn despawn_layers(&mut self, commands: &mut Commands) {
        // Remove all layer entities
        for entity in self.layer_entities.iter() {
            commands.entity(*entity).despawn_recursive();
        }

        // Empty the layer entity vector
        self.layer_entities = vec![];
    }

    /// Create layers from layer data
    pub fn create_layers(
        &mut self,
        commands: &mut Commands,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) {
        // Despawn any existing layers
        self.despawn_layers(commands);

        // Spawn new layers using layer_data
        for (i, layer) in self.layer_data.iter().enumerate() {
            // Setup texture
            let texture_handle = asset_server.load(&layer.path);
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, layer.tile_size, layer.cols, layer.rows, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let spritesheet_bundle = SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..Default::default()
            };

            // Three textures always spawned
            let mut texture_count = 3.0;

            // Spawn parallax layer entity
            let mut entity_commands = commands.spawn();
            entity_commands
                .insert(Name::new(format!("Parallax Layer ({})", i)))
                .insert_bundle(SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(layer.position.x, layer.position.y, layer.z),
                        scale: Vec3::new(layer.scale, layer.scale, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Spawn center texture
                    parent.spawn_bundle(spritesheet_bundle.clone()).insert(
                        layer::LayerTextureComponent {
                            width: layer.tile_size.x,
                        },
                    );

                    let mut max_x = (layer.tile_size.x / 2.0) * layer.scale;
                    let mut adjusted_spritesheet_bundle = spritesheet_bundle.clone();

                    // Spawn right texture
                    adjusted_spritesheet_bundle.transform.translation.x += layer.tile_size.x;
                    max_x += layer.tile_size.x * layer.scale;
                    parent
                        .spawn_bundle(adjusted_spritesheet_bundle.clone())
                        .insert(layer::LayerTextureComponent {
                            width: layer.tile_size.x,
                        });

                    // Spawn left texture
                    parent
                        .spawn_bundle({
                            let mut bundle = adjusted_spritesheet_bundle.clone();
                            bundle.transform.translation.x *= -1.0;
                            bundle
                        })
                        .insert(layer::LayerTextureComponent {
                            width: layer.tile_size.x,
                        });

                    // Spawn additional textures to make 2 windows length of background textures
                    while max_x < self.window_size.x {
                        adjusted_spritesheet_bundle.transform.translation.x += layer.tile_size.x;
                        max_x += layer.tile_size.x * layer.scale;
                        parent
                            .spawn_bundle(adjusted_spritesheet_bundle.clone())
                            .insert(layer::LayerTextureComponent {
                                width: layer.tile_size.x,
                            });

                        parent
                            .spawn_bundle({
                                let mut bundle = adjusted_spritesheet_bundle.clone();
                                bundle.transform.translation.x *= -1.0;
                                bundle
                            })
                            .insert(layer::LayerTextureComponent {
                                width: layer.tile_size.x,
                            });

                        texture_count += 2.0;
                    }
                });

            // Add layer component to entity
            entity_commands.insert(layer::LayerComponent {
                speed: layer.speed,
                texture_count,
                transition_factor: layer.transition_factor,
            });

            // Push parallax layer entity to layer_entities
            self.layer_entities.push(entity_commands.id());
        }
    }
}

impl Plugin for ParallaxBG {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
