//! Workflow definitions and lightweight planning primitives.

/// Workflow trigger types supported by the foundation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowTrigger {
    /// Run when a file is imported.
    OnImport,
    /// Run on user demand.
    OnDemand,
    /// Run on a schedule.
    Scheduled,
}

/// A single workflow step definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowStep {
    /// Detect the media type.
    DetectType,
    /// Fetch metadata from an API provider.
    FetchApi { provider: String },
    /// Run a local AI analysis profile.
    RunAiAnalysis { profile: String },
    /// Ask the user for manual input.
    AskUser { field_name: String },
    /// Transform metadata or paths.
    Transform { rule: String },
    /// Persist YAML sidecars.
    SaveYaml,
}

/// A workflow definition that can later be persisted in YAML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowDefinition {
    /// Workflow name.
    pub name: String,
    /// Trigger type.
    pub trigger: WorkflowTrigger,
    /// Ordered list of steps.
    pub steps: Vec<WorkflowStep>,
}

impl WorkflowDefinition {
    /// Creates a new workflow definition.
    pub fn new(name: impl Into<String>, trigger: WorkflowTrigger, steps: Vec<WorkflowStep>) -> Self {
        Self {
            name: name.into(),
            trigger,
            steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_definition_keeps_ordered_steps() {
        let workflow = WorkflowDefinition::new(
            "auto-import",
            WorkflowTrigger::OnImport,
            vec![WorkflowStep::DetectType, WorkflowStep::SaveYaml],
        );

        assert_eq!(workflow.steps.len(), 2);
        assert!(matches!(workflow.steps[0], WorkflowStep::DetectType));
    }
}
