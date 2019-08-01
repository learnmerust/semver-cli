#[macro_use]
extern crate clap;
extern crate semver;

use clap::{App, ArgMatches};
use semver::Version;

fn version_ok(version: &str) -> bool {
    Version::parse(version).is_ok()
}

fn increment_parsed(mut parsed: Version, increment: &str) -> Version {
    match increment {
        "none" => parsed.increment_patch(),
        "patch" => parsed.increment_patch(),
        "minor" => parsed.increment_minor(),
        "major" => parsed.increment_major(),
        _ => parsed.increment_patch(),
    }
    parsed
}

fn handle_input(input: &str, matches: &ArgMatches) {
    if matches.occurrences_of("increment") != 0 {
        if version_ok(input) {
            let increment = matches.value_of("increment").unwrap();
            let parsed = Version::parse(input).unwrap();
            let incremented = increment_parsed(parsed, increment);
            return println!("{}", incremented);
        }
    }

    if version_ok(input) {
        println!("{}", input)
    }
}

fn init() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let ref_to_matches = &matches;
    let input = matches.value_of("INPUT").unwrap();

    handle_input(input, ref_to_matches)
}

fn main() {
    init()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_version_ok() {
        assert_eq!(version_ok("1.2.3"), true);
    }

    #[test]
    fn test_increment_parsed_empty() {
        let a = increment_parsed(Version::parse("1.2.3").unwrap(), "");
        let b = Version::parse("1.2.4").unwrap();
        assert_eq!(a, b)
    }

    #[test]
    fn test_increment_parsed_patch() {
        let a = increment_parsed(Version::parse("1.2.3").unwrap(), "patch");
        let b = Version::parse("1.2.4").unwrap();
        assert_eq!(a, b)
    }

    #[test]
    fn test_increment_parsed_minor() {
        let a = increment_parsed(Version::parse("1.2.3").unwrap(), "minor");
        let b = Version::parse("1.3.0").unwrap();
        assert_eq!(a, b)
    }

    #[test]
    fn test_increment_parsed_major() {
        let a = increment_parsed(Version::parse("1.2.3").unwrap(), "major");
        let b = Version::parse("2.0.0").unwrap();
        assert_eq!(a, b)
    }
}
