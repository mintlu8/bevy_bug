use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};
use bevy_defer::{AsyncExtension, AsyncWorld};

fn main() {
    let mut image = Image::new(
        Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        }, 
        TextureDimension::D2, 
        vec![[0, 0, 0, 255]; 256 * 256].into_flattened(), 
        TextureFormat::Rgba8Unorm, 
        RenderAssetUsages::all()
    );
    for x in 0..128 {
        for y in 0..128 {
            // Make first quadrant red.
            image.data[(y * 256 + x) * 4] = 255;
        }
    }
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    // Used to induce a delay
    app.add_plugins(bevy_defer::AsyncPlugin::default_settings());
    let img = app.world_mut().resource_mut::<Assets<Image>>().add(image);
    let mesh = app.world_mut().resource_mut::<Assets<Mesh>>().add(
        Plane3d::new(Vec3::Y, Vec2::ONE).mesh()
    );
    let mat = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(
        StandardMaterial {
            base_color_texture: Some(img.clone()),
            unlit: true,
            ..Default::default()
        }
    );

    app.world_mut().spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0., 2., 0.))
            .looking_at(Vec3::ZERO, Vec3::Y)
    ));
    app.world_mut().spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat)
    ));
    let img = img.id();
    app.spawn_task(async move {
        AsyncWorld.sleep(2.).await;
        // Update the image.
        AsyncWorld.run_cached_system(
            move |mut assets: ResMut<Assets<Image>>| {
                if let Some(img) = assets.get_mut(img) {
                    for x in 0..128 {
                        for y in 0..128 {
                            // Make fourth quadrant blue.
                            img.data[((y + 128) * 256 + x + 128) * 4 + 2] = 255;
                        }
                    }
                    println!("Changed fourth quadrant to blue!");
                }
            }
        )?;
        AsyncWorld.sleep(2.).await;
        // Show the image has updated, despite the display did not.
        AsyncWorld.run_cached_system(
            move |mut assets: ResMut<Assets<Image>>| {
                if let Some(img) = assets.get_mut(img) {
                    ::image::RgbaImage::from_vec(256, 256, img.data.clone())
                        .unwrap()
                        .save("result.png")
                        .unwrap();
                    println!("Saved the image!")
                }
            }
        )?;
        Ok(())
    });
    
    app.run();
}
