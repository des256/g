## 20.10.2019

Set up the repo, regurgitated and collated from all the other projects. Basic OpenGL code for linux. Basic ALSA code for linux. Threads still difficult in Rust. Verified documentation. Reorganized some code. Renamed canvas to console to also include audio. Built viewports with rudimentary compositing and postprocessing.

## 21.10.2019

Researched some other efforts. Found Pixpil who have also discovered deferred shading for pixel art. But they don't seem to do it tile-based. Some other interesting finds include smooth zoom (i.e. revise forcing everything to 512x288), and very bright, almost full-screen lighting effects. Decided to put up an idea register in this document. Fixed layer rendering, scaling from mouse drags on XCB window, started text loading. Started text rendering shader and organize the code around it.

## 22.10.2019

Added utils directory with mkfnt to generate minimal MSDF-based font files. Organized font handling mostly.

## 23.10.2019

Font rendering works, with proper MSDF rendering. Layer is somewhat chosen as the place where graphics operations are done (i.e. text rendering).

## 25.10.2019

Audio playing works. The problem was setting the ALSA access mode to RWInterleaved. Still needs adjustment to access synths when the playback thread is running.

## 26.10.2019

Some refactoring here and there. Designed audio compositing architecture. Several attempts at getting this to work properly in rust. Realized that the playback architecture might be different from the editing architecture.

## 27.10.2019

So it's time for a bit of reflection. This project seems to turn out to be somewhat of a Not Invented Here Syndrome showcase, and perhaps that is a very interesting angle. How much time does it really take if you go that route? What do you learn? What looks like the hard part? What is the real hard part? In general I believe that reducing the engine and tools to something more simple should increase the possibilities for creativity, so that's good. Clearly the absolute hard part here is going to be art and art direction. Meanwhile, audio. In order to access the system from the editing tools as well, I thought about a completely separate engine, but settled for an integrated system with two modes.

## 2.11.2019

Some challenge designing the audio pipeline with separate thread. Currently doing everything with mspc channels, which could be a good idea. Decided to have the entire thing play one music track (with channels of polyphonic synths), multiple sound effects (possibly filtered), one editable music track and one editable real-time synth. Set up Dogma synthesizer with square wave and ADSR. Maybe need to separate tick processing. Audio solved reasonably in the end. Also added python interface towards building the tools. pyo3 doesn't work, so this required cpython, and a separate cargo project to separate python specifics from the core. With the testing, ALSA seems to be a bit unstable.

## 3.11.2019

Set up most of the studio projects as PyQt tools in attempt to re-use as much sub-tools as possibly.

## 8.11.2019

After the third version of the interface for the music tool, I settled on something similar to Ableton or Cubase. There is a zoomable track view with channels, everything scrollable, and you can put segments onto these channels. Segments can be doubleclicked, and then the track is replaced by that segment, which is a piano roll MIDI-like widget. Under this view are the faders, so you always have the mixer controls available, and under are the synthesizer and filters. Also, for the game, I'm looking at existing AAA games and how they ease a user into the game. There are many ideas of which I wonder how they would translate into a 2.5D top-down tile style. One interesting one is hand camera movements for cut scenes, and another one I came across is birds flying away because something got them spooked.

## 9.11.2019

Added draggable note corners on the clip widget.

## 13.11.2019

I gotta redo the README to something more coherent. Meanwhile, it's time for audio playback and designing the data formats on the rust side of things. How is a track going to be structured. Resurrected the bunches and updated general audio interface with clear functions and made the notes similar. So the idea is to use clear and note messages to edit the rust version of a track, and then play that. This should also include mute/solo functionality, as well as the ability to read back the playback position for screen updates. And, everything should be made python accessible via pyg. And don't forget the filters.

## 18.11.2019

Made pyg a properly installable python module, and made the piano work in the music tool.

## 25.11.2019

Turned Dogma into a Roland Juno 106 simulator. Still needs testing and debugging.

## 26.11.2019

Dogma106 works nicely.

# 2.12.2019

Some work on the PNG loader. Putting everything in a huge prefix table is generally not a good idea, better split into tree of small tables.

# 3.12.2019

PNG loader profiling on huge.png:

