use std::env;
use std::fmt::{Display, Formatter};
use crate::document::html_convertible_error::HtmlConvertibleError;

pub enum ApplicationEnvParameter {
    MaxIterationsLimit,
    MaxCycleIterations,
    MaxVariableLength
}

impl Display for ApplicationEnvParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ApplicationEnvParameter::MaxIterationsLimit => {"MaxIterationsLimit"},
            ApplicationEnvParameter::MaxCycleIterations => {"MaxCycleIterations"},
            ApplicationEnvParameter::MaxVariableLength => {"MaxVariableLength"}
        })
    }
}

impl ApplicationEnvParameter {

    fn default(&self) -> String {
        match self {
            ApplicationEnvParameter::MaxIterationsLimit => 100.to_string(),
            ApplicationEnvParameter::MaxCycleIterations => 3.to_string(),
            ApplicationEnvParameter::MaxVariableLength => 8.to_string()
        }
    }

    /// Get application parameter as env variable
    /// If error occurs or none is found, return empty
    pub fn get(&self) -> Option<String> {
        env::var(self.to_string()).ok()
    }

    pub fn get_or_default(&self) -> String {
        let a = self.get().unwrap_or_default();
        if a.is_empty() {
            self.default()
        } else {
            a
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use crate::utils::env_parameters::ApplicationEnvParameter::MaxIterationsLimit;

    #[test]
    fn get_or_default_returns_default_value_when_env_var_not_presetn() {
        let p = MaxIterationsLimit.get_or_default();
        assert_eq!(p, "100");
    }

    #[test]
    fn get_or_default_returns_value_when_evn_present() {
        env::set_var("MaxIterationsLimit", "101");
        let p = MaxIterationsLimit.get_or_default();
        assert_eq!(p, "101");
    }

}