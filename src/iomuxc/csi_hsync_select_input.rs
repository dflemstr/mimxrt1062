#[doc = "Reader of register CSI_HSYNC_SELECT_INPUT"]
pub type R = crate::R<u32, super::CSI_HSYNC_SELECT_INPUT>;
#[doc = "Writer for register CSI_HSYNC_SELECT_INPUT"]
pub type W = crate::W<u32, super::CSI_HSYNC_SELECT_INPUT>;
#[doc = "Register CSI_HSYNC_SELECT_INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::CSI_HSYNC_SELECT_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selecting Pads Involved in Daisy Chain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAISY_A {
    #[doc = "0: Selecting Pad: GPIO_AD_B0_15 for Mode: ALT4"]
    GPIO_AD_B0_15_ALT4,
    #[doc = "1: Selecting Pad: GPIO_AD_B1_07 for Mode: ALT4"]
    GPIO_AD_B1_07_ALT4,
    #[doc = "2: Selecting Pad: GPIO_B1_14 for Mode: ALT2"]
    GPIO_B1_14_ALT2,
}
impl From<DAISY_A> for u8 {
    #[inline(always)]
    fn from(variant: DAISY_A) -> Self {
        match variant {
            DAISY_A::GPIO_AD_B0_15_ALT4 => 0,
            DAISY_A::GPIO_AD_B1_07_ALT4 => 1,
            DAISY_A::GPIO_B1_14_ALT2 => 2,
        }
    }
}
#[doc = "Reader of field `DAISY`"]
pub type DAISY_R = crate::R<u8, DAISY_A>;
impl DAISY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DAISY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DAISY_A::GPIO_AD_B0_15_ALT4),
            1 => Val(DAISY_A::GPIO_AD_B1_07_ALT4),
            2 => Val(DAISY_A::GPIO_B1_14_ALT2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_15_ALT4`"]
    #[inline(always)]
    pub fn is_gpio_ad_b0_15_alt4(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B0_15_ALT4
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B1_07_ALT4`"]
    #[inline(always)]
    pub fn is_gpio_ad_b1_07_alt4(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B1_07_ALT4
    }
    #[doc = "Checks if the value of the field is `GPIO_B1_14_ALT2`"]
    #[inline(always)]
    pub fn is_gpio_b1_14_alt2(&self) -> bool {
        *self == DAISY_A::GPIO_B1_14_ALT2
    }
}
#[doc = "Write proxy for field `DAISY`"]
pub struct DAISY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAISY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAISY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT4"]
    #[inline(always)]
    pub fn gpio_ad_b0_15_alt4(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B0_15_ALT4)
    }
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT4"]
    #[inline(always)]
    pub fn gpio_ad_b1_07_alt4(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B1_07_ALT4)
    }
    #[doc = "Selecting Pad: GPIO_B1_14 for Mode: ALT2"]
    #[inline(always)]
    pub fn gpio_b1_14_alt2(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_B1_14_ALT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&self) -> DAISY_R {
        DAISY_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&mut self) -> DAISY_W {
        DAISY_W { w: self }
    }
}