```
naive (one 65536-entry table):
     363  58.5%  58.5%      421  67.9% e::video::image_formats::png::create_huffman_table
      76  12.3%  70.8%       76  12.3% e::video::image_formats::png::ZipReader::read_symbol
      56   9.0%  79.8%       62  10.0% e::video::image_formats::png::unfilter
      55   8.9%  88.7%       55   8.9% core::cmp::impls::::lt (inline)
      25   4.0%  92.7%      543  87.6% e::video::image_formats::png::inflate
      21   3.4%  96.1%       21   3.4% __memmove_avx_unaligned_erms
       8   1.3%  97.4%       12   1.9% e::video::image_formats::png::decode
       7   1.1%  98.5%        7   1.1% ::index (inline)
       3   0.5%  99.0%       58   9.4% core::iter::range::::next (inline)
       2   0.3%  99.4%        2   0.3% ::index_mut (inline)

after improving the huffman table generation (few 256-entry tables):
      57  36.5%  36.5%       63  40.4% e::video::image_formats::png::unfilter
      45  28.8%  65.4%       52  33.3% e::video::image_formats::png::ZipReader::read_symbol
      14   9.0%  74.4%       80  51.3% e::video::image_formats::png::inflate
      11   7.1%  81.4%       12   7.7% e::video::image_formats::png::decode
       7   4.5%  85.9%        7   4.5% ::deref (inline)
       7   4.5%  90.4%       14   9.0% ::index (inline)
       4   2.6%  92.9%        4   2.6% e::video::image_formats::png::ZipReader::read_bits (inline)
       2   1.3%  94.2%        2   1.3% ::index_mut (inline)
       2   1.3%  95.5%        2   1.3% core::cmp::impls::::lt (inline)
       2   1.3%  96.8%        4   2.6% core::iter::range::::next (inline)
```

So it's quite slow to use one big huffman table.

# 4.12.2019

png suite doesn't crash, but still some small issues to fix.

# 12.12.2019

So, it appears having a UI toolkit at the ready is really interesting. Too bad existing toolkits don't allow for enough fidelity, and Riks things is not working yet, nor will he support the tools we need. This means the entire project is turning into what E used to be, which is fine, but needs perhaps an introduction of a crate tree inside E.

/ -+- math: provide basic linear algebra and such
   +- canvas: provide basic video+input functionality (merge OpenGL, GLES, DirectX, Metal, WebGL, Vulkan, etc.)
   +- sound: provide basic audio functionality (merge ALSA, DirectSound, etc.)
   +- interact: provide UI framework
   +- discuss: serialization
   +- conclude: deep learning

Extracted the various subcrates using rust workspace. Now extract the game specific stuff from canvas and audio, into a game engine specific system crate.

# 15.12.2019

Reshuffled the subcrates again into one single one, and decided to do the UI framework. Rust is extremely difficult for this. The direct translation from old E and MFC/Qt days doesn't work. Parent referencing is a nightmare. So, perhaps it's possible to separate the concerns somewhat, and use closures instead of methods. The entire interface for a window would be described with one function call, containing everything. Existing widgets could be represented as traits onto a basic widget struct that does the geometry. The widgets are owned by the UI, and only very light communication is done to the user... somehow...

# 16.12.2019

Browsed through all sorts of discussions from rust-minded people that figure out what a UI is for. There are several philosophies. Rik is doing sort of hybrid immediate mode, others try to recreate MFC/Qt, yet others only do immediate mode, and there are also many people that try to leverage what web dev. has taught them. Rik believes that fully incremental is too slow and bloated (which was necessary in the old days and in web dev). Yet full immediate mode is stupid. I don't have enough understanding of either.

# 17.12.2019

Before I had a small experiment where I tried to push as much onto the GPU as possible, and this might be an interesting avenue. The core fragment shader is a shader that draws pixels in a few modes: direct, texture, direct * texture, direct + texture, direct - texture, texture - direct, direct * MSDF. The core vertex shader is a shader that manages stacks of nested zoomable/scrollable spaces defined in the UBO. WebGL compatible drawing options: glDrawArrays, glDrawElements; WebGL2 compatible drawing options: glDrawArraysInstanced, glDrawElementsInstanced, glDrawRangeElements. instanced arrays are also available over extension to WebGL.

Possible routes to consider:

