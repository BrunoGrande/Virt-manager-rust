//! `<sound>` — upstream's `devices/sound.py`. Grounded in the real
//! `XMLProperty` declarations: note `multichannel` uses `is_yesno=True`
//! (unlike the `is_onoff` fields skipped in a couple of earlier
//! devices) — the first real exercise of `Option<bool>` since
//! `DeviceGraphics`'s `autoport`. `streams` has no `is_int` flag
//! upstream despite the name, so it stays `Option<String>` rather than
//! guessed into `Option<u32>`.

use virtinst_xml::XmlBound;

/// ```xml
/// <sound model="ich9" multichannel="yes">
///   <audio id="1"/>
/// </sound>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "sound")]
pub struct DeviceSound {
    #[xml(attribute = "model")]
    pub model: Option<String>,

    #[xml(attribute = "multichannel")]
    pub multichannel: Option<bool>,

    #[xml(attribute = "streams")]
    pub streams: Option<String>,

    #[xml(path = "audio", attribute = "id")]
    pub audio_id: Option<String>,
}
