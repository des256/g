// G Sprite Editor - Main Application
// by Desmond Germans, 2020

use e::*;
use std::{
    rc::Rc,
};

use crate::document::*;
use crate::editcanvas::*;

pub struct Application {
    _uistate: Rc<ui::UIState>,
    edit_canvas: EditCanvas,
    // needs collection of open documents with associated edit_canvases in the book
}

impl Application {
    pub fn new(state: &Rc<ui::UIState>) -> Result<Application,SystemError> {
        let mat = image::load::<pixel::ARGB8>("test.png").expect("Unable to open test.png");
        let document = Rc::new(Document::new(&state.graphics,mat.size));
        document.layers[0].texture.load(vec2!(0,0),&mat);
        let edit_canvas = EditCanvas::new(state,&document)?;

        Ok(Application {
            _uistate: Rc::clone(state),
            edit_canvas: edit_canvas,
        })
    }
}

impl ui::Widget for Application {
    fn get_rect(&self) -> Rect<i32> {
        self.edit_canvas.get_rect()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.edit_canvas.set_rect(r)
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        self.edit_canvas.calc_min_size()
    }

    fn draw(&self,context: Vec2<i32>) {
        self.edit_canvas.draw(context);
    }

    fn handle_mouse_press(&self,p: Vec2<i32>,b: MouseButton) {
        self.edit_canvas.handle_mouse_press(p,b);
    }

    fn handle_mouse_release(&self,p: Vec2<i32>,b: MouseButton) {
        self.edit_canvas.handle_mouse_release(p,b);
    }

    fn handle_mouse_move(&self,p: Vec2<i32>) -> bool {
        self.edit_canvas.handle_mouse_move(p)
    }

    fn handle_mouse_wheel(&self,w: MouseWheel) {
        self.edit_canvas.handle_mouse_wheel(w)
    }
}

/*

make multiple shaders, one for each layer, and a final one for the grid and pixel grid

selection is a binary mask, can be texture stage for the final shader

to fit this in UI as separate draw call, first use an overlay; draw the image after the whole UI; 


Features:

* zoom/unzoom
* pixel grid visible/invisible
* tiling horizontal/vertical
+ grid visible/invisible
+ multiple layers in the image
+ layers blending modes
- complicated pixel format
- save/load image
- MDI
- recently opened files list
- save/load sprite sheet setup
- undo/redo queue
- layers can be visible/invisible
- layers can be locked
- draw/erase with brush
- color palette to choose brush color
- sample color
- paint bucket
- draw line with brush
- draw rectangle with brush
- blur
- select all/none
- invert selection
- select color range
- select border
- grow selection
- shrink selection
- save/load selection mask
- cut/copy/paste/clear selection
- paste as new sprite
- paste as new layer
- paste as new cell
- paste as new sprite
- rotate selection by 90, -90, 180
- flip horizontal/vertical
- transform 2D
- shift up/down/left/right
- replace color
- invert colors
- adjust brightness/contrast
- adjust hue/saturation
- adjust color curve
- outline
- convolution matrix
- despeckle
- insert text (as floating selection)
- preferences
- properties of the sprite/image
- specify pixel format, don't forget 2-color
- duplicate image to new image
- set sprite/image size (scale)
- set canvas size (add more free space)
- rotate entire canvas by 90, -90, 180
- flip entire canvas horizontal/vertical
- trim away transparent parts
- new layer
- duplicate layer
- flatten layers
- flatten all visible layers
- properties of the frames
- new frame(s)
- delete frames
- grid settings
- play/stop/reverse animation
- various help screens
- 3D normal editor
- map editor features




Aseprite main window:

File
    New...
    Open...
    Open Recent
        <recently opened files>
    ---
    Save
    Save As...
    Export...
    Close
    Close All
    ---
    Import Sprite Sheet
    Export Sprite Sheet
    Repeat Last Export
    ---
    Scripts
        <recently opened scripts>
    ---
    Exit

Edit
    Undo
    Redo
    Undo History
    ---
    Cut
    Copy
    Copy Merged
    Paste >
    Paste Special
        Paste as New Sprite
        Paste as New Layer
        Paste as New Reference Layer
    Delete
    ---
    Fill
    Stroke
    ---
    Rotate
        180
        90 CW
        90 CCW
    Flip Horizontal
    Flip Vertical
    Transform
    Shift
        Left
        Right
        Up
        Down
    ---
    New Brush
    New Sprite from Selection
    ---
    Replace Color...
    Invert...
    Adjustments
        Brightness/Contrast...
        Hue/Saturation...
        Color Curve...
    FX
        Outline
        Convolution Matrix...
        Despeckle (Median Filter)
    Insert Text
    ---
    Keyboard Shortcuts...
    Preferences...

Sprite
    Properties
    Color Mode
        RGB Color
        Grayscale
        Indexed
        ---
        More
    ---
    Duplicate
    ---
    Sprite Size...
    Canvas Size...
    Rotate Canvas
        180
        90 CW
        90 CCW
        ---
        Flip Canvas Horizontal
        Flip Canvas Vertical
    ---
    Crop
    Trim

Layer
    Properties...
    Visible
    Lock Layers
    Open Group
    ---
    New...
        New Layer
        New Group
        ---
        New Layer via Copy
        New Layer via Cut
        ---
        New Reference Layer from File
    Delete Layer
    Background from Layer
    Layer from Background
    ---
    Duplicate
    Merge Down
    Flatten
    Flatten Visible

Frame
    Frame Properties...
    Cel Properties...
    ---
    New Frame
    New Empty Frame
    Duplicate Cel(s)
    Duplicate Linked Cel(s)
    Delete Frame
    ---
    Tags
        Tag Properties...
        ---
        New Tag
        Delete Tag
    Jump to
        First Frame
        Previous Frame
        Next Frame
        Last Frame
        ---
        First Frame in Tag
        Last Frame in Tag
        ---
        Go to Frame
    Play Animation
    ---
    Constant Frame Rate
    Reverse Frames

Select
    All
    Deselect
    Reselect
    Inverse
    ---
    Color Range
    Modify
        Border
        Expand
        Contract
    ---
    Load from MSK file
    Save to MSK file

View
    Duplicate View
    ---
    Extras
    Show
        Layer Edges
        Selection Edges
        Grid
        Auto Guides
        Slices
        Pixel Grid
        ---
        Brush Preview
    ---
    Grid
        Grid Settings
        Selection as Grid
        Snap to Grid
    Tiled Mode
        None
        Tiled in Both Axes
        Tiled in X Axis
        Tiled in Y Axis
    Symmetry Options
    ---
    Set Loop Section
    Show Onion Skin
    ---
    Timeline
    Preview
    Full Screen Mode
    Full Screen Preview
    Home
    ---
    Refresh & Reload Skin

Help
    Readme
    ---
    Quick Reference
    Documentation
    Tutorial
    ---
    Release Notes
    Twitter
    ---
    About
*/