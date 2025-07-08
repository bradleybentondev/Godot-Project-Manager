use std::env;

enum Environment {
    DEV,
    TEST,
    PROD,
}

pub fn get_environment() -> Environment {
    match env::var("environment") {
        Ok(s) => match s.as_str() {
            "dev" => Environment::DEV,
            "test" => Environment::TEST,
            "prod" => Environment::PROD,
            _ => Environment::DEV,
        },
        _ => Environment::DEV,
    }
}

pub fn is_dev() -> bool {
    match get_environment() {
        Environment::DEV => true,
        _ => false,
    }
}

pub fn is_test() -> bool {
    match get_environment() {
        Environment::TEST => true,
        _ => false,
    }
}

pub fn is_prod() -> bool {
    match get_environment() {
        Environment::PROD => true,
        _ => false,
    }
}
