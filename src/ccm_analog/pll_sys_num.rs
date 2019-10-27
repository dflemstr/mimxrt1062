#[doc = "Reader of register PLL_SYS_NUM"]
pub type R = crate::R<u32, super::PLL_SYS_NUM>;
#[doc = "Writer for register PLL_SYS_NUM"]
pub type W = crate::W<u32, super::PLL_SYS_NUM>;
#[doc = "Register PLL_SYS_NUM `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL_SYS_NUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `A`"]
pub type A_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `A`"]
pub struct A_W<'a> {
    w: &'a mut W,
}
impl<'a> A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - 30 bit numerator (A) of fractional loop divider (signed integer)."]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - 30 bit numerator (A) of fractional loop divider (signed integer)."]
    #[inline(always)]
    pub fn a(&mut self) -> A_W {
        A_W { w: self }
    }
}
