#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_axes_actor.h");

        type vtkAxesActor;

        fn axes_actor_new() -> *mut vtkAxesActor;
        fn axes_actor_delete(actor: Pin<&mut vtkAxesActor>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkAxesActor.html",
    @name AxesActor, ffi::vtkAxesActor,
    @new ffi::axes_actor_new,
    @delete ffi::axes_actor_delete
);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkAxesActor`](https://vtk.org/doc/nightly/html/classvtkAxesActor.html)
///
/// A 3D axes representation with labeled X, Y, Z axes.
#[allow(non_camel_case_types)]
pub trait vtkAxesActor: private::Sealed {}
