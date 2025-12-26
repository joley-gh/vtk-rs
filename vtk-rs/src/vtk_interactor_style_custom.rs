use std::sync::Mutex;
use std::collections::HashMap;

// Ensure the vtkrs static library is linked
#[link(name = "vtkrs", kind = "static")]
extern "C" {}

// Global registry for callbacks
static CALLBACK_REGISTRY: Mutex<Option<CallbackRegistry>> = Mutex::new(None);

struct CallbackRegistry {
    next_id: i64,
    left_press_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
    left_release_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
    mouse_move_callbacks: HashMap<i64, Box<dyn Fn(i32, i32) + Send>>,
    key_press_callbacks: HashMap<i64, Box<dyn Fn(&str) + Send>>,
}

impl CallbackRegistry {
    fn new() -> Self {
        Self {
            next_id: 1,
            left_press_callbacks: HashMap::new(),
            left_release_callbacks: HashMap::new(),
            mouse_move_callbacks: HashMap::new(),
            key_press_callbacks: HashMap::new(),
        }
    }

    fn register_left_press<F>(&mut self, callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
        let id = self.next_id;
        self.next_id += 1;
        self.left_press_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_left_release<F>(&mut self, callback: F) -> i64
        where F: Fn(i32, i32) + Send + 'static
    {
        let id = self.next_id;
        self.next_id += 1;
        self.left_release_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_mouse_move<F>(&mut self, callback: F) -> i64 where F: Fn(i32, i32) + Send + 'static {
        let id = self.next_id;
        self.next_id += 1;
        self.mouse_move_callbacks.insert(id, Box::new(callback));
        id
    }

    fn register_key_press<F>(&mut self, callback: F) -> i64 where F: Fn(&str) + Send + 'static {
        let id = self.next_id;
        self.next_id += 1;
        self.key_press_callbacks.insert(id, Box::new(callback));
        id
    }
}

fn get_or_create_registry() -> &'static Mutex<Option<CallbackRegistry>> {
    let mut registry = CALLBACK_REGISTRY.lock().unwrap();
    if registry.is_none() {
        *registry = Some(CallbackRegistry::new());
    }
    drop(registry);
    &CALLBACK_REGISTRY
}

// C callback trampolines - these are called from C++
#[no_mangle]
pub extern "C" fn vtk_rs_left_button_press_callback(callback_id: i64, x: i32, y: i32) {
    // Add defensive check and error handling
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.left_press_callbacks.get(&callback_id) {
                // Use catch_unwind to prevent panics from crossing FFI boundary
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_left_button_release_callback(callback_id: i64, x: i32, y: i32) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.left_release_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_mouse_move_callback(callback_id: i64, x: i32, y: i32) {
    if callback_id == 0 {
        return;
    }

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.mouse_move_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(x, y);
                    })
                );
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn vtk_rs_key_press_callback(callback_id: i64, key: *const std::os::raw::c_char) {
    if callback_id == 0 || key.is_null() {
        return;
    }

    // Convert C string to Rust string
    let key_str = unsafe { std::ffi::CStr::from_ptr(key).to_str().unwrap_or("") };

    if let Ok(registry) = CALLBACK_REGISTRY.try_lock() {
        if let Some(ref reg) = *registry {
            if let Some(callback) = reg.key_press_callbacks.get(&callback_id) {
                let _ = std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        callback(key_str);
                    })
                );
            }
        }
    }
}

// Direct extern "C" FFI (bypassing cxx bridge which was crashing)
#[repr(C)]
pub struct vtkInteractorStyleCustom {
    _private: [u8; 0],
}

extern "C" {
    fn interactor_style_custom_new() -> *mut vtkInteractorStyleCustom;
    fn interactor_style_custom_delete(style: *mut vtkInteractorStyleCustom);
    fn interactor_style_custom_set_left_button_press_callback_id(
        style: *mut vtkInteractorStyleCustom,
        callback_id: i64
    );
    fn interactor_style_custom_set_left_button_release_callback_id(
        style: *mut vtkInteractorStyleCustom,
        callback_id: i64
    );
    fn interactor_style_custom_set_mouse_move_callback_id(
        style: *mut vtkInteractorStyleCustom,
        callback_id: i64
    );
    fn interactor_style_custom_set_key_press_callback_id(
        style: *mut vtkInteractorStyleCustom,
        callback_id: i64
    );
}

// Wrapper struct for safe Rust API
pub struct InteractorStyleCustom {
    ptr: *mut vtkInteractorStyleCustom,
}

impl InteractorStyleCustom {
    pub fn new() -> Self {
        crate::init_vtk();
        let ptr = unsafe { interactor_style_custom_new() };
        if ptr.is_null() {
            panic!("Failed to create InteractorStyleCustom");
        }
        Self { ptr }
    }

    pub fn as_mut_ptr(&mut self) -> *mut vtkInteractorStyleCustom {
        self.ptr
    }

    /// Set callback for left mouse button press events.
    /// The callback receives the (x, y) position of the click.
    pub fn set_left_button_press_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        get_or_create_registry();
        let callback_id = {
            let mut registry = CALLBACK_REGISTRY.lock().unwrap();
            registry.as_mut().unwrap().register_left_press(callback)
        };

        unsafe {
            interactor_style_custom_set_left_button_press_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for left mouse button release events.
    /// The callback receives the (x, y) position where the button was released.
    pub fn set_left_button_release_callback<F>(&mut self, callback: F)
        where F: Fn(i32, i32) + Send + 'static
    {
        get_or_create_registry();
        let callback_id = {
            let mut registry = CALLBACK_REGISTRY.lock().unwrap();
            registry.as_mut().unwrap().register_left_release(callback)
        };

        unsafe {
            interactor_style_custom_set_left_button_release_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for mouse move events.
    /// The callback receives the current (x, y) position of the mouse.
    /// Note: This fires frequently during mouse movement.
    pub fn set_mouse_move_callback<F>(&mut self, callback: F) where F: Fn(i32, i32) + Send + 'static {
        get_or_create_registry();
        let callback_id = {
            let mut registry = CALLBACK_REGISTRY.lock().unwrap();
            registry.as_mut().unwrap().register_mouse_move(callback)
        };

        unsafe {
            interactor_style_custom_set_mouse_move_callback_id(self.ptr, callback_id);
        }
    }

    /// Set callback for key press events.
    /// The callback receives the key symbol as a string (e.g., "m", "Escape", "F1").
    pub fn set_key_press_callback<F>(&mut self, callback: F) where F: Fn(&str) + Send + 'static {
        get_or_create_registry();
        let callback_id = {
            let mut registry = CALLBACK_REGISTRY.lock().unwrap();
            registry.as_mut().unwrap().register_key_press(callback)
        };

        unsafe {
            interactor_style_custom_set_key_press_callback_id(self.ptr, callback_id);
        }
    }
}

impl Drop for InteractorStyleCustom {
    fn drop(&mut self) {
        unsafe {
            interactor_style_custom_delete(self.ptr);
        }
    }
}
