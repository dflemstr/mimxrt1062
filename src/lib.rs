#![doc = "Peripheral access API for MIMXRT1062 microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0_DMA16();
    fn DMA1_DMA17();
    fn DMA2_DMA18();
    fn DMA3_DMA19();
    fn DMA4_DMA20();
    fn DMA5_DMA21();
    fn DMA6_DMA22();
    fn DMA7_DMA23();
    fn DMA8_DMA24();
    fn DMA9_DMA25();
    fn DMA10_DMA26();
    fn DMA11_DMA27();
    fn DMA12_DMA28();
    fn DMA13_DMA29();
    fn DMA14_DMA30();
    fn DMA15_DMA31();
    fn DMA_ERROR();
    fn CTI0_ERROR();
    fn CTI1_ERROR();
    fn CORE();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn LPUART5();
    fn LPUART6();
    fn LPUART7();
    fn LPUART8();
    fn LPI2C1();
    fn LPI2C2();
    fn LPI2C3();
    fn LPI2C4();
    fn LPSPI1();
    fn LPSPI2();
    fn LPSPI3();
    fn LPSPI4();
    fn CAN1();
    fn CAN2();
    fn FLEXRAM();
    fn KPP();
    fn TSC_DIG();
    fn GPR_IRQ();
    fn LCDIF();
    fn CSI();
    fn PXP();
    fn WDOG2();
    fn SNVS_HP_WRAPPER();
    fn SNVS_HP_WRAPPER_TZ();
    fn SNVS_LP_WRAPPER();
    fn CSU();
    fn DCP();
    fn DCP_VMI();
    fn RESERVED68();
    fn TRNG();
    fn SJC();
    fn BEE();
    fn SAI1();
    fn SAI2();
    fn SAI3_RX();
    fn SAI3_TX();
    fn SPDIF();
    fn PMU_EVENT();
    fn RESERVED78();
    fn TEMP_LOW_HIGH();
    fn TEMP_PANIC();
    fn USB_PHY1();
    fn USB_PHY2();
    fn ADC1();
    fn ADC2();
    fn DCDC();
    fn RESERVED86();
    fn RESERVED87();
    fn GPIO1_INT0();
    fn GPIO1_INT1();
    fn GPIO1_INT2();
    fn GPIO1_INT3();
    fn GPIO1_INT4();
    fn GPIO1_INT5();
    fn GPIO1_INT6();
    fn GPIO1_INT7();
    fn GPIO1_COMBINED_0_15();
    fn GPIO1_COMBINED_16_31();
    fn GPIO2_COMBINED_0_15();
    fn GPIO2_COMBINED_16_31();
    fn GPIO3_COMBINED_0_15();
    fn GPIO3_COMBINED_16_31();
    fn GPIO4_COMBINED_0_15();
    fn GPIO4_COMBINED_16_31();
    fn GPIO5_COMBINED_0_15();
    fn GPIO5_COMBINED_16_31();
    fn FLEXIO1();
    fn FLEXIO2();
    fn WDOG1();
    fn RTWDOG();
    fn EWM();
    fn CCM_1();
    fn CCM_2();
    fn GPC();
    fn SRC();
    fn RESERVED115();
    fn GPT1();
    fn GPT2();
    fn PWM1_0();
    fn PWM1_1();
    fn PWM1_2();
    fn PWM1_3();
    fn PWM1_FAULT();
    fn FLEXSPI2();
    fn FLEXSPI();
    fn SEMC();
    fn USDHC1();
    fn USDHC2();
    fn USB_OTG2();
    fn USB_OTG1();
    fn ENET();
    fn ENET_1588_TIMER();
    fn XBAR1_IRQ_0_1();
    fn XBAR1_IRQ_2_3();
    fn ADC_ETC_IRQ0();
    fn ADC_ETC_IRQ1();
    fn ADC_ETC_IRQ2();
    fn ADC_ETC_ERROR_IRQ();
    fn PIT();
    fn ACMP1();
    fn ACMP2();
    fn ACMP3();
    fn ACMP4();
    fn RESERVED143();
    fn RESERVED144();
    fn ENC1();
    fn ENC2();
    fn ENC3();
    fn ENC4();
    fn TMR1();
    fn TMR2();
    fn TMR3();
    fn TMR4();
    fn PWM2_0();
    fn PWM2_1();
    fn PWM2_2();
    fn PWM2_3();
    fn PWM2_FAULT();
    fn PWM3_0();
    fn PWM3_1();
    fn PWM3_2();
    fn PWM3_3();
    fn PWM3_FAULT();
    fn PWM4_0();
    fn PWM4_1();
    fn PWM4_2();
    fn PWM4_3();
    fn PWM4_FAULT();
    fn ENET2();
    fn ENET2_1588_TIMER();
    fn CAN3();
    fn RESERVED171();
    fn FLEXIO3();
    fn GPIO6_7_8_9();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 158] = [
    Vector {
        _handler: DMA0_DMA16,
    },
    Vector {
        _handler: DMA1_DMA17,
    },
    Vector {
        _handler: DMA2_DMA18,
    },
    Vector {
        _handler: DMA3_DMA19,
    },
    Vector {
        _handler: DMA4_DMA20,
    },
    Vector {
        _handler: DMA5_DMA21,
    },
    Vector {
        _handler: DMA6_DMA22,
    },
    Vector {
        _handler: DMA7_DMA23,
    },
    Vector {
        _handler: DMA8_DMA24,
    },
    Vector {
        _handler: DMA9_DMA25,
    },
    Vector {
        _handler: DMA10_DMA26,
    },
    Vector {
        _handler: DMA11_DMA27,
    },
    Vector {
        _handler: DMA12_DMA28,
    },
    Vector {
        _handler: DMA13_DMA29,
    },
    Vector {
        _handler: DMA14_DMA30,
    },
    Vector {
        _handler: DMA15_DMA31,
    },
    Vector {
        _handler: DMA_ERROR,
    },
    Vector {
        _handler: CTI0_ERROR,
    },
    Vector {
        _handler: CTI1_ERROR,
    },
    Vector { _handler: CORE },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _handler: LPUART5 },
    Vector { _handler: LPUART6 },
    Vector { _handler: LPUART7 },
    Vector { _handler: LPUART8 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: LPI2C3 },
    Vector { _handler: LPI2C4 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: LPSPI3 },
    Vector { _handler: LPSPI4 },
    Vector { _handler: CAN1 },
    Vector { _handler: CAN2 },
    Vector { _handler: FLEXRAM },
    Vector { _handler: KPP },
    Vector { _handler: TSC_DIG },
    Vector { _handler: GPR_IRQ },
    Vector { _handler: LCDIF },
    Vector { _handler: CSI },
    Vector { _handler: PXP },
    Vector { _handler: WDOG2 },
    Vector {
        _handler: SNVS_HP_WRAPPER,
    },
    Vector {
        _handler: SNVS_HP_WRAPPER_TZ,
    },
    Vector {
        _handler: SNVS_LP_WRAPPER,
    },
    Vector { _handler: CSU },
    Vector { _handler: DCP },
    Vector { _handler: DCP_VMI },
    Vector {
        _handler: RESERVED68,
    },
    Vector { _handler: TRNG },
    Vector { _handler: SJC },
    Vector { _handler: BEE },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: SAI3_RX },
    Vector { _handler: SAI3_TX },
    Vector { _handler: SPDIF },
    Vector {
        _handler: PMU_EVENT,
    },
    Vector {
        _handler: RESERVED78,
    },
    Vector {
        _handler: TEMP_LOW_HIGH,
    },
    Vector {
        _handler: TEMP_PANIC,
    },
    Vector { _handler: USB_PHY1 },
    Vector { _handler: USB_PHY2 },
    Vector { _handler: ADC1 },
    Vector { _handler: ADC2 },
    Vector { _handler: DCDC },
    Vector {
        _handler: RESERVED86,
    },
    Vector {
        _handler: RESERVED87,
    },
    Vector {
        _handler: GPIO1_INT0,
    },
    Vector {
        _handler: GPIO1_INT1,
    },
    Vector {
        _handler: GPIO1_INT2,
    },
    Vector {
        _handler: GPIO1_INT3,
    },
    Vector {
        _handler: GPIO1_INT4,
    },
    Vector {
        _handler: GPIO1_INT5,
    },
    Vector {
        _handler: GPIO1_INT6,
    },
    Vector {
        _handler: GPIO1_INT7,
    },
    Vector {
        _handler: GPIO1_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO1_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO2_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO2_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO3_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO3_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO4_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO4_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO5_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO5_COMBINED_16_31,
    },
    Vector { _handler: FLEXIO1 },
    Vector { _handler: FLEXIO2 },
    Vector { _handler: WDOG1 },
    Vector { _handler: RTWDOG },
    Vector { _handler: EWM },
    Vector { _handler: CCM_1 },
    Vector { _handler: CCM_2 },
    Vector { _handler: GPC },
    Vector { _handler: SRC },
    Vector {
        _handler: RESERVED115,
    },
    Vector { _handler: GPT1 },
    Vector { _handler: GPT2 },
    Vector { _handler: PWM1_0 },
    Vector { _handler: PWM1_1 },
    Vector { _handler: PWM1_2 },
    Vector { _handler: PWM1_3 },
    Vector {
        _handler: PWM1_FAULT,
    },
    Vector { _handler: FLEXSPI2 },
    Vector { _handler: FLEXSPI },
    Vector { _handler: SEMC },
    Vector { _handler: USDHC1 },
    Vector { _handler: USDHC2 },
    Vector { _handler: USB_OTG2 },
    Vector { _handler: USB_OTG1 },
    Vector { _handler: ENET },
    Vector {
        _handler: ENET_1588_TIMER,
    },
    Vector {
        _handler: XBAR1_IRQ_0_1,
    },
    Vector {
        _handler: XBAR1_IRQ_2_3,
    },
    Vector {
        _handler: ADC_ETC_IRQ0,
    },
    Vector {
        _handler: ADC_ETC_IRQ1,
    },
    Vector {
        _handler: ADC_ETC_IRQ2,
    },
    Vector {
        _handler: ADC_ETC_ERROR_IRQ,
    },
    Vector { _handler: PIT },
    Vector { _handler: ACMP1 },
    Vector { _handler: ACMP2 },
    Vector { _handler: ACMP3 },
    Vector { _handler: ACMP4 },
    Vector {
        _handler: RESERVED143,
    },
    Vector {
        _handler: RESERVED144,
    },
    Vector { _handler: ENC1 },
    Vector { _handler: ENC2 },
    Vector { _handler: ENC3 },
    Vector { _handler: ENC4 },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR2 },
    Vector { _handler: TMR3 },
    Vector { _handler: TMR4 },
    Vector { _handler: PWM2_0 },
    Vector { _handler: PWM2_1 },
    Vector { _handler: PWM2_2 },
    Vector { _handler: PWM2_3 },
    Vector {
        _handler: PWM2_FAULT,
    },
    Vector { _handler: PWM3_0 },
    Vector { _handler: PWM3_1 },
    Vector { _handler: PWM3_2 },
    Vector { _handler: PWM3_3 },
    Vector {
        _handler: PWM3_FAULT,
    },
    Vector { _handler: PWM4_0 },
    Vector { _handler: PWM4_1 },
    Vector { _handler: PWM4_2 },
    Vector { _handler: PWM4_3 },
    Vector {
        _handler: PWM4_FAULT,
    },
    Vector { _handler: ENET2 },
    Vector {
        _handler: ENET2_1588_TIMER,
    },
    Vector { _handler: CAN3 },
    Vector {
        _handler: RESERVED171,
    },
    Vector { _handler: FLEXIO3 },
    Vector {
        _handler: GPIO6_7_8_9,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - DMA0_DMA16"]
    DMA0_DMA16,
    #[doc = "1 - DMA1_DMA17"]
    DMA1_DMA17,
    #[doc = "2 - DMA2_DMA18"]
    DMA2_DMA18,
    #[doc = "3 - DMA3_DMA19"]
    DMA3_DMA19,
    #[doc = "4 - DMA4_DMA20"]
    DMA4_DMA20,
    #[doc = "5 - DMA5_DMA21"]
    DMA5_DMA21,
    #[doc = "6 - DMA6_DMA22"]
    DMA6_DMA22,
    #[doc = "7 - DMA7_DMA23"]
    DMA7_DMA23,
    #[doc = "8 - DMA8_DMA24"]
    DMA8_DMA24,
    #[doc = "9 - DMA9_DMA25"]
    DMA9_DMA25,
    #[doc = "10 - DMA10_DMA26"]
    DMA10_DMA26,
    #[doc = "11 - DMA11_DMA27"]
    DMA11_DMA27,
    #[doc = "12 - DMA12_DMA28"]
    DMA12_DMA28,
    #[doc = "13 - DMA13_DMA29"]
    DMA13_DMA29,
    #[doc = "14 - DMA14_DMA30"]
    DMA14_DMA30,
    #[doc = "15 - DMA15_DMA31"]
    DMA15_DMA31,
    #[doc = "16 - DMA_ERROR"]
    DMA_ERROR,
    #[doc = "17 - CTI0_ERROR"]
    CTI0_ERROR,
    #[doc = "18 - CTI1_ERROR"]
    CTI1_ERROR,
    #[doc = "19 - CORE"]
    CORE,
    #[doc = "20 - LPUART1"]
    LPUART1,
    #[doc = "21 - LPUART2"]
    LPUART2,
    #[doc = "22 - LPUART3"]
    LPUART3,
    #[doc = "23 - LPUART4"]
    LPUART4,
    #[doc = "24 - LPUART5"]
    LPUART5,
    #[doc = "25 - LPUART6"]
    LPUART6,
    #[doc = "26 - LPUART7"]
    LPUART7,
    #[doc = "27 - LPUART8"]
    LPUART8,
    #[doc = "28 - LPI2C1"]
    LPI2C1,
    #[doc = "29 - LPI2C2"]
    LPI2C2,
    #[doc = "30 - LPI2C3"]
    LPI2C3,
    #[doc = "31 - LPI2C4"]
    LPI2C4,
    #[doc = "32 - LPSPI1"]
    LPSPI1,
    #[doc = "33 - LPSPI2"]
    LPSPI2,
    #[doc = "34 - LPSPI3"]
    LPSPI3,
    #[doc = "35 - LPSPI4"]
    LPSPI4,
    #[doc = "36 - CAN1"]
    CAN1,
    #[doc = "37 - CAN2"]
    CAN2,
    #[doc = "38 - FLEXRAM"]
    FLEXRAM,
    #[doc = "39 - KPP"]
    KPP,
    #[doc = "40 - TSC_DIG"]
    TSC_DIG,
    #[doc = "41 - GPR_IRQ"]
    GPR_IRQ,
    #[doc = "42 - LCDIF"]
    LCDIF,
    #[doc = "43 - CSI"]
    CSI,
    #[doc = "44 - PXP"]
    PXP,
    #[doc = "45 - WDOG2"]
    WDOG2,
    #[doc = "46 - SNVS_HP_WRAPPER"]
    SNVS_HP_WRAPPER,
    #[doc = "47 - SNVS_HP_WRAPPER_TZ"]
    SNVS_HP_WRAPPER_TZ,
    #[doc = "48 - SNVS_LP_WRAPPER"]
    SNVS_LP_WRAPPER,
    #[doc = "49 - CSU"]
    CSU,
    #[doc = "50 - DCP"]
    DCP,
    #[doc = "51 - DCP_VMI"]
    DCP_VMI,
    #[doc = "52 - Reserved68"]
    RESERVED68,
    #[doc = "53 - TRNG"]
    TRNG,
    #[doc = "54 - SJC"]
    SJC,
    #[doc = "55 - BEE"]
    BEE,
    #[doc = "56 - SAI1"]
    SAI1,
    #[doc = "57 - SAI2"]
    SAI2,
    #[doc = "58 - SAI3_RX"]
    SAI3_RX,
    #[doc = "59 - SAI3_TX"]
    SAI3_TX,
    #[doc = "60 - SPDIF"]
    SPDIF,
    #[doc = "61 - PMU_EVENT"]
    PMU_EVENT,
    #[doc = "62 - Reserved78"]
    RESERVED78,
    #[doc = "63 - TEMP_LOW_HIGH"]
    TEMP_LOW_HIGH,
    #[doc = "64 - TEMP_PANIC"]
    TEMP_PANIC,
    #[doc = "65 - USB_PHY1"]
    USB_PHY1,
    #[doc = "66 - USB_PHY2"]
    USB_PHY2,
    #[doc = "67 - ADC1"]
    ADC1,
    #[doc = "68 - ADC2"]
    ADC2,
    #[doc = "69 - DCDC"]
    DCDC,
    #[doc = "70 - Reserved86"]
    RESERVED86,
    #[doc = "71 - Reserved87"]
    RESERVED87,
    #[doc = "72 - GPIO1_INT0"]
    GPIO1_INT0,
    #[doc = "73 - GPIO1_INT1"]
    GPIO1_INT1,
    #[doc = "74 - GPIO1_INT2"]
    GPIO1_INT2,
    #[doc = "75 - GPIO1_INT3"]
    GPIO1_INT3,
    #[doc = "76 - GPIO1_INT4"]
    GPIO1_INT4,
    #[doc = "77 - GPIO1_INT5"]
    GPIO1_INT5,
    #[doc = "78 - GPIO1_INT6"]
    GPIO1_INT6,
    #[doc = "79 - GPIO1_INT7"]
    GPIO1_INT7,
    #[doc = "80 - GPIO1_Combined_0_15"]
    GPIO1_COMBINED_0_15,
    #[doc = "81 - GPIO1_Combined_16_31"]
    GPIO1_COMBINED_16_31,
    #[doc = "82 - GPIO2_Combined_0_15"]
    GPIO2_COMBINED_0_15,
    #[doc = "83 - GPIO2_Combined_16_31"]
    GPIO2_COMBINED_16_31,
    #[doc = "84 - GPIO3_Combined_0_15"]
    GPIO3_COMBINED_0_15,
    #[doc = "85 - GPIO3_Combined_16_31"]
    GPIO3_COMBINED_16_31,
    #[doc = "86 - GPIO4_Combined_0_15"]
    GPIO4_COMBINED_0_15,
    #[doc = "87 - GPIO4_Combined_16_31"]
    GPIO4_COMBINED_16_31,
    #[doc = "88 - GPIO5_Combined_0_15"]
    GPIO5_COMBINED_0_15,
    #[doc = "89 - GPIO5_Combined_16_31"]
    GPIO5_COMBINED_16_31,
    #[doc = "90 - FLEXIO1"]
    FLEXIO1,
    #[doc = "91 - FLEXIO2"]
    FLEXIO2,
    #[doc = "92 - WDOG1"]
    WDOG1,
    #[doc = "93 - RTWDOG"]
    RTWDOG,
    #[doc = "94 - EWM"]
    EWM,
    #[doc = "95 - CCM_1"]
    CCM_1,
    #[doc = "96 - CCM_2"]
    CCM_2,
    #[doc = "97 - GPC"]
    GPC,
    #[doc = "98 - SRC"]
    SRC,
    #[doc = "99 - Reserved115"]
    RESERVED115,
    #[doc = "100 - GPT1"]
    GPT1,
    #[doc = "101 - GPT2"]
    GPT2,
    #[doc = "102 - PWM1_0"]
    PWM1_0,
    #[doc = "103 - PWM1_1"]
    PWM1_1,
    #[doc = "104 - PWM1_2"]
    PWM1_2,
    #[doc = "105 - PWM1_3"]
    PWM1_3,
    #[doc = "106 - PWM1_FAULT"]
    PWM1_FAULT,
    #[doc = "107 - FLEXSPI2"]
    FLEXSPI2,
    #[doc = "108 - FLEXSPI"]
    FLEXSPI,
    #[doc = "109 - SEMC"]
    SEMC,
    #[doc = "110 - USDHC1"]
    USDHC1,
    #[doc = "111 - USDHC2"]
    USDHC2,
    #[doc = "112 - USB_OTG2"]
    USB_OTG2,
    #[doc = "113 - USB_OTG1"]
    USB_OTG1,
    #[doc = "114 - ENET"]
    ENET,
    #[doc = "115 - ENET_1588_Timer"]
    ENET_1588_TIMER,
    #[doc = "116 - XBAR1_IRQ_0_1"]
    XBAR1_IRQ_0_1,
    #[doc = "117 - XBAR1_IRQ_2_3"]
    XBAR1_IRQ_2_3,
    #[doc = "118 - ADC_ETC_IRQ0"]
    ADC_ETC_IRQ0,
    #[doc = "119 - ADC_ETC_IRQ1"]
    ADC_ETC_IRQ1,
    #[doc = "120 - ADC_ETC_IRQ2"]
    ADC_ETC_IRQ2,
    #[doc = "121 - ADC_ETC_ERROR_IRQ"]
    ADC_ETC_ERROR_IRQ,
    #[doc = "122 - PIT"]
    PIT,
    #[doc = "123 - ACMP1"]
    ACMP1,
    #[doc = "124 - ACMP2"]
    ACMP2,
    #[doc = "125 - ACMP3"]
    ACMP3,
    #[doc = "126 - ACMP4"]
    ACMP4,
    #[doc = "127 - Reserved143"]
    RESERVED143,
    #[doc = "128 - Reserved144"]
    RESERVED144,
    #[doc = "129 - ENC1"]
    ENC1,
    #[doc = "130 - ENC2"]
    ENC2,
    #[doc = "131 - ENC3"]
    ENC3,
    #[doc = "132 - ENC4"]
    ENC4,
    #[doc = "133 - TMR1"]
    TMR1,
    #[doc = "134 - TMR2"]
    TMR2,
    #[doc = "135 - TMR3"]
    TMR3,
    #[doc = "136 - TMR4"]
    TMR4,
    #[doc = "137 - PWM2_0"]
    PWM2_0,
    #[doc = "138 - PWM2_1"]
    PWM2_1,
    #[doc = "139 - PWM2_2"]
    PWM2_2,
    #[doc = "140 - PWM2_3"]
    PWM2_3,
    #[doc = "141 - PWM2_FAULT"]
    PWM2_FAULT,
    #[doc = "142 - PWM3_0"]
    PWM3_0,
    #[doc = "143 - PWM3_1"]
    PWM3_1,
    #[doc = "144 - PWM3_2"]
    PWM3_2,
    #[doc = "145 - PWM3_3"]
    PWM3_3,
    #[doc = "146 - PWM3_FAULT"]
    PWM3_FAULT,
    #[doc = "147 - PWM4_0"]
    PWM4_0,
    #[doc = "148 - PWM4_1"]
    PWM4_1,
    #[doc = "149 - PWM4_2"]
    PWM4_2,
    #[doc = "150 - PWM4_3"]
    PWM4_3,
    #[doc = "151 - PWM4_FAULT"]
    PWM4_FAULT,
    #[doc = "152 - ENET2"]
    ENET2,
    #[doc = "153 - ENET2_1588_Timer"]
    ENET2_1588_TIMER,
    #[doc = "154 - CAN3"]
    CAN3,
    #[doc = "155 - Reserved171"]
    RESERVED171,
    #[doc = "156 - FLEXIO3"]
    FLEXIO3,
    #[doc = "157 - GPIO6_7_8_9"]
    GPIO6_7_8_9,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DMA0_DMA16 => 0,
            Interrupt::DMA1_DMA17 => 1,
            Interrupt::DMA2_DMA18 => 2,
            Interrupt::DMA3_DMA19 => 3,
            Interrupt::DMA4_DMA20 => 4,
            Interrupt::DMA5_DMA21 => 5,
            Interrupt::DMA6_DMA22 => 6,
            Interrupt::DMA7_DMA23 => 7,
            Interrupt::DMA8_DMA24 => 8,
            Interrupt::DMA9_DMA25 => 9,
            Interrupt::DMA10_DMA26 => 10,
            Interrupt::DMA11_DMA27 => 11,
            Interrupt::DMA12_DMA28 => 12,
            Interrupt::DMA13_DMA29 => 13,
            Interrupt::DMA14_DMA30 => 14,
            Interrupt::DMA15_DMA31 => 15,
            Interrupt::DMA_ERROR => 16,
            Interrupt::CTI0_ERROR => 17,
            Interrupt::CTI1_ERROR => 18,
            Interrupt::CORE => 19,
            Interrupt::LPUART1 => 20,
            Interrupt::LPUART2 => 21,
            Interrupt::LPUART3 => 22,
            Interrupt::LPUART4 => 23,
            Interrupt::LPUART5 => 24,
            Interrupt::LPUART6 => 25,
            Interrupt::LPUART7 => 26,
            Interrupt::LPUART8 => 27,
            Interrupt::LPI2C1 => 28,
            Interrupt::LPI2C2 => 29,
            Interrupt::LPI2C3 => 30,
            Interrupt::LPI2C4 => 31,
            Interrupt::LPSPI1 => 32,
            Interrupt::LPSPI2 => 33,
            Interrupt::LPSPI3 => 34,
            Interrupt::LPSPI4 => 35,
            Interrupt::CAN1 => 36,
            Interrupt::CAN2 => 37,
            Interrupt::FLEXRAM => 38,
            Interrupt::KPP => 39,
            Interrupt::TSC_DIG => 40,
            Interrupt::GPR_IRQ => 41,
            Interrupt::LCDIF => 42,
            Interrupt::CSI => 43,
            Interrupt::PXP => 44,
            Interrupt::WDOG2 => 45,
            Interrupt::SNVS_HP_WRAPPER => 46,
            Interrupt::SNVS_HP_WRAPPER_TZ => 47,
            Interrupt::SNVS_LP_WRAPPER => 48,
            Interrupt::CSU => 49,
            Interrupt::DCP => 50,
            Interrupt::DCP_VMI => 51,
            Interrupt::RESERVED68 => 52,
            Interrupt::TRNG => 53,
            Interrupt::SJC => 54,
            Interrupt::BEE => 55,
            Interrupt::SAI1 => 56,
            Interrupt::SAI2 => 57,
            Interrupt::SAI3_RX => 58,
            Interrupt::SAI3_TX => 59,
            Interrupt::SPDIF => 60,
            Interrupt::PMU_EVENT => 61,
            Interrupt::RESERVED78 => 62,
            Interrupt::TEMP_LOW_HIGH => 63,
            Interrupt::TEMP_PANIC => 64,
            Interrupt::USB_PHY1 => 65,
            Interrupt::USB_PHY2 => 66,
            Interrupt::ADC1 => 67,
            Interrupt::ADC2 => 68,
            Interrupt::DCDC => 69,
            Interrupt::RESERVED86 => 70,
            Interrupt::RESERVED87 => 71,
            Interrupt::GPIO1_INT0 => 72,
            Interrupt::GPIO1_INT1 => 73,
            Interrupt::GPIO1_INT2 => 74,
            Interrupt::GPIO1_INT3 => 75,
            Interrupt::GPIO1_INT4 => 76,
            Interrupt::GPIO1_INT5 => 77,
            Interrupt::GPIO1_INT6 => 78,
            Interrupt::GPIO1_INT7 => 79,
            Interrupt::GPIO1_COMBINED_0_15 => 80,
            Interrupt::GPIO1_COMBINED_16_31 => 81,
            Interrupt::GPIO2_COMBINED_0_15 => 82,
            Interrupt::GPIO2_COMBINED_16_31 => 83,
            Interrupt::GPIO3_COMBINED_0_15 => 84,
            Interrupt::GPIO3_COMBINED_16_31 => 85,
            Interrupt::GPIO4_COMBINED_0_15 => 86,
            Interrupt::GPIO4_COMBINED_16_31 => 87,
            Interrupt::GPIO5_COMBINED_0_15 => 88,
            Interrupt::GPIO5_COMBINED_16_31 => 89,
            Interrupt::FLEXIO1 => 90,
            Interrupt::FLEXIO2 => 91,
            Interrupt::WDOG1 => 92,
            Interrupt::RTWDOG => 93,
            Interrupt::EWM => 94,
            Interrupt::CCM_1 => 95,
            Interrupt::CCM_2 => 96,
            Interrupt::GPC => 97,
            Interrupt::SRC => 98,
            Interrupt::RESERVED115 => 99,
            Interrupt::GPT1 => 100,
            Interrupt::GPT2 => 101,
            Interrupt::PWM1_0 => 102,
            Interrupt::PWM1_1 => 103,
            Interrupt::PWM1_2 => 104,
            Interrupt::PWM1_3 => 105,
            Interrupt::PWM1_FAULT => 106,
            Interrupt::FLEXSPI2 => 107,
            Interrupt::FLEXSPI => 108,
            Interrupt::SEMC => 109,
            Interrupt::USDHC1 => 110,
            Interrupt::USDHC2 => 111,
            Interrupt::USB_OTG2 => 112,
            Interrupt::USB_OTG1 => 113,
            Interrupt::ENET => 114,
            Interrupt::ENET_1588_TIMER => 115,
            Interrupt::XBAR1_IRQ_0_1 => 116,
            Interrupt::XBAR1_IRQ_2_3 => 117,
            Interrupt::ADC_ETC_IRQ0 => 118,
            Interrupt::ADC_ETC_IRQ1 => 119,
            Interrupt::ADC_ETC_IRQ2 => 120,
            Interrupt::ADC_ETC_ERROR_IRQ => 121,
            Interrupt::PIT => 122,
            Interrupt::ACMP1 => 123,
            Interrupt::ACMP2 => 124,
            Interrupt::ACMP3 => 125,
            Interrupt::ACMP4 => 126,
            Interrupt::RESERVED143 => 127,
            Interrupt::RESERVED144 => 128,
            Interrupt::ENC1 => 129,
            Interrupt::ENC2 => 130,
            Interrupt::ENC3 => 131,
            Interrupt::ENC4 => 132,
            Interrupt::TMR1 => 133,
            Interrupt::TMR2 => 134,
            Interrupt::TMR3 => 135,
            Interrupt::TMR4 => 136,
            Interrupt::PWM2_0 => 137,
            Interrupt::PWM2_1 => 138,
            Interrupt::PWM2_2 => 139,
            Interrupt::PWM2_3 => 140,
            Interrupt::PWM2_FAULT => 141,
            Interrupt::PWM3_0 => 142,
            Interrupt::PWM3_1 => 143,
            Interrupt::PWM3_2 => 144,
            Interrupt::PWM3_3 => 145,
            Interrupt::PWM3_FAULT => 146,
            Interrupt::PWM4_0 => 147,
            Interrupt::PWM4_1 => 148,
            Interrupt::PWM4_2 => 149,
            Interrupt::PWM4_3 => 150,
            Interrupt::PWM4_FAULT => 151,
            Interrupt::ENET2 => 152,
            Interrupt::ENET2_1588_TIMER => 153,
            Interrupt::CAN3 => 154,
            Interrupt::RESERVED171 => 155,
            Interrupt::FLEXIO3 => 156,
            Interrupt::GPIO6_7_8_9 => 157,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "AIPSTZ Control Registers"]
