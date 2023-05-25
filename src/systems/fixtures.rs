use std::time::Duration;

use bevy::prelude::*;

use crate::components::{MovablePerson, ProximityActivated};

const LIT: f32 = 4.0;
pub const DIMMED: f32 = 0.1;
const TIME_TILL_DEACTIVATED: Duration = Duration::from_millis(10000);

pub const BOX_HEIGHT: f32 = 1.25;
pub const BOX_WIDTH: f32 = 0.5;
pub const SPACING: f32 = 2.0; // centre-to-centre

pub fn light_up_activated(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entities: Query<(&Handle<StandardMaterial>, &mut ProximityActivated)>,
    time: Res<Time>,
) {
    for (material_handle, mut proximity) in &mut entities {
        let material = materials.get_mut(material_handle).unwrap();
        if proximity.is_activated {
            proximity.elapsed_activated += time.delta();
            if proximity.elapsed_activated >= TIME_TILL_DEACTIVATED {
                println!(
                    "Deactivated B, {} >= {}",
                    proximity.elapsed_activated.as_millis(),
                    TIME_TILL_DEACTIVATED.as_millis()
                );
                proximity.is_activated = false;
            } else {
                let total_time = TIME_TILL_DEACTIVATED.as_secs_f32();
                let progress = proximity.elapsed_activated.as_secs_f32() / total_time;
                // println!("{} / {} = {}", time_left, total_time, percentage);
                let brightness = (1.0 - progress) * LIT.max(DIMMED);
                material.emissive.set_r(brightness);
                material.emissive.set_g(brightness);
                material.emissive.set_b(brightness);
            }
        } else {
            material.emissive.set_r(DIMMED);
            material.emissive.set_g(DIMMED);
            material.emissive.set_b(DIMMED);
        }
    }
    // for (mut fixture, proximity) in &mut fixtures {}
}

pub fn make_close_activated(
    mut fixtures: Query<(&GlobalTransform, &mut ProximityActivated)>,
    user: Query<&GlobalTransform, With<MovablePerson>>,
) {
    // TODO: this works for single user
    let user_transform = user.single();

    for (transform, mut proximity) in &mut fixtures {
        let delta = transform.translation() - user_transform.translation();
        let distance = delta.length().abs();
        if !proximity.is_activated && distance < proximity.detection_radius {
            proximity.is_activated = true;
            proximity.elapsed_activated = Duration::ZERO;
            println!("Activated! {}", proximity.elapsed_activated.as_millis());
        }
    }
}
