extern crate engine;
#[macro_use]
extern crate cpython;

use std::cell::RefCell;

use cpython::{Python,PyResult};

py_class!(class Synth |py| {
    data obj: e::audio::Synth;
});

fn py_create_dogma(py: Python) -> PyResult<Synth> {

    Synth::create_instance(py,e::audio::Synth::AsDogma(e::audio::Dogma106::new()))
}

py_class!(class Track |py| {
    data obj: e::audio::Track;
});

fn py_create_track(py: Python) -> PyResult<Track> {
    Track::create_instance(py,e::audio::Track::new())
}

py_class!(class Effect |py| {
    data obj: e::audio::Effect;
});

fn py_create_effect(py: Python) -> PyResult<Effect> {
    Effect::create_instance(py,e::audio::Effect::new())
}

py_class!(class Filter |py| {
    data obj: e::audio::Filter;
});

fn py_create_filter(py: Python) -> PyResult<Filter> {
    Filter::create_instance(py,e::audio::Filter::new())
}

py_class!(class Audio |py| {

    data obj: e::audio::Audio;

    def silence(&self) -> PyResult<bool> {
        self.obj(py).silence();
        Ok(true)
    }

    def track(&self,track: &Track) -> PyResult<bool> {
        self.obj(py).track((*track.obj(py)).clone());
        Ok(true)
    }

    def effect(&self,effect: &Effect) -> PyResult<bool> {
        self.obj(py).effect((*effect.obj(py)).clone());
        Ok(true)
    }

    def track_volume(&self,volume: f32) -> PyResult<bool> {
        self.obj(py).track_volume(volume);
        Ok(true)
    }

    def track_panning(&self,panning: f32) -> PyResult<bool> {
        self.obj(py).track_panning(panning);
        Ok(true)
    }

    def effect_volume(&self,volume: f32) -> PyResult<bool> {
        self.obj(py).effect_volume(volume);
        Ok(true)
    }

    def effect_panning(&self,panning: f32) -> PyResult<bool> {
        self.obj(py).effect_panning(panning);
        Ok(true)
    }

    def effect_filter(&self,filter: &Filter) -> PyResult<bool> {
        self.obj(py).effect_filter(Some((*filter.obj(py)).clone()));
        Ok(true)
    }

    def effect_no_filter(&self) -> PyResult<bool> {
        self.obj(py).effect_filter(None);
        Ok(true)
    }

    def edit_set_track(&self,track: &Track) -> PyResult<bool> {
        self.obj(py).edit_set_track((*track.obj(py)).clone());
        Ok(true)
    }

    // def edit_get_track(&self) -> PyResult<Track> { }

    def edit_play(&self) -> PyResult<bool> {
        self.obj(py).edit_play();
        Ok(true)
    }

    def edit_pause(&self) -> PyResult<bool> {
        self.obj(py).edit_pause();
        Ok(true)
    }

    def edit_stop(&self) -> PyResult<bool> {
        self.obj(py).edit_stop();
        Ok(true)
    }

    def edit_speed(&self,bpm: u32) -> PyResult<bool> {
        self.obj(py).edit_speed(bpm);
        Ok(true)
    }

    def edit_channels(&self,n: u32) -> PyResult<bool> {
        self.obj(py).edit_channels(n);
        Ok(true)
    }

    def edit_clear(&self,pos: u64,length: u64) -> PyResult<bool> {
        self.obj(py).edit_clear(pos,length);
        Ok(true)
    }

    def edit_channel_clear(&self,n: u32,pos: u64,length: u64) -> PyResult<bool> {
        self.obj(py).edit_channel_clear(n,pos,length);
        Ok(true)
    }

    def edit_channel_volume(&self,n: u32,volume: f32) -> PyResult<bool> {
        self.obj(py).edit_channel_volume(n,volume);
        Ok(true)
    }

    def edit_channel_panning(&self,n: u32,panning: f32) -> PyResult<bool> {
        self.obj(py).edit_channel_panning(n,panning);
        Ok(true)
    }

    def edit_channel_enabled(&self,n: u32,enabled: bool) -> PyResult<bool> {
        self.obj(py).edit_channel_enabled(n,enabled);
        Ok(true)
    }

    def edit_channel_synth(&self,n: u32,synth: &Synth) -> PyResult<bool> {
        self.obj(py).edit_channel_synth(n,(*synth.obj(py)).clone());
        Ok(true)
    }

    def edit_channel_note(&self,n: u32,pos: u64, note: usize, velocity: f32,length: u64) -> PyResult<bool> {
        self.obj(py).edit_channel_note(n,pos,note,velocity,length);
        Ok(true)
    }

    def edit_set_synth(&self,synth: &Synth) -> PyResult<bool> {
        self.obj(py).edit_set_synth((*synth.obj(py)).clone());
        Ok(true)
    }

    // def edit_get_synth(&self) -> PyResult<Synth> { }

    def edit_note(&self,note: usize, velocity: f32) -> PyResult<bool> {
        self.obj(py).edit_note(note,velocity);
        Ok(true)
    }

    def edit_release(&self,note: usize) -> PyResult<bool> {
        self.obj(py).edit_release(note);
        Ok(true)
    }
});

fn py_create_audio(py: Python) -> PyResult<Audio> {

    Audio::create_instance(py,e::audio::Audio::new())
}

py_module_initializer!(pye, initpye, PyInit_pye, |py, m| {
    m.add(py, "__doc__", "Python wrapper for E.")?;
    m.add(py, "create_audio", py_fn!(py, py_create_audio()))?;
    m.add(py, "create_dogma", py_fn!(py, py_create_dogma()))?;
    m.add(py, "create_track", py_fn!(py, py_create_track()))?;
    m.add(py, "create_effect", py_fn!(py, py_create_effect()))?;
    m.add(py, "create_filter", py_fn!(py, py_create_filter()))?;
    Ok(())
});