pub struct AIPSTZ1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPSTZ1 {}
impl AIPSTZ1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aipstz1::RegisterBlock {
        0x4007_c000 as *const _
    }
}
impl Deref for AIPSTZ1 {
    type Target = aipstz1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AIPSTZ1::ptr() }
    }
}
#[doc = "AIPSTZ Control Registers"]
pub mod aipstz1;
#[doc = "AIPSTZ Control Registers"]
pub struct AIPSTZ2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPSTZ2 {}
impl AIPSTZ2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aipstz1::RegisterBlock {
        0x4017_c000 as *const _
    }
}
impl Deref for AIPSTZ2 {
    type Target = aipstz1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AIPSTZ2::ptr() }
    }
}
#[doc = "AIPSTZ Control Registers"]
pub struct AIPSTZ3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPSTZ3 {}
impl AIPSTZ3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aipstz1::RegisterBlock {
        0x4027_c000 as *const _
    }
}
impl Deref for AIPSTZ3 {
    type Target = aipstz1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AIPSTZ3::ptr() }
    }
}
#[doc = "AIPSTZ Control Registers"]
pub struct AIPSTZ4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPSTZ4 {}
impl AIPSTZ4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aipstz1::RegisterBlock {
        0x4037_c000 as *const _
    }
}
impl Deref for AIPSTZ4 {
    type Target = aipstz1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AIPSTZ4::ptr() }
    }
}
#[doc = "DCDC"]
pub struct DCDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCDC {}
impl DCDC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dcdc::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for DCDC {
    type Target = dcdc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCDC::ptr() }
    }
}
#[doc = "DCDC"]
pub mod dcdc;
#[doc = "PIT"]
pub struct PIT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIT {}
impl PIT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pit::RegisterBlock {
        0x4008_4000 as *const _
    }
}
impl Deref for PIT {
    type Target = pit::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIT::ptr() }
    }
}
#[doc = "PIT"]
pub mod pit;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP1 {}
impl CMP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp1::RegisterBlock {
        0x4009_4000 as *const _
    }
}
impl Deref for CMP1 {
    type Target = cmp1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP1::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp1;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP2 {}
impl CMP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp1::RegisterBlock {
        0x4009_4008 as *const _
    }
}
impl Deref for CMP2 {
    type Target = cmp1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP2::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP3 {}
impl CMP3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp1::RegisterBlock {
        0x4009_4010 as *const _
    }
}
impl Deref for CMP3 {
    type Target = cmp1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP3::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP4 {}
impl CMP4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp1::RegisterBlock {
        0x4009_4018 as *const _
    }
}
impl Deref for CMP4 {
    type Target = cmp1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP4::ptr() }
    }
}
#[doc = "IOMUXC"]
pub struct IOMUXC_SNVS_GPR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMUXC_SNVS_GPR {}
impl IOMUXC_SNVS_GPR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iomuxc_snvs_gpr::RegisterBlock {
        0x400a_4000 as *const _
    }
}
impl Deref for IOMUXC_SNVS_GPR {
    type Target = iomuxc_snvs_gpr::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOMUXC_SNVS_GPR::ptr() }
    }
}
#[doc = "IOMUXC"]
pub mod iomuxc_snvs_gpr;
#[doc = "IOMUXC_SNVS"]
pub struct IOMUXC_SNVS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMUXC_SNVS {}
impl IOMUXC_SNVS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iomuxc_snvs::RegisterBlock {
        0x400a_8000 as *const _
    }
}
impl Deref for IOMUXC_SNVS {
    type Target = iomuxc_snvs::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOMUXC_SNVS::ptr() }
    }
}
#[doc = "IOMUXC_SNVS"]
pub mod iomuxc_snvs;
#[doc = "IOMUXC_GPR"]
pub struct IOMUXC_GPR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMUXC_GPR {}
impl IOMUXC_GPR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iomuxc_gpr::RegisterBlock {
        0x400a_c000 as *const _
    }
}
impl Deref for IOMUXC_GPR {
    type Target = iomuxc_gpr::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOMUXC_GPR::ptr() }
    }
}
#[doc = "IOMUXC_GPR"]
pub mod iomuxc_gpr;
#[doc = "FLEXRAM"]
pub struct FLEXRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXRAM {}
impl FLEXRAM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexram::RegisterBlock {
        0x400b_0000 as *const _
    }
}
impl Deref for FLEXRAM {
    type Target = flexram::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXRAM::ptr() }
    }
}
#[doc = "FLEXRAM"]
pub mod flexram;
#[doc = "EWM"]
pub struct EWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EWM {}
impl EWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ewm::RegisterBlock {
        0x400b_4000 as *const _
    }
}
impl Deref for EWM {
    type Target = ewm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EWM::ptr() }
    }
}
#[doc = "EWM"]
pub mod ewm;
#[doc = "WDOG"]
pub struct WDOG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG1 {}
impl WDOG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog1::RegisterBlock {
        0x400b_8000 as *const _
    }
}
impl Deref for WDOG1 {
    type Target = wdog1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDOG1::ptr() }
    }
}
#[doc = "WDOG"]
pub mod wdog1;
#[doc = "WDOG"]
pub struct WDOG2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG2 {}
impl WDOG2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog1::RegisterBlock {
        0x400d_0000 as *const _
    }
}
impl Deref for WDOG2 {
    type Target = wdog1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDOG2::ptr() }
    }
}
#[doc = "WDOG"]
pub struct RTWDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTWDOG {}
impl RTWDOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtwdog::RegisterBlock {
        0x400b_c000 as *const _
    }
}
impl Deref for RTWDOG {
    type Target = rtwdog::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTWDOG::ptr() }
    }
}
#[doc = "WDOG"]
pub mod rtwdog;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x400c_4000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc1;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x400c_8000 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "TRNG"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x400c_c000 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "TRNG"]
pub mod trng;
#[doc = "SNVS"]
pub struct SNVS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SNVS {}
impl SNVS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const snvs::RegisterBlock {
        0x400d_4000 as *const _
    }
}
impl Deref for SNVS {
    type Target = snvs::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SNVS::ptr() }
    }
}
#[doc = "SNVS"]
pub mod snvs;
#[doc = "CCM_ANALOG"]
pub struct CCM_ANALOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM_ANALOG {}
impl CCM_ANALOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccm_analog::RegisterBlock {
        0x400d_8000 as *const _
    }
}
impl Deref for CCM_ANALOG {
    type Target = ccm_analog::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCM_ANALOG::ptr() }
    }
}
#[doc = "CCM_ANALOG"]
pub mod ccm_analog;
#[doc = "PMU"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu::RegisterBlock {
        0x400d_8000 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "PMU"]
