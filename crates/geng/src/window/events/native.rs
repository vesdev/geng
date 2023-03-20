use super::*;

impl Window {
    pub(crate) fn internal_get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        {
            let mut mouse_move = None;
            let mut handle_event = |e: winit::event::WindowEvent| match e {
                winit::event::WindowEvent::Focused(focus) => self.focused.set(focus),
                winit::event::WindowEvent::CloseRequested => self.should_close.set(true),
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    events.push(Event::Wheel {
                        delta: match delta {
                            winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y,
                            winit::event::MouseScrollDelta::LineDelta(_, dy) => dy as f64 * 51.0,
                        },
                    });
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    let position = vec2(position.x, self.size().y as f64 - 1.0 - position.y);
                    mouse_move = Some(position);
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        winit::event::MouseButton::Left => Some(MouseButton::Left),
                        winit::event::MouseButton::Middle => Some(MouseButton::Middle),
                        winit::event::MouseButton::Right => Some(MouseButton::Right),
                        _ => None,
                    };
                    if let Some(button) = button {
                        let position = self.mouse_pos.get();
                        events.push(match state {
                            winit::event::ElementState::Pressed => {
                                Event::MouseDown { position, button }
                            }
                            winit::event::ElementState::Released => {
                                Event::MouseUp { position, button }
                            }
                        });
                    }
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        let key = from_glutin_key(key);
                        events.push(match input.state {
                            winit::event::ElementState::Pressed => Event::KeyDown { key },
                            winit::event::ElementState::Released => Event::KeyUp { key },
                        });
                    }
                }
                winit::event::WindowEvent::Resized(new_size) => {
                    glutin_winit::GlWindow::resize_surface(
                        &self.window,
                        &self.gl_surface,
                        &self.gl_ctx,
                    );
                }
                _ => {}
            };
            use winit::platform::run_return::EventLoopExtRunReturn;
            self.event_loop.borrow_mut().run_return(|e, _, flow| {
                if let winit::event::Event::WindowEvent { event: e, .. } = e {
                    handle_event(e)
                }
                *flow = winit::event_loop::ControlFlow::Exit;
            });
            if let Some(position) = mouse_move {
                // This is here because of weird delta
                events.push(Event::MouseMove {
                    position,
                    delta: position - self.mouse_pos.get(),
                });
                self.mouse_pos.set(position);
            }
        }
        events
    }
}

fn from_glutin_key(key: winit::event::VirtualKeyCode) -> Key {
    use winit::event::VirtualKeyCode as GKey;
    match key {
        GKey::Key0 => Key::Num0,
        GKey::Key1 => Key::Num1,
        GKey::Key2 => Key::Num2,
        GKey::Key3 => Key::Num3,
        GKey::Key4 => Key::Num4,
        GKey::Key5 => Key::Num5,
        GKey::Key6 => Key::Num6,
        GKey::Key7 => Key::Num7,
        GKey::Key8 => Key::Num8,
        GKey::Key9 => Key::Num9,

        GKey::A => Key::A,
        GKey::B => Key::B,
        GKey::C => Key::C,
        GKey::D => Key::D,
        GKey::E => Key::E,
        GKey::F => Key::F,
        GKey::G => Key::G,
        GKey::H => Key::H,
        GKey::I => Key::I,
        GKey::J => Key::J,
        GKey::K => Key::K,
        GKey::L => Key::L,
        GKey::M => Key::M,
        GKey::N => Key::N,
        GKey::O => Key::O,
        GKey::P => Key::P,
        GKey::Q => Key::Q,
        GKey::R => Key::R,
        GKey::S => Key::S,
        GKey::T => Key::T,
        GKey::U => Key::U,
        GKey::V => Key::V,
        GKey::W => Key::W,
        GKey::X => Key::X,
        GKey::Y => Key::Y,
        GKey::Z => Key::Z,

        GKey::Escape => Key::Escape,
        GKey::Space => Key::Space,
        GKey::Return => Key::Enter,
        GKey::Back => Key::Backspace,
        GKey::Tab => Key::Tab,

        GKey::LShift => Key::LShift,
        GKey::RShift => Key::RShift,

        GKey::LControl => Key::LCtrl,
        GKey::RControl => Key::RCtrl,

        GKey::LAlt => Key::LAlt,
        GKey::RAlt => Key::RAlt,

        GKey::Left => Key::Left,
        GKey::Right => Key::Right,
        GKey::Up => Key::Up,
        GKey::Down => Key::Down,

        GKey::PageUp => Key::PageUp,
        GKey::PageDown => Key::PageDown,
        GKey::Insert => Key::Insert,
        GKey::Delete => Key::Delete,
        GKey::Home => Key::Home,
        GKey::End => Key::End,

        GKey::F1 => Key::F1,
        GKey::F2 => Key::F2,
        GKey::F3 => Key::F3,
        GKey::F4 => Key::F4,
        GKey::F5 => Key::F5,
        GKey::F6 => Key::F6,
        GKey::F7 => Key::F7,
        GKey::F8 => Key::F8,
        GKey::F9 => Key::F9,
        GKey::F10 => Key::F10,
        GKey::F11 => Key::F11,
        GKey::F12 => Key::F12,

        _ => {
            warn!("Unrecognized key: {:?}", key);
            Key::Unknown
        }
    }
}
