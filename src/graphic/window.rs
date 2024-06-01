extern  crate glfw;
extern  crate gl;
use glfw::{Action, Context, Key, WindowEvent, PWindow, fail_on_errors, WindowHint, OpenGlProfileHint, GlfwReceiver};
use std::sync::mpsc::Receiver;
use gl::types::*;
use core::ffi::c_uint;
use std::ffi::c_void;
use std::mem::size_of;
use std::ffi::CString;
use std::env;
use std::fs;
pub struct window{
    glfw : glfw::Glfw,
    window_handle : PWindow,
    events : GlfwReceiver<(f64, WindowEvent)>,
    width : u32,
    height : u32,
    VBOs : [GLuint; 2],
    VAOs : [GLuint; 2]
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
    let mut r_window : window = window{
        width : width,
        height : height,
        glfw : _glfw,
        window_handle : _window,
        events : events,
        VBOs : [0, 0],
        VAOs : [0, 0]
    };
    return r_window;
}
fn read_text_file(filePath : &str) -> CString{
    let bind :String = fs::read_to_string(filePath).unwrap();
    let concept : &str = bind.as_str();

    CString::new(concept).unwrap()

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

        self.drawing_two_trig_diff();
        while !self.window_handle.should_close(){
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

              //  gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
             //   gl::BindVertexArray(0);
                gl::BindVertexArray(self.VAOs[0]);
                gl::DrawArrays(gl::TRIANGLES,0, 3);
                gl::BindVertexArray(self.VAOs[1]);
                gl::DrawArrays(gl::TRIANGLES,0, 3);

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
    pub fn drawing_things(&mut self){
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            let mut buffer : c_uint = 0;
            let pos : [f32; 12] = [
                //first triangle
                0.5,  0.5, 0.0, // top right
                0.5, -0.5, 0.0, // bottom right
                -0.5, -0.5, 0.0, // bottom left
                -0.5,  0.5, 0.0  // top left
            ];

            let indices : [i32; 6] = [
                0, 1, 3, // first triangle (top right, bottom right, top left)
                1, 2, 3  // second triangle (bottom right, bottom left, top left)
            ];
            let buffer_size = (pos.len() * std::mem::size_of::<f64>()) as GLsizeiptr;
            let EBO_buffer_size  = (indices.len() * std::mem::size_of::<i32>()) as GLsizeiptr;


            let mut VAO : GLuint = 0;
            gl::GenVertexArrays(1, &mut VAO);
            gl::BindVertexArray(VAO);

            let mut VBO : c_uint = 0;
            gl::GenBuffers(1, &mut VBO);
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER, buffer_size, pos.as_ptr() as *const c_void, gl::STATIC_DRAW);

            let mut EBO : GLuint = 0;
            gl::GenBuffers(1,&mut EBO);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, EBO_buffer_size, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);

//            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
  //          gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, EBO_buffer_size, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);


            let vertex_shader_source = "#version 330 core\n layout (location = 0) in vec3 aPos; void main() { gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0); }";
            let c_str_source = CString::new(vertex_shader_source).unwrap();
            let mut vertexshader: GLuint = 0;

            vertexshader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertexshader, 1, &c_str_source.as_ptr(), std::ptr::null());
            gl::CompileShader(vertexshader);

            let mut fragmentshader : GLuint;
            fragmentshader = gl::CreateShader(gl::FRAGMENT_SHADER);

            let fragment_shader_source = "#version 330 core\n out vec4 FragColor; \n void main(){ \n FragColor = vec4(1.0f); \n }";

            let fragment_c_source = CString::new(fragment_shader_source).unwrap();
            gl::ShaderSource(fragmentshader, 1,  &fragment_c_source.as_ptr(), std::ptr::null());
            gl::CompileShader(fragmentshader);

            let mut shader_program : GLuint;
            shader_program = gl::CreateProgram();

            gl::AttachShader(shader_program, vertexshader);
            gl::AttachShader(shader_program, fragmentshader);
            gl::LinkProgram(shader_program);

            gl::UseProgram(shader_program);

            gl::DeleteShader(vertexshader);
            gl::DeleteShader(fragmentshader);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::UseProgram(shader_program);

        }
    }


    pub fn drawing_two_trig(&mut self){
        let vert : [f32 ; 18] = [
            -0.9, -0.5, 0.0,  // left
            -0.0, -0.5, 0.0,  // right
            -0.45, 0.5, 0.0,  // top
            // second triangle
            0.0, -0.5, 0.0,  // left
            0.9, -0.5, 0.,  // right
            0.45, 0.5, 0.0   // top
        ];

        unsafe {
            let vertex_shader_source = "#version 330 core\n layout (location = 0) in vec3 aPos; void main() { gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0); }";
            let c_str_source = CString::new(vertex_shader_source).unwrap();
            let mut vertexshader: GLuint = 0;

            vertexshader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertexshader, 1, &c_str_source.as_ptr(), std::ptr::null());
            gl::CompileShader(vertexshader);

            let mut fragmentshader : GLuint;
            fragmentshader = gl::CreateShader(gl::FRAGMENT_SHADER);

            let fragment_shader_source = "#version 330 core\n out vec4 FragColor; \n void main(){ \n FragColor = vec4(.2f); \n }";

            let fragment_c_source = CString::new(fragment_shader_source).unwrap();
            gl::ShaderSource(fragmentshader, 1,  &fragment_c_source.as_ptr(), std::ptr::null());
            gl::CompileShader(fragmentshader);

            let mut VAO : GLuint = 0;
            gl::GenVertexArrays(1, &mut VAO);
            gl::BindVertexArray(VAO);

            let mut VBO : GLuint = 0;
            gl::GenBuffers(1, &mut VBO);
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

            gl::BufferData(gl::ARRAY_BUFFER, (vert.len() * std::mem::size_of::<f32>()).try_into().unwrap(), vert.as_ptr() as *const c_void, gl::STATIC_DRAW);

            let mut Programe = gl::CreateProgram();
            gl::AttachShader(Programe, vertexshader);
            gl::AttachShader(Programe, fragmentshader);
            gl::LinkProgram(Programe);
            gl::DeleteShader(vertexshader);
            gl::DeleteShader(fragmentshader);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 4 *3, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::UseProgram(Programe);

        }


    }


    pub fn drawing_two_trig_diff(&mut self){
        let mut fvert : [f32; 9] =[
            -0.9, -0.5, 0.0,
            -0.0, -0.5, 0.0,
            -0.45, 0.5, 0.0,

        ];

        let mut svert : [f32; 9] =[
            0.0, -0.5, 0.0,  // left
            0.9, -0.5, 0.0,  // right
            0.45, 0.5, 0.0
        ];
        unsafe {
            // coppying vertex from cpu to gpu!!11
            gl::GenVertexArrays(2, self.VAOs.as_mut_ptr());
            gl::GenBuffers(2, self.VBOs.as_mut_ptr());

            //first trig setup

            gl::BindVertexArray(self.VAOs[0]);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.VBOs[0]);
            gl::BufferData(gl::ARRAY_BUFFER, (fvert.len() * std::mem::size_of::<f32>()).try_into().unwrap(), fvert.as_ptr() as *const c_void, gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()).try_into().unwrap(), std::ptr::null());
            gl::EnableVertexAttribArray(0);
            //second trig setup
            gl::BindVertexArray(self.VAOs[1]);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.VBOs[1]);
            gl::BufferData(gl::ARRAY_BUFFER, (svert.len() * std::mem::size_of::<f32>()).try_into().unwrap(), svert.as_ptr() as *const c_void, gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()).try_into().unwrap(), std::ptr::null());
            gl::EnableVertexAttribArray(0);
            // vertex shade and fragment shader!!11!!
            let vertex_shader_source = "#version 330 core\n layout (location = 0) in vec3 aPos; void main() { gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0); }";
            let fragment_shader_source = "#version 330 core\n out vec4 FragColor; \n void main(){ \n FragColor = vec4(1.0f); \n }";
            let vertex_shader_ptr = CString::new(vertex_shader_source).unwrap();
            let fragment_shader_ptr = CString::new(fragment_shader_source).unwrap();

            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &vertex_shader_ptr.as_ptr() , std::ptr::null());
            gl::CompileShader(vertex_shader);


            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &fragment_shader_ptr.as_ptr() , std::ptr::null());
            gl::CompileShader(fragment_shader);

            let program : GLuint = gl::CreateProgram();
            gl::DetachShader(program, vertex_shader);
            gl::DetachShader(program, fragment_shader);



            gl::UseProgram(program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

        }
    }
}