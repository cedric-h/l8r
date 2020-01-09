pub struct L8r<W>(Vec<Box<dyn FnOnce(&mut W)>>);
#[derive(Default)]
impl<W> L8r<W> {
    pub fn new() -> Self {
        L8r(Vec::new())
    }

    pub fn schedule(&mut self, then: Box<dyn FnOnce(&mut W)>) {
        self.0.push(then);
    }

    pub fn l8r<F: 'static + Send + Sync + FnOnce(&mut W)>(&mut self, then: F) {
        self.0.push(Box::new(then));
    }

    pub fn drain(&mut self) -> Vec<Box<dyn FnOnce(&mut W)>> {
        self.0.drain(..).collect::<Vec<_>>()
    }

    pub fn now(l8rs: Vec<Box<dyn FnOnce(&mut W)>>, world: &mut W) {
        for l8r in l8rs.into_iter() {
            l8r(world);
        }
    }
}
impl<W> L8r<W>
where W: ContainsHecsWorld {
    pub fn insert_one<C: hecs::Component>(&mut self, ent: hecs::Entity, component: C) {
        self.l8r(move |world| world.ecs_mut().insert_one(ent, component).unwrap())
    }

    pub fn remove_one<C: hecs::Component>(&mut self, ent: hecs::Entity) {
        self.l8r(move |world| drop(world.ecs_mut().remove_one::<C>(ent)))
    }

    pub fn insert<C: 'static + Send + Sync + hecs::DynamicBundle>(
        &mut self,
        ent: hecs::Entity,
        components_bundle: C,
    ) {
        self.l8r(move |world| world.ecs_mut().insert(ent, components_bundle).unwrap())
    }

    pub fn spawn<C: 'static + Send + Sync + hecs::DynamicBundle>(&mut self, components_bundle: C) {
        self.l8r(move |world| drop(world.ecs_mut().spawn(components_bundle)))
    }

    pub fn despawn(&mut self, entity: hecs::Entity) {
        self.l8r(move |world| drop(world.ecs_mut().despawn(entity)))
    }
}

pub trait ContainsHecsWorld {
    fn ecs(&self) -> &hecs::World;

    fn ecs_mut(&mut self) -> &mut hecs::World;
}

