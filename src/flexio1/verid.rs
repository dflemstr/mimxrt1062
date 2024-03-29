#[doc = "Reader of register VERID"]
pub type R = crate::R<u32, super::VERID>;
#[doc = "Feature Specification Number\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEATURE_A {
    #[doc = "0: Standard features implemented."]
    FEATURE_0,
    #[doc = "1: Supports state, logic and parallel modes."]
    FEATURE_1,
}
impl From<FEATURE_A> for u16 {
    #[inline(always)]
    fn from(variant: FEATURE_A) -> Self {
        match variant {
            FEATURE_A::FEATURE_0 => 0,
            FEATURE_A::FEATURE_1 => 1,
        }
    }
}
#[doc = "Reader of field `FEATURE`"]
pub type FEATURE_R = crate::R<u16, FEATURE_A>;
impl FEATURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FEATURE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FEATURE_A::FEATURE_0),
            1 => Val(FEATURE_A::FEATURE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FEATURE_0`"]
    #[inline(always)]
    pub fn is_feature_0(&self) -> bool {
        *self == FEATURE_A::FEATURE_0
    }
    #[doc = "Checks if the value of the field is `FEATURE_1`"]
    #[inline(always)]
    pub fn is_feature_1(&self) -> bool {
        *self == FEATURE_A::FEATURE_1
    }
}
#[doc = "Reader of field `MINOR`"]
pub type MINOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJOR`"]
pub type MAJOR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Feature Specification Number"]
    #[inline(always)]
    pub fn feature(&self) -> FEATURE_R {
        FEATURE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