- huge collection of 1D vertices in VBO (0) (1) (2) and then clip triangle in pixel shader? needs glDrawArrays with GL_TRIANGLES, potentially slow in clipping
- huge collection of 1D vertices in VBO (0) (1) (2) (0) (2) (3), needs glDrawArrays with GL_TRIANGLES
- huge collection of 1D vertices in VBO (0) (1) (2) (3) and indices 0 1 2 0 2 3 in IBO, needs glDrawElements with GL_TRIANGLES
- huge collection of 1D vertices in VBO (0) (1) (2) (3) and primitive reset, needs glDrawArrays with GL_TRIANGLE_FAN and primitive reset
- huge collection of 4D+4D+4D vertices in VBO and use geometry shader to expand into quads, needs glDrawArays, GL_VERTICES and geometry shaders

UI should update the 12D array, so all of the above can be used. We start with 6N array, because that seems to be available everywhere.

The vertex shader is like the geometry shader, so it will repeat transformation and clipping code, this might be problematic... question, is it still faster than transforming by CPU? most likely yes.

All textures would be stored in one 2D texture array. This is only intended for icons, fonts, etc. Fonts are MSDF. The frame tree is composed of rectangles (with position and size) that are able to scale and offset whatever goes inside (for infinite scrolling and zooming).

# 18.12.2019

