#[doc = "Register `GPIO_INT_STAT1` reader"]
pub type R = crate::R<GpioIntStat1Spec>;
#[doc = "Interrupt status register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio0interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio0interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_interrupt_status` reader - Interrupt status register for GPIO0."]
pub type RegGpio0InterruptStatusR = crate::BitReader<Gpio0interruptStatus>;
impl RegGpio0InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0interruptStatus {
        match self.bits {
            false => Gpio0interruptStatus::Reset,
            true => Gpio0interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio0interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio0interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio1interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio1interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_interrupt_status` reader - Interrupt status register for GPIO1."]
pub type RegGpio1InterruptStatusR = crate::BitReader<Gpio1interruptStatus>;
impl RegGpio1InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1interruptStatus {
        match self.bits {
            false => Gpio1interruptStatus::Reset,
            true => Gpio1interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio1interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio1interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio2interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio2interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_interrupt_status` reader - Interrupt status register for GPIO2."]
pub type RegGpio2InterruptStatusR = crate::BitReader<Gpio2interruptStatus>;
impl RegGpio2InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2interruptStatus {
        match self.bits {
            false => Gpio2interruptStatus::Reset,
            true => Gpio2interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio2interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio2interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio3interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio3interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_interrupt_status` reader - Interrupt status register for GPIO3."]
pub type RegGpio3InterruptStatusR = crate::BitReader<Gpio3interruptStatus>;
impl RegGpio3InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3interruptStatus {
        match self.bits {
            false => Gpio3interruptStatus::Reset,
            true => Gpio3interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio3interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio3interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio4interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio4interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_interrupt_status` reader - Interrupt status register for GPIO4."]
pub type RegGpio4InterruptStatusR = crate::BitReader<Gpio4interruptStatus>;
impl RegGpio4InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4interruptStatus {
        match self.bits {
            false => Gpio4interruptStatus::Reset,
            true => Gpio4interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio4interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio4interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio5interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio5interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_interrupt_status` reader - Interrupt status register for GPIO5."]
pub type RegGpio5InterruptStatusR = crate::BitReader<Gpio5interruptStatus>;
impl RegGpio5InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5interruptStatus {
        match self.bits {
            false => Gpio5interruptStatus::Reset,
            true => Gpio5interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio5interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio5interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio6interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio6interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_interrupt_status` reader - Interrupt status register for GPIO6."]
pub type RegGpio6InterruptStatusR = crate::BitReader<Gpio6interruptStatus>;
impl RegGpio6InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6interruptStatus {
        match self.bits {
            false => Gpio6interruptStatus::Reset,
            true => Gpio6interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio6interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio6interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio7interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio7interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_interrupt_status` reader - Interrupt status register for GPIO7."]
pub type RegGpio7InterruptStatusR = crate::BitReader<Gpio7interruptStatus>;
impl RegGpio7InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7interruptStatus {
        match self.bits {
            false => Gpio7interruptStatus::Reset,
            true => Gpio7interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio7interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio7interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio8interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio8interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_interrupt_status` reader - Interrupt status register for GPIO8."]
pub type RegGpio8InterruptStatusR = crate::BitReader<Gpio8interruptStatus>;
impl RegGpio8InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8interruptStatus {
        match self.bits {
            false => Gpio8interruptStatus::Reset,
            true => Gpio8interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio8interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio8interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio9interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio9interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_interrupt_status` reader - Interrupt status register for GPIO9."]
pub type RegGpio9InterruptStatusR = crate::BitReader<Gpio9interruptStatus>;
impl RegGpio9InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9interruptStatus {
        match self.bits {
            false => Gpio9interruptStatus::Reset,
            true => Gpio9interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio9interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio9interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio10interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio10interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_interrupt_status` reader - Interrupt status register for GPIO10."]
pub type RegGpio10InterruptStatusR = crate::BitReader<Gpio10interruptStatus>;
impl RegGpio10InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10interruptStatus {
        match self.bits {
            false => Gpio10interruptStatus::Reset,
            true => Gpio10interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio10interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio10interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio11interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio11interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_interrupt_status` reader - Interrupt status register for GPIO11."]
pub type RegGpio11InterruptStatusR = crate::BitReader<Gpio11interruptStatus>;
impl RegGpio11InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11interruptStatus {
        match self.bits {
            false => Gpio11interruptStatus::Reset,
            true => Gpio11interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio11interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio11interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio12interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio12interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_interrupt_status` reader - Interrupt status register for GPIO12."]
pub type RegGpio12InterruptStatusR = crate::BitReader<Gpio12interruptStatus>;
impl RegGpio12InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12interruptStatus {
        match self.bits {
            false => Gpio12interruptStatus::Reset,
            true => Gpio12interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio12interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio12interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio13interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio13interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_interrupt_status` reader - Interrupt status register for GPIO13."]
pub type RegGpio13InterruptStatusR = crate::BitReader<Gpio13interruptStatus>;
impl RegGpio13InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13interruptStatus {
        match self.bits {
            false => Gpio13interruptStatus::Reset,
            true => Gpio13interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio13interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio13interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio14interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio14interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_interrupt_status` reader - Interrupt status register for GPIO14."]
pub type RegGpio14InterruptStatusR = crate::BitReader<Gpio14interruptStatus>;
impl RegGpio14InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14interruptStatus {
        match self.bits {
            false => Gpio14interruptStatus::Reset,
            true => Gpio14interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio14interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio14interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio15interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio15interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_interrupt_status` reader - Interrupt status register for GPIO15."]
pub type RegGpio15InterruptStatusR = crate::BitReader<Gpio15interruptStatus>;
impl RegGpio15InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15interruptStatus {
        match self.bits {
            false => Gpio15interruptStatus::Reset,
            true => Gpio15interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio15interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio15interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio16interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio16interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_interrupt_status` reader - Interrupt status register for GPIO16."]
pub type RegGpio16InterruptStatusR = crate::BitReader<Gpio16interruptStatus>;
impl RegGpio16InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16interruptStatus {
        match self.bits {
            false => Gpio16interruptStatus::Reset,
            true => Gpio16interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio16interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio16interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio17interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio17interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_interrupt_status` reader - Interrupt status register for GPIO17."]
pub type RegGpio17InterruptStatusR = crate::BitReader<Gpio17interruptStatus>;
impl RegGpio17InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17interruptStatus {
        match self.bits {
            false => Gpio17interruptStatus::Reset,
            true => Gpio17interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio17interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio17interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio18interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio18interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_interrupt_status` reader - Interrupt status register for GPIO18."]
pub type RegGpio18InterruptStatusR = crate::BitReader<Gpio18interruptStatus>;
impl RegGpio18InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18interruptStatus {
        match self.bits {
            false => Gpio18interruptStatus::Reset,
            true => Gpio18interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio18interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio18interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio19interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio19interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_interrupt_status` reader - Interrupt status register for GPIO19."]
pub type RegGpio19InterruptStatusR = crate::BitReader<Gpio19interruptStatus>;
impl RegGpio19InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19interruptStatus {
        match self.bits {
            false => Gpio19interruptStatus::Reset,
            true => Gpio19interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio19interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio19interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio20interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio20interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_interrupt_status` reader - Interrupt status register for GPIO20."]
pub type RegGpio20InterruptStatusR = crate::BitReader<Gpio20interruptStatus>;
impl RegGpio20InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20interruptStatus {
        match self.bits {
            false => Gpio20interruptStatus::Reset,
            true => Gpio20interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio20interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio20interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio21interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio21interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_interrupt_status` reader - Interrupt status register for GPIO21."]
pub type RegGpio21InterruptStatusR = crate::BitReader<Gpio21interruptStatus>;
impl RegGpio21InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21interruptStatus {
        match self.bits {
            false => Gpio21interruptStatus::Reset,
            true => Gpio21interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio21interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio21interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio22interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio22interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_interrupt_status` reader - Interrupt status register for GPIO22."]
pub type RegGpio22InterruptStatusR = crate::BitReader<Gpio22interruptStatus>;
impl RegGpio22InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22interruptStatus {
        match self.bits {
            false => Gpio22interruptStatus::Reset,
            true => Gpio22interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio22interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio22interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio23interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio23interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio23interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_interrupt_status` reader - Interrupt status register for GPIO23."]
pub type RegGpio23InterruptStatusR = crate::BitReader<Gpio23interruptStatus>;
impl RegGpio23InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio23interruptStatus {
        match self.bits {
            false => Gpio23interruptStatus::Reset,
            true => Gpio23interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio23interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio23interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio24interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio24interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio24interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_interrupt_status` reader - Interrupt status register for GPIO24."]
pub type RegGpio24InterruptStatusR = crate::BitReader<Gpio24interruptStatus>;
impl RegGpio24InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio24interruptStatus {
        match self.bits {
            false => Gpio24interruptStatus::Reset,
            true => Gpio24interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio24interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio24interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio25interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio25interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio25interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_interrupt_status` reader - Interrupt status register for GPIO25."]
pub type RegGpio25InterruptStatusR = crate::BitReader<Gpio25interruptStatus>;
impl RegGpio25InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio25interruptStatus {
        match self.bits {
            false => Gpio25interruptStatus::Reset,
            true => Gpio25interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio25interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio25interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio26interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio26interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio26interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_interrupt_status` reader - Interrupt status register for GPIO26."]
pub type RegGpio26InterruptStatusR = crate::BitReader<Gpio26interruptStatus>;
impl RegGpio26InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio26interruptStatus {
        match self.bits {
            false => Gpio26interruptStatus::Reset,
            true => Gpio26interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio26interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio26interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio27interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio27interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio27interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_interrupt_status` reader - Interrupt status register for GPIO27."]
pub type RegGpio27InterruptStatusR = crate::BitReader<Gpio27interruptStatus>;
impl RegGpio27InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio27interruptStatus {
        match self.bits {
            false => Gpio27interruptStatus::Reset,
            true => Gpio27interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio27interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio27interruptStatus::Set
    }
}
#[doc = "Interrupt status register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio28interruptStatus {
    #[doc = "0: `0`"]
    Reset = 0,
    #[doc = "1: `1`"]
    Set = 1,
}
impl From<Gpio28interruptStatus> for bool {
    #[inline(always)]
    fn from(variant: Gpio28interruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_interrupt_status` reader - Interrupt status register for GPIO28."]
pub type RegGpio28InterruptStatusR = crate::BitReader<Gpio28interruptStatus>;
impl RegGpio28InterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio28interruptStatus {
        match self.bits {
            false => Gpio28interruptStatus::Reset,
            true => Gpio28interruptStatus::Set,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gpio28interruptStatus::Reset
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Gpio28interruptStatus::Set
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt status register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_status(&self) -> RegGpio0InterruptStatusR {
        RegGpio0InterruptStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_status(&self) -> RegGpio1InterruptStatusR {
        RegGpio1InterruptStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_status(&self) -> RegGpio2InterruptStatusR {
        RegGpio2InterruptStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_status(&self) -> RegGpio3InterruptStatusR {
        RegGpio3InterruptStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_status(&self) -> RegGpio4InterruptStatusR {
        RegGpio4InterruptStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_status(&self) -> RegGpio5InterruptStatusR {
        RegGpio5InterruptStatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_status(&self) -> RegGpio6InterruptStatusR {
        RegGpio6InterruptStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt status register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_status(&self) -> RegGpio7InterruptStatusR {
        RegGpio7InterruptStatusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt status register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_status(&self) -> RegGpio8InterruptStatusR {
        RegGpio8InterruptStatusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt status register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_status(&self) -> RegGpio9InterruptStatusR {
        RegGpio9InterruptStatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt status register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_status(&self) -> RegGpio10InterruptStatusR {
        RegGpio10InterruptStatusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt status register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_status(&self) -> RegGpio11InterruptStatusR {
        RegGpio11InterruptStatusR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_status(&self) -> RegGpio12InterruptStatusR {
        RegGpio12InterruptStatusR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt status register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_status(&self) -> RegGpio13InterruptStatusR {
        RegGpio13InterruptStatusR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt status register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_status(&self) -> RegGpio14InterruptStatusR {
        RegGpio14InterruptStatusR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt status register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_status(&self) -> RegGpio15InterruptStatusR {
        RegGpio15InterruptStatusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt status register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_status(&self) -> RegGpio16InterruptStatusR {
        RegGpio16InterruptStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt status register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_status(&self) -> RegGpio17InterruptStatusR {
        RegGpio17InterruptStatusR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt status register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_status(&self) -> RegGpio18InterruptStatusR {
        RegGpio18InterruptStatusR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt status register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_status(&self) -> RegGpio19InterruptStatusR {
        RegGpio19InterruptStatusR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt status register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_status(&self) -> RegGpio20InterruptStatusR {
        RegGpio20InterruptStatusR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt status register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_status(&self) -> RegGpio21InterruptStatusR {
        RegGpio21InterruptStatusR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt status register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_status(&self) -> RegGpio22InterruptStatusR {
        RegGpio22InterruptStatusR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt status register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_status(&self) -> RegGpio23InterruptStatusR {
        RegGpio23InterruptStatusR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt status register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_status(&self) -> RegGpio24InterruptStatusR {
        RegGpio24InterruptStatusR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt status register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_status(&self) -> RegGpio25InterruptStatusR {
        RegGpio25InterruptStatusR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt status register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_status(&self) -> RegGpio26InterruptStatusR {
        RegGpio26InterruptStatusR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt status register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_status(&self) -> RegGpio27InterruptStatusR {
        RegGpio27InterruptStatusR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt status register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_status(&self) -> RegGpio28InterruptStatusR {
        RegGpio28InterruptStatusR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt status register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_stat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntStat1Spec;
impl crate::RegisterSpec for GpioIntStat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_stat1::R`](R) reader structure"]
impl crate::Readable for GpioIntStat1Spec {}
#[doc = "`reset()` method sets GPIO_INT_STAT1 to value 0"]
impl crate::Resettable for GpioIntStat1Spec {}
