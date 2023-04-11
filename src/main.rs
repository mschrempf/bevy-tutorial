use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(PeoplePlugin)
        .add_plugins(DefaultPlugins)
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

struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(persons_with_jobs)
            .add_system(persons_and_jobs);
    }
}

fn print_names(person_query: Query<&Person>) {
    person_query.for_each(|person| println!("Hello {}", person.name));
}

fn persons_with_jobs(person_query: Query<&Person, With<Employed>>) {
    person_query.for_each(|person| println!("Hello {}, you have a job :-)", person.name));
}

fn persons_and_jobs(q: Query<(&Person, &Employed)>) {
    q.for_each(|(p, e)| println!("{} works as a {:?}", p.name, e.job));
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
