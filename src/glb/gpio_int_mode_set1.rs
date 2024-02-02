#[doc = "Register `GPIO_INT_MODE_SET1` reader"]
pub type R = crate::R<GpioIntModeSet1Spec>;
#[doc = "Register `GPIO_INT_MODE_SET1` writer"]
pub type W = crate::W<GpioIntModeSet1Spec>;
#[doc = "Interrupt trigger mode register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio0triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio0triggerMode {}
#[doc = "Field `reg_gpio_0_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO0."]
pub type RegGpio0InterruptTriggerModeR = crate::FieldReader<Gpio0triggerMode>;
impl RegGpio0InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0triggerMode {
        match self.bits {
            0 => Gpio0triggerMode::NegativePulse,
            1 => Gpio0triggerMode::PositivePulse,
            2 => Gpio0triggerMode::NegativeLevel,
            3 => Gpio0triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio0triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio0triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio0triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio0triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_0_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO0."]
pub type RegGpio0InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio0triggerMode, crate::Safe>;
impl<'a, REG> RegGpio0InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio0controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio0controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_interrupt_control_mode` reader - Interrupt control mode register for GPIO0."]
pub type RegGpio0InterruptControlModeR = crate::BitReader<Gpio0controlMode>;
impl RegGpio0InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0controlMode {
        match self.bits {
            false => Gpio0controlMode::Synchronous,
            true => Gpio0controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio0controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio0controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_0_interrupt_control_mode` writer - Interrupt control mode register for GPIO0."]
pub type RegGpio0InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio0controlMode>;
impl<'a, REG> RegGpio0InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio1triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio1triggerMode {}
#[doc = "Field `reg_gpio_1_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO1."]
pub type RegGpio1InterruptTriggerModeR = crate::FieldReader<Gpio1triggerMode>;
impl RegGpio1InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1triggerMode {
        match self.bits {
            0 => Gpio1triggerMode::NegativePulse,
            1 => Gpio1triggerMode::PositivePulse,
            2 => Gpio1triggerMode::NegativeLevel,
            3 => Gpio1triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio1triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio1triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio1triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio1triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_1_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO1."]
pub type RegGpio1InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio1triggerMode, crate::Safe>;
impl<'a, REG> RegGpio1InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio1controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio1controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_interrupt_control_mode` reader - Interrupt control mode register for GPIO1."]
pub type RegGpio1InterruptControlModeR = crate::BitReader<Gpio1controlMode>;
impl RegGpio1InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1controlMode {
        match self.bits {
            false => Gpio1controlMode::Synchronous,
            true => Gpio1controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio1controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio1controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_1_interrupt_control_mode` writer - Interrupt control mode register for GPIO1."]
pub type RegGpio1InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio1controlMode>;
impl<'a, REG> RegGpio1InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio2triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio2triggerMode {}
#[doc = "Field `reg_gpio_2_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO2."]
pub type RegGpio2InterruptTriggerModeR = crate::FieldReader<Gpio2triggerMode>;
impl RegGpio2InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2triggerMode {
        match self.bits {
            0 => Gpio2triggerMode::NegativePulse,
            1 => Gpio2triggerMode::PositivePulse,
            2 => Gpio2triggerMode::NegativeLevel,
            3 => Gpio2triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio2triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio2triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio2triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio2triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_2_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO2."]
pub type RegGpio2InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio2triggerMode, crate::Safe>;
impl<'a, REG> RegGpio2InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio2controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio2controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_interrupt_control_mode` reader - Interrupt control mode register for GPIO2."]
pub type RegGpio2InterruptControlModeR = crate::BitReader<Gpio2controlMode>;
impl RegGpio2InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2controlMode {
        match self.bits {
            false => Gpio2controlMode::Synchronous,
            true => Gpio2controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio2controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio2controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_2_interrupt_control_mode` writer - Interrupt control mode register for GPIO2."]
pub type RegGpio2InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio2controlMode>;
impl<'a, REG> RegGpio2InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio3triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio3triggerMode {}
#[doc = "Field `reg_gpio_3_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO3."]
pub type RegGpio3InterruptTriggerModeR = crate::FieldReader<Gpio3triggerMode>;
impl RegGpio3InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3triggerMode {
        match self.bits {
            0 => Gpio3triggerMode::NegativePulse,
            1 => Gpio3triggerMode::PositivePulse,
            2 => Gpio3triggerMode::NegativeLevel,
            3 => Gpio3triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio3triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio3triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio3triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio3triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_3_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO3."]
pub type RegGpio3InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio3triggerMode, crate::Safe>;
impl<'a, REG> RegGpio3InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio3controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio3controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_interrupt_control_mode` reader - Interrupt control mode register for GPIO3."]
pub type RegGpio3InterruptControlModeR = crate::BitReader<Gpio3controlMode>;
impl RegGpio3InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3controlMode {
        match self.bits {
            false => Gpio3controlMode::Synchronous,
            true => Gpio3controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio3controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio3controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_3_interrupt_control_mode` writer - Interrupt control mode register for GPIO3."]
pub type RegGpio3InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio3controlMode>;
impl<'a, REG> RegGpio3InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio4triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio4triggerMode {}
#[doc = "Field `reg_gpio_4_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO4."]
pub type RegGpio4InterruptTriggerModeR = crate::FieldReader<Gpio4triggerMode>;
impl RegGpio4InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4triggerMode {
        match self.bits {
            0 => Gpio4triggerMode::NegativePulse,
            1 => Gpio4triggerMode::PositivePulse,
            2 => Gpio4triggerMode::NegativeLevel,
            3 => Gpio4triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio4triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio4triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio4triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio4triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_4_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO4."]
pub type RegGpio4InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio4triggerMode, crate::Safe>;
impl<'a, REG> RegGpio4InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio4controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio4controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_interrupt_control_mode` reader - Interrupt control mode register for GPIO4."]
pub type RegGpio4InterruptControlModeR = crate::BitReader<Gpio4controlMode>;
impl RegGpio4InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4controlMode {
        match self.bits {
            false => Gpio4controlMode::Synchronous,
            true => Gpio4controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio4controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio4controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_4_interrupt_control_mode` writer - Interrupt control mode register for GPIO4."]
pub type RegGpio4InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio4controlMode>;
impl<'a, REG> RegGpio4InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio5triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio5triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio5triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio5triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio5triggerMode {}
#[doc = "Field `reg_gpio_5_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO5."]
pub type RegGpio5InterruptTriggerModeR = crate::FieldReader<Gpio5triggerMode>;
impl RegGpio5InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5triggerMode {
        match self.bits {
            0 => Gpio5triggerMode::NegativePulse,
            1 => Gpio5triggerMode::PositivePulse,
            2 => Gpio5triggerMode::NegativeLevel,
            3 => Gpio5triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio5triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio5triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio5triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio5triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_5_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO5."]
pub type RegGpio5InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio5triggerMode, crate::Safe>;
impl<'a, REG> RegGpio5InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio5controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio5controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_interrupt_control_mode` reader - Interrupt control mode register for GPIO5."]
pub type RegGpio5InterruptControlModeR = crate::BitReader<Gpio5controlMode>;
impl RegGpio5InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5controlMode {
        match self.bits {
            false => Gpio5controlMode::Synchronous,
            true => Gpio5controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio5controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio5controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_5_interrupt_control_mode` writer - Interrupt control mode register for GPIO5."]
pub type RegGpio5InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio5controlMode>;
impl<'a, REG> RegGpio5InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio6triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio6triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio6triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio6triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio6triggerMode {}
#[doc = "Field `reg_gpio_6_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO6."]
pub type RegGpio6InterruptTriggerModeR = crate::FieldReader<Gpio6triggerMode>;
impl RegGpio6InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6triggerMode {
        match self.bits {
            0 => Gpio6triggerMode::NegativePulse,
            1 => Gpio6triggerMode::PositivePulse,
            2 => Gpio6triggerMode::NegativeLevel,
            3 => Gpio6triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio6triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio6triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio6triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio6triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_6_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO6."]
pub type RegGpio6InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio6triggerMode, crate::Safe>;
impl<'a, REG> RegGpio6InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio6controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio6controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_interrupt_control_mode` reader - Interrupt control mode register for GPIO6."]
pub type RegGpio6InterruptControlModeR = crate::BitReader<Gpio6controlMode>;
impl RegGpio6InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6controlMode {
        match self.bits {
            false => Gpio6controlMode::Synchronous,
            true => Gpio6controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio6controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio6controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_6_interrupt_control_mode` writer - Interrupt control mode register for GPIO6."]
pub type RegGpio6InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio6controlMode>;
impl<'a, REG> RegGpio6InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio7triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio7triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio7triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio7triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio7triggerMode {}
#[doc = "Field `reg_gpio_7_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO7."]
pub type RegGpio7InterruptTriggerModeR = crate::FieldReader<Gpio7triggerMode>;
impl RegGpio7InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7triggerMode {
        match self.bits {
            0 => Gpio7triggerMode::NegativePulse,
            1 => Gpio7triggerMode::PositivePulse,
            2 => Gpio7triggerMode::NegativeLevel,
            3 => Gpio7triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio7triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio7triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio7triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio7triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_7_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO7."]
pub type RegGpio7InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio7triggerMode, crate::Safe>;
impl<'a, REG> RegGpio7InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio7controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio7controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_interrupt_control_mode` reader - Interrupt control mode register for GPIO7."]
pub type RegGpio7InterruptControlModeR = crate::BitReader<Gpio7controlMode>;
impl RegGpio7InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7controlMode {
        match self.bits {
            false => Gpio7controlMode::Synchronous,
            true => Gpio7controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio7controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio7controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_7_interrupt_control_mode` writer - Interrupt control mode register for GPIO7."]
pub type RegGpio7InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio7controlMode>;
impl<'a, REG> RegGpio7InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio8triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio8triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio8triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio8triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio8triggerMode {}
#[doc = "Field `reg_gpio_8_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO8."]
pub type RegGpio8InterruptTriggerModeR = crate::FieldReader<Gpio8triggerMode>;
impl RegGpio8InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8triggerMode {
        match self.bits {
            0 => Gpio8triggerMode::NegativePulse,
            1 => Gpio8triggerMode::PositivePulse,
            2 => Gpio8triggerMode::NegativeLevel,
            3 => Gpio8triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio8triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio8triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio8triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio8triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_8_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO8."]
pub type RegGpio8InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio8triggerMode, crate::Safe>;
impl<'a, REG> RegGpio8InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio8controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio8controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_interrupt_control_mode` reader - Interrupt control mode register for GPIO8."]
pub type RegGpio8InterruptControlModeR = crate::BitReader<Gpio8controlMode>;
impl RegGpio8InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8controlMode {
        match self.bits {
            false => Gpio8controlMode::Synchronous,
            true => Gpio8controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio8controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio8controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_8_interrupt_control_mode` writer - Interrupt control mode register for GPIO8."]
pub type RegGpio8InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio8controlMode>;
impl<'a, REG> RegGpio8InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio9triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio9triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio9triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio9triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio9triggerMode {}
#[doc = "Field `reg_gpio_9_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO9."]
pub type RegGpio9InterruptTriggerModeR = crate::FieldReader<Gpio9triggerMode>;
impl RegGpio9InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9triggerMode {
        match self.bits {
            0 => Gpio9triggerMode::NegativePulse,
            1 => Gpio9triggerMode::PositivePulse,
            2 => Gpio9triggerMode::NegativeLevel,
            3 => Gpio9triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio9triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio9triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio9triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio9triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_9_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO9."]
pub type RegGpio9InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio9triggerMode, crate::Safe>;
impl<'a, REG> RegGpio9InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio9controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio9controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_interrupt_control_mode` reader - Interrupt control mode register for GPIO9."]
pub type RegGpio9InterruptControlModeR = crate::BitReader<Gpio9controlMode>;
impl RegGpio9InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9controlMode {
        match self.bits {
            false => Gpio9controlMode::Synchronous,
            true => Gpio9controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio9controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio9controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_9_interrupt_control_mode` writer - Interrupt control mode register for GPIO9."]
pub type RegGpio9InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio9controlMode>;
impl<'a, REG> RegGpio9InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9controlMode::Asynchronous)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_trigger_mode(&self) -> RegGpio0InterruptTriggerModeR {
        RegGpio0InterruptTriggerModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_control_mode(&self) -> RegGpio0InterruptControlModeR {
        RegGpio0InterruptControlModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_trigger_mode(&self) -> RegGpio1InterruptTriggerModeR {
        RegGpio1InterruptTriggerModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_control_mode(&self) -> RegGpio1InterruptControlModeR {
        RegGpio1InterruptControlModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_trigger_mode(&self) -> RegGpio2InterruptTriggerModeR {
        RegGpio2InterruptTriggerModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_control_mode(&self) -> RegGpio2InterruptControlModeR {
        RegGpio2InterruptControlModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_trigger_mode(&self) -> RegGpio3InterruptTriggerModeR {
        RegGpio3InterruptTriggerModeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_control_mode(&self) -> RegGpio3InterruptControlModeR {
        RegGpio3InterruptControlModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_trigger_mode(&self) -> RegGpio4InterruptTriggerModeR {
        RegGpio4InterruptTriggerModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_control_mode(&self) -> RegGpio4InterruptControlModeR {
        RegGpio4InterruptControlModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_trigger_mode(&self) -> RegGpio5InterruptTriggerModeR {
        RegGpio5InterruptTriggerModeR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_control_mode(&self) -> RegGpio5InterruptControlModeR {
        RegGpio5InterruptControlModeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_trigger_mode(&self) -> RegGpio6InterruptTriggerModeR {
        RegGpio6InterruptTriggerModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_control_mode(&self) -> RegGpio6InterruptControlModeR {
        RegGpio6InterruptControlModeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_trigger_mode(&self) -> RegGpio7InterruptTriggerModeR {
        RegGpio7InterruptTriggerModeR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_control_mode(&self) -> RegGpio7InterruptControlModeR {
        RegGpio7InterruptControlModeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_trigger_mode(&self) -> RegGpio8InterruptTriggerModeR {
        RegGpio8InterruptTriggerModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_control_mode(&self) -> RegGpio8InterruptControlModeR {
        RegGpio8InterruptControlModeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_trigger_mode(&self) -> RegGpio9InterruptTriggerModeR {
        RegGpio9InterruptTriggerModeR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_control_mode(&self) -> RegGpio9InterruptControlModeR {
        RegGpio9InterruptControlModeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio0InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio0InterruptTriggerModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_control_mode(
        &mut self,
    ) -> RegGpio0InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio0InterruptControlModeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio1InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio1InterruptTriggerModeW::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_control_mode(
        &mut self,
    ) -> RegGpio1InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio1InterruptControlModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio2InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio2InterruptTriggerModeW::new(self, 6)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_control_mode(
        &mut self,
    ) -> RegGpio2InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio2InterruptControlModeW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio3InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio3InterruptTriggerModeW::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_control_mode(
        &mut self,
    ) -> RegGpio3InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio3InterruptControlModeW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio4InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio4InterruptTriggerModeW::new(self, 12)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_control_mode(
        &mut self,
    ) -> RegGpio4InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio4InterruptControlModeW::new(self, 14)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio5InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio5InterruptTriggerModeW::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_control_mode(
        &mut self,
    ) -> RegGpio5InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio5InterruptControlModeW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio6InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio6InterruptTriggerModeW::new(self, 18)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_control_mode(
        &mut self,
    ) -> RegGpio6InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio6InterruptControlModeW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio7InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio7InterruptTriggerModeW::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_control_mode(
        &mut self,
    ) -> RegGpio7InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio7InterruptControlModeW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio8InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio8InterruptTriggerModeW::new(self, 24)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_control_mode(
        &mut self,
    ) -> RegGpio8InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio8InterruptControlModeW::new(self, 26)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio9InterruptTriggerModeW<'_, GpioIntModeSet1Spec> {
        RegGpio9InterruptTriggerModeW::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_control_mode(
        &mut self,
    ) -> RegGpio9InterruptControlModeW<'_, GpioIntModeSet1Spec> {
        RegGpio9InterruptControlModeW::new(self, 29)
    }
}
#[doc = "GPIO interrupt trigger and control register for GPIO0-GPIO9.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mode_set1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mode_set1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntModeSet1Spec;
impl crate::RegisterSpec for GpioIntModeSet1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_mode_set1::R`](R) reader structure"]
impl crate::Readable for GpioIntModeSet1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_mode_set1::W`](W) writer structure"]
impl crate::Writable for GpioIntModeSet1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET1 to value 0"]
impl crate::Resettable for GpioIntModeSet1Spec {}