pub mod pmu;
#[doc = "Temperature Monitor"]
pub struct TEMPMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TEMPMON {}
impl TEMPMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tempmon::RegisterBlock {
        0x400d_8000 as *const _
    }
}
impl Deref for TEMPMON {
    type Target = tempmon::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TEMPMON::ptr() }
    }
}
#[doc = "Temperature Monitor"]
pub mod tempmon;
#[doc = "USB Analog"]
pub struct USB_ANALOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_ANALOG {}
impl USB_ANALOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb_analog::RegisterBlock {
        0x400d_8000 as *const _
    }
}
impl Deref for USB_ANALOG {
    type Target = usb_analog::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB_ANALOG::ptr() }
    }
}
#[doc = "USB Analog"]
pub mod usb_analog;
#[doc = "XTALOSC24M"]
pub struct XTALOSC24M {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XTALOSC24M {}
impl XTALOSC24M {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xtalosc24m::RegisterBlock {
        0x400d_8000 as *const _
    }
}
impl Deref for XTALOSC24M {
    type Target = xtalosc24m::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*XTALOSC24M::ptr() }
    }
}
#[doc = "XTALOSC24M"]
pub mod xtalosc24m;
#[doc = "USBPHY Register Reference Index"]
pub struct USBPHY1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBPHY1 {}
impl USBPHY1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbphy1::RegisterBlock {
        0x400d_9000 as *const _
    }
}
impl Deref for USBPHY1 {
    type Target = usbphy1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBPHY1::ptr() }
    }
}
#[doc = "USBPHY Register Reference Index"]
pub mod usbphy1;
#[doc = "USBPHY Register Reference Index"]
pub struct USBPHY2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBPHY2 {}
impl USBPHY2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbphy1::RegisterBlock {
        0x400d_a000 as *const _
    }
}
impl Deref for USBPHY2 {
    type Target = usbphy1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBPHY2::ptr() }
    }
}
#[doc = "CSU registers"]
pub struct CSU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CSU {}
impl CSU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const csu::RegisterBlock {
        0x400d_c000 as *const _
    }
}
impl Deref for CSU {
    type Target = csu::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CSU::ptr() }
    }
}
#[doc = "CSU registers"]
pub mod csu;
#[doc = "Touch Screen Controller"]
pub struct TSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSC {}
impl TSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tsc::RegisterBlock {
        0x400e_0000 as *const _
    }
}
impl Deref for TSC {
    type Target = tsc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TSC::ptr() }
    }
}
#[doc = "Touch Screen Controller"]
pub mod tsc;
#[doc = "DMA"]
pub struct DMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA0 {}
impl DMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma0::RegisterBlock {
        0x400e_8000 as *const _
    }
}
impl Deref for DMA0 {
    type Target = dma0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA0::ptr() }
    }
}
#[doc = "DMA"]
pub mod dma0;
#[doc = "DMA_CH_MUX"]
pub struct DMAMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX {}
impl DMAMUX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux::RegisterBlock {
        0x400e_c000 as *const _
    }
}
impl Deref for DMAMUX {
    type Target = dmamux::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX::ptr() }
    }
}
#[doc = "DMA_CH_MUX"]
pub mod dmamux;
#[doc = "GPC"]
pub struct GPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPC {}
impl GPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpc::RegisterBlock {
        0x400f_4000 as *const _
    }
}
impl Deref for GPC {
    type Target = gpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPC::ptr() }
    }
}
#[doc = "GPC"]
pub mod gpc;
#[doc = "PGC"]
pub struct PGC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PGC {}
impl PGC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pgc::RegisterBlock {
        0x400f_4000 as *const _
    }
}
impl Deref for PGC {
    type Target = pgc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PGC::ptr() }
    }
}
#[doc = "PGC"]
pub mod pgc;
#[doc = "SRC"]
pub struct SRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRC {}
impl SRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const src::RegisterBlock {
        0x400f_8000 as *const _
    }
}
impl Deref for SRC {
    type Target = src::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SRC::ptr() }
    }
}
#[doc = "SRC"]
pub mod src;
#[doc = "CCM"]
pub struct CCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM {}
impl CCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccm::RegisterBlock {
        0x400f_c000 as *const _
    }
}
impl Deref for CCM {
    type Target = ccm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCM::ptr() }
    }
}
#[doc = "CCM"]
pub mod ccm;
#[doc = "ROMC"]
pub struct ROMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ROMC {}
impl ROMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const romc::RegisterBlock {
        0x4018_0000 as *const _
    }
}
impl Deref for ROMC {
    type Target = romc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ROMC::ptr() }
    }
}
#[doc = "ROMC"]
pub mod romc;
#[doc = "LPUART"]
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4018_4000 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART1::ptr() }
    }
}
#[doc = "LPUART"]
pub mod lpuart1;
#[doc = "LPUART"]
pub struct LPUART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART2 {}
impl LPUART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4018_8000 as *const _
    }
}
impl Deref for LPUART2 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART2::ptr() }
    }
}
#[doc = "LPUART"]
pub struct LPUART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART3 {}
impl LPUART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4018_c000 as *const _
    }
}
impl Deref for LPUART3 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART3::ptr() }
    }
}
#[doc = "LPUART"]
pub struct LPUART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART4 {}
impl LPUART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4019_0000 as *const _
    }
}
impl Deref for LPUART4 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART4::ptr() }
    }
}
#[doc = "LPUART"]
pub struct LPUART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART5 {}
impl LPUART5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4019_4000 as *const _
    }
}
impl Deref for LPUART5 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART5::ptr() }
    }
}
#[doc = "LPUART"]
pub struct LPUART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART6 {}
impl LPUART6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4019_8000 as *const _
    }
}
impl Deref for LPUART6 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART6::ptr() }
    }
}
#[doc = "LPUART"]
pub struct LPUART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART7 {}
impl LPUART7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4019_c000 as *const _
    }
}
impl Deref for LPUART7 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART7::ptr() }
    }
}
#[doc = "LPUART"]
pub struct LPUART8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART8 {}
impl LPUART8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x401a_0000 as *const _
    }
}
impl Deref for LPUART8 {
    type Target = lpuart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART8::ptr() }
    }
}
#[doc = "FLEXIO"]
pub struct FLEXIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXIO1 {}
impl FLEXIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexio1::RegisterBlock {
        0x401a_c000 as *const _
    }
}
impl Deref for FLEXIO1 {
    type Target = flexio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXIO1::ptr() }
    }
}
#[doc = "FLEXIO"]
pub mod flexio1;
#[doc = "FLEXIO"]
pub struct FLEXIO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXIO2 {}
impl FLEXIO2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexio1::RegisterBlock {
        0x401b_0000 as *const _
    }
}
impl Deref for FLEXIO2 {
    type Target = flexio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXIO2::ptr() }
    }
}
#[doc = "FLEXIO"]
pub struct FLEXIO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXIO3 {}
impl FLEXIO3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexio1::RegisterBlock {
        0x4202_0000 as *const _
    }
}
impl Deref for FLEXIO3 {
    type Target = flexio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXIO3::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO1 {}
impl GPIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio1::RegisterBlock {
        0x401b_8000 as *const _
    }
}
impl Deref for GPIO1 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO1::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpio1;
#[doc = "GPIO"]
pub struct GPIO5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO5 {}
impl GPIO5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio1::RegisterBlock {
        0x400c_0000 as *const _
    }
}
impl Deref for GPIO5 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO5::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO2 {}
impl GPIO2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio1::RegisterBlock {
        0x401b_c000 as *const _
    }
}
impl Deref for GPIO2 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO2::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO3 {}
impl GPIO3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio1::RegisterBlock {
        0x401c_0000 as *const _
    }
}
impl Deref for GPIO3 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO3::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO4 {}
impl GPIO4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio1::RegisterBlock {
        0x401c_4000 as *const _
    }
}
impl Deref for GPIO4 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO4::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO6 {}
impl GPIO6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio1::RegisterBlock {
        0x4200_0000 as *const _
    }
}
impl Deref for GPIO6 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO6::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO7 {}
impl GPIO7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio1::RegisterBlock {
        0x4200_4000 as *const _
    }
}
impl Deref for GPIO7 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO7::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO8 {}
impl GPIO8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio1::RegisterBlock {
        0x4200_8000 as *const _
    }
}
impl Deref for GPIO8 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO8::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO9 {}
impl GPIO9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio1::RegisterBlock {
        0x4200_c000 as *const _
    }
}
impl Deref for GPIO9 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO9::ptr() }
    }
}
#[doc = "FLEXCAN"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x401d_0000 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "FLEXCAN"]
pub mod can1;
#[doc = "FLEXCAN"]
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN2 {}
impl CAN2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x401d_4000 as *const _
    }
}
impl Deref for CAN2 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN2::ptr() }
    }
}
#[doc = "CAN"]
pub struct CAN3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN3 {}
impl CAN3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can3::RegisterBlock {
        0x401d_8000 as *const _
    }
}
impl Deref for CAN3 {
    type Target = can3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN3::ptr() }
    }
}
#[doc = "CAN"]
pub mod can3;
#[doc = "Quad Timer"]
pub struct TMR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR1 {}
impl TMR1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr1::RegisterBlock {
        0x401d_c000 as *const _
    }
}
impl Deref for TMR1 {
    type Target = tmr1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR1::ptr() }
    }
}
#[doc = "Quad Timer"]
pub mod tmr1;
#[doc = "Quad Timer"]
pub struct TMR2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR2 {}
impl TMR2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr1::RegisterBlock {
        0x401e_0000 as *const _
    }
}
impl Deref for TMR2 {
    type Target = tmr1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR2::ptr() }
    }
}
#[doc = "Quad Timer"]
pub struct TMR3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR3 {}
impl TMR3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr1::RegisterBlock {
        0x401e_4000 as *const _
    }
}
impl Deref for TMR3 {
    type Target = tmr1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR3::ptr() }
    }
}
#[doc = "Quad Timer"]
pub struct TMR4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR4 {}
impl TMR4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr1::RegisterBlock {
        0x401e_8000 as *const _
    }
}
impl Deref for TMR4 {
    type Target = tmr1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR4::ptr() }
    }
}
#[doc = "GPT"]
pub struct GPT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT1 {}
impl GPT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt1::RegisterBlock {
        0x401e_c000 as *const _
    }
}
impl Deref for GPT1 {
    type Target = gpt1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT1::ptr() }
    }
}
#[doc = "GPT"]
pub mod gpt1;
#[doc = "GPT"]
pub struct GPT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT2 {}
impl GPT2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt1::RegisterBlock {
        0x401f_0000 as *const _
    }
}
impl Deref for GPT2 {
    type Target = gpt1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT2::ptr() }
    }
}
#[doc = "OCOTP"]
pub struct OCOTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCOTP {}
impl OCOTP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ocotp::RegisterBlock {
        0x401f_4000 as *const _
    }
}
impl Deref for OCOTP {
    type Target = ocotp::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCOTP::ptr() }
    }
}
#[doc = "OCOTP"]
pub mod ocotp;
#[doc = "IOMUXC"]
pub struct IOMUXC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMUXC {}
impl IOMUXC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iomuxc::RegisterBlock {
        0x401f_8000 as *const _
    }
}
impl Deref for IOMUXC {
    type Target = iomuxc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOMUXC::ptr() }
    }
}
#[doc = "IOMUXC"]
pub mod iomuxc;
#[doc = "KPP Registers"]
pub struct KPP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for KPP {}
impl KPP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const kpp::RegisterBlock {
        0x401f_c000 as *const _
    }
}
impl Deref for KPP {
    type Target = kpp::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*KPP::ptr() }
    }
}
#[doc = "KPP Registers"]
pub mod kpp;
#[doc = "FlexSPI"]
pub struct FLEXSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXSPI {}
impl FLEXSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexspi::RegisterBlock {
        0x402a_8000 as *const _
    }
}
impl Deref for FLEXSPI {
    type Target = flexspi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXSPI::ptr() }
    }
}
#[doc = "FlexSPI"]
pub mod flexspi;
#[doc = "FlexSPI"]
pub struct FLEXSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXSPI2 {}
impl FLEXSPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexspi::RegisterBlock {
        0x402a_4000 as *const _
    }
}
impl Deref for FLEXSPI2 {
    type Target = flexspi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXSPI2::ptr() }
    }
}
#[doc = "PXP v2.0 Register Reference Index"]
pub struct PXP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PXP {}
impl PXP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pxp::RegisterBlock {
        0x402b_4000 as *const _
    }
}
impl Deref for PXP {
    type Target = pxp::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PXP::ptr() }
    }
}
#[doc = "PXP v2.0 Register Reference Index"]
pub mod pxp;
#[doc = "LCDIF Register Reference Index"]
pub struct LCDIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCDIF {}
impl LCDIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lcdif::RegisterBlock {
        0x402b_8000 as *const _
    }
}
impl Deref for LCDIF {
    type Target = lcdif::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LCDIF::ptr() }
    }
}
#[doc = "LCDIF Register Reference Index"]
pub mod lcdif;
#[doc = "CSI"]
pub struct CSI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CSI {}
impl CSI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const csi::RegisterBlock {
        0x402b_c000 as *const _
    }
}
impl Deref for CSI {
    type Target = csi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CSI::ptr() }
    }
}
#[doc = "CSI"]
pub mod csi;
#[doc = "uSDHC"]
pub struct USDHC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USDHC1 {}
impl USDHC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usdhc1::RegisterBlock {
        0x402c_0000 as *const _
    }
}
impl Deref for USDHC1 {
    type Target = usdhc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USDHC1::ptr() }
    }
}
#[doc = "uSDHC"]
pub mod usdhc1;
#[doc = "uSDHC"]
pub struct USDHC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USDHC2 {}
impl USDHC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usdhc1::RegisterBlock {
        0x402c_4000 as *const _
    }
}
impl Deref for USDHC2 {
    type Target = usdhc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USDHC2::ptr() }
    }
}
#[doc = "Ethernet MAC-NET Core"]
pub struct ENET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ENET {}
impl ENET {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const enet::RegisterBlock {
        0x402d_8000 as *const _
    }
}
impl Deref for ENET {
    type Target = enet::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ENET::ptr() }
    }
}
#[doc = "Ethernet MAC-NET Core"]
pub mod enet;
#[doc = "Ethernet MAC-NET Core"]
pub struct ENET2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ENET2 {}
impl ENET2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const enet::RegisterBlock {
        0x402d_4000 as *const _
    }
}
impl Deref for ENET2 {
    type Target = enet::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ENET2::ptr() }
    }
}
#[doc = "USB"]
pub struct USB1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB1 {}
impl USB1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb1::RegisterBlock {
        0x402e_0000 as *const _
    }
}
impl Deref for USB1 {
    type Target = usb1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB1::ptr() }
    }
}
#[doc = "USB"]
pub mod usb1;
#[doc = "USB"]
pub struct USB2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB2 {}
impl USB2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb1::RegisterBlock {
        0x402e_0200 as *const _
    }
}
impl Deref for USB2 {
    type Target = usb1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB2::ptr() }
    }
}
#[doc = "USB"]
pub struct USBNC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBNC1 {}
impl USBNC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbnc1::RegisterBlock {
        0x402e_0000 as *const _
    }
}
impl Deref for USBNC1 {
    type Target = usbnc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBNC1::ptr() }
    }
}
#[doc = "USB"]
pub mod usbnc1;
#[doc = "USB"]
pub struct USBNC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBNC2 {}
impl USBNC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbnc1::RegisterBlock {
        0x402e_0004 as *const _
    }
}
impl Deref for USBNC2 {
    type Target = usbnc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBNC2::ptr() }
    }
}
#[doc = "SEMC"]
pub struct SEMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEMC {}
impl SEMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const semc::RegisterBlock {
        0x402f_0000 as *const _
    }
}
impl Deref for SEMC {
    type Target = semc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEMC::ptr() }
    }
}
#[doc = "SEMC"]
pub mod semc;
#[doc = "DCP register reference index"]
pub struct DCP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCP {}
impl DCP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dcp::RegisterBlock {
        0x402f_c000 as *const _
    }
}
impl Deref for DCP {
    type Target = dcp::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCP::ptr() }
    }
}
#[doc = "DCP register reference index"]
pub mod dcp;
#[doc = "SPDIF"]
pub struct SPDIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPDIF {}
impl SPDIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spdif::RegisterBlock {
        0x4038_0000 as *const _
    }
}
impl Deref for SPDIF {
    type Target = spdif::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPDIF::ptr() }
    }
}
#[doc = "SPDIF"]
pub mod spdif;
#[doc = "I2S"]
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4038_4000 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI1::ptr() }
    }
}
#[doc = "I2S"]
pub mod sai1;
#[doc = "I2S"]
pub struct SAI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI2 {}
impl SAI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4038_8000 as *const _
    }
}
impl Deref for SAI2 {
    type Target = sai1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI2::ptr() }
    }
}
#[doc = "I2S"]
pub struct SAI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI3 {}
impl SAI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4038_c000 as *const _
    }
}
impl Deref for SAI3 {
    type Target = sai1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI3::ptr() }
    }
}
#[doc = "LPSPI"]
pub struct LPSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI1 {}
impl LPSPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpspi1::RegisterBlock {
        0x4039_4000 as *const _
    }
}
impl Deref for LPSPI1 {
    type Target = lpspi1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPSPI1::ptr() }
    }
}
#[doc = "LPSPI"]
pub mod lpspi1;
#[doc = "LPSPI"]
pub struct LPSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI2 {}
impl LPSPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpspi1::RegisterBlock {
        0x4039_8000 as *const _
    }
}
impl Deref for LPSPI2 {
    type Target = lpspi1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPSPI2::ptr() }
    }
}
#[doc = "LPSPI"]
pub struct LPSPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI3 {}
impl LPSPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpspi1::RegisterBlock {
        0x4039_c000 as *const _
    }
}
impl Deref for LPSPI3 {
    type Target = lpspi1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPSPI3::ptr() }
    }
}
#[doc = "LPSPI"]
pub struct LPSPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI4 {}
impl LPSPI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpspi1::RegisterBlock {
        0x403a_0000 as *const _
    }
}
impl Deref for LPSPI4 {
    type Target = lpspi1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPSPI4::ptr() }
    }
}
#[doc = "ADC_ETC"]
pub struct ADC_ETC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC_ETC {}
impl ADC_ETC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc_etc::RegisterBlock {
        0x403b_0000 as *const _
    }
}
impl Deref for ADC_ETC {
    type Target = adc_etc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC_ETC::ptr() }
    }
}
#[doc = "ADC_ETC"]
pub mod adc_etc;
#[doc = "AND/OR/INVERT module"]
pub struct AOI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AOI1 {}
impl AOI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aoi1::RegisterBlock {
        0x403b_4000 as *const _
    }
}
impl Deref for AOI1 {
    type Target = aoi1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AOI1::ptr() }
    }
}
#[doc = "AND/OR/INVERT module"]
pub mod aoi1;
#[doc = "AND/OR/INVERT module"]
pub struct AOI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AOI2 {}
impl AOI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aoi1::RegisterBlock {
        0x403b_8000 as *const _
    }
}
impl Deref for AOI2 {
    type Target = aoi1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AOI2::ptr() }
    }
}
#[doc = "Crossbar Switch"]
pub struct XBARA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XBARA1 {}
impl XBARA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xbara1::RegisterBlock {
        0x403b_c000 as *const _
    }
}
impl Deref for XBARA1 {
    type Target = xbara1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*XBARA1::ptr() }
    }
}
#[doc = "Crossbar Switch"]
pub mod xbara1;
#[doc = "Crossbar Switch"]
pub struct XBARB2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XBARB2 {}
impl XBARB2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xbarb2::RegisterBlock {
        0x403c_0000 as *const _
    }
}
impl Deref for XBARB2 {
    type Target = xbarb2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*XBARB2::ptr() }
    }
}
#[doc = "Crossbar Switch"]
pub mod xbarb2;
#[doc = "Crossbar Switch"]
pub struct XBARB3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XBARB3 {}
impl XBARB3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xbarb2::RegisterBlock {
        0x403c_4000 as *const _
    }
}
impl Deref for XBARB3 {
    type Target = xbarb2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*XBARB3::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub struct ENC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ENC1 {}
impl ENC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const enc1::RegisterBlock {
        0x403c_8000 as *const _
    }
}
impl Deref for ENC1 {
    type Target = enc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ENC1::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub mod enc1;
#[doc = "Quadrature Decoder"]
pub struct ENC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ENC2 {}
impl ENC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const enc1::RegisterBlock {
        0x403c_c000 as *const _
    }
}
impl Deref for ENC2 {
    type Target = enc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ENC2::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub struct ENC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ENC3 {}
impl ENC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const enc1::RegisterBlock {
        0x403d_0000 as *const _
    }
}
impl Deref for ENC3 {
    type Target = enc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ENC3::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub struct ENC4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ENC4 {}
impl ENC4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const enc1::RegisterBlock {
        0x403d_4000 as *const _
    }
}
impl Deref for ENC4 {
    type Target = enc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ENC4::ptr() }
    }
}
#[doc = "PWM"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm1::RegisterBlock {
        0x403d_c000 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "PWM"]
