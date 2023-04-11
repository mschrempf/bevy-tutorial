use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(print_names)
        .add_system(persons_with_jobs)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Matthias".to_string(),
        },
        Employed { job: Job::Engineer },
    ));
    commands.spawn((
        Person {
            name: "Katja".to_string(),
        },
        Employed { job: Job::Forester },
    ));
    commands.spawn(Person {
        name: "Frodo".to_string(),
    });
}

fn print_names(person_query: Query<&Person>) {
    person_query.for_each(|person| println!("Hello {}", person.name));
}

fn persons_with_jobs(person_query: Query<&Person, With<Employed>>) {
    person_query.for_each(|person| println!("Hello {}, you have a job :-)", person.name));
}

#[derive(Component)]
struct Person {
    name: String,
}

#[derive(Component)]
struct Employed {
    job: Job,
}

#[derive(Debug)]
enum Job {
    Engineer,
    Forester,
}
