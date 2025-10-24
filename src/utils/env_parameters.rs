use std::env;
use std::fmt::{Display, Formatter};

enum ApplicationEnvParameter {
    MAX_ITERATIONS_LIMIT,
    MAX_CYCLE_ITERATIONS
}

impl Display for ApplicationEnvParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ApplicationEnvParameter::MAX_ITERATIONS_LIMIT => {"MAX_ITERATIONS_LIMIT"},
            ApplicationEnvParameter::MAX_CYCLE_ITERATIONS => {"MAX_CYCLE_ITERATIONS"},
        })
    }
}

impl ApplicationEnvParameter {
    /// Get application parameter as env variable
    /// If error occurs or none is found, return empty
    pub fn get(&self) -> Option<String> {
        env::var(self.to_string()).ok()
    }
}