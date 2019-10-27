#[doc = "Reader of register MCR0"]
pub type R = crate::R<u32, super::MCR0>;
#[doc = "Writer for register MCR0"]
pub type W = crate::W<u32, super::MCR0>;
#[doc = "Register MCR0 `reset()`'s with value 0xffff_80c2"]
impl crate::ResetValue for super::MCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_80c2
    }
}
#[doc = "Reader of field `SWRESET`"]
pub type SWRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRESET`"]
pub struct SWRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MDIS`"]
pub type MDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDIS`"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
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
#[doc = "Sample Clock source selection for Flash Reading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCLKSRC_A {
    #[doc = "0: Dummy Read strobe generated by FlexSPI Controller and loopback internally."]
    RXCLKSRC_0,
    #[doc = "1: Dummy Read strobe generated by FlexSPI Controller and loopback from DQS pad."]
    RXCLKSRC_1,
    #[doc = "3: Flash provided Read strobe and input from DQS pad"]
    RXCLKSRC_3,
}
impl From<RXCLKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RXCLKSRC_A) -> Self {
        match variant {
            RXCLKSRC_A::RXCLKSRC_0 => 0,
            RXCLKSRC_A::RXCLKSRC_1 => 1,
            RXCLKSRC_A::RXCLKSRC_3 => 3,
        }
    }
}
#[doc = "Reader of field `RXCLKSRC`"]
pub type RXCLKSRC_R = crate::R<u8, RXCLKSRC_A>;
impl RXCLKSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXCLKSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXCLKSRC_A::RXCLKSRC_0),
            1 => Val(RXCLKSRC_A::RXCLKSRC_1),
            3 => Val(RXCLKSRC_A::RXCLKSRC_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXCLKSRC_0`"]
    #[inline(always)]
    pub fn is_rxclksrc_0(&self) -> bool {
        *self == RXCLKSRC_A::RXCLKSRC_0
    }
    #[doc = "Checks if the value of the field is `RXCLKSRC_1`"]
    #[inline(always)]
    pub fn is_rxclksrc_1(&self) -> bool {
        *self == RXCLKSRC_A::RXCLKSRC_1
    }
    #[doc = "Checks if the value of the field is `RXCLKSRC_3`"]
    #[inline(always)]
    pub fn is_rxclksrc_3(&self) -> bool {
        *self == RXCLKSRC_A::RXCLKSRC_3
    }
}
#[doc = "Write proxy for field `RXCLKSRC`"]
pub struct RXCLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCLKSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback internally."]
    #[inline(always)]
    pub fn rxclksrc_0(self) -> &'a mut W {
        self.variant(RXCLKSRC_A::RXCLKSRC_0)
    }
    #[doc = "Dummy Read strobe generated by FlexSPI Controller and loopback from DQS pad."]
    #[inline(always)]
    pub fn rxclksrc_1(self) -> &'a mut W {
        self.variant(RXCLKSRC_A::RXCLKSRC_1)
    }
    #[doc = "Flash provided Read strobe and input from DQS pad"]
    #[inline(always)]
    pub fn rxclksrc_3(self) -> &'a mut W {
        self.variant(RXCLKSRC_A::RXCLKSRC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Enable AHB bus Read Access to IP RX FIFO.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARDFEN_A {
    #[doc = "0: IP RX FIFO should be read by IP Bus. AHB Bus read access to IP RX FIFO memory space will get bus error response."]
    ARDFEN_0,
    #[doc = "1: IP RX FIFO should be read by AHB Bus. IP Bus read access to IP RX FIFO memory space will always return data zero but no bus error response."]
    ARDFEN_1,
}
impl From<ARDFEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARDFEN_A) -> Self {
        match variant {
            ARDFEN_A::ARDFEN_0 => false,
            ARDFEN_A::ARDFEN_1 => true,
        }
    }
}
#[doc = "Reader of field `ARDFEN`"]
pub type ARDFEN_R = crate::R<bool, ARDFEN_A>;
impl ARDFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARDFEN_A {
        match self.bits {
            false => ARDFEN_A::ARDFEN_0,
            true => ARDFEN_A::ARDFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARDFEN_0`"]
    #[inline(always)]
    pub fn is_ardfen_0(&self) -> bool {
        *self == ARDFEN_A::ARDFEN_0
    }
    #[doc = "Checks if the value of the field is `ARDFEN_1`"]
    #[inline(always)]
    pub fn is_ardfen_1(&self) -> bool {
        *self == ARDFEN_A::ARDFEN_1
    }
}
#[doc = "Write proxy for field `ARDFEN`"]
pub struct ARDFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARDFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IP RX FIFO should be read by IP Bus. AHB Bus read access to IP RX FIFO memory space will get bus error response."]
    #[inline(always)]
    pub fn ardfen_0(self) -> &'a mut W {
        self.variant(ARDFEN_A::ARDFEN_0)
    }
    #[doc = "IP RX FIFO should be read by AHB Bus. IP Bus read access to IP RX FIFO memory space will always return data zero but no bus error response."]
    #[inline(always)]
    pub fn ardfen_1(self) -> &'a mut W {
        self.variant(ARDFEN_A::ARDFEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable AHB bus Write Access to IP TX FIFO.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATDFEN_A {
    #[doc = "0: IP TX FIFO should be written by IP Bus. AHB Bus write access to IP TX FIFO memory space will get bus error response."]
    ATDFEN_0,
    #[doc = "1: IP TX FIFO should be written by AHB Bus. IP Bus write access to IP TX FIFO memory space will be ignored but no bus error response."]
    ATDFEN_1,
}
impl From<ATDFEN_A> for bool {
    #[inline(always)]
    fn from(variant: ATDFEN_A) -> Self {
        match variant {
            ATDFEN_A::ATDFEN_0 => false,
            ATDFEN_A::ATDFEN_1 => true,
        }
    }
}
#[doc = "Reader of field `ATDFEN`"]
pub type ATDFEN_R = crate::R<bool, ATDFEN_A>;
impl ATDFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATDFEN_A {
        match self.bits {
            false => ATDFEN_A::ATDFEN_0,
            true => ATDFEN_A::ATDFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ATDFEN_0`"]
    #[inline(always)]
    pub fn is_atdfen_0(&self) -> bool {
        *self == ATDFEN_A::ATDFEN_0
    }
    #[doc = "Checks if the value of the field is `ATDFEN_1`"]
    #[inline(always)]
    pub fn is_atdfen_1(&self) -> bool {
        *self == ATDFEN_A::ATDFEN_1
    }
}
#[doc = "Write proxy for field `ATDFEN`"]
pub struct ATDFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATDFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATDFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IP TX FIFO should be written by IP Bus. AHB Bus write access to IP TX FIFO memory space will get bus error response."]
    #[inline(always)]
    pub fn atdfen_0(self) -> &'a mut W {
        self.variant(ATDFEN_A::ATDFEN_0)
    }
    #[doc = "IP TX FIFO should be written by AHB Bus. IP Bus write access to IP TX FIFO memory space will be ignored but no bus error response."]
    #[inline(always)]
    pub fn atdfen_1(self) -> &'a mut W {
        self.variant(ATDFEN_A::ATDFEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Half Speed Serial Flash access Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEN_A {
    #[doc = "0: Disable divide by 2 of serial flash clock for half speed commands."]
    HSEN_0,
    #[doc = "1: Enable divide by 2 of serial flash clock for half speed commands."]
    HSEN_1,
}
impl From<HSEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSEN_A) -> Self {
        match variant {
            HSEN_A::HSEN_0 => false,
            HSEN_A::HSEN_1 => true,
        }
    }
}
#[doc = "Reader of field `HSEN`"]
pub type HSEN_R = crate::R<bool, HSEN_A>;
impl HSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEN_A {
        match self.bits {
            false => HSEN_A::HSEN_0,
            true => HSEN_A::HSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSEN_0`"]
    #[inline(always)]
    pub fn is_hsen_0(&self) -> bool {
        *self == HSEN_A::HSEN_0
    }
    #[doc = "Checks if the value of the field is `HSEN_1`"]
    #[inline(always)]
    pub fn is_hsen_1(&self) -> bool {
        *self == HSEN_A::HSEN_1
    }
}
#[doc = "Write proxy for field `HSEN`"]
pub struct HSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable divide by 2 of serial flash clock for half speed commands."]
    #[inline(always)]
    pub fn hsen_0(self) -> &'a mut W {
        self.variant(HSEN_A::HSEN_0)
    }
    #[doc = "Enable divide by 2 of serial flash clock for half speed commands."]
    #[inline(always)]
    pub fn hsen_1(self) -> &'a mut W {
        self.variant(HSEN_A::HSEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Doze mode enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEEN_A {
    #[doc = "0: Doze mode support disabled. AHB clock and serial clock will not be gated off when there is doze mode request from system."]
    DOZEEN_0,
    #[doc = "1: Doze mode support enabled. AHB clock and serial clock will be gated off when there is doze mode request from system."]
    DOZEEN_1,
}
impl From<DOZEEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEEN_A) -> Self {
        match variant {
            DOZEEN_A::DOZEEN_0 => false,
            DOZEEN_A::DOZEEN_1 => true,
        }
    }
}
#[doc = "Reader of field `DOZEEN`"]
pub type DOZEEN_R = crate::R<bool, DOZEEN_A>;
impl DOZEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEEN_A {
        match self.bits {
            false => DOZEEN_A::DOZEEN_0,
            true => DOZEEN_A::DOZEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEEN_0`"]
    #[inline(always)]
    pub fn is_dozeen_0(&self) -> bool {
        *self == DOZEEN_A::DOZEEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEEN_1`"]
    #[inline(always)]
    pub fn is_dozeen_1(&self) -> bool {
        *self == DOZEEN_A::DOZEEN_1
    }
}
#[doc = "Write proxy for field `DOZEEN`"]
pub struct DOZEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Doze mode support disabled. AHB clock and serial clock will not be gated off when there is doze mode request from system."]
    #[inline(always)]
    pub fn dozeen_0(self) -> &'a mut W {
        self.variant(DOZEEN_A::DOZEEN_0)
    }
    #[doc = "Doze mode support enabled. AHB clock and serial clock will be gated off when there is doze mode request from system."]
    #[inline(always)]
    pub fn dozeen_1(self) -> &'a mut W {
        self.variant(DOZEEN_A::DOZEEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "This bit is to support Flash Octal mode access by combining Port A and B Data pins (SIOA\\[3:0\\] and SIOB\\[3:0\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBINATIONEN_A {
    #[doc = "0: Disable."]
    COMBINATIONEN_0,
    #[doc = "1: Enable."]
    COMBINATIONEN_1,
}
impl From<COMBINATIONEN_A> for bool {
    #[inline(always)]
    fn from(variant: COMBINATIONEN_A) -> Self {
        match variant {
            COMBINATIONEN_A::COMBINATIONEN_0 => false,
            COMBINATIONEN_A::COMBINATIONEN_1 => true,
        }
    }
}
#[doc = "Reader of field `COMBINATIONEN`"]
pub type COMBINATIONEN_R = crate::R<bool, COMBINATIONEN_A>;
impl COMBINATIONEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBINATIONEN_A {
        match self.bits {
            false => COMBINATIONEN_A::COMBINATIONEN_0,
            true => COMBINATIONEN_A::COMBINATIONEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COMBINATIONEN_0`"]
    #[inline(always)]
    pub fn is_combinationen_0(&self) -> bool {
        *self == COMBINATIONEN_A::COMBINATIONEN_0
    }
    #[doc = "Checks if the value of the field is `COMBINATIONEN_1`"]
    #[inline(always)]
    pub fn is_combinationen_1(&self) -> bool {
        *self == COMBINATIONEN_A::COMBINATIONEN_1
    }
}
#[doc = "Write proxy for field `COMBINATIONEN`"]
pub struct COMBINATIONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINATIONEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMBINATIONEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn combinationen_0(self) -> &'a mut W {
        self.variant(COMBINATIONEN_A::COMBINATIONEN_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn combinationen_1(self) -> &'a mut W {
        self.variant(COMBINATIONEN_A::COMBINATIONEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "This bit is used to force SCK output free-running. For FPGA applications, external device may use SCK clock as reference clock to its internal PLL. If SCK free-running is enabled, data sampling with loopback clock from SCK pad is not supported (MCR0\\[RXCLKSRC\\]=2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKFREERUNEN_A {
    #[doc = "0: Disable."]
    SCKFREERUNEN_0,
    #[doc = "1: Enable."]
    SCKFREERUNEN_1,
}
impl From<SCKFREERUNEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCKFREERUNEN_A) -> Self {
        match variant {
            SCKFREERUNEN_A::SCKFREERUNEN_0 => false,
            SCKFREERUNEN_A::SCKFREERUNEN_1 => true,
        }
    }
}
#[doc = "Reader of field `SCKFREERUNEN`"]
pub type SCKFREERUNEN_R = crate::R<bool, SCKFREERUNEN_A>;
impl SCKFREERUNEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKFREERUNEN_A {
        match self.bits {
            false => SCKFREERUNEN_A::SCKFREERUNEN_0,
            true => SCKFREERUNEN_A::SCKFREERUNEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCKFREERUNEN_0`"]
    #[inline(always)]
    pub fn is_sckfreerunen_0(&self) -> bool {
        *self == SCKFREERUNEN_A::SCKFREERUNEN_0
    }
    #[doc = "Checks if the value of the field is `SCKFREERUNEN_1`"]
    #[inline(always)]
    pub fn is_sckfreerunen_1(&self) -> bool {
        *self == SCKFREERUNEN_A::SCKFREERUNEN_1
    }
}
#[doc = "Write proxy for field `SCKFREERUNEN`"]
pub struct SCKFREERUNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKFREERUNEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCKFREERUNEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn sckfreerunen_0(self) -> &'a mut W {
        self.variant(SCKFREERUNEN_A::SCKFREERUNEN_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn sckfreerunen_1(self) -> &'a mut W {
        self.variant(SCKFREERUNEN_A::SCKFREERUNEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `IPGRANTWAIT`"]
pub type IPGRANTWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IPGRANTWAIT`"]
pub struct IPGRANTWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> IPGRANTWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `AHBGRANTWAIT`"]
pub type AHBGRANTWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHBGRANTWAIT`"]
pub struct AHBGRANTWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBGRANTWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swreset(&self) -> SWRESET_R {
        SWRESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Sample Clock source selection for Flash Reading"]
    #[inline(always)]
    pub fn rxclksrc(&self) -> RXCLKSRC_R {
        RXCLKSRC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Enable AHB bus Read Access to IP RX FIFO."]
    #[inline(always)]
    pub fn ardfen(&self) -> ARDFEN_R {
        ARDFEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable AHB bus Write Access to IP TX FIFO."]
    #[inline(always)]
    pub fn atdfen(&self) -> ATDFEN_R {
        ATDFEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Half Speed Serial Flash access Enable."]
    #[inline(always)]
    pub fn hsen(&self) -> HSEN_R {
        HSEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Doze mode enable bit"]
    #[inline(always)]
    pub fn dozeen(&self) -> DOZEEN_R {
        DOZEEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is to support Flash Octal mode access by combining Port A and B Data pins (SIOA\\[3:0\\] and SIOB\\[3:0\\])."]
    #[inline(always)]
    pub fn combinationen(&self) -> COMBINATIONEN_R {
        COMBINATIONEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit is used to force SCK output free-running. For FPGA applications, external device may use SCK clock as reference clock to its internal PLL. If SCK free-running is enabled, data sampling with loopback clock from SCK pad is not supported (MCR0\\[RXCLKSRC\\]=2)."]
    #[inline(always)]
    pub fn sckfreerunen(&self) -> SCKFREERUNEN_R {
        SCKFREERUNEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Time out wait cycle for IP command grant."]
    #[inline(always)]
    pub fn ipgrantwait(&self) -> IPGRANTWAIT_R {
        IPGRANTWAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Timeout wait cycle for AHB command grant."]
    #[inline(always)]
    pub fn ahbgrantwait(&self) -> AHBGRANTWAIT_R {
        AHBGRANTWAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swreset(&mut self) -> SWRESET_W {
        SWRESET_W { w: self }
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
    #[doc = "Bits 4:5 - Sample Clock source selection for Flash Reading"]
    #[inline(always)]
    pub fn rxclksrc(&mut self) -> RXCLKSRC_W {
        RXCLKSRC_W { w: self }
    }
    #[doc = "Bit 6 - Enable AHB bus Read Access to IP RX FIFO."]
    #[inline(always)]
    pub fn ardfen(&mut self) -> ARDFEN_W {
        ARDFEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable AHB bus Write Access to IP TX FIFO."]
    #[inline(always)]
    pub fn atdfen(&mut self) -> ATDFEN_W {
        ATDFEN_W { w: self }
    }
    #[doc = "Bit 11 - Half Speed Serial Flash access Enable."]
    #[inline(always)]
    pub fn hsen(&mut self) -> HSEN_W {
        HSEN_W { w: self }
    }
    #[doc = "Bit 12 - Doze mode enable bit"]
    #[inline(always)]
    pub fn dozeen(&mut self) -> DOZEEN_W {
        DOZEEN_W { w: self }
    }
    #[doc = "Bit 13 - This bit is to support Flash Octal mode access by combining Port A and B Data pins (SIOA\\[3:0\\] and SIOB\\[3:0\\])."]
    #[inline(always)]
    pub fn combinationen(&mut self) -> COMBINATIONEN_W {
        COMBINATIONEN_W { w: self }
    }
    #[doc = "Bit 14 - This bit is used to force SCK output free-running. For FPGA applications, external device may use SCK clock as reference clock to its internal PLL. If SCK free-running is enabled, data sampling with loopback clock from SCK pad is not supported (MCR0\\[RXCLKSRC\\]=2)."]
    #[inline(always)]
    pub fn sckfreerunen(&mut self) -> SCKFREERUNEN_W {
        SCKFREERUNEN_W { w: self }
    }
    #[doc = "Bits 16:23 - Time out wait cycle for IP command grant."]
    #[inline(always)]
    pub fn ipgrantwait(&mut self) -> IPGRANTWAIT_W {
        IPGRANTWAIT_W { w: self }
    }
    #[doc = "Bits 24:31 - Timeout wait cycle for AHB command grant."]
    #[inline(always)]
    pub fn ahbgrantwait(&mut self) -> AHBGRANTWAIT_W {
        AHBGRANTWAIT_W { w: self }
    }
}
