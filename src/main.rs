use engage::data;
use engage::glfw;
use engage::imgui;
use engage::imgui_wgpu;
use engage::renderer;
use engage::wgpu;

use std::sync::Arc;

fn main() {
    let mut cvar = data::Cvar::new();
    cvar.register("screen_width",data::CVarValue::Int(1600), None);
    cvar.register("screen_height",data::CVarValue::Int(900), None);
    cvar.register("screen_title",data::CVarValue::Str("Hello world !".to_string()), None);

    let width = cvar.get("screen_width").unwrap_int() as u32;
    let height = cvar.get("screen_height").unwrap_int() as u32;
    let title = cvar.get("screen_title").unwrap_str();

    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to init glfw !");
    
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    let (mut window, event_reciever) = glfw.create_window(width,
        height,
        title.as_str(),
        glfw::WindowMode::Windowed
    ).expect("Failed to create window !");
    window.set_all_polling(true);
    let window = Arc::new(window);
     let mut renderer = renderer::Renderer::new(window.clone());
        let mut main_camera = data::camera::Camera::new();

        
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        {
            let mut imgui_io = imgui.io_mut();
            //imgui_io.config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
        }

        let mut imgui_renderer = imgui_wgpu::Renderer::new(
            &mut imgui,
            &renderer.get_device(),
            &renderer.get_queue(),
            imgui_wgpu::RendererConfig {
                texture_format: renderer.get_surface_format(),
                ..Default::default()
            },
        );
        
        
        
        let mut last_cursor: [f32; 2] = [0.0, 0.0];
        while !window.should_close() {
            
            glfw.poll_events();

            //Update 
            {
                let imgui_io = imgui.io_mut();  
                imgui_io.config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
                for (_, event) in glfw::flush_messages(&event_reciever) {
                    match event {
                        glfw::WindowEvent::FramebufferSize(width, height) => {
                            cvar.set("screen_width", data::CVarValue::Int(width as i64));
                            cvar.set("screen_height", data::CVarValue::Int(height as i64));
                            renderer.resize((width as u32, height as u32));
                        }
                        glfw::WindowEvent::CursorPos(x, y) => {
                            let pos = [x as f32, y as f32];
                            imgui_io.mouse_pos = pos;
                            last_cursor = pos;
                        }
                        glfw::WindowEvent::MouseButton(button, action, _) => {
                            let pressed = action == glfw::Action::Press;
                            match button {
                                glfw::MouseButton::Left => imgui_io.add_mouse_button_event(imgui::MouseButton::Left, pressed),
                                glfw::MouseButton::Right => imgui_io.add_mouse_button_event(imgui::MouseButton::Right, pressed),
                                glfw::MouseButton::Middle => imgui_io.add_mouse_button_event(imgui::MouseButton::Middle, pressed),
                                _ => {}
                            }
                        }
                        glfw::WindowEvent::Scroll(x, y) => {
                            imgui_io.add_mouse_wheel_event([x as f32, y as f32]);
                        }
                        glfw::WindowEvent::Char(c) => {
                            imgui_io.add_input_character(c);
                        }
                        glfw::WindowEvent::Key(key, _, action, mods) => {
                            let pressed = action != glfw::Action::Release;
                            let io_key = match key {
                                glfw::Key::Tab => imgui::Key::Tab,
                                _ => imgui::Key::ScrollLock
                            };
                            imgui_io.add_key_event(io_key, pressed);
                            imgui_io.key_shift = mods.contains(glfw::Modifiers::Shift);
                            imgui_io.key_ctrl = mods.contains(glfw::Modifiers::Control);
                            imgui_io.key_alt = mods.contains(glfw::Modifiers::Alt);
                            imgui_io.key_super = mods.contains(glfw::Modifiers::Super);
                        }
                        _ => {},
                    }
                }
                let width = cvar.get("screen_width").unwrap_int() as f32;
                let height = cvar.get("screen_height").unwrap_int() as f32;
                imgui_io.display_framebuffer_scale = [1.0, 1.0];
                imgui_io.display_size = [width, height];
            }

            //Gui render
            {
                
                let ui = imgui.new_frame();
                ui.dockspace_over_main_viewport();
                ui.window("Inspector")
                .size([320.0, 240.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    ui.text("Dockable window 1");
                    ui.arrow_button("WALA", imgui::Direction::Down);
                });

                let imgui_draw_data = imgui.render();

                renderer.set_camera(&main_camera);
                match renderer.render(&cvar, imgui_draw_data, &mut imgui_renderer) {
                    Ok(()) => (),
                    Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                        let size = window.get_framebuffer_size();
                        renderer.resize((size.0 as u32, size.1 as u32));
                    }
                    Err(x) => {
                        eprintln!("Render error {:?}", x);
                    }
                }
            }

        }
        renderer.wait_idle();

}
