use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct MyConfig {
    db_url: String,
    verbosity: String,
    git_config_path: String
}

pub fn get_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: MyConfig = confy::load("barst", "config")?;
    dbg!(cfg);
    Ok(())
}

#[test]
fn test_config_values() {
    let vals: MyConfig = confy::load("barst", "config").expect("failed");
    assert!(vals.verbosity == "DEBUG", "Expected Verbosity to Debug, but got {}", vals.verbosity);
}

fn config_values() -> MyConfig {
    let vals: MyConfig = confy::load("barst", "config").expect("failed");
    return vals;
}

#[test]
fn check_config_file() {
    let result = get_config_file();
    match result {
        Ok(()) => println!("Success!"),
        Err(e) => {
            println!("Error occurred: {}", e)
        },
    }
    let vals = config_values();
    println!("data: {}", vals.verbosity)
}