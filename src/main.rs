use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(print_names)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Matthias".to_string(),
    });
    commands.spawn(Person {
        name: "Katja".to_string(),
    });
}

fn print_names(person_query: Query<&Person>) {
    person_query.for_each(|person| println!("Hello {}", person.name));
}

#[derive(Component)]
struct Person {
    name: String,
}
