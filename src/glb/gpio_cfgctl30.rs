#[doc = "Register `GPIO_CFGCTL30` reader"]
pub type R = crate::R<GpioCfgctl30Spec>;
#[doc = "Field `reg_gpio_0_i` reader - "]
pub type RegGpio0IR = crate::BitReader;
#[doc = "Input register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio1input> for bool {
    #[inline(always)]
    fn from(variant: Gpio1input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_i` reader - Input register for GPIO1."]
pub type RegGpio1IR = crate::BitReader<Gpio1input>;
impl RegGpio1IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1input {
        match self.bits {
            false => Gpio1input::Disabled,
            true => Gpio1input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio1input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio1input::Enabled
    }
}
#[doc = "Input register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio2input> for bool {
    #[inline(always)]
    fn from(variant: Gpio2input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_i` reader - Input register for GPIO2."]
pub type RegGpio2IR = crate::BitReader<Gpio2input>;
impl RegGpio2IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2input {
        match self.bits {
            false => Gpio2input::Disabled,
            true => Gpio2input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio2input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio2input::Enabled
    }
}
#[doc = "Input register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio3input> for bool {
    #[inline(always)]
    fn from(variant: Gpio3input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_i` reader - Input register for GPIO3."]
pub type RegGpio3IR = crate::BitReader<Gpio3input>;
impl RegGpio3IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3input {
        match self.bits {
            false => Gpio3input::Disabled,
            true => Gpio3input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio3input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio3input::Enabled
    }
}
#[doc = "Input register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio4input> for bool {
    #[inline(always)]
    fn from(variant: Gpio4input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_i` reader - Input register for GPIO4."]
pub type RegGpio4IR = crate::BitReader<Gpio4input>;
impl RegGpio4IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4input {
        match self.bits {
            false => Gpio4input::Disabled,
            true => Gpio4input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio4input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio4input::Enabled
    }
}
#[doc = "Input register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio5input> for bool {
    #[inline(always)]
    fn from(variant: Gpio5input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_i` reader - Input register for GPIO5."]
pub type RegGpio5IR = crate::BitReader<Gpio5input>;
impl RegGpio5IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5input {
        match self.bits {
            false => Gpio5input::Disabled,
            true => Gpio5input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio5input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio5input::Enabled
    }
}
#[doc = "Input register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio6input> for bool {
    #[inline(always)]
    fn from(variant: Gpio6input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_i` reader - Input register for GPIO6."]
pub type RegGpio6IR = crate::BitReader<Gpio6input>;
impl RegGpio6IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6input {
        match self.bits {
            false => Gpio6input::Disabled,
            true => Gpio6input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio6input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio6input::Enabled
    }
}
#[doc = "Input register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio7input> for bool {
    #[inline(always)]
    fn from(variant: Gpio7input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_i` reader - Input register for GPIO7."]
pub type RegGpio7IR = crate::BitReader<Gpio7input>;
impl RegGpio7IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7input {
        match self.bits {
            false => Gpio7input::Disabled,
            true => Gpio7input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio7input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio7input::Enabled
    }
}
#[doc = "Input register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio8input> for bool {
    #[inline(always)]
    fn from(variant: Gpio8input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_i` reader - Input register for GPIO8."]
pub type RegGpio8IR = crate::BitReader<Gpio8input>;
impl RegGpio8IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8input {
        match self.bits {
            false => Gpio8input::Disabled,
            true => Gpio8input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio8input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio8input::Enabled
    }
}
#[doc = "Input register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio9input> for bool {
    #[inline(always)]
    fn from(variant: Gpio9input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_i` reader - Input register for GPIO9."]
pub type RegGpio9IR = crate::BitReader<Gpio9input>;
impl RegGpio9IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9input {
        match self.bits {
            false => Gpio9input::Disabled,
            true => Gpio9input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio9input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio9input::Enabled
    }
}
#[doc = "Input register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio10input> for bool {
    #[inline(always)]
    fn from(variant: Gpio10input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_i` reader - Input register for GPIO10."]
pub type RegGpio10IR = crate::BitReader<Gpio10input>;
impl RegGpio10IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10input {
        match self.bits {
            false => Gpio10input::Disabled,
            true => Gpio10input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio10input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio10input::Enabled
    }
}
#[doc = "Input register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio11input> for bool {
    #[inline(always)]
    fn from(variant: Gpio11input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_i` reader - Input register for GPIO11."]
pub type RegGpio11IR = crate::BitReader<Gpio11input>;
impl RegGpio11IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11input {
        match self.bits {
            false => Gpio11input::Disabled,
            true => Gpio11input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio11input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio11input::Enabled
    }
}
#[doc = "Input register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio12input> for bool {
    #[inline(always)]
    fn from(variant: Gpio12input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_i` reader - Input register for GPIO12."]
pub type RegGpio12IR = crate::BitReader<Gpio12input>;
impl RegGpio12IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12input {
        match self.bits {
            false => Gpio12input::Disabled,
            true => Gpio12input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio12input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio12input::Enabled
    }
}
#[doc = "Input register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio13input> for bool {
    #[inline(always)]
    fn from(variant: Gpio13input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_i` reader - Input register for GPIO13."]
pub type RegGpio13IR = crate::BitReader<Gpio13input>;
impl RegGpio13IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13input {
        match self.bits {
            false => Gpio13input::Disabled,
            true => Gpio13input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio13input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio13input::Enabled
    }
}
#[doc = "Input register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio14input> for bool {
    #[inline(always)]
    fn from(variant: Gpio14input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_i` reader - Input register for GPIO14."]
pub type RegGpio14IR = crate::BitReader<Gpio14input>;
impl RegGpio14IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14input {
        match self.bits {
            false => Gpio14input::Disabled,
            true => Gpio14input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio14input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio14input::Enabled
    }
}
#[doc = "Input register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio15input> for bool {
    #[inline(always)]
    fn from(variant: Gpio15input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_i` reader - Input register for GPIO15."]
pub type RegGpio15IR = crate::BitReader<Gpio15input>;
impl RegGpio15IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15input {
        match self.bits {
            false => Gpio15input::Disabled,
            true => Gpio15input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio15input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio15input::Enabled
    }
}
#[doc = "Input register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio16input> for bool {
    #[inline(always)]
    fn from(variant: Gpio16input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_i` reader - Input register for GPIO16."]
pub type RegGpio16IR = crate::BitReader<Gpio16input>;
impl RegGpio16IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16input {
        match self.bits {
            false => Gpio16input::Disabled,
            true => Gpio16input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio16input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio16input::Enabled
    }
}
#[doc = "Input register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio17input> for bool {
    #[inline(always)]
    fn from(variant: Gpio17input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_i` reader - Input register for GPIO17."]
pub type RegGpio17IR = crate::BitReader<Gpio17input>;
impl RegGpio17IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17input {
        match self.bits {
            false => Gpio17input::Disabled,
            true => Gpio17input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio17input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio17input::Enabled
    }
}
#[doc = "Input register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio18input> for bool {
    #[inline(always)]
    fn from(variant: Gpio18input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_i` reader - Input register for GPIO18."]
pub type RegGpio18IR = crate::BitReader<Gpio18input>;
impl RegGpio18IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18input {
        match self.bits {
            false => Gpio18input::Disabled,
            true => Gpio18input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio18input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio18input::Enabled
    }
}
#[doc = "Input register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio19input> for bool {
    #[inline(always)]
    fn from(variant: Gpio19input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_i` reader - Input register for GPIO19."]
pub type RegGpio19IR = crate::BitReader<Gpio19input>;
impl RegGpio19IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19input {
        match self.bits {
            false => Gpio19input::Disabled,
            true => Gpio19input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio19input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio19input::Enabled
    }
}
#[doc = "Input register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio20input> for bool {
    #[inline(always)]
    fn from(variant: Gpio20input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_i` reader - Input register for GPIO20."]
pub type RegGpio20IR = crate::BitReader<Gpio20input>;
impl RegGpio20IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20input {
        match self.bits {
            false => Gpio20input::Disabled,
            true => Gpio20input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio20input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio20input::Enabled
    }
}
#[doc = "Input register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio21input> for bool {
    #[inline(always)]
    fn from(variant: Gpio21input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_i` reader - Input register for GPIO21."]
pub type RegGpio21IR = crate::BitReader<Gpio21input>;
impl RegGpio21IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21input {
        match self.bits {
            false => Gpio21input::Disabled,
            true => Gpio21input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio21input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio21input::Enabled
    }
}
#[doc = "Input register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22input {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio22input> for bool {
    #[inline(always)]
    fn from(variant: Gpio22input) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_i` reader - Input register for GPIO22."]
pub type RegGpio22IR = crate::BitReader<Gpio22input>;
impl RegGpio22IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22input {
        match self.bits {
            false => Gpio22input::Disabled,
            true => Gpio22input::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio22input::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio22input::Enabled
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_i(&self) -> RegGpio0IR {
        RegGpio0IR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_i(&self) -> RegGpio1IR {
        RegGpio1IR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_i(&self) -> RegGpio2IR {
        RegGpio2IR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_i(&self) -> RegGpio3IR {
        RegGpio3IR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_i(&self) -> RegGpio4IR {
        RegGpio4IR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_i(&self) -> RegGpio5IR {
        RegGpio5IR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_i(&self) -> RegGpio6IR {
        RegGpio6IR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_i(&self) -> RegGpio7IR {
        RegGpio7IR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_i(&self) -> RegGpio8IR {
        RegGpio8IR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_i(&self) -> RegGpio9IR {
        RegGpio9IR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_i(&self) -> RegGpio10IR {
        RegGpio10IR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_i(&self) -> RegGpio11IR {
        RegGpio11IR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_i(&self) -> RegGpio12IR {
        RegGpio12IR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_i(&self) -> RegGpio13IR {
        RegGpio13IR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_i(&self) -> RegGpio14IR {
        RegGpio14IR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_i(&self) -> RegGpio15IR {
        RegGpio15IR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_i(&self) -> RegGpio16IR {
        RegGpio16IR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_i(&self) -> RegGpio17IR {
        RegGpio17IR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_i(&self) -> RegGpio18IR {
        RegGpio18IR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_i(&self) -> RegGpio19IR {
        RegGpio19IR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_i(&self) -> RegGpio20IR {
        RegGpio20IR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_i(&self) -> RegGpio21IR {
        RegGpio21IR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_i(&self) -> RegGpio22IR {
        RegGpio22IR::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Input register for all GPIO pins. Input Enabled bit must be set in configuration register to work.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl30::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl30Spec;
impl crate::RegisterSpec for GpioCfgctl30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl30::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl30Spec {}
#[doc = "`reset()` method sets GPIO_CFGCTL30 to value 0"]
impl crate::Resettable for GpioCfgctl30Spec {}
