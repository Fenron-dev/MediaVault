//! Audit trail structures for property changes and review actions.

use crate::media::PropertySource;

/// One immutable change record in the audit trail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    /// Stable event identifier within the log.
    pub id: u64,
    /// Target entry identifier.
    pub entry_id: String,
    /// Changed field name.
    pub field_name: String,
    /// Previous value, if any.
    pub previous_value: Option<String>,
    /// New value, if any.
    pub new_value: Option<String>,
    /// Source that caused the change.
    pub source: PropertySource,
    /// UNIX timestamp in seconds.
    pub occurred_at_unix: u64,
    /// Optional human-readable note.
    pub note: Option<String>,
}

/// In-memory audit log used by the initial foundation.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AuditLog {
    events: Vec<AuditEvent>,
}

impl AuditLog {
    /// Creates an empty audit log.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Records a field change and returns the stored event.
    // Each parameter maps to a distinct audit column; bundling them into a
    // struct would only shift the argument list to the call sites.
    #[allow(clippy::too_many_arguments)]
    pub fn record_change(
        &mut self,
        entry_id: impl Into<String>,
        field_name: impl Into<String>,
        previous_value: Option<String>,
        new_value: Option<String>,
        source: PropertySource,
        occurred_at_unix: u64,
        note: Option<String>,
    ) -> AuditEvent {
        let id = self.events.len() as u64 + 1;
        let event = AuditEvent {
            id,
            entry_id: entry_id.into(),
            field_name: field_name.into(),
            previous_value,
            new_value,
            source,
            occurred_at_unix,
            note,
        };

        self.events.push(event.clone());
        event
    }

    /// Returns all recorded events.
    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Returns all events for a specific entry.
    pub fn events_for_entry(&self, entry_id: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|event| event.entry_id == entry_id)
            .collect()
    }

    /// Returns all events for a specific field.
    pub fn events_for_field(&self, field_name: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|event| event.field_name == field_name)
            .collect()
    }
}
