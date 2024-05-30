extern  crate glfw;
extern  crate gl;
use glfw::{Action, Context, Key, WindowEvent, PWindow, fail_on_errors, WindowHint, OpenGlProfileHint, GlfwReceiver};
use std::sync::mpsc::Receiver;
use gl::types::*;
use core::ffi::c_uint;
use std::mem::size_of;

pub struct window{
    glfw : glfw::Glfw,
    window_handle : PWindow,
    events : GlfwReceiver<(f64, WindowEvent)>,
    width : u32,
    height : u32
    //lol

}


pub fn new(width : u32 , height : u32, title : &str) -> window {
    let mut _glfw = glfw::init(fail_on_errors!()).unwrap();
    _glfw.window_hint(WindowHint::ContextVersionMajor(3));
    _glfw.window_hint(WindowHint::ContextVersionMinor(3));
    _glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    let (mut _window, events) = _glfw.create_window(width, height, title, glfw::WindowMode::Windowed).unwrap();
    _window.make_current();
    _window.set_key_polling(true);
    let r_window : window = window{
        width : width,
        height : height,
        glfw : _glfw,
        window_handle : _window,
        events : events
    };
    return r_window;
}

impl window {
    pub fn start(&mut self){
        gl::Viewport::load_with(|s| self.window_handle.get_proc_address(s));
        gl::load_with(|s| self.window_handle.get_proc_address(s));
        if gl::Viewport::is_loaded() {
           println!("Loalded");
        }
        unsafe {
            gl::Viewport(0, 0, self.width.try_into().unwrap(), self.height.try_into().unwrap());
        }
        self.drawing_things();
        while !self.window_handle.should_close(){
            unsafe {
          //      gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::DrawArrays(gl::TRIANGLES, 0, 3);
               // gl::DrawElements(gl::TRIANGLES, 3, )
            }


            self.window_handle.swap_buffers();
            self.glfw.poll_events();
            for (_,event) in glfw::flush_messages(&self.events){
                match event{
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window_handle.set_should_close(true);
                    },
                    _ => {}
                }
            }

        }
    }
    pub fn drawing_things(&self){
        unsafe {
            let mut buffer : c_uint = 0;
            let pos : [f32; 6] = [
                -0.5, 0.5,
                0.0, 0.5,
                0.5, -0.5
            ];

            let mut VAO : c_uint = 0;
            gl::GenVertexArrays(1, &mut VAO);
            gl::BindVertexArray(VAO);

            let buffer_size = (pos.len() * std::mem::size_of::<f64>()) as GLsizeiptr;
            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
            gl::BufferData(gl::ARRAY_BUFFER, buffer_size ,pos.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 8, std::ptr::null());
        }
    }
}