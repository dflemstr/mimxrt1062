#[doc = "Reader of register TDR[%s]"]
pub type R = crate::R<u32, super::TDR>;
#[doc = "Writer for register TDR[%s]"]
pub type W = crate::W<u32, super::TDR>;
#[doc = "Register TDR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::TDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDR`"]
pub type TDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TDR`"]
pub struct TDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmit Data Register"]
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data Register"]
    #[inline(always)]
    pub fn tdr(&mut self) -> TDR_W {
        TDR_W { w: self }
    }
}
