# Basic System

Create a `Video` object, using `VideoConfig` to specify the details, like so:

```
let config = VideoConfig {
    window: WindowConfig { width: 1280,height: 720, },
    framebuffer: FramebufferConfig { width: 256,height: 144, },
};

let mut video = Video::new(config).expect("Cannot open video.");

video.set_window_title("My Game");
```

The window size is the initial size of the window on the screen. The framebuffer size is the size of the internal framebuffer that is used as fixed game screen.

Now, in the main loop, call either `wait_for_event` or `poll_for_event` methods to receive the next event. `wait_for_event` blocks until something happens, and then returns `Some(event)` with the event, or `None` if there was an error. `poll_for_event` does not block, but when something happens, it returns `Some(event)` with the event, or `None` otherwise.

The event is of type `Event`, which is an enum:

```
pub enum Button {
    Left,
    Middle,
    Right,
}

pub enum Wheel {
    Up,
    Down,
    Left,
    Right,
}

pub enum Event {
    KeyPress(u8),
    KeyRelease(u8),
    MousePress(i32,i32,Button),
    MouseRelease(i32,i32,Button),
    MouseWheel(Wheel),
    MouseMove(i32,i32),
    Paint(i32,i32,u32,u32),
    Resize(u32,u32),
    Close,
}
```

For now, `Paint()` is handled automatically, and currently renders a list of layers. Each layer is a rectangular area on the screen, that blends together with the underlying layers. The idea is that the map is one layer, a HUD, popup messages, speech text, etc.

Layers are rendered with the layer shader, which can be configured to do a variety of postprocessing effects on that layer. Finally, the entire framebuffer is rendered to screen with the final shader, which also has the ability to do a variety of postprocessing effects.

## Experiment: Print Each Event

`test-window` opens a `Video` object, and displays each event as it occurs.

## Experiment: Display Pixel Art

`test-image` opens a `Video` object, loads an image into a texture, and displays it as layer.

# Rendering the Map

A map has multiple planes. The lower planes are generally the background data, the middle planes have some structural updates and additions, and the top planes contain the small clutter. Each plane draws from a different tile texture. Each tile texture contains 16x16 tiles. The map indices are uvec?, and each bit references something specific.

## Experiment: Simple Map

`test-map1` opens a `Video` object, loads a tileset, builds a small map and displays it as layer.
