trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

struct Logger;
struct Validator;

impl Plugin for Logger {
    fn name(&self) -> &str { "Logger" }
    fn execute(&self) { println!("Logging..."); }
}

impl Plugin for Validator {
    fn name(&self) -> &str { "Validator" }
    fn execute(&self) { println!("Validating..."); }
}

fn run_plugins1(plugins: Vec<Box<dyn Plugin>>) {
    for plugin in &plugins {
        println!("{}: ", plugin.name());
        plugin.execute();
    }
}

fn run_plugins2<T>(plugins: Vec<T>)
where
    T: Plugin,
{
    for plugin in &plugins {
        println!("{}: ", plugin.name());
        plugin.execute();
    }
}

fn run_plugins2_logger(plugins: Vec<Logger>) {
    for plugin in &plugins {
        println!("{}: ", plugin.name());
        plugin.execute();
    }
}

fn run_plugins2_validator(plugins: Vec<Validator>) {
    for plugin in &plugins {
        println!("{}: ", plugin.name());
        plugin.execute();
    }
}

fn main() {
    let plugins: Vec<Box<dyn Plugin>> = vec![
        Box::new(Logger),
        Box::new(Validator),
    ];
    
    run_plugins2_logger(vec![Logger, Logger, Logger]);
    run_plugins2_validator(vec![Validator, Validator]);
    run_plugins2(vec![Logger, Validator]);
}

