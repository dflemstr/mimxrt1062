#[doc = "Reader of register HFSR"]
pub type R = crate::R<u32, super::HFSR>;
#[doc = "Writer for register HFSR"]
pub type W = crate::W<u32, super::HFSR>;
#[doc = "Register HFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::HFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicates a BusFault on a vector table read during exception processing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTTBL_A {
    #[doc = "0: no BusFault on vector table read"]
    VECTTBL_0,
    #[doc = "1: BusFault on vector table read"]
    VECTTBL_1,
}
impl From<VECTTBL_A> for bool {
    #[inline(always)]
    fn from(variant: VECTTBL_A) -> Self {
        match variant {
            VECTTBL_A::VECTTBL_0 => false,
            VECTTBL_A::VECTTBL_1 => true,
        }
    }
}
#[doc = "Reader of field `VECTTBL`"]
pub type VECTTBL_R = crate::R<bool, VECTTBL_A>;
impl VECTTBL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VECTTBL_A {
        match self.bits {
            false => VECTTBL_A::VECTTBL_0,
            true => VECTTBL_A::VECTTBL_1,
        }
    }
    #[doc = "Checks if the value of the field is `VECTTBL_0`"]
    #[inline(always)]
    pub fn is_vecttbl_0(&self) -> bool {
        *self == VECTTBL_A::VECTTBL_0
    }
    #[doc = "Checks if the value of the field is `VECTTBL_1`"]
    #[inline(always)]
    pub fn is_vecttbl_1(&self) -> bool {
        *self == VECTTBL_A::VECTTBL_1
    }
}
#[doc = "Write proxy for field `VECTTBL`"]
pub struct VECTTBL_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTTBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VECTTBL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl_0(self) -> &'a mut W {
        self.variant(VECTTBL_A::VECTTBL_0)
    }
    #[doc = "BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl_1(self) -> &'a mut W {
        self.variant(VECTTBL_A::VECTTBL_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCED_A {
    #[doc = "0: no forced HardFault"]
    FORCED_0,
    #[doc = "1: forced HardFault"]
    FORCED_1,
}
impl From<FORCED_A> for bool {
    #[inline(always)]
    fn from(variant: FORCED_A) -> Self {
        match variant {
            FORCED_A::FORCED_0 => false,
            FORCED_A::FORCED_1 => true,
        }
    }
}
#[doc = "Reader of field `FORCED`"]
pub type FORCED_R = crate::R<bool, FORCED_A>;
impl FORCED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCED_A {
        match self.bits {
            false => FORCED_A::FORCED_0,
            true => FORCED_A::FORCED_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCED_0`"]
    #[inline(always)]
    pub fn is_forced_0(&self) -> bool {
        *self == FORCED_A::FORCED_0
    }
    #[doc = "Checks if the value of the field is `FORCED_1`"]
    #[inline(always)]
    pub fn is_forced_1(&self) -> bool {
        *self == FORCED_A::FORCED_1
    }
}
#[doc = "Write proxy for field `FORCED`"]
pub struct FORCED_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no forced HardFault"]
    #[inline(always)]
    pub fn forced_0(self) -> &'a mut W {
        self.variant(FORCED_A::FORCED_0)
    }
    #[doc = "forced HardFault"]
    #[inline(always)]
    pub fn forced_1(self) -> &'a mut W {
        self.variant(FORCED_A::FORCED_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGEVT_A {
    #[doc = "0: No Debug event has occurred."]
    DEBUGEVT_0,
    #[doc = "1: Debug event has occurred. The Debug Fault Status Register has been updated."]
    DEBUGEVT_1,
}
impl From<DEBUGEVT_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGEVT_A) -> Self {
        match variant {
            DEBUGEVT_A::DEBUGEVT_0 => false,
            DEBUGEVT_A::DEBUGEVT_1 => true,
        }
    }
}
#[doc = "Reader of field `DEBUGEVT`"]
pub type DEBUGEVT_R = crate::R<bool, DEBUGEVT_A>;
impl DEBUGEVT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGEVT_A {
        match self.bits {
            false => DEBUGEVT_A::DEBUGEVT_0,
            true => DEBUGEVT_A::DEBUGEVT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBUGEVT_0`"]
    #[inline(always)]
    pub fn is_debugevt_0(&self) -> bool {
        *self == DEBUGEVT_A::DEBUGEVT_0
    }
    #[doc = "Checks if the value of the field is `DEBUGEVT_1`"]
    #[inline(always)]
    pub fn is_debugevt_1(&self) -> bool {
        *self == DEBUGEVT_A::DEBUGEVT_1
    }
}
#[doc = "Write proxy for field `DEBUGEVT`"]
pub struct DEBUGEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGEVT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBUGEVT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Debug event has occurred."]
    #[inline(always)]
    pub fn debugevt_0(self) -> &'a mut W {
        self.variant(DEBUGEVT_A::DEBUGEVT_0)
    }
    #[doc = "Debug event has occurred. The Debug Fault Status Register has been updated."]
    #[inline(always)]
    pub fn debugevt_1(self) -> &'a mut W {
        self.variant(DEBUGEVT_A::DEBUGEVT_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Indicates a BusFault on a vector table read during exception processing."]
    #[inline(always)]
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled."]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Indicates a BusFault on a vector table read during exception processing."]
    #[inline(always)]
    pub fn vecttbl(&mut self) -> VECTTBL_W {
        VECTTBL_W { w: self }
    }
    #[doc = "Bit 30 - Indicates a forced hard fault, generated by escalation of a fault with configurable priority that cannot be handles, either because of priority or because it is disabled."]
    #[inline(always)]
    pub fn forced(&mut self) -> FORCED_W {
        FORCED_W { w: self }
    }
    #[doc = "Bit 31 - Reserved for Debug use. When writing to the register you must write 0 to this bit, otherwise behavior is Unpredictable."]
    #[inline(always)]
    pub fn debugevt(&mut self) -> DEBUGEVT_W {
        DEBUGEVT_W { w: self }
    }
}
