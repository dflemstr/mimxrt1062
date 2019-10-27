#[doc = "Reader of register SW_MUX_CTL_PAD_GPIO_AD_B0_02"]
pub type R = crate::R<u32, super::SW_MUX_CTL_PAD_GPIO_AD_B0_02>;
#[doc = "Writer for register SW_MUX_CTL_PAD_GPIO_AD_B0_02"]
pub type W = crate::W<u32, super::SW_MUX_CTL_PAD_GPIO_AD_B0_02>;
#[doc = "Register SW_MUX_CTL_PAD_GPIO_AD_B0_02 `reset()`'s with value 0x05"]
impl crate::ResetValue for super::SW_MUX_CTL_PAD_GPIO_AD_B0_02 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05
    }
}
#[doc = "MUX Mode Select Field.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_MODE_A {
    #[doc = "0: Select mux mode: ALT0 mux port: FLEXCAN2_TX of instance: flexcan2"]
    ALT0,
    #[doc = "1: Select mux mode: ALT1 mux port: XBAR1_INOUT16 of instance: xbar1"]
    ALT1,
    #[doc = "2: Select mux mode: ALT2 mux port: LPUART6_TX of instance: lpuart6"]
    ALT2,
    #[doc = "3: Select mux mode: ALT3 mux port: USB_OTG1_PWR of instance: usb"]
    ALT3,
    #[doc = "4: Select mux mode: ALT4 mux port: FLEXPWM1_PWMX00 of instance: flexpwm1"]
    ALT4,
    #[doc = "5: Select mux mode: ALT5 mux port: GPIO1_IO02 of instance: gpio1"]
    ALT5,
    #[doc = "6: Select mux mode: ALT6 mux port: LPI2C1_HREQ of instance: lpi2c1"]
    ALT6,
    #[doc = "7: Select mux mode: ALT7 mux port: LPSPI3_SDI of instance: lpspi3"]
    ALT7,
}
impl From<MUX_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_MODE_A) -> Self {
        match variant {
            MUX_MODE_A::ALT0 => 0,
            MUX_MODE_A::ALT1 => 1,
            MUX_MODE_A::ALT2 => 2,
            MUX_MODE_A::ALT3 => 3,
            MUX_MODE_A::ALT4 => 4,
            MUX_MODE_A::ALT5 => 5,
            MUX_MODE_A::ALT6 => 6,
            MUX_MODE_A::ALT7 => 7,
        }
    }
}
#[doc = "Reader of field `MUX_MODE`"]
pub type MUX_MODE_R = crate::R<u8, MUX_MODE_A>;
impl MUX_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_MODE_A {
        match self.bits {
            0 => MUX_MODE_A::ALT0,
            1 => MUX_MODE_A::ALT1,
            2 => MUX_MODE_A::ALT2,
            3 => MUX_MODE_A::ALT3,
            4 => MUX_MODE_A::ALT4,
            5 => MUX_MODE_A::ALT5,
            6 => MUX_MODE_A::ALT6,
            7 => MUX_MODE_A::ALT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        *self == MUX_MODE_A::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == MUX_MODE_A::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == MUX_MODE_A::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == MUX_MODE_A::ALT3
    }
    #[doc = "Checks if the value of the field is `ALT4`"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == MUX_MODE_A::ALT4
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == MUX_MODE_A::ALT5
    }
    #[doc = "Checks if the value of the field is `ALT6`"]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        *self == MUX_MODE_A::ALT6
    }
    #[doc = "Checks if the value of the field is `ALT7`"]
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        *self == MUX_MODE_A::ALT7
    }
}
#[doc = "Write proxy for field `MUX_MODE`"]
pub struct MUX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Select mux mode: ALT0 mux port: FLEXCAN2_TX of instance: flexcan2"]
    #[inline(always)]
    pub fn alt0(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT0)
    }
    #[doc = "Select mux mode: ALT1 mux port: XBAR1_INOUT16 of instance: xbar1"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT1)
    }
    #[doc = "Select mux mode: ALT2 mux port: LPUART6_TX of instance: lpuart6"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT2)
    }
    #[doc = "Select mux mode: ALT3 mux port: USB_OTG1_PWR of instance: usb"]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT3)
    }
    #[doc = "Select mux mode: ALT4 mux port: FLEXPWM1_PWMX00 of instance: flexpwm1"]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT4)
    }
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO02 of instance: gpio1"]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT5)
    }
    #[doc = "Select mux mode: ALT6 mux port: LPI2C1_HREQ of instance: lpi2c1"]
    #[inline(always)]
    pub fn alt6(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT6)
    }
    #[doc = "Select mux mode: ALT7 mux port: LPSPI3_SDI of instance: lpspi3"]
    #[inline(always)]
    pub fn alt7(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Software Input On Field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SION_A {
    #[doc = "0: Input Path is determined by functionality"]
    DISABLED,
    #[doc = "1: Force input path of pad GPIO_AD_B0_02"]
    ENABLED,
}
impl From<SION_A> for bool {
    #[inline(always)]
    fn from(variant: SION_A) -> Self {
        match variant {
            SION_A::DISABLED => false,
            SION_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `SION`"]
pub type SION_R = crate::R<bool, SION_A>;
impl SION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SION_A {
        match self.bits {
            false => SION_A::DISABLED,
            true => SION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SION_A::ENABLED
    }
}
#[doc = "Write proxy for field `SION`"]
pub struct SION_W<'a> {
    w: &'a mut W,
}
impl<'a> SION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input Path is determined by functionality"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SION_A::DISABLED)
    }
    #[doc = "Force input path of pad GPIO_AD_B0_02"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SION_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - MUX Mode Select Field."]
    #[inline(always)]
    pub fn mux_mode(&self) -> MUX_MODE_R {
        MUX_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Software Input On Field."]
    #[inline(always)]
    pub fn sion(&self) -> SION_R {
        SION_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MUX Mode Select Field."]
    #[inline(always)]
    pub fn mux_mode(&mut self) -> MUX_MODE_W {
        MUX_MODE_W { w: self }
    }
    #[doc = "Bit 4 - Software Input On Field."]
    #[inline(always)]
    pub fn sion(&mut self) -> SION_W {
        SION_W { w: self }
    }
}
