use super::*;

/// The Entity offspring, represented as a list of heap allocated Entity traits.
#[derive(Debug)]
pub struct Offspring<'e, K, C> {
    entities: Vec<Box<entity::Trait<'e, K, C>>>,
}

impl<'e, K, C> Default for Offspring<'e, K, C> {
    /// Gets an empty Offspring.
    fn default() -> Self {
        Self {
            entities: Vec::default(),
        }
    }
}

impl<'e, K, C> Offspring<'e, K, C> {
    /// Adds a new Entity to the Offspring.
    pub fn insert<Q>(&mut self, entity: Q)
    where
        Q: Entity<'e, Kind = K, Context = C> + 'e,
    {
        self.entities.push(Box::new(entity));
    }

    /// Takes the entities out of Self to create a new Offspring. Useful when
    /// you want to release a new Entity offspring into the Environment.
    pub fn drain(&mut self) -> Self {
        Self {
            entities: self.entities.drain(..).collect(),
        }
    }

    /// Takes the entities out of the Offspring.
    pub(crate) fn take_entities(self) -> Vec<Box<entity::Trait<'e, K, C>>> {
        self.entities
    }
}