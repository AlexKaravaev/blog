use leptos::logging::log;
use three_d::*;
use wasm_bindgen::JsCast;
use std::panic;

#[cfg(target_arch = "wasm32")]
pub async fn scene() {

    // This is cursed.
    // Explanation: when you use multi-page website, if you go to another page, canvas is lost here and nothing is being rendered
    // You could spawn scene() on every page load, but then event_loop is created once per application.
    // So basically once we encounter specific event_loop panic - we reload app with this page
    // TODO probably we could use once_cell somewhere here
    panic::set_hook(Box::new(|info| {
        if let Some(message) = info.payload().downcast_ref::<&str>() {
            if message.contains("Creating EventLoop multiple times is not supported.") {
         
                web_sys::window().unwrap().location().set_href("/about").unwrap();
            }
        }
    }));

    // // Create the event loop
    // let event_loop = winit::event_loop::EventLoop::new();

    let event_loop = winit::event_loop::EventLoop::new();
    log!("Enter");

    let window_builder = {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        winit::window::WindowBuilder::new()
            .with_canvas(Some(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("myCanvas")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap(),
            ))  
            .with_inner_size(winit::dpi::LogicalSize::new(720, 720))
            .with_prevent_default(true)
    };

    let render_window = window_builder.build(&event_loop).unwrap();
    log!("Window initialized");

    let mut context = initialize_context(&render_window);
    log!("Context initialized");

    let mut loaded_assets = load_assets().await;
    log!("Assets loaded");

    let mut camera = initialize_camera();
    let mut control = OrbitControl::new(*camera.target(), 0.01, 100.0);
    let mut penguin = initialize_model(&context, &mut loaded_assets);
    let lights = initialize_lights(&context);

    log!("Entering main loop");
    let mut frame_input_generator = FrameInputGenerator::from_winit_window(&render_window);

    event_loop.run(move |event, _, control_flow| 
        match event {
            winit::event::Event::MainEventsCleared => {
                
                render_window.request_redraw();
            }
            winit::event::Event::RedrawRequested(_) => {
                log!("TEst");
                let mut frame_input = frame_input_generator.generate(&context);
    
                control.handle_events(&mut camera, &mut frame_input.events);
                camera.set_viewport(frame_input.viewport);
    
                penguin.iter_mut().for_each(|m| {
                    m.set_transformation(Mat4::from_angle_y(radians(
                        (frame_input.accumulated_time * 0.0005) as f32,
                    )));
                });
    
                frame_input
                    .screen()
                    .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
                    .render(&camera, &penguin, &[&lights.1, &lights.2, &lights.0]);
    
                context.swap_buffers().unwrap();
                control_flow.set_poll();
                render_window.request_redraw();
            }
            winit::event::Event::WindowEvent { ref event, .. } => {
                frame_input_generator.handle_winit_window_event(event);
                match event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        context.resize(**new_inner_size);
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        log!("Close");
                        control_flow.set_exit();
                    }
                    _ => {log!("{:?}", event);},
                }
            }
            _ => {}
            // log!("{:?}", event);
        }
    );
}


#[cfg(target_arch = "wasm32")]
fn initialize_context(render_window: &winit::window::Window) -> WindowedContext {
    WindowedContext::from_winit_window(
        render_window,
        three_d::SurfaceSettings {
            vsync: false,
            ..three_d::SurfaceSettings::default()
        },
    )
    .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn initialize_camera() -> Camera {
    Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(0.125, 0.35, -0.5),
        vec3(0.0, 0.35, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.01,
        10.0,
    )
}

#[cfg(target_arch = "wasm32")]
async fn load_assets() -> three_d_asset::io::RawAssets {
    three_d_asset::io::load_async(&[
        "assets/face2.glb",
        "assets/wireframe_me.png",
        "assets/color.png",
    ])
    .await
    .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn initialize_model(
    context: &WindowedContext,
    loaded: &mut three_d_asset::io::RawAssets,
) -> Model<PhysicalMaterial> {
    let mut cpu_texture: CpuTexture = loaded.deserialize("wireframe_me.png").unwrap();
    cpu_texture.data.to_linear_srgb();

    let mut face_cpu_texture: CpuTexture = loaded.deserialize("color.png").unwrap();
    face_cpu_texture.data.to_linear_srgb();

    let model = loaded.deserialize("face2.glb").unwrap();
    let mut penguin = Model::<PhysicalMaterial>::new(context, &model).unwrap();

    penguin.iter_mut().for_each(|m| {
        m.set_transformation(Mat4::from_translation(vec3(0.0, 0.0, 0.0)));
        m.material.albedo = Srgba::WHITE;
        m.material.emissive = Srgba::BLACK;
        let texture = Texture2D::new(context, &cpu_texture);
        m.material.albedo_texture = Some(Texture2DRef::from_texture(texture));
    });

    penguin
}

#[cfg(target_arch = "wasm32")]
fn initialize_lights(context: &WindowedContext) -> (AmbientLight, DirectionalLight, DirectionalLight) {
    let ambient = AmbientLight::new(context, 1000.0, Srgba::WHITE);
    let directional = DirectionalLight::new(context, 50.0, Srgba::WHITE, &vec3(0.0, -1.0, -1.0));
    let directional_back = DirectionalLight::new(context, 50.0, Srgba::WHITE, &vec3(0.0, 1.0, 1.0));

    (ambient, directional, directional_back)
}