#[doc = r"Enumeration of all the interrupts."]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - WIFI_MAC"]
    WIFI_MAC = 0,
    #[doc = "1 - WIFI_MAC_NMI"]
    WIFI_MAC_NMI = 1,
    #[doc = "2 - WIFI_PWR"]
    WIFI_PWR = 2,
    #[doc = "3 - WIFI_BB"]
    WIFI_BB = 3,
    #[doc = "4 - BT_MAC"]
    BT_MAC = 4,
    #[doc = "5 - BT_BB"]
    BT_BB = 5,
    #[doc = "6 - BT_BB_NMI"]
    BT_BB_NMI = 6,
    #[doc = "7 - LP_TIMER"]
    LP_TIMER = 7,
    #[doc = "8 - COEX"]
    COEX = 8,
    #[doc = "9 - BLE_TIMER"]
    BLE_TIMER = 9,
    #[doc = "10 - BLE_SEC"]
    BLE_SEC = 10,
    #[doc = "11 - I2C_MASTER"]
    I2C_MASTER = 11,
    #[doc = "12 - ZB_MAC"]
    ZB_MAC = 12,
    #[doc = "13 - PMU"]
    PMU = 13,
    #[doc = "14 - EFUSE"]
    EFUSE = 14,
    #[doc = "15 - LP_RTC_TIMER"]
    LP_RTC_TIMER = 15,
    #[doc = "16 - LP_UART"]
    LP_UART = 16,
    #[doc = "17 - LP_I2C"]
    LP_I2C = 17,
    #[doc = "18 - LP_WDT"]
    LP_WDT = 18,
    #[doc = "19 - LP_PERI_TIMEOUT"]
    LP_PERI_TIMEOUT = 19,
    #[doc = "20 - LP_APM_M0"]
    LP_APM_M0 = 20,
    #[doc = "21 - LP_APM_M1"]
    LP_APM_M1 = 21,
    #[doc = "22 - FROM_CPU_INTR0"]
    FROM_CPU_INTR0 = 22,
    #[doc = "23 - FROM_CPU_INTR1"]
    FROM_CPU_INTR1 = 23,
    #[doc = "24 - FROM_CPU_INTR2"]
    FROM_CPU_INTR2 = 24,
    #[doc = "25 - FROM_CPU_INTR3"]
    FROM_CPU_INTR3 = 25,
    #[doc = "26 - ASSIST_DEBUG"]
    ASSIST_DEBUG = 26,
    #[doc = "27 - TRACE"]
    TRACE = 27,
    #[doc = "28 - CACHE"]
    CACHE = 28,
    #[doc = "29 - CPU_PERI_TIMEOUT"]
    CPU_PERI_TIMEOUT = 29,
    #[doc = "30 - GPIO"]
    GPIO = 30,
    #[doc = "31 - GPIO_NMI"]
    GPIO_NMI = 31,
    #[doc = "32 - PAU"]
    PAU = 32,
    #[doc = "33 - HP_PERI_TIMEOUT"]
    HP_PERI_TIMEOUT = 33,
    #[doc = "34 - MODEM_PERI_TIMEOUT"]
    MODEM_PERI_TIMEOUT = 34,
    #[doc = "35 - HP_APM_M0"]
    HP_APM_M0 = 35,
    #[doc = "36 - HP_APM_M1"]
    HP_APM_M1 = 36,
    #[doc = "37 - HP_APM_M2"]
    HP_APM_M2 = 37,
    #[doc = "38 - HP_APM_M3"]
    HP_APM_M3 = 38,
    #[doc = "39 - LP_APM0"]
    LP_APM0 = 39,
    #[doc = "40 - MSPI"]
    MSPI = 40,
    #[doc = "41 - I2S0"]
    I2S0 = 41,
    #[doc = "42 - UHCI0"]
    UHCI0 = 42,
    #[doc = "43 - UART0"]
    UART0 = 43,
    #[doc = "44 - UART1"]
    UART1 = 44,
    #[doc = "45 - LEDC"]
    LEDC = 45,
    #[doc = "46 - TWAI0"]
    TWAI0 = 46,
    #[doc = "47 - TWAI1"]
    TWAI1 = 47,
    #[doc = "48 - USB_DEVICE"]
    USB_DEVICE = 48,
    #[doc = "49 - RMT"]
    RMT = 49,
    #[doc = "50 - I2C_EXT0"]
    I2C_EXT0 = 50,
    #[doc = "51 - TG0_T0_LEVEL"]
    TG0_T0_LEVEL = 51,
    #[doc = "52 - TG0_T1_LEVEL"]
    TG0_T1_LEVEL = 52,
    #[doc = "53 - TG0_WDT_LEVEL"]
    TG0_WDT_LEVEL = 53,
    #[doc = "54 - TG1_T0_LEVEL"]
    TG1_T0_LEVEL = 54,
    #[doc = "55 - TG1_T1_LEVEL"]
    TG1_T1_LEVEL = 55,
    #[doc = "56 - TG1_WDT_LEVEL"]
    TG1_WDT_LEVEL = 56,
    #[doc = "57 - SYSTIMER_TARGET0"]
    SYSTIMER_TARGET0 = 57,
    #[doc = "58 - SYSTIMER_TARGET1"]
    SYSTIMER_TARGET1 = 58,
    #[doc = "59 - SYSTIMER_TARGET2"]
    SYSTIMER_TARGET2 = 59,
    #[doc = "60 - APB_SARADC"]
    APB_SARADC = 60,
    #[doc = "61 - MCPWM0"]
    MCPWM0 = 61,
    #[doc = "62 - PCNT"]
    PCNT = 62,
    #[doc = "63 - PARL_IO"]
    PARL_IO = 63,
    #[doc = "64 - SLC0"]
    SLC0 = 64,
    #[doc = "65 - SLC1"]
    SLC1 = 65,
    #[doc = "66 - DMA_IN_CH0"]
    DMA_IN_CH0 = 66,
    #[doc = "67 - DMA_IN_CH1"]
    DMA_IN_CH1 = 67,
    #[doc = "68 - DMA_IN_CH2"]
    DMA_IN_CH2 = 68,
    #[doc = "69 - DMA_OUT_CH0"]
    DMA_OUT_CH0 = 69,
    #[doc = "70 - DMA_OUT_CH1"]
    DMA_OUT_CH1 = 70,
    #[doc = "71 - DMA_OUT_CH2"]
    DMA_OUT_CH2 = 71,
    #[doc = "72 - SPI2"]
    SPI2 = 72,
    #[doc = "73 - AES"]
    AES = 73,
    #[doc = "74 - SHA"]
    SHA = 74,
    #[doc = "75 - RSA"]
    RSA = 75,
    #[doc = "76 - ECC"]
    ECC = 76,
}
#[doc = r" TryFromInterruptError"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::WIFI_MAC),
            1 => Ok(Interrupt::WIFI_MAC_NMI),
            2 => Ok(Interrupt::WIFI_PWR),
            3 => Ok(Interrupt::WIFI_BB),
            4 => Ok(Interrupt::BT_MAC),
            5 => Ok(Interrupt::BT_BB),
            6 => Ok(Interrupt::BT_BB_NMI),
            7 => Ok(Interrupt::LP_TIMER),
            8 => Ok(Interrupt::COEX),
            9 => Ok(Interrupt::BLE_TIMER),
            10 => Ok(Interrupt::BLE_SEC),
            11 => Ok(Interrupt::I2C_MASTER),
            12 => Ok(Interrupt::ZB_MAC),
            13 => Ok(Interrupt::PMU),
            14 => Ok(Interrupt::EFUSE),
            15 => Ok(Interrupt::LP_RTC_TIMER),
            16 => Ok(Interrupt::LP_UART),
            17 => Ok(Interrupt::LP_I2C),
            18 => Ok(Interrupt::LP_WDT),
            19 => Ok(Interrupt::LP_PERI_TIMEOUT),
            20 => Ok(Interrupt::LP_APM_M0),
            21 => Ok(Interrupt::LP_APM_M1),
            22 => Ok(Interrupt::FROM_CPU_INTR0),
            23 => Ok(Interrupt::FROM_CPU_INTR1),
            24 => Ok(Interrupt::FROM_CPU_INTR2),
            25 => Ok(Interrupt::FROM_CPU_INTR3),
            26 => Ok(Interrupt::ASSIST_DEBUG),
            27 => Ok(Interrupt::TRACE),
            28 => Ok(Interrupt::CACHE),
            29 => Ok(Interrupt::CPU_PERI_TIMEOUT),
            30 => Ok(Interrupt::GPIO),
            31 => Ok(Interrupt::GPIO_NMI),
            32 => Ok(Interrupt::PAU),
            33 => Ok(Interrupt::HP_PERI_TIMEOUT),
            34 => Ok(Interrupt::MODEM_PERI_TIMEOUT),
            35 => Ok(Interrupt::HP_APM_M0),
            36 => Ok(Interrupt::HP_APM_M1),
            37 => Ok(Interrupt::HP_APM_M2),
            38 => Ok(Interrupt::HP_APM_M3),
            39 => Ok(Interrupt::LP_APM0),
            40 => Ok(Interrupt::MSPI),
            41 => Ok(Interrupt::I2S0),
            42 => Ok(Interrupt::UHCI0),
            43 => Ok(Interrupt::UART0),
            44 => Ok(Interrupt::UART1),
            45 => Ok(Interrupt::LEDC),
            46 => Ok(Interrupt::TWAI0),
            47 => Ok(Interrupt::TWAI1),
            48 => Ok(Interrupt::USB_DEVICE),
            49 => Ok(Interrupt::RMT),
            50 => Ok(Interrupt::I2C_EXT0),
            51 => Ok(Interrupt::TG0_T0_LEVEL),
            52 => Ok(Interrupt::TG0_T1_LEVEL),
            53 => Ok(Interrupt::TG0_WDT_LEVEL),
            54 => Ok(Interrupt::TG1_T0_LEVEL),
            55 => Ok(Interrupt::TG1_T1_LEVEL),
            56 => Ok(Interrupt::TG1_WDT_LEVEL),
            57 => Ok(Interrupt::SYSTIMER_TARGET0),
            58 => Ok(Interrupt::SYSTIMER_TARGET1),
            59 => Ok(Interrupt::SYSTIMER_TARGET2),
            60 => Ok(Interrupt::APB_SARADC),
            61 => Ok(Interrupt::MCPWM0),
            62 => Ok(Interrupt::PCNT),
            63 => Ok(Interrupt::PARL_IO),
            64 => Ok(Interrupt::SLC0),
            65 => Ok(Interrupt::SLC1),
            66 => Ok(Interrupt::DMA_IN_CH0),
            67 => Ok(Interrupt::DMA_IN_CH1),
            68 => Ok(Interrupt::DMA_IN_CH2),
            69 => Ok(Interrupt::DMA_OUT_CH0),
            70 => Ok(Interrupt::DMA_OUT_CH1),
            71 => Ok(Interrupt::DMA_OUT_CH2),
            72 => Ok(Interrupt::SPI2),
            73 => Ok(Interrupt::AES),
            74 => Ok(Interrupt::SHA),
            75 => Ok(Interrupt::RSA),
            76 => Ok(Interrupt::ECC),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
