use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

pub trait Event: Any + Send + Sync {
    /// Returns a `type name` of the [`Event`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

pub trait EventGroup {
    /// Adds a [`Event`]s in group to the [`EventRegistry`].
    fn configure(&self, registry: &mut EventRegistry);

    /// Returns a `type name` of the [`EventRegistry`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

struct EventEntry {
    event: Box<dyn Event>,
    enabled: bool,
}

#[derive(Default)]
pub struct EventRegistry {
    events: HashMap<TypeId, EventEntry>,
}

impl EventRegistry {
    /// Adds a [`Event`] in [`EventRegistry`] at the end.
    /// If the event was already in the group, its removed from its previous place.
    pub fn add<T>(&mut self, event: T) -> &mut Self
    where
        T: Event,
    {
        let i = self.events.len();
        self.upsert(event, i);
        self
    }

    /// Enables a [`Event`].
    pub fn enable<T>(&mut self) -> &mut Self
    where
        T: Event,
    {
        let mut e = self
            .events
            .get_mut(&TypeId::of::<T>())
            .expect("Cannot enable a event that does not exist.");
        e.enabled = true;

        self
    }

    /// Disables a [`Event`].
    pub fn disable<T>(&mut self) -> &mut Self
    where
        T: Event,
    {
        let mut e = self
            .events
            .get_mut(&TypeId::of::<T>())
            .expect("Cannot disable a event that does not exist.");
        e.enabled = false;

        self
    }

    /// Insert the new [`Event`] as enabled, and removes its previous ordering if it was already present.
    fn upsert<T>(&mut self, event: T, index: usize)
    where
        T: Event,
    {
        if let Some(e) = self.events.insert(
            TypeId::of::<T>(),
            EventEntry {
                event: Box::new(event),
                enabled: true,
            },
        ) {
            if e.enabled {
                todo!();
            }
        }
    }
}
