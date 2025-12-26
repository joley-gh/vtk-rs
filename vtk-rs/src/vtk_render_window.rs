#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_render_window.h");
        include!("vtk_renderer.h");

        type vtkRenderWindow;
        type vtkRenderer;

        fn render_window_new() -> *mut vtkRenderWindow;
        fn render_window_delete(window: Pin<&mut vtkRenderWindow>);
        unsafe fn render_window_add_renderer(
            window: Pin<&mut vtkRenderWindow>,
            renderer: *mut vtkRenderer
        );
        fn render_window_set_size(window: Pin<&mut vtkRenderWindow>, width: i32, height: i32);
        fn render_window_set_window_name(window: Pin<&mut vtkRenderWindow>, name: &str);
        fn render_window_render(window: Pin<&mut vtkRenderWindow>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkRenderWindow.html",
    @name RenderWindow, ffi::vtkRenderWindow,
    @new ffi::render_window_new,
    @delete ffi::render_window_delete
);

impl RenderWindow {
    pub fn add_renderer(&mut self, renderer: &mut crate::Renderer) {
        unsafe {
            let renderer_ptr = renderer.as_mut_ptr() as *mut ffi::vtkRenderer;
            ffi::render_window_add_renderer(self.ptr.as_mut(), renderer_ptr);
        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        ffi::render_window_set_size(self.ptr.as_mut(), width, height);
    }

    pub fn set_window_name(&mut self, name: &str) {
        ffi::render_window_set_window_name(self.ptr.as_mut(), name);
    }

    pub fn render(&mut self) {
        ffi::render_window_render(self.ptr.as_mut());
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkRenderWindow`](https://vtk.org/doc/nightly/html/classvtkRenderWindow.html)
#[allow(non_camel_case_types)]
pub trait vtkRenderWindow: private::Sealed {}
