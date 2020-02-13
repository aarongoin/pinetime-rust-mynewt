//! This program was automatically generated by Visual Embedded Rust. Don't edit here!
extern crate macros as mynewt_macros;   //  Declare the Mynewt Procedural Macros library
use mynewt_macros::infer_type;          //  Import Mynewt procedural macros
use mynewt::{
  result::*,              // Import Mynewt API Result and Error types
  sys::console,           // Import Mynewt Console API
};
use druid::{
  AppLauncher, Data, EventCtx, LocalizedString, Widget, WindowDesc,
  widget::{
      Align, Button, Column, Label, Padding,
  },
  argvalue::ArgValue,
  env::Env,
};

/// Application State
#[infer_type]  //  Infer the missing types
#[derive(Clone, Data, Default)]
struct State {
    count: _,
}

/// Will be run upon startup to launch the app
#[infer_type]  //  Infer the missing types
pub fn on_start() -> MynewtResult<()> {
    console::print("on_start\n");
    //  Build a new window
    let main_window = WindowDesc::new(ui_builder);
    //  Create application state
    let mut state = State::default();
    state.count = 0;

    //  Launch the window with the application state
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(state)
        .expect("launch failed");
    //  Return success to `main()` function
    Ok(())
}

/// Build the UI for the window
#[infer_type]  //  Infer the missing types
fn ui_builder() -> impl Widget {  //  `State` is the Application State
    console::print("Rust UI builder\n"); console::flush();
    //  Create a line of text
    let my_label_text = LocalizedString::new("hello-counter")
        .with_arg("count", on_my_label_show);  //  Call `on_my_label_show` to get label text
    //  Create a label widget `my_label`
    let my_label = Label::new(my_label_text);
    //  Create a button widget `my_button`
    let my_button = Button::new("increment", on_my_button_press);  //  Call `on_my_button_press` when pressed

    //  Create a column
    let mut col = Column::new();
    //  Add the label widget to the column, centered with padding
    col.add_child(
        Align::centered(
            Padding::new(5.0,
                my_label
            )
        ),
        1.0
    );
    //  Add the button widget to the column, with padding
    col.add_child(
        Padding::new(5.0,
            my_button
        ),
        1.0
    );
    //  Return the column containing the widgets
    col
}  //  ;

/// Callback function that will be called to create the formatted text for the label `my_label`
#[infer_type]  //  Infer the missing types
fn on_my_label_show(state: _, env: _) -> ArgValue {
    console::print("on_my_label_show\n");
    state.count.into()
}

/// Callback function that will be called when the button `my_button` is pressed
#[infer_type]  //  Infer the missing types
fn on_my_button_press(ctx: _, state: _, env: _) {
    console::print("on_my_button_press\n");
    state.count = state.count + 1;
}

////////////////////////////// TODO: Generate via Data trait

/// Static list of `Widgets` for embedded platforms
/// TODO: Generate via Data trait
static mut WIDGET_STATE_STATE: [ druid::WidgetType<State>; druid::MAX_WIDGETS ] = [ 
    druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
    druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
];

/// Specialised Trait will store `Widgets` statically on embedded platforms
/// TODO: Generate via Data trait
impl druid::GlobalWidgets<State> for druid::WidgetBox<State> {
    /// Fetch the static `Widgets` for the Data type
    fn get_widgets(&self) -> &'static mut [ druid::WidgetType<State> ] {
        unsafe { &mut WIDGET_STATE_STATE }
    }
    /// Add a `Widget` for the Data type
    fn add_widget(&self, widget: druid::WidgetType<State>) {
        assert!(self.0 < druid::MAX_WIDGETS as u32, "too many widgets");
        unsafe { WIDGET_STATE_STATE[self.0 as usize] = widget; }        
    }    
}

/// ALL_WINDOWS[i] is the WindowBox for the Window with window ID i. i=0 is not used.
/// TODO: Generate via Data trait
static mut ALL_WINDOWS_STATE: [ druid::WindowBox<State>; druid::MAX_WINDOWS ] = [
    druid::WindowBox::<State>( druid::WindowType::None ), 
    druid::WindowBox::<State>( druid::WindowType::None ), 
    druid::WindowBox::<State>( druid::WindowType::None ), 
];
/// ALL_HANDLERS[i] is the Window Handler for the Window with window ID i. i=0 is not used.
/// TODO: Generate via Data trait
static mut ALL_HANDLERS_STATE: [ druid::DruidHandler<State>; druid::MAX_WINDOWS ] = [
    druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
    druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
    druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
];
/// DATA is the Application Data
/// TODO: Generate via Data trait
static mut DATA_STATE: State = Default::default();

/// TODO: Generate via Data trait
pub fn handle_touch(x: u16, y: u16) {
    let mut ctx = druid::DruidContext::new();
    let handler = unsafe { &mut ALL_HANDLERS_STATE[1] };  //  Assume first window has ID 1
    handler.mouse_down(
        &druid::MouseEvent {
            pos: druid::Point::new(x as f64, y as f64),
            count: 1,
            button: druid::MouseButton::Left,
        },
        &mut ctx,
    );
    handler.mouse_up(
        &druid::MouseEvent {
            pos: druid::Point::new(x as f64, y as f64),
            count: 0,
            button: druid::MouseButton::Left,
        },
        &mut ctx,
    );
}

/// Specialised Trait will store Windows and Window Handlers statically on embedded platforms
/// TODO: Generate via Data trait
impl druid::GlobalWindows<State> for druid::AppState<State> {
    fn add_window(&self, window_id: druid::WindowId, window: druid::WindowBox<State>) {
        unsafe { ALL_WINDOWS_STATE[window_id.0 as usize] = window; }
    }
    fn add_handler(&self, window_id: druid::WindowId, handler: druid::DruidHandler<State>) {
        unsafe { ALL_HANDLERS_STATE[window_id.0 as usize] = handler; }
    }
    fn get_handle(&self, window_id: druid::WindowId) -> druid::WindowHandle<druid::DruidHandler<State>> {
        let handler = unsafe { ALL_HANDLERS_STATE[window_id.0 as usize].clone() };
        druid::WindowHandle(
            crate::shell::platform::window::WindowHandle {
                window_id: window_id.0,
                state: crate::shell::platform::window::WindowState {
                    window_id: window_id.0,
                    handler,                
                }            
            }
        )
    }
    fn set_data(&self, data: State) {
        unsafe { DATA_STATE = data; }
    }
    fn window_event(
        &mut self, 
        window_id: druid::WindowId,
        ctx: &mut druid::EventCtx<State>, 
        event: &druid::Event, 
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].event(
                ctx, 
                event, 
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            );
        }
    }
    fn window_update(
        &mut self, 
        window_id: druid::WindowId,
        ctx: &mut druid::UpdateCtx<State>, 
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].update(
                ctx,
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            ); 
        }
    }
    fn window_layout(
        &mut self,
        window_id: druid::WindowId,
        layout_ctx: &mut druid::LayoutCtx,
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].layout(
                layout_ctx, 
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            ); 
        }
    }
    fn window_paint(
        &mut self, 
        window_id: druid::WindowId,
        paint_ctx: &mut druid::PaintCtx, 
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].paint(
                paint_ctx, 
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            ); 
        }
    }
    fn window_has_active(
        &mut self,
        window_id: druid::WindowId,
    ) -> bool {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].has_active() 
        }
    }
}