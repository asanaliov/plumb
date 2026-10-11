use plumb_core::{ObjectId, Scene, Vec3};
use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}\n\nRun `plumb --help` for usage.");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(usage().into());
    };
    match command {
        "--help" | "-h" | "help" => {
            println!("{}", usage());
            Ok(())
        }
        "new" => {
            let path = required(&args, 1, "new <scene.json>")?;
            write_scene(path, &Scene::new())
        }
        "add-box" => {
            let usage = "add-box <scene.json> <name> <size-x> <size-y> <size-z>";
            let path = required(&args, 1, usage)?;
            let mut scene = read_scene(path)?;
            let size = Vec3::new(
                number(&args, 3, usage)?,
                number(&args, 4, usage)?,
                number(&args, 5, usage)?,
            );
            let id = scene
                .add_box(required(&args, 2, usage)?, size)
                .map_err(|e| e.to_string())?;
            write_scene(path, &scene)?;
            println!("created object {}", id.get());
            Ok(())
        }
        "move" => {
            let usage = "move <scene.json> <object-id> <x> <y> <z>";
            let path = required(&args, 1, usage)?;
            let mut scene = read_scene(path)?;
            let id = required(&args, 2, usage)?
                .parse::<u64>()
                .map(ObjectId::from)
                .map_err(|_| format!("invalid object id\nUsage: plumb {usage}"))?;
            let position = Vec3::new(
                number(&args, 3, usage)?,
                number(&args, 4, usage)?,
                number(&args, 5, usage)?,
            );
            scene
                .set_position(id, position)
                .map_err(|e| e.to_string())?;
            write_scene(path, &scene)
        }
        "inspect" => {
            let path = required(&args, 1, "inspect <scene.json>")?;
            let scene = read_scene(path)?;
            println!("{}", scene.to_json().map_err(|e| e.to_string())?);
            Ok(())
        }
        _ => Err(format!("unknown command `{command}`\n\n{}", usage())),
    }
}

fn required<'a>(args: &'a [String], index: usize, usage: &str) -> Result<&'a str, String> {
    args.get(index)
        .map(String::as_str)
        .ok_or_else(|| format!("missing argument\nUsage: plumb {usage}"))
}

fn number(args: &[String], index: usize, usage: &str) -> Result<f64, String> {
    required(args, index, usage)?.parse::<f64>().map_err(|_| {
        format!(
            "expected a number for argument {}\nUsage: plumb {usage}",
            index + 1
        )
    })
}

fn read_scene(path: &str) -> Result<Scene, String> {
    let json =
        fs::read_to_string(path).map_err(|error| format!("could not read `{path}`: {error}"))?;
    Scene::from_json(&json).map_err(|error| error.to_string())
}

fn write_scene(path: &str, scene: &Scene) -> Result<(), String> {
    let json = scene.to_json().map_err(|error| error.to_string())?;
    fs::write(path, format!("{json}\n"))
        .map_err(|error| format!("could not write `{path}`: {error}"))
}

fn usage() -> &'static str {
    "Plumb - programmable 3D scene tools\n\nCommands:\n  plumb new <scene.json>\n  plumb add-box <scene.json> <name> <size-x> <size-y> <size-z>\n  plumb move <scene.json> <object-id> <x> <y> <z>\n  plumb inspect <scene.json>\n\nCoordinates and dimensions use scene units."
}
