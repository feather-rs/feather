use std::{
    any::{type_name, Any, TypeId},
    marker::PhantomData,
};

use ahash::AHashMap;

use crate::{HasResources, SysResult};

struct Handler<Input, E> {
    function: Box<dyn FnMut(&mut Input, &E) -> SysResult>,
    name: String,
}

impl<Input, E> Handler<Input, E> {
    fn from_fn<F: FnMut(&mut Input, &E) -> SysResult + 'static>(f: F) -> Self {
        Self {
            function: Box::new(f),
            name: type_name::<F>().to_owned(),
        }
    }
}

trait DynHandler<Input> {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<Input, E> DynHandler<Input> for Handler<Input, E>
where
    Input: 'static,
    E: 'static,
{
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// An event bus for handling events.
///
/// This struct maintains a sequence of event
/// handlers for each event type. When [`handle`] is
/// called, the handlers for the corresponding event
/// are invoked.
pub struct EventBus<Input> {
    handlers: AHashMap<TypeId, Vec<Box<dyn DynHandler<Input>>>>,
}

impl<Input: 'static> EventBus<Input> {
    /// Creates a new, empty `EventBus`.
    pub fn new() -> Self {
        Self {
            handlers: AHashMap::new(),
        }
    }

    /// Adds an event handler.
    pub fn add_handler<E: 'static>(
        &mut self,
        handler: impl FnMut(&mut Input, &E) -> SysResult + 'static,
    ) -> &mut Self {
        let handler = Handler::from_fn(handler);
        self.handlers
            .entry(TypeId::of::<E>())
            .or_default()
            .push(Box::new(handler));
        self
    }

    /// Starts a new group of event handlers with
    /// convenient access to the given `State` type.
    ///
    /// The `State` must be stored in the input's resources.
    pub fn group<State>(&mut self) -> HandlerGroupBuilder<Input, State> {
        HandlerGroupBuilder {
            bus: self,
            _marker: PhantomData,
        }
    }

    /// Invokes event handlers for the given event.
    pub fn handle<E: 'static>(&mut self, input: &mut Input, event: &E) {
        if let Some(handlers) = self.handlers.get_mut(&TypeId::of::<E>()) {
            for handler in handlers {
                let handler = handler
                    .as_any_mut()
                    .downcast_mut::<Handler<Input, E>>()
                    .expect("invalid handler type");

                let result = (handler.function)(input, event);

                if let Err(e) = result {
                    log::error!(
                        "Error while handling event of type `{}` caused by handler '{}': {:?}",
                        type_name::<E>(),
                        handler.name,
                        e
                    );
                }
            }
        }
    }
}

/// Builder for a group of event handlers. Created
/// with [`EventBus::group`].
pub struct HandlerGroupBuilder<'a, Input, State> {
    bus: &'a mut EventBus<Input>,
    _marker: PhantomData<State>,
}

impl<'a, Input, State> HandlerGroupBuilder<'a, Input, State>
where
    Input: HasResources + 'static,
    State: Send + Sync + 'static,
{
    /// Adds a handler to this group.
    pub fn add_handler<E: 'static>(
        &mut self,
        handler: impl FnMut(&mut Input, &mut State, &E) -> SysResult + 'static,
    ) -> &mut Self {
        let function = Self::make_function(handler);
        self.bus.add_handler(function);
        self
    }

    fn make_function<E: 'static>(
        mut handler: impl FnMut(&mut Input, &mut State, &E) -> SysResult + 'static,
    ) -> impl FnMut(&mut Input, &E) -> SysResult + 'static {
        move |input, event| {
            let resources = input.resources();
            let mut state = resources
                .get_mut::<State>()
                .expect("missing state resource for event handler group");
            handler(input, &mut *state, event)
        }
    }
}
