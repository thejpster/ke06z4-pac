#[doc = "Reader of register UUIDMH"]
pub type R = crate::R<u32, super::UUIDMH>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Universally Unique Identifier"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff) as u16)
    }
}
