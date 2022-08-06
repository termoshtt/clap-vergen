#[derive(Debug, Clone)]
pub struct Git {
    /// `VERGEN_GIT_BRANCH` e.g. `feature/fun`
    pub branch: String,
    /// `VERGEN_GIT_COMMIT_TIMESTAMP` e.g. `2021-02-24T20:55:21+00:00`
    pub commit_timestamp: String,
    /// `VERGEN_GIT_SEMVER` e.g. `5.0.0-2-gf49246c`
    pub semver: String,
    /// `VERGEN_GIT_SHA` e.g. `f49246ce334567bff9f950bfd0f3078184a2738a`
    pub sha: String,
}

impl Default for Git {
    fn default() -> Self {
        Git {
            branch: env!("VERGEN_GIT_BRANCH").to_string(),
            commit_timestamp: env!("VERGEN_GIT_COMMIT_TIMESTAMP").to_string(),
            semver: env!("VERGEN_GIT_SEMVER").to_string(),
            sha: env!("VERGEN_GIT_SHA").to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VergenInfo {
    pub git: Git,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_env() {
        let info = VergenInfo::default();
        dbg!(info);
    }
}