Found interesting explanation of what a UI actually is in this talk (https://www.youtube.com/watch?v=xH2x99FTY4k). Raph Levien explains that a UI is a pipeline of tree transformations, and that seems to be a very interesting way to look at it. The Widget trait then looks like this:

```
trait Widget {
       fn event();
       fn update();
       fn layout();
       fn paint();
}
```

Where:
- `event()` updates the application state as a result of interaction with the widgets
- `update()` updates widgets from changes in the application state
- `layout()` updates the layout of the widgets by recalculating positions and sizes where needed
- `paint()` draws the widgets, resulting in graphical updates

Note that all of these could potentially be run in parallel, in multiple threads of the UI, reducing UI overhead even further.

# 20.12.2019

So it's:

1. Wait for incoming mouse and key events (and others)
2. Traverse widget tree to figure out where the mouse and keyboard events need to go, call widget.event() accordingly to adjust application state
3. Application business logic and incoming widget callbacks, call widget.update() accordingly
4. Traverse widget tree to recalculate positions and sizes where needed, call widget.layout() for traversal
5. Traverse widget tree to update frame tree and upload UBO and VBO deltas
6. Render frame tree

# 22.12.2019

So, let's assume that exposing the current state of the interface is just blasting the quad VBO for everything in one block to the GPU. Quads can be allocated and freed. Same for frames.

# 25.12.2019

It's not practical to put everything into one VBO, because of the overhead to properly fill up a background behind characters or icons. This leaves us with two abstractions:

1. the OS-provided "traditional" windows (app window, popups, tooltips, menus, etc.), which we'll just also call Windows

2. a group of elements that can be rendered in one go, for instance, a background with character quads, a row of icons, a menu, a piece of dialog with buttons, etc. We'll call this Panels.

Every window owns a tree of panels. Each panel manages one frame and multiple quads. One could say, the frame is the panel. The quads should be drawn in order.

Now, let's look at classical widgets (for instance, from Qt), and how they can be represented with this technology:

| widget       | purpose                                        | implementation | notes |
|--------------|------------------------------------------------|----------------|-------|
| checkbox     | check box interface                            | Q              |       |
| combobox     | editline with dropdown                         | QWF(Q)         |       |
| editdate     | tool to edit date                              | Q              |       |
| edittime     | tool to edit time                              | Q              |       |
| editdatetime | tool to edit date and time                     | Q              |       |
| dropdown     | popup with listview                            | WF(Q)          |       |
| tooltip      | popup with label                               | WF(Q)          |       |
| knob         | rotary controller                              | Q              |       |
| label        | static text                                    | Q              |       |
| lcdnumber    | number output                                  | Q              |       |
| editline     | tool to edit scrolling line of text            | F(Q)           |       |
| menubox      | popup with menu items                          | WF(Q)          |       |
| menubar      | bar with items that open menuboxes             | Q              |       |
| contextmenu  | generic popup with menu items                  | WF(Q)          |       |
| progressbar  | bar that represents task completion            | Q              |       |
| pushbutton   | clickable interface                            | Q              |       |
| radiobutton  | single alternative for a buttongroup           | Q              |       |
| scrollarea   | frame holding a potentially larger region      | F(Q)           |       |
| scrollbar    | sliding controller, usually for scrolling      | Q              |       |
| slider       | sliding controller, for other things           | Q              |       |
| spinbox      | up and down buttons next to a number           | Q              |       |
| tabbar       | bar with tabs to index stack pages             | Q              |       |
| toolbar      | bar with buttons and small controls            | Q              |       |
| listview     | scrollarea with text items                     | F(Q)           |       |
| tableview    | scrollarea with multiple column data interface | F(Q)           |       |
| calendar     | tool to edit calendar                          | Q              |       |
| treeview     | scrollarea with collapsible tree items         | F(Q)           |       |
| buttongroup  | group of radiobuttons                          | -              |       |
| groupbox     | group of controls with label                   | Q              |       |
| splitter     | draggable bar that governs two subregions      | F()QF()        |       |
| stack        | one widget visible at a time                   | Q              |       |
| command      | command pattern via multiple controls          | -              |       |
| desktop      | area that can hold multiple windows            | Q              |       |
| statusbar    | line of text at the bottom of a window         | Q              |       |
| filebox      | box for choosing file for save/load            | QF(Q)          |       |
| harmonica    | bunch of widgets that can be opened/closed     | F(Q)QF(Q)      |       |

Here, Q means bunch of quads, W means OS Window, F() means Frame.

For layout, there are several alternatives in Qt that can prettymuch all be reduced to the grid layout, where widgets can occupy one or more cells.

# 27.12.2019

So, taking the pipeline steps from Druid guy, the only thing I know how to do right now is to just start implementing this list of widgets and support with tests. First start with per-widget drawing calls, and we can always revisit this later.

# 29.12.2019

So who owns the widget? It seems the Widget trait has some potency, and perhaps sharing ownership via a `Rc<>` or a `Arc<>` is also a good idea. After refactoring to `Rc<RefCell<dyn Widget>>` for the window handlers, and later also the other widgets, there seems to be some success. I think it's possible to build all widgets using this paradigm. A few questions arise:

1. The widget tree is ad-hoc, and at each `update()` call, the frames change (but don't move or disappear); is that enough and properly fast?
2. When a new widget is inserted somewhere, added as someone's child, etc. a `Rc<RefCell<Widget>>` will be added to the child widget, and the entire frame/quad set needs to be restructured. Ideally, the frames don't change index, so some sort of alloc/free scheme is needed there; The maximum number of frames depends on the available uniform space on the GPU (this is potentially a problem).
3. What about frameless widgets? Is that a good idea? Can we somehow make sure layout only affects the frames, and not the quads?
4. How can we measure if any of this is even a good idea?
5. Would a simpler `Rc<dyn Widget>` also suffice?

# 30.12.2019

So let's just build some widgets now, and see what happens (towards answering 1.), starting with a per-window frame allocator (answering 2.). Each widget needs a frame, so it can be moved around for free (answering 3.).

I first tried to make `Window` creatable and ownable by the user via a `Window::new`, and then somehow manage the pointers via `Rc<RefCell<Window>>` and `Weak<RefCell<UI>>`. This soon turned into a lot of code that didn't quite do what I wanted it to do. I then made the windows owned by, and internal to, `UI`, and refer to them via `usize` indices. This seems a lot more stable. However, using an array like this is kind of defeating the purpose of the borrow checker.

Now, to have `UI` access the widgets works with `Rc<RefCell<dyn Widget>>`, but it turns out to be quite difficult to give the `Rc<RefCell<dyn Widget>>` to `UI`, and simultaneously retain control and ownership of the original `Application` struct. The construction needed to access the `Application` once again is `widget.borrow_mut().as_any().downcast_ref::<Application>()`, and that returns a `Some(application)` or a `None` (if it's not actually an `Application`).

A possible way to solve this might be something along the lines of `Widget<T>` where `T` is `Application`, and then pass `context: &mut T` to the `Widget` methods as needed. Then call `ui.handle_events(&mut application)` to present the widgetry with a context. This may or may not eliminate the need for `Rc<RefCell<dyn Widget>>` altogether... Let's see...

Ok, that didn't work (again there is this double mutable borrow), so I decided to removed the entire notion of Widgets and callbacks, and went back to a very old way of doing this, comparable to X11 and Win32. This actually works, and is surprisingly simple:

```
pub fn main() {
    let mut ui = e::UI::new();
    let app_id = ui.create_app_window(e::isize_r::new(100,100,1920,1080),"Hello, World!");
    loop {
        thread::sleep(time::Duration::from_millis(10));
        while let Some(window_event) = ui.next_event() {
            match window_event.event {
                e::Event::Close => {
                    if window_event.id == app_id {
                        return;
                    }
                },
                _ => { },
            }
        }
    }
}
```

Essentially, the user is responsible for everything, and for now, that might be perfectly fine. It's relatively easy to combine all the OS-specific APIs this way, so let's see if Rust allows for interesting implementations of the widgets, and the frame/quad thing.

Lastly, I made a basic example, where a green box stays in the corner of the app window. This was all about building accessors for frames and quads.

# 31.12.2019

So, reasoning:

UI programming started as a purely academic effort (Xerox Parc, 1974), and then remained to be mostly an engineering-dominated field. Styling was not a thing, as long as the interface worked. Later, when Steve Jobs pushed designers through yet another rebirth, they started to invade and dominate the field, resulting in the galactic collapse that we call "frontend development" nowadays.

Whatever you want to call it, at the core we're still trying to: draw stuff, respond to input and manage the application software. A UI is essentially an event loop and a hierarchy of control elements, that allow the user to interact with the application. Inherently, this hierarchy is object-oriented. What I mean by that is, it facilitates a tree of components. This was true in the C days, C++, Microsoft MFC, X11, etc. all the way up to using Bootstrap HTML tags on a webpage.

Performance: When talking about improving UI performance, this mostly relates to graphics, and also somewhat to event geometry, component hierarchy and cache performance. It's the stuff that web browser developers have to think about, and it's also the stuff I want to address. I'm making the following assumptions:

- Any UI system currently in use is only ever 1. drawing quads, and 2. managing scrolling/zooming windows onto virtual spaces. => Let's explore if it is possible to offload all of that work onto the GPU.

- The component hierarchy seemed to fit the OOP paradigm somewhat in C++, but Rust is not C++. Maybe there are different ways that this can work. => Let's explore compile time and generic features that Rust has to offer.

# 1.1.2020

Happy new year.

Fiddled about with pixelformats and textures. It seems that the big power of Rust is the zero-cost abstractions, rather than the borrow checker. This is indeed quite interesting, but also makes the code harder to read for traditional eyes.

Did mostly font-related stuff. I'm getting a little better at expressing myself. in Rust, but also a bit tired today.

# 2.1.2020

MSDF works. I just realized that most likely it is possible to flatten all frame operations in the geometry shader to one linear matrix and one clipping rectangle, much like in a 3D scene graph. So, the pipeline is something like this:

update frames -> recalculate matrix and clipping -> upload to GPU

update quads -> upload to GPU

The question then becomes, is this recalculate step more expensive than doing all of this inside the geometry shader? Perhaps design a test to compare both options; that would need to be 5 traces then:

1. do regular old WIMP on CPU from a paint call, and update pixels in a framebuffer, maybe have a vector engine
2. do panel-immediate Makepad-style rendering on GPU, intertwining events with rendering as well
3. do quad/frame from the paint callbacks
4. do quad/frame where all frames and quads are retained in one buffer, maybe two passes (basically what I'm trying now)
5. do quad/frame where the frames are flattened at each frame update, maybe two passes

It's outside of the scope to really set up an experiment like this (although this might be very insightful for the Rust/UI community). I just want to build a good UI. Also, option 4 still seems like a good option for now.

So, now it seems reasonable to build a few widgets, next to Label, a grid layout and a button. Then perhaps test with a huge number of buttons and labels on screen. Another thing to try is the scrollable window.

# 3.1.2020

Replacing all the quads in a window with one upload function is faulty, because only the latest quads are assumed. The labels in the example would need to be embedded in some sort of higher structure. This structure needs to manage the quads.. This is not ideal, maybe. Explored a bit. It seems fruitful to write code like back in the C days, and then have the compiler inline/optimize everything into handy functions.

# 4.1.2020

Decided to focus back on games. This UI business is going to become a huge drain (it's already taking too long to even get started), and I really wanted to focus on something else instead. I will leave E generally in tact, move the UI-specific part to `try`, and go focus on the engine. Done. E is now only a very OS-near layer with utilities, and the game engine will do the rest. The engine needs a name, though...
