

//! In this example we add a counter resource and increase its value in one system,
//! while a different system prints the current count to the console.

#![expect(clippy::std_instead_of_core)]

use bevy_ecs::prelude::*;
use bevy_ecs::world::OnMutate;
use bevy_reflect::Reflect;

fn main() {
    let mut world = World::new();

    world.add_observer(ob);

    let mut schedule = Schedule::default();

    schedule
        .add_systems(
            (
                first,
                second,
            )
                .chain()
        );

    schedule.run(&mut world);

    println!("{:?}", world);
}

#[derive(Reflect, Default, Component, Debug)]
struct Count(usize);

fn first(mut commands: Commands) {
    commands.spawn(Count(0));
}

fn second(mut counts: Query<Mut<Count>>) {
    counts.iter_mut().for_each(|mut count| {
        count.0 += 1;
    })
}

fn ob(
    trigger: Trigger<OnMutate, Count>,
    counts:  Query<&Count>,
) {
    let Ok(count) = counts.get(trigger.entity()) else {
        return;
    };
    dbg!(count);
}