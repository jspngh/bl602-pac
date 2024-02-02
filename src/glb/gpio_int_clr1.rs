#[doc = "Register `GPIO_INT_CLR1` reader"]
pub type R = crate::R<GpioIntClr1Spec>;
#[doc = "Register `GPIO_INT_CLR1` writer"]
pub type W = crate::W<GpioIntClr1Spec>;
#[doc = "Interrupt clearing register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio0interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio0interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_interrupt_clear` reader - Interrupt clearing register for GPIO0."]
pub type RegGpio0InterruptClearR = crate::BitReader<Gpio0interruptClear>;
impl RegGpio0InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0interruptClear {
        match self.bits {
            false => Gpio0interruptClear::NoClear,
            true => Gpio0interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio0interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio0interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_0_interrupt_clear` writer - Interrupt clearing register for GPIO0."]
pub type RegGpio0InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio0interruptClear>;
impl<'a, REG> RegGpio0InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio1interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio1interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_interrupt_clear` reader - Interrupt clearing register for GPIO1."]
pub type RegGpio1InterruptClearR = crate::BitReader<Gpio1interruptClear>;
impl RegGpio1InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1interruptClear {
        match self.bits {
            false => Gpio1interruptClear::NoClear,
            true => Gpio1interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio1interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio1interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_1_interrupt_clear` writer - Interrupt clearing register for GPIO1."]
pub type RegGpio1InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio1interruptClear>;
impl<'a, REG> RegGpio1InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio2interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio2interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_interrupt_clear` reader - Interrupt clearing register for GPIO2."]
pub type RegGpio2InterruptClearR = crate::BitReader<Gpio2interruptClear>;
impl RegGpio2InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2interruptClear {
        match self.bits {
            false => Gpio2interruptClear::NoClear,
            true => Gpio2interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio2interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio2interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_2_interrupt_clear` writer - Interrupt clearing register for GPIO2."]
pub type RegGpio2InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio2interruptClear>;
impl<'a, REG> RegGpio2InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio3interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio3interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_interrupt_clear` reader - Interrupt clearing register for GPIO3."]
pub type RegGpio3InterruptClearR = crate::BitReader<Gpio3interruptClear>;
impl RegGpio3InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3interruptClear {
        match self.bits {
            false => Gpio3interruptClear::NoClear,
            true => Gpio3interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio3interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio3interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_3_interrupt_clear` writer - Interrupt clearing register for GPIO3."]
pub type RegGpio3InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio3interruptClear>;
impl<'a, REG> RegGpio3InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio4interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio4interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_interrupt_clear` reader - Interrupt clearing register for GPIO4."]
pub type RegGpio4InterruptClearR = crate::BitReader<Gpio4interruptClear>;
impl RegGpio4InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4interruptClear {
        match self.bits {
            false => Gpio4interruptClear::NoClear,
            true => Gpio4interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio4interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio4interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_4_interrupt_clear` writer - Interrupt clearing register for GPIO4."]
pub type RegGpio4InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio4interruptClear>;
impl<'a, REG> RegGpio4InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio5interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio5interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_interrupt_clear` reader - Interrupt clearing register for GPIO5."]
pub type RegGpio5InterruptClearR = crate::BitReader<Gpio5interruptClear>;
impl RegGpio5InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5interruptClear {
        match self.bits {
            false => Gpio5interruptClear::NoClear,
            true => Gpio5interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio5interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio5interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_5_interrupt_clear` writer - Interrupt clearing register for GPIO5."]
pub type RegGpio5InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio5interruptClear>;
impl<'a, REG> RegGpio5InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio6interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio6interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_interrupt_clear` reader - Interrupt clearing register for GPIO6."]
pub type RegGpio6InterruptClearR = crate::BitReader<Gpio6interruptClear>;
impl RegGpio6InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6interruptClear {
        match self.bits {
            false => Gpio6interruptClear::NoClear,
            true => Gpio6interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio6interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio6interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_6_interrupt_clear` writer - Interrupt clearing register for GPIO6."]
pub type RegGpio6InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio6interruptClear>;
impl<'a, REG> RegGpio6InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio7interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio7interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_interrupt_clear` reader - Interrupt clearing register for GPIO7."]
pub type RegGpio7InterruptClearR = crate::BitReader<Gpio7interruptClear>;
impl RegGpio7InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7interruptClear {
        match self.bits {
            false => Gpio7interruptClear::NoClear,
            true => Gpio7interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio7interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio7interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_7_interrupt_clear` writer - Interrupt clearing register for GPIO7."]
pub type RegGpio7InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio7interruptClear>;
impl<'a, REG> RegGpio7InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio8interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio8interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_interrupt_clear` reader - Interrupt clearing register for GPIO8."]
pub type RegGpio8InterruptClearR = crate::BitReader<Gpio8interruptClear>;
impl RegGpio8InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8interruptClear {
        match self.bits {
            false => Gpio8interruptClear::NoClear,
            true => Gpio8interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio8interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio8interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_8_interrupt_clear` writer - Interrupt clearing register for GPIO8."]
pub type RegGpio8InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio8interruptClear>;
impl<'a, REG> RegGpio8InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio9interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio9interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_interrupt_clear` reader - Interrupt clearing register for GPIO9."]
pub type RegGpio9InterruptClearR = crate::BitReader<Gpio9interruptClear>;
impl RegGpio9InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9interruptClear {
        match self.bits {
            false => Gpio9interruptClear::NoClear,
            true => Gpio9interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio9interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio9interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_9_interrupt_clear` writer - Interrupt clearing register for GPIO9."]
pub type RegGpio9InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio9interruptClear>;
impl<'a, REG> RegGpio9InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio10interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio10interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_interrupt_clear` reader - Interrupt clearing register for GPIO10."]
pub type RegGpio10InterruptClearR = crate::BitReader<Gpio10interruptClear>;
impl RegGpio10InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10interruptClear {
        match self.bits {
            false => Gpio10interruptClear::NoClear,
            true => Gpio10interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio10interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio10interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_10_interrupt_clear` writer - Interrupt clearing register for GPIO10."]
pub type RegGpio10InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio10interruptClear>;
impl<'a, REG> RegGpio10InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio11interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio11interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_interrupt_clear` reader - Interrupt clearing register for GPIO11."]
pub type RegGpio11InterruptClearR = crate::BitReader<Gpio11interruptClear>;
impl RegGpio11InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11interruptClear {
        match self.bits {
            false => Gpio11interruptClear::NoClear,
            true => Gpio11interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio11interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio11interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_11_interrupt_clear` writer - Interrupt clearing register for GPIO11."]
pub type RegGpio11InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio11interruptClear>;
impl<'a, REG> RegGpio11InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio12interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio12interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_interrupt_clear` reader - Interrupt clearing register for GPIO12."]
pub type RegGpio12InterruptClearR = crate::BitReader<Gpio12interruptClear>;
impl RegGpio12InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12interruptClear {
        match self.bits {
            false => Gpio12interruptClear::NoClear,
            true => Gpio12interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio12interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio12interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_12_interrupt_clear` writer - Interrupt clearing register for GPIO12."]
pub type RegGpio12InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio12interruptClear>;
impl<'a, REG> RegGpio12InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio13interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio13interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_interrupt_clear` reader - Interrupt clearing register for GPIO13."]
pub type RegGpio13InterruptClearR = crate::BitReader<Gpio13interruptClear>;
impl RegGpio13InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13interruptClear {
        match self.bits {
            false => Gpio13interruptClear::NoClear,
            true => Gpio13interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio13interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio13interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_13_interrupt_clear` writer - Interrupt clearing register for GPIO13."]
pub type RegGpio13InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio13interruptClear>;
impl<'a, REG> RegGpio13InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio14interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio14interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_interrupt_clear` reader - Interrupt clearing register for GPIO14."]
pub type RegGpio14InterruptClearR = crate::BitReader<Gpio14interruptClear>;
impl RegGpio14InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14interruptClear {
        match self.bits {
            false => Gpio14interruptClear::NoClear,
            true => Gpio14interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio14interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio14interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_14_interrupt_clear` writer - Interrupt clearing register for GPIO14."]
pub type RegGpio14InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio14interruptClear>;
impl<'a, REG> RegGpio14InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio15interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio15interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_interrupt_clear` reader - Interrupt clearing register for GPIO15."]
pub type RegGpio15InterruptClearR = crate::BitReader<Gpio15interruptClear>;
impl RegGpio15InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15interruptClear {
        match self.bits {
            false => Gpio15interruptClear::NoClear,
            true => Gpio15interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio15interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio15interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_15_interrupt_clear` writer - Interrupt clearing register for GPIO15."]
pub type RegGpio15InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio15interruptClear>;
impl<'a, REG> RegGpio15InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio16interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio16interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_interrupt_clear` reader - Interrupt clearing register for GPIO16."]
pub type RegGpio16InterruptClearR = crate::BitReader<Gpio16interruptClear>;
impl RegGpio16InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16interruptClear {
        match self.bits {
            false => Gpio16interruptClear::NoClear,
            true => Gpio16interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio16interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio16interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_16_interrupt_clear` writer - Interrupt clearing register for GPIO16."]
pub type RegGpio16InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio16interruptClear>;
impl<'a, REG> RegGpio16InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio17interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio17interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_interrupt_clear` reader - Interrupt clearing register for GPIO17."]
pub type RegGpio17InterruptClearR = crate::BitReader<Gpio17interruptClear>;
impl RegGpio17InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17interruptClear {
        match self.bits {
            false => Gpio17interruptClear::NoClear,
            true => Gpio17interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio17interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio17interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_17_interrupt_clear` writer - Interrupt clearing register for GPIO17."]
pub type RegGpio17InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio17interruptClear>;
impl<'a, REG> RegGpio17InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio18interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio18interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_interrupt_clear` reader - Interrupt clearing register for GPIO18."]
pub type RegGpio18InterruptClearR = crate::BitReader<Gpio18interruptClear>;
impl RegGpio18InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18interruptClear {
        match self.bits {
            false => Gpio18interruptClear::NoClear,
            true => Gpio18interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio18interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio18interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_18_interrupt_clear` writer - Interrupt clearing register for GPIO18."]
pub type RegGpio18InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio18interruptClear>;
impl<'a, REG> RegGpio18InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio19interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio19interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_interrupt_clear` reader - Interrupt clearing register for GPIO19."]
pub type RegGpio19InterruptClearR = crate::BitReader<Gpio19interruptClear>;
impl RegGpio19InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19interruptClear {
        match self.bits {
            false => Gpio19interruptClear::NoClear,
            true => Gpio19interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio19interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio19interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_19_interrupt_clear` writer - Interrupt clearing register for GPIO19."]
pub type RegGpio19InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio19interruptClear>;
impl<'a, REG> RegGpio19InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio20interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio20interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_interrupt_clear` reader - Interrupt clearing register for GPIO20."]
pub type RegGpio20InterruptClearR = crate::BitReader<Gpio20interruptClear>;
impl RegGpio20InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20interruptClear {
        match self.bits {
            false => Gpio20interruptClear::NoClear,
            true => Gpio20interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio20interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio20interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_20_interrupt_clear` writer - Interrupt clearing register for GPIO20."]
pub type RegGpio20InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio20interruptClear>;
impl<'a, REG> RegGpio20InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio21interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio21interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_interrupt_clear` reader - Interrupt clearing register for GPIO21."]
pub type RegGpio21InterruptClearR = crate::BitReader<Gpio21interruptClear>;
impl RegGpio21InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21interruptClear {
        match self.bits {
            false => Gpio21interruptClear::NoClear,
            true => Gpio21interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio21interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio21interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_21_interrupt_clear` writer - Interrupt clearing register for GPIO21."]
pub type RegGpio21InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio21interruptClear>;
impl<'a, REG> RegGpio21InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio22interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio22interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_interrupt_clear` reader - Interrupt clearing register for GPIO22."]
pub type RegGpio22InterruptClearR = crate::BitReader<Gpio22interruptClear>;
impl RegGpio22InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22interruptClear {
        match self.bits {
            false => Gpio22interruptClear::NoClear,
            true => Gpio22interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio22interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio22interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_22_interrupt_clear` writer - Interrupt clearing register for GPIO22."]
pub type RegGpio22InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio22interruptClear>;
impl<'a, REG> RegGpio22InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio23interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio23interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio23interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_interrupt_clear` reader - Interrupt clearing register for GPIO23."]
pub type RegGpio23InterruptClearR = crate::BitReader<Gpio23interruptClear>;
impl RegGpio23InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio23interruptClear {
        match self.bits {
            false => Gpio23interruptClear::NoClear,
            true => Gpio23interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio23interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio23interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_23_interrupt_clear` writer - Interrupt clearing register for GPIO23."]
pub type RegGpio23InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio23interruptClear>;
impl<'a, REG> RegGpio23InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio24interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio24interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio24interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_interrupt_clear` reader - Interrupt clearing register for GPIO24."]
pub type RegGpio24InterruptClearR = crate::BitReader<Gpio24interruptClear>;
impl RegGpio24InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio24interruptClear {
        match self.bits {
            false => Gpio24interruptClear::NoClear,
            true => Gpio24interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio24interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio24interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_24_interrupt_clear` writer - Interrupt clearing register for GPIO24."]
pub type RegGpio24InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio24interruptClear>;
impl<'a, REG> RegGpio24InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio25interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio25interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio25interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_interrupt_clear` reader - Interrupt clearing register for GPIO25."]
pub type RegGpio25InterruptClearR = crate::BitReader<Gpio25interruptClear>;
impl RegGpio25InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio25interruptClear {
        match self.bits {
            false => Gpio25interruptClear::NoClear,
            true => Gpio25interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio25interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio25interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_25_interrupt_clear` writer - Interrupt clearing register for GPIO25."]
pub type RegGpio25InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio25interruptClear>;
impl<'a, REG> RegGpio25InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio26interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio26interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio26interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_interrupt_clear` reader - Interrupt clearing register for GPIO26."]
pub type RegGpio26InterruptClearR = crate::BitReader<Gpio26interruptClear>;
impl RegGpio26InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio26interruptClear {
        match self.bits {
            false => Gpio26interruptClear::NoClear,
            true => Gpio26interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio26interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio26interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_26_interrupt_clear` writer - Interrupt clearing register for GPIO26."]
pub type RegGpio26InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio26interruptClear>;
impl<'a, REG> RegGpio26InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio27interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio27interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio27interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_interrupt_clear` reader - Interrupt clearing register for GPIO27."]
pub type RegGpio27InterruptClearR = crate::BitReader<Gpio27interruptClear>;
impl RegGpio27InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio27interruptClear {
        match self.bits {
            false => Gpio27interruptClear::NoClear,
            true => Gpio27interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio27interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio27interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_27_interrupt_clear` writer - Interrupt clearing register for GPIO27."]
pub type RegGpio27InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio27interruptClear>;
impl<'a, REG> RegGpio27InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27interruptClear::Clear)
    }
}
#[doc = "Interrupt clearing register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio28interruptClear {
    #[doc = "0: `0`"]
    NoClear = 0,
    #[doc = "1: `1`"]
    Clear = 1,
}
impl From<Gpio28interruptClear> for bool {
    #[inline(always)]
    fn from(variant: Gpio28interruptClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_interrupt_clear` reader - Interrupt clearing register for GPIO28."]
pub type RegGpio28InterruptClearR = crate::BitReader<Gpio28interruptClear>;
impl RegGpio28InterruptClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio28interruptClear {
        match self.bits {
            false => Gpio28interruptClear::NoClear,
            true => Gpio28interruptClear::Clear,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        *self == Gpio28interruptClear::NoClear
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Gpio28interruptClear::Clear
    }
}
#[doc = "Field `reg_gpio_28_interrupt_clear` writer - Interrupt clearing register for GPIO28."]
pub type RegGpio28InterruptClearW<'a, REG> = crate::BitWriter<'a, REG, Gpio28interruptClear>;
impl<'a, REG> RegGpio28InterruptClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28interruptClear::NoClear)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28interruptClear::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt clearing register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_clear(&self) -> RegGpio0InterruptClearR {
        RegGpio0InterruptClearR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt clearing register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_clear(&self) -> RegGpio1InterruptClearR {
        RegGpio1InterruptClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt clearing register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_clear(&self) -> RegGpio2InterruptClearR {
        RegGpio2InterruptClearR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt clearing register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_clear(&self) -> RegGpio3InterruptClearR {
        RegGpio3InterruptClearR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt clearing register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_clear(&self) -> RegGpio4InterruptClearR {
        RegGpio4InterruptClearR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt clearing register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_clear(&self) -> RegGpio5InterruptClearR {
        RegGpio5InterruptClearR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt clearing register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_clear(&self) -> RegGpio6InterruptClearR {
        RegGpio6InterruptClearR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt clearing register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_clear(&self) -> RegGpio7InterruptClearR {
        RegGpio7InterruptClearR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt clearing register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_clear(&self) -> RegGpio8InterruptClearR {
        RegGpio8InterruptClearR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt clearing register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_clear(&self) -> RegGpio9InterruptClearR {
        RegGpio9InterruptClearR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt clearing register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_clear(&self) -> RegGpio10InterruptClearR {
        RegGpio10InterruptClearR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt clearing register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_clear(&self) -> RegGpio11InterruptClearR {
        RegGpio11InterruptClearR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt clearing register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_clear(&self) -> RegGpio12InterruptClearR {
        RegGpio12InterruptClearR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt clearing register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_clear(&self) -> RegGpio13InterruptClearR {
        RegGpio13InterruptClearR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt clearing register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_clear(&self) -> RegGpio14InterruptClearR {
        RegGpio14InterruptClearR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt clearing register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_clear(&self) -> RegGpio15InterruptClearR {
        RegGpio15InterruptClearR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt clearing register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_clear(&self) -> RegGpio16InterruptClearR {
        RegGpio16InterruptClearR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt clearing register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_clear(&self) -> RegGpio17InterruptClearR {
        RegGpio17InterruptClearR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt clearing register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_clear(&self) -> RegGpio18InterruptClearR {
        RegGpio18InterruptClearR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt clearing register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_clear(&self) -> RegGpio19InterruptClearR {
        RegGpio19InterruptClearR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt clearing register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_clear(&self) -> RegGpio20InterruptClearR {
        RegGpio20InterruptClearR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt clearing register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_clear(&self) -> RegGpio21InterruptClearR {
        RegGpio21InterruptClearR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt clearing register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_clear(&self) -> RegGpio22InterruptClearR {
        RegGpio22InterruptClearR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt clearing register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_clear(&self) -> RegGpio23InterruptClearR {
        RegGpio23InterruptClearR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt clearing register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_clear(&self) -> RegGpio24InterruptClearR {
        RegGpio24InterruptClearR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt clearing register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_clear(&self) -> RegGpio25InterruptClearR {
        RegGpio25InterruptClearR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt clearing register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_clear(&self) -> RegGpio26InterruptClearR {
        RegGpio26InterruptClearR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt clearing register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_clear(&self) -> RegGpio27InterruptClearR {
        RegGpio27InterruptClearR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt clearing register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_clear(&self) -> RegGpio28InterruptClearR {
        RegGpio28InterruptClearR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt clearing register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_clear(&mut self) -> RegGpio0InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio0InterruptClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt clearing register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_clear(&mut self) -> RegGpio1InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio1InterruptClearW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt clearing register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_clear(&mut self) -> RegGpio2InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio2InterruptClearW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt clearing register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_clear(&mut self) -> RegGpio3InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio3InterruptClearW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt clearing register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_clear(&mut self) -> RegGpio4InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio4InterruptClearW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt clearing register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_clear(&mut self) -> RegGpio5InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio5InterruptClearW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt clearing register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_clear(&mut self) -> RegGpio6InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio6InterruptClearW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt clearing register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_clear(&mut self) -> RegGpio7InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio7InterruptClearW::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt clearing register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_clear(&mut self) -> RegGpio8InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio8InterruptClearW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt clearing register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_clear(&mut self) -> RegGpio9InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio9InterruptClearW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt clearing register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_clear(&mut self) -> RegGpio10InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio10InterruptClearW::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt clearing register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_clear(&mut self) -> RegGpio11InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio11InterruptClearW::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt clearing register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_clear(&mut self) -> RegGpio12InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio12InterruptClearW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt clearing register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_clear(&mut self) -> RegGpio13InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio13InterruptClearW::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt clearing register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_clear(&mut self) -> RegGpio14InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio14InterruptClearW::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt clearing register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_clear(&mut self) -> RegGpio15InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio15InterruptClearW::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt clearing register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_clear(&mut self) -> RegGpio16InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio16InterruptClearW::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt clearing register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_clear(&mut self) -> RegGpio17InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio17InterruptClearW::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt clearing register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_clear(&mut self) -> RegGpio18InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio18InterruptClearW::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt clearing register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_clear(&mut self) -> RegGpio19InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio19InterruptClearW::new(self, 19)
    }
    #[doc = "Bit 20 - Interrupt clearing register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_clear(&mut self) -> RegGpio20InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio20InterruptClearW::new(self, 20)
    }
    #[doc = "Bit 21 - Interrupt clearing register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_clear(&mut self) -> RegGpio21InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio21InterruptClearW::new(self, 21)
    }
    #[doc = "Bit 22 - Interrupt clearing register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_clear(&mut self) -> RegGpio22InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio22InterruptClearW::new(self, 22)
    }
    #[doc = "Bit 23 - Interrupt clearing register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_clear(&mut self) -> RegGpio23InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio23InterruptClearW::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt clearing register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_clear(&mut self) -> RegGpio24InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio24InterruptClearW::new(self, 24)
    }
    #[doc = "Bit 25 - Interrupt clearing register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_clear(&mut self) -> RegGpio25InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio25InterruptClearW::new(self, 25)
    }
    #[doc = "Bit 26 - Interrupt clearing register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_clear(&mut self) -> RegGpio26InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio26InterruptClearW::new(self, 26)
    }
    #[doc = "Bit 27 - Interrupt clearing register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_clear(&mut self) -> RegGpio27InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio27InterruptClearW::new(self, 27)
    }
    #[doc = "Bit 28 - Interrupt clearing register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_clear(&mut self) -> RegGpio28InterruptClearW<'_, GpioIntClr1Spec> {
        RegGpio28InterruptClearW::new(self, 28)
    }
}
#[doc = "Interrupt clearing register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_clr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_clr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntClr1Spec;
impl crate::RegisterSpec for GpioIntClr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_clr1::R`](R) reader structure"]
impl crate::Readable for GpioIntClr1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_clr1::W`](W) writer structure"]
impl crate::Writable for GpioIntClr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_CLR1 to value 0"]
impl crate::Resettable for GpioIntClr1Spec {}
