/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::{Display, Formatter, Result as FmtResult};

// Stub for various other built-in classes, which are currently incomplete, but whose types
// are required for codegen
use crate::builtin::{GodotString, StringName, Vector2};
use crate::obj::{Gd, GodotClass};
use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

// TODO: Swap more inner math types with glam types
// Note: ordered by enum ord in extension JSON
impl_builtin_stub!(Rect2, OpaqueRect2);
impl_builtin_stub!(Rect2i, OpaqueRect2i);
impl_builtin_stub!(Plane, OpaquePlane);
impl_builtin_stub!(Quaternion, OpaqueQuaternion);
impl_builtin_stub!(AABB, OpaqueAABB);
impl_builtin_stub!(Basis, OpaqueBasis);
impl_builtin_stub!(Transform2D, OpaqueTransform2D);
impl_builtin_stub!(Transform3D, OpaqueTransform3D);
impl_builtin_stub!(Projection, OpaqueProjection);
impl_builtin_stub!(NodePath, OpaqueNodePath);
impl_builtin_stub!(RID, OpaqueRID);
impl_builtin_stub!(Callable, OpaqueCallable);
impl_builtin_stub!(Signal, OpaqueSignal);
impl_builtin_stub!(Dictionary, OpaqueDictionary);

#[repr(C)]
struct InnerRect {
    position: Vector2,
    size: Vector2,
}

impl Rect2 {
    pub fn size(self) -> Vector2 {
        self.inner().size
    }

    fn inner(self) -> InnerRect {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<&GodotString> for NodePath {
    fn from(path: &GodotString) -> Self {
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = sys::method_table().node_path_from_string;
                let args = [path.sys()];
                ctor(self_ptr, args.as_ptr());
            })
        }
    }
}

impl From<&NodePath> for GodotString {
    fn from(path: &NodePath) -> Self {
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = sys::method_table().string_from_node_path;
                let args = [path.sys()];
                ctor(self_ptr, args.as_ptr());
            })
        }
    }
}

impl From<&str> for NodePath {
    fn from(path: &str) -> Self {
        Self::from(&GodotString::from(path))
    }
}

impl Display for NodePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let string = GodotString::from(self);
        <GodotString as Display>::fmt(&string, f)
    }
}

impl Callable {
    pub fn from_object_method<T, S>(object: Gd<T>, method: S) -> Self
    where
        T: GodotClass, // + Inherits<Object>,
        S: Into<StringName>,
    {
        // upcast not needed
        let method = method.into();
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = sys::method_table().callable_from_object_method;
                let args = [object.sys(), method.sys()];
                ctor(self_ptr, args.as_ptr());
            })
        }
    }
}
