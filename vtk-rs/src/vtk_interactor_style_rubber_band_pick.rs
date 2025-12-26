use std::ffi::c_void;

#[repr(C)]
pub struct vtkInteractorStyleRubberBandPick {
    _private: [u8; 0],
}

extern "C" {
    fn interactor_style_rubber_band_pick_new() -> *mut vtkInteractorStyleRubberBandPick;
    fn interactor_style_rubber_band_pick_delete(style: *mut vtkInteractorStyleRubberBandPick);
    fn interactor_style_rubber_band_pick_set_interactor(
        style: *mut vtkInteractorStyleRubberBandPick,
        interactor: *mut c_void
    );
}

pub struct InteractorStyleRubberBandPick {
    ptr: *mut vtkInteractorStyleRubberBandPick,
}

impl InteractorStyleRubberBandPick {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: interactor_style_rubber_band_pick_new(),
            }
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut vtkInteractorStyleRubberBandPick {
        self.ptr
    }

    pub fn set_interactor(&mut self, interactor: &mut crate::RenderWindowInteractor) {
        unsafe {
            interactor_style_rubber_band_pick_set_interactor(
                self.ptr,
                interactor.as_mut_ptr() as *mut c_void
            );
        }
    }
}

impl Drop for InteractorStyleRubberBandPick {
    fn drop(&mut self) {
        unsafe {
            interactor_style_rubber_band_pick_delete(self.ptr);
        }
    }
}

unsafe impl Send for InteractorStyleRubberBandPick {}