pub mod pwm1;
#[doc = "PWM"]
pub struct PWM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2 {}
impl PWM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm1::RegisterBlock {
        0x403e_0000 as *const _
    }
}
impl Deref for PWM2 {
    type Target = pwm1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM2::ptr() }
    }
}
#[doc = "PWM"]
pub struct PWM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM3 {}
impl PWM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm1::RegisterBlock {
        0x403e_4000 as *const _
    }
}
impl Deref for PWM3 {
    type Target = pwm1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM3::ptr() }
    }
}
#[doc = "PWM"]
pub struct PWM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM4 {}
impl PWM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm1::RegisterBlock {
        0x403e_8000 as *const _
    }
}
impl Deref for PWM4 {
    type Target = pwm1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM4::ptr() }
    }
}
#[doc = "Bus Encryption Engine"]
pub struct BEE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BEE {}
impl BEE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bee::RegisterBlock {
        0x403e_c000 as *const _
    }
}
impl Deref for BEE {
    type Target = bee::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*BEE::ptr() }
    }
}
#[doc = "Bus Encryption Engine"]
pub mod bee;
#[doc = "LPI2C"]
pub struct LPI2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPI2C1 {}
impl LPI2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpi2c1::RegisterBlock {
        0x403f_0000 as *const _
    }
}
impl Deref for LPI2C1 {
    type Target = lpi2c1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPI2C1::ptr() }
    }
}
#[doc = "LPI2C"]
pub mod lpi2c1;
#[doc = "LPI2C"]
pub struct LPI2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPI2C2 {}
impl LPI2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpi2c1::RegisterBlock {
        0x403f_4000 as *const _
    }
}
impl Deref for LPI2C2 {
    type Target = lpi2c1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPI2C2::ptr() }
    }
}
#[doc = "LPI2C"]
pub struct LPI2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPI2C3 {}
impl LPI2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpi2c1::RegisterBlock {
        0x403f_8000 as *const _
    }
}
impl Deref for LPI2C3 {
    type Target = lpi2c1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPI2C3::ptr() }
    }
}
#[doc = "LPI2C"]
pub struct LPI2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPI2C4 {}
impl LPI2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpi2c1::RegisterBlock {
        0x403f_c000 as *const _
    }
}
impl Deref for LPI2C4 {
    type Target = lpi2c1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPI2C4::ptr() }
    }
}
#[doc = "System Control Block"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_control::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control Block"]
pub mod system_control;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AIPSTZ1"]
    pub AIPSTZ1: AIPSTZ1,
    #[doc = "AIPSTZ2"]
    pub AIPSTZ2: AIPSTZ2,
    #[doc = "AIPSTZ3"]
    pub AIPSTZ3: AIPSTZ3,
    #[doc = "AIPSTZ4"]
    pub AIPSTZ4: AIPSTZ4,
    #[doc = "DCDC"]
    pub DCDC: DCDC,
    #[doc = "PIT"]
    pub PIT: PIT,
    #[doc = "CMP1"]
    pub CMP1: CMP1,
    #[doc = "CMP2"]
    pub CMP2: CMP2,
    #[doc = "CMP3"]
    pub CMP3: CMP3,
    #[doc = "CMP4"]
    pub CMP4: CMP4,
    #[doc = "IOMUXC_SNVS_GPR"]
    pub IOMUXC_SNVS_GPR: IOMUXC_SNVS_GPR,
    #[doc = "IOMUXC_SNVS"]
    pub IOMUXC_SNVS: IOMUXC_SNVS,
    #[doc = "IOMUXC_GPR"]
    pub IOMUXC_GPR: IOMUXC_GPR,
    #[doc = "FLEXRAM"]
    pub FLEXRAM: FLEXRAM,
    #[doc = "EWM"]
    pub EWM: EWM,
    #[doc = "WDOG1"]
    pub WDOG1: WDOG1,
    #[doc = "WDOG2"]
    pub WDOG2: WDOG2,
    #[doc = "RTWDOG"]
    pub RTWDOG: RTWDOG,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "SNVS"]
    pub SNVS: SNVS,
    #[doc = "CCM_ANALOG"]
    pub CCM_ANALOG: CCM_ANALOG,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "TEMPMON"]
    pub TEMPMON: TEMPMON,
    #[doc = "USB_ANALOG"]
    pub USB_ANALOG: USB_ANALOG,
    #[doc = "XTALOSC24M"]
    pub XTALOSC24M: XTALOSC24M,
    #[doc = "USBPHY1"]
    pub USBPHY1: USBPHY1,
    #[doc = "USBPHY2"]
    pub USBPHY2: USBPHY2,
    #[doc = "CSU"]
    pub CSU: CSU,
    #[doc = "TSC"]
    pub TSC: TSC,
    #[doc = "DMA0"]
    pub DMA0: DMA0,
    #[doc = "DMAMUX"]
    pub DMAMUX: DMAMUX,
    #[doc = "GPC"]
    pub GPC: GPC,
    #[doc = "PGC"]
    pub PGC: PGC,
    #[doc = "SRC"]
    pub SRC: SRC,
    #[doc = "CCM"]
    pub CCM: CCM,
    #[doc = "ROMC"]
    pub ROMC: ROMC,
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[doc = "LPUART2"]
    pub LPUART2: LPUART2,
    #[doc = "LPUART3"]
    pub LPUART3: LPUART3,
    #[doc = "LPUART4"]
    pub LPUART4: LPUART4,
    #[doc = "LPUART5"]
    pub LPUART5: LPUART5,
    #[doc = "LPUART6"]
    pub LPUART6: LPUART6,
    #[doc = "LPUART7"]
    pub LPUART7: LPUART7,
    #[doc = "LPUART8"]
    pub LPUART8: LPUART8,
    #[doc = "FLEXIO1"]
    pub FLEXIO1: FLEXIO1,
    #[doc = "FLEXIO2"]
    pub FLEXIO2: FLEXIO2,
    #[doc = "FLEXIO3"]
    pub FLEXIO3: FLEXIO3,
    #[doc = "GPIO1"]
    pub GPIO1: GPIO1,
    #[doc = "GPIO5"]
    pub GPIO5: GPIO5,
    #[doc = "GPIO2"]
    pub GPIO2: GPIO2,
    #[doc = "GPIO3"]
    pub GPIO3: GPIO3,
    #[doc = "GPIO4"]
    pub GPIO4: GPIO4,
    #[doc = "GPIO6"]
    pub GPIO6: GPIO6,
    #[doc = "GPIO7"]
    pub GPIO7: GPIO7,
    #[doc = "GPIO8"]
    pub GPIO8: GPIO8,
    #[doc = "GPIO9"]
    pub GPIO9: GPIO9,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    #[doc = "CAN3"]
    pub CAN3: CAN3,
    #[doc = "TMR1"]
    pub TMR1: TMR1,
    #[doc = "TMR2"]
    pub TMR2: TMR2,
    #[doc = "TMR3"]
    pub TMR3: TMR3,
    #[doc = "TMR4"]
    pub TMR4: TMR4,
    #[doc = "GPT1"]
    pub GPT1: GPT1,
    #[doc = "GPT2"]
    pub GPT2: GPT2,
    #[doc = "OCOTP"]
    pub OCOTP: OCOTP,
    #[doc = "IOMUXC"]
    pub IOMUXC: IOMUXC,
    #[doc = "KPP"]
    pub KPP: KPP,
    #[doc = "FLEXSPI"]
    pub FLEXSPI: FLEXSPI,
    #[doc = "FLEXSPI2"]
    pub FLEXSPI2: FLEXSPI2,
    #[doc = "PXP"]
    pub PXP: PXP,
    #[doc = "LCDIF"]
    pub LCDIF: LCDIF,
    #[doc = "CSI"]
    pub CSI: CSI,
    #[doc = "USDHC1"]
    pub USDHC1: USDHC1,
    #[doc = "USDHC2"]
    pub USDHC2: USDHC2,
    #[doc = "ENET"]
    pub ENET: ENET,
    #[doc = "ENET2"]
    pub ENET2: ENET2,
    #[doc = "USB1"]
    pub USB1: USB1,
    #[doc = "USB2"]
    pub USB2: USB2,
    #[doc = "USBNC1"]
    pub USBNC1: USBNC1,
    #[doc = "USBNC2"]
    pub USBNC2: USBNC2,
    #[doc = "SEMC"]
    pub SEMC: SEMC,
    #[doc = "DCP"]
    pub DCP: DCP,
    #[doc = "SPDIF"]
    pub SPDIF: SPDIF,
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[doc = "SAI2"]
    pub SAI2: SAI2,
    #[doc = "SAI3"]
    pub SAI3: SAI3,
    #[doc = "LPSPI1"]
    pub LPSPI1: LPSPI1,
    #[doc = "LPSPI2"]
    pub LPSPI2: LPSPI2,
    #[doc = "LPSPI3"]
    pub LPSPI3: LPSPI3,
    #[doc = "LPSPI4"]
    pub LPSPI4: LPSPI4,
    #[doc = "ADC_ETC"]
    pub ADC_ETC: ADC_ETC,
    #[doc = "AOI1"]
    pub AOI1: AOI1,
    #[doc = "AOI2"]
    pub AOI2: AOI2,
    #[doc = "XBARA1"]
    pub XBARA1: XBARA1,
    #[doc = "XBARB2"]
    pub XBARB2: XBARB2,
    #[doc = "XBARB3"]
    pub XBARB3: XBARB3,
    #[doc = "ENC1"]
    pub ENC1: ENC1,
    #[doc = "ENC2"]
    pub ENC2: ENC2,
    #[doc = "ENC3"]
    pub ENC3: ENC3,
    #[doc = "ENC4"]
    pub ENC4: ENC4,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "PWM2"]
    pub PWM2: PWM2,
    #[doc = "PWM3"]
    pub PWM3: PWM3,
    #[doc = "PWM4"]
    pub PWM4: PWM4,
    #[doc = "BEE"]
    pub BEE: BEE,
    #[doc = "LPI2C1"]
    pub LPI2C1: LPI2C1,
    #[doc = "LPI2C2"]
    pub LPI2C2: LPI2C2,
    #[doc = "LPI2C3"]
    pub LPI2C3: LPI2C3,
    #[doc = "LPI2C4"]
    pub LPI2C4: LPI2C4,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AIPSTZ1: AIPSTZ1 {
                _marker: PhantomData,
            },
            AIPSTZ2: AIPSTZ2 {
                _marker: PhantomData,
            },
            AIPSTZ3: AIPSTZ3 {
                _marker: PhantomData,
            },
            AIPSTZ4: AIPSTZ4 {
                _marker: PhantomData,
            },
            DCDC: DCDC {
                _marker: PhantomData,
            },
            PIT: PIT {
                _marker: PhantomData,
            },
            CMP1: CMP1 {
                _marker: PhantomData,
            },
            CMP2: CMP2 {
                _marker: PhantomData,
            },
            CMP3: CMP3 {
                _marker: PhantomData,
            },
            CMP4: CMP4 {
                _marker: PhantomData,
            },
            IOMUXC_SNVS_GPR: IOMUXC_SNVS_GPR {
                _marker: PhantomData,
            },
            IOMUXC_SNVS: IOMUXC_SNVS {
                _marker: PhantomData,
            },
            IOMUXC_GPR: IOMUXC_GPR {
                _marker: PhantomData,
            },
            FLEXRAM: FLEXRAM {
                _marker: PhantomData,
            },
            EWM: EWM {
                _marker: PhantomData,
            },
            WDOG1: WDOG1 {
                _marker: PhantomData,
            },
            WDOG2: WDOG2 {
                _marker: PhantomData,
            },
            RTWDOG: RTWDOG {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            SNVS: SNVS {
                _marker: PhantomData,
            },
            CCM_ANALOG: CCM_ANALOG {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            TEMPMON: TEMPMON {
                _marker: PhantomData,
            },
            USB_ANALOG: USB_ANALOG {
                _marker: PhantomData,
            },
            XTALOSC24M: XTALOSC24M {
                _marker: PhantomData,
            },
            USBPHY1: USBPHY1 {
                _marker: PhantomData,
            },
            USBPHY2: USBPHY2 {
                _marker: PhantomData,
            },
            CSU: CSU {
                _marker: PhantomData,
            },
            TSC: TSC {
                _marker: PhantomData,
            },
            DMA0: DMA0 {
                _marker: PhantomData,
            },
            DMAMUX: DMAMUX {
                _marker: PhantomData,
            },
            GPC: GPC {
                _marker: PhantomData,
            },
            PGC: PGC {
                _marker: PhantomData,
            },
            SRC: SRC {
                _marker: PhantomData,
            },
            CCM: CCM {
                _marker: PhantomData,
            },
            ROMC: ROMC {
                _marker: PhantomData,
            },
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            LPUART2: LPUART2 {
                _marker: PhantomData,
            },
            LPUART3: LPUART3 {
                _marker: PhantomData,
            },
            LPUART4: LPUART4 {
                _marker: PhantomData,
            },
            LPUART5: LPUART5 {
                _marker: PhantomData,
            },
            LPUART6: LPUART6 {
                _marker: PhantomData,
            },
            LPUART7: LPUART7 {
                _marker: PhantomData,
            },
            LPUART8: LPUART8 {
                _marker: PhantomData,
            },
            FLEXIO1: FLEXIO1 {
                _marker: PhantomData,
            },
            FLEXIO2: FLEXIO2 {
                _marker: PhantomData,
            },
            FLEXIO3: FLEXIO3 {
                _marker: PhantomData,
            },
            GPIO1: GPIO1 {
                _marker: PhantomData,
            },
            GPIO5: GPIO5 {
                _marker: PhantomData,
            },
            GPIO2: GPIO2 {
                _marker: PhantomData,
            },
            GPIO3: GPIO3 {
                _marker: PhantomData,
            },
            GPIO4: GPIO4 {
                _marker: PhantomData,
            },
            GPIO6: GPIO6 {
                _marker: PhantomData,
            },
            GPIO7: GPIO7 {
                _marker: PhantomData,
            },
            GPIO8: GPIO8 {
                _marker: PhantomData,
            },
            GPIO9: GPIO9 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            CAN3: CAN3 {
                _marker: PhantomData,
            },
            TMR1: TMR1 {
                _marker: PhantomData,
            },
            TMR2: TMR2 {
                _marker: PhantomData,
            },
            TMR3: TMR3 {
                _marker: PhantomData,
            },
            TMR4: TMR4 {
                _marker: PhantomData,
            },
            GPT1: GPT1 {
                _marker: PhantomData,
            },
            GPT2: GPT2 {
                _marker: PhantomData,
            },
            OCOTP: OCOTP {
                _marker: PhantomData,
            },
            IOMUXC: IOMUXC {
                _marker: PhantomData,
            },
            KPP: KPP {
                _marker: PhantomData,
            },
            FLEXSPI: FLEXSPI {
                _marker: PhantomData,
            },
            FLEXSPI2: FLEXSPI2 {
                _marker: PhantomData,
            },
            PXP: PXP {
                _marker: PhantomData,
            },
            LCDIF: LCDIF {
                _marker: PhantomData,
            },
            CSI: CSI {
                _marker: PhantomData,
            },
            USDHC1: USDHC1 {
                _marker: PhantomData,
            },
            USDHC2: USDHC2 {
                _marker: PhantomData,
            },
            ENET: ENET {
                _marker: PhantomData,
            },
            ENET2: ENET2 {
                _marker: PhantomData,
            },
            USB1: USB1 {
                _marker: PhantomData,
            },
            USB2: USB2 {
                _marker: PhantomData,
            },
            USBNC1: USBNC1 {
                _marker: PhantomData,
            },
            USBNC2: USBNC2 {
                _marker: PhantomData,
            },
            SEMC: SEMC {
                _marker: PhantomData,
            },
            DCP: DCP {
                _marker: PhantomData,
            },
            SPDIF: SPDIF {
                _marker: PhantomData,
            },
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            SAI2: SAI2 {
                _marker: PhantomData,
            },
            SAI3: SAI3 {
                _marker: PhantomData,
            },
            LPSPI1: LPSPI1 {
                _marker: PhantomData,
            },
            LPSPI2: LPSPI2 {
                _marker: PhantomData,
            },
            LPSPI3: LPSPI3 {
                _marker: PhantomData,
            },
            LPSPI4: LPSPI4 {
                _marker: PhantomData,
            },
            ADC_ETC: ADC_ETC {
                _marker: PhantomData,
            },
            AOI1: AOI1 {
                _marker: PhantomData,
            },
            AOI2: AOI2 {
                _marker: PhantomData,
            },
            XBARA1: XBARA1 {
                _marker: PhantomData,
            },
            XBARB2: XBARB2 {
                _marker: PhantomData,
            },
            XBARB3: XBARB3 {
                _marker: PhantomData,
            },
            ENC1: ENC1 {
                _marker: PhantomData,
            },
            ENC2: ENC2 {
                _marker: PhantomData,
            },
            ENC3: ENC3 {
                _marker: PhantomData,
            },
            ENC4: ENC4 {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            PWM2: PWM2 {
                _marker: PhantomData,
            },
            PWM3: PWM3 {
                _marker: PhantomData,
            },
            PWM4: PWM4 {
                _marker: PhantomData,
            },
            BEE: BEE {
                _marker: PhantomData,
            },
            LPI2C1: LPI2C1 {
                _marker: PhantomData,
            },
            LPI2C2: LPI2C2 {
                _marker: PhantomData,
            },
            LPI2C3: LPI2C3 {
                _marker: PhantomData,
            },
            LPI2C4: LPI2C4 {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
        }
    }
}
