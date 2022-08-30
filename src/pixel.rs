
use glium::glutin;
use glam::*;

pub fn pixel_test() {
    let window_size = UVec2 {x: 800, y: 700};
    let event_loop = glutin::event_loop::EventLoopBuilder::with_user_event().build();
    let display = create_display(&event_loop, window_size.into());

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);

    //
    // let empty_texture
    // let empty_texture = vec![255_u8; (window_size.x * window_size.y * 3) as usize];
    // let raw = glium::texture::RawImage2d::from_raw_rgb(empty_texture, window_size.into());
    // dbg!(raw.format.get_size());
    // dbg!(std::mem::size_of::<u32>());
    // let texture = glium::texture::srgb_texture2d::SrgbTexture2d::new(&display, raw).unwrap();
    let texture = glium::texture::srgb_texture2d::SrgbTexture2d::empty(&display, 80, 70).unwrap();

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
    let shape_wf_ibo = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::LineLoop, &[0_u32, 1, 2]).unwrap();

    let vertex1 = Vertex { position: [-1.0, -1.0], uv: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 1.0,  -1.0], uv: [1.0, 0.0] };
    let vertex3 = Vertex { position: [ 1.0, 1.0], uv: [1.0, 1.0] };
    let vertex4 = Vertex { position: [ -1.0, 1.0], uv: [0.0, 1.0] };
    let quad = vec![vertex1, vertex2, vertex3, vertex4];
    let quad_indices = [0_u32, 1, 2, 0, 2, 3];

    let quad_shape_vbo = glium::VertexBuffer::new(&display, &quad).unwrap();
    let quad_ibo = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &quad_indices).unwrap();

    let quad_vsh_src = r#"
        #version 140
        in vec2 position;
        in vec2 uv;
        out vec2 uvi;

        uniform float t;

        void main() {
            uvi = uv;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let quad_fsh_src = r#"
        #version 140

        in vec2 uvi;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            // color = vec4(uvi, 0.0, 1.0);
            color = texture(tex, uvi);
        }
    "#;

    let guad_program = glium::Program::from_source(&display, quad_vsh_src, quad_fsh_src, None).unwrap();

    let triangle_vsh_src = r#"
        #version 140
        in vec2 position;
        in vec2 uv;
        out vec2 uvi;

        uniform float t;

        void main() {
            uvi = uv;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let triangle_fsh_src = r#"
        #version 140

        in vec2 uvi;
        out vec4 color;

        void main() {
            color = vec4(uvi, 0.0, 1.0);
        }
    "#;
    let triangle_program = glium::Program::from_source(&display, triangle_vsh_src, triangle_fsh_src, None).unwrap();

    let wireframe_vsh_src = r#"
        #version 140
        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let wireframe_fsh_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        }
    "#;

    let triangle_wf_program = glium::Program::from_source(&display, wireframe_vsh_src, wireframe_fsh_src, None).unwrap();

    let params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    let mut t: f32 = 0.;

    let mut dt: f32 = 0.;
    let mut prev_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let frame_begin_time = std::time::Instant::now();
        dt = (frame_begin_time - prev_frame_time).as_secs_f32();
        prev_frame_time = frame_begin_time;
        
        let mut redraw = || {
            let mut quit = false;

            let repaint_after = egui_glium.run(&display, |egui_ctx| {
                egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                    ui.heading("Hello World!");
                    ui.label(format!("{}", 1./dt));
                    if ui.button("Quit").clicked() {
                        quit = true;
                    }
                    ui.add(egui::Slider::new(&mut t, 0.0..=1.0));
                });
            });

            *control_flow = if quit {
                glutin::event_loop::ControlFlow::Exit
            } else if repaint_after.is_zero() {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else if let Some(repaint_after_instant) =
                std::time::Instant::now().checked_add(repaint_after)
            {
                glutin::event_loop::ControlFlow::WaitUntil(repaint_after_instant)
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                use glium::Surface as _;

                let mut fb = glium::framebuffer::SimpleFrameBuffer::new(&display, &texture).unwrap();

                fb.clear_color(0., 0., 0., 0.);


                fb.draw(&vertex_buffer, &indices, &triangle_program, &uniform!{t:t},
                            &Default::default()).unwrap();
                fb.draw(&vertex_buffer, &shape_wf_ibo, &triangle_wf_program, &uniform!{t:t},
                            &Default::default()).unwrap();
                // fb.finish().unwrap();






                let mut target = display.draw();


                // let color = egui::Rgba::from_rgb(0.1, 0.3, 0.2);
                // target.clear_color(color[0], color[1], color[2], color[3]);

                // draw things behind egui here
                target.clear_color(70./256., 102./256., 101./256., 1.0);

                // t += 100. * dt;

                use glium::uniforms::*;
                let behavior = glium::uniforms::SamplerBehavior {
                    minify_filter: MinifySamplerFilter::Nearest,
                    magnify_filter: MagnifySamplerFilter::Nearest,
                    ..Default::default()
                };
                let uniforms = uniform! {
                    t: t,
                    tex: glium::uniforms::Sampler(&texture, behavior),
                };

                target.draw(&quad_shape_vbo, &quad_ibo, &guad_program, 
                            &uniforms,
                            &params).unwrap();

                egui_glium.paint(&display, &mut target);

                // draw things on top of egui here

                target.finish().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                egui_glium.on_event(&event);

                display.gl_window().window().request_redraw(); // TODO(emilk): ask egui if the events warrants a repaint instead
            }
            glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached {
                ..
            }) => {
                display.gl_window().window().request_redraw();
            }
            _ => (),
        }
    });
}

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>, window_size: (u32, u32)) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: window_size.0,
            height: window_size.1,
        })
        .with_title("egui_glium example");

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}