#[doc = "Reader of register TTSRH"]
pub type R = crate::R<u8, super::TTSRH>;
#[doc = "Reader of field `TTS`"]
pub type TTS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp"]
    #[inline(always)]
    pub fn tts(&self) -> TTS_R {
        TTS_R::new((self.bits & 0xff) as u8)
    }
}
