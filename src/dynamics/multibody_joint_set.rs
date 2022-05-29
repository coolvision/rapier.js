use crate::dynamics::{RawGenericJoint, RawIslandManager, RawRigidBodySet};
use crate::utils::{self, FlatHandle};
use rapier::dynamics::{MultibodyJoint, MultibodyJointSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RawMultibodyJointSet(pub(crate) MultibodyJointSet);

impl RawMultibodyJointSet {
    pub(crate) fn map<T>(&self, handle: FlatHandle, f: impl FnOnce(&MultibodyJoint) -> T) -> T {
        let (body, link_id) = self
            .0
            .get(utils::multibody_joint_handle(handle))
            .expect("Invalid Joint reference. It may have been removed from the physics World.");
        f(body.link(link_id).unwrap().joint())
    }

    // pub(crate) fn map_mut<T>(
    //     &mut self,
    //     handle: FlatHandle,
    //     f: impl FnOnce(&mut MultibodyJoint) -> T,
    // ) -> T {
    //     let (body, link_id, _) = self
    //         .0
    //         .get_mut(utils::multibody_handle(handle))
    //         .expect("Invalid Joint reference. It may have been removed from the physics World.");
    //     f(body.link(link_id).unwrap().joint())
    // }
}

#[wasm_bindgen]
impl RawMultibodyJointSet {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        RawMultibodyJointSet(MultibodyJointSet::new())
    }

    pub fn createJoint(
        &mut self,
        params: &RawGenericJoint,
        parent1: FlatHandle,
        parent2: FlatHandle,
    ) -> FlatHandle {
        // TODO: avoid the unwrap?
        let parent1 = utils::body_handle(parent1);
        let parent2 = utils::body_handle(parent2);

        self.0
            .insert(parent1, parent2, params.0.clone())
            .map(|h| utils::fuse_handle(h.0))
            .unwrap_or(FlatHandle::MAX)
    }

    pub fn remove(
        &mut self,
        handle: FlatHandle,
        islands: &mut RawIslandManager,
        bodies: &mut RawRigidBodySet,
        wakeUp: bool,
    ) {
        let handle = utils::multibody_joint_handle(handle);
        self.0.remove(handle, &mut islands.0, &mut bodies.0, wakeUp);
    }

    pub fn contains(&self, handle: FlatHandle) -> bool {
        self.0.get(utils::multibody_joint_handle(handle)).is_some()
    }

    /// Applies the given JavaScript function to the integer handle of each joint managed by this physics world.
    ///
    /// # Parameters
    /// - `f(handle)`: the function to apply to the integer handle of each joint managed by this set. Called as `f(collider)`.
    pub fn forEachJointHandle(&self, f: &js_sys::Function) {
        let this = JsValue::null();
        for (handle, _, _) in self.0.iter() {
            let _ = f.call1(&this, &JsValue::from(utils::fuse_handle(handle.0)));
        }
    }
}
