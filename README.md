# Kvasir

Game engine in Rust, because RPGMaker isn't quite there yet.

## Architecture

```
    games
=============
     rpg
-------------
scene   audio
-------------
     sys
```

### sys: System Interface

This is the hardware abstraction, and should work the same on all supported platforms. Ultimately, this is where the video layers live. Also key/mouse/touch responses. Probably also text rendering, and some basic presentation things, like layer blending, zooming, scaling, color filtering, etc.

### scene: Scene Rendering

A layer of the UI can be the target of scene rendering. This is the current map, map tiles, and insane methods to use shading to squeeze the last life out of this ancient technology. Here also sprites are rendered, animation cycles run, etc.

### sound: Music and Sound Effects

This controls all forms of audio. It can play multiple audio media (MP3 or FLAC probably), and if there is time I would like to code some synthesizers here. Originally this would have been an entire player with DAW, but lmms comes pretty close to what that would have looked like.

### rpg: The RPG Engine

Here characters with stats live, the story, events, quests, save/load, menus with selfies, a battle system, inventory, etc.

## sys

```
per layer:
     render to texture
     run texture through layer postprocessing, and output as blend source onto final texture
postprocess final texture, and output onto display
```

### Layer

A layer is a texture to which a layer of the game can be rendered, for instance, by the scene renderer. Each layer has a small configurable postprocessing shader, as well as blend parameters.

### Postprocessing

When all layers are blended together into a texture, this texture is rendered by the final postprocessing shader.