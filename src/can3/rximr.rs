#[doc = "Reader of register RXIMR[%s]"]
pub type R = crate::R<u32, super::RXIMR>;
#[doc = "Writer for register RXIMR[%s]"]
pub type W = crate::W<u32, super::RXIMR>;
#[doc = "Register RXIMR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::RXIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MI`"]
pub type MI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MI`"]
pub struct MI_W<'a> {
    w: &'a mut W,
}
impl<'a> MI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi(&self) -> MI_R {
        MI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi(&mut self) -> MI_W {
        MI_W { w: self }
    }
}
