#[macro_use]
extern crate glium;

pub fn init() {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        uv: [f32; 2],
    }

    implement_vertex!(Vertex, position, uv);

    let vertex1 = Vertex { position: [-0.5, -0.5], uv: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5], uv: [1.0, 0.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25], uv: [0.0, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 uv;
        out vec2 uvi;

        uniform float t;

        void main() {
            uvi = uv;
            gl_Position = vec4(position + vec2(t,t), 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        in vec2 uvi;
        void main() {
            color = vec4(uvi, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let mut t: f32 = 0.;

    let mut dt: f32 = 0.;
    let mut prev_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let frame_begin_time = std::time::Instant::now();
        dt = (frame_begin_time - prev_frame_time).as_secs_f32();
        prev_frame_time = frame_begin_time;
        // println!("{:?}", dt);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }


        let mut target = display.draw();
        target.clear_color(70./256., 102./256., 101./256., 1.0);

        target.draw(&vertex_buffer, &indices, &program, &uniform! {t:t},
                    &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = frame_begin_time +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}

pub static t: f32 = 0.;

pub fn update(dt: f32, mut target: glium::Frame) {
    t += dt;

    target.clear_color(70./256., 102./256., 101./256., 1.0);

    target.draw(&vertex_buffer, &indices, &program, &uniform! {t:t},
                &Default::default()).unwrap();
    target.finish().unwrap();
}