
// macro accepts an object with a single keyboard object
// the object can have zero or more comma separated `key:value` pairs
macro_rules! struct_events {
    (
        keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },
        else: { $( $e_alias:ident : $e_sdl:pat ),* }
    ) => {
        use self::sdl2::EventPump;

        pub struct ImmediateEvents {
            // For every keyboard event we have an Option< bool >
            // Some(true) presed
            // Some(false) released
            // None - nothin is happening
            $(pub $k_alias: Option<bool> , )*
            $(pub $e_alias: bool),*
        }

        impl ImmediateEvents {
            pub fn new() -> Self {
                ImmediateEvents{
                    // nothing has happened during initialization
                    // set it to None
                    $( $k_alias: None,)*
                    $( $e_alias: false),*
                }
            }
        }

        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,
        
            // true => pressed
            // false => not pressed
            $(pub $k_alias: bool),*
        }

        impl Events {
            pub fn new( pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    // by default initialize every key as not pressed
                    now: ImmediateEvents::new(),
                    $($k_alias: false),*
                }
            }

            pub fn pump( &mut self, renderer: &mut self::sdl2::render::Renderer ) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::event::WindowEventId::Resized;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        Window { win_event_id: Resized, .. } =>{
                            self.now.resize = Some(renderer.output_size().unwrap());
                        },
                        
                        KeyDown { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl)=>{
                                    if !self.$k_alias {
                                        self.now.$k_alias = Some(true)
                                    }

                                    self.$k_alias = true;
                                }    
                            ),*
                            _ => {} 
                        },
                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false
                                }    
                            ),*
                            _ => {}
                        },
                        $(
                            $e_sdl => {
                                self.now.$e_alias = true
                            }  
                        )*,
                        _ => {}
                    }
                }
            }
        }
    }
}
