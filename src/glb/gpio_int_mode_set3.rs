#[doc = "Register `GPIO_INT_MODE_SET3` reader"]
pub type R = crate::R<GpioIntModeSet3Spec>;
#[doc = "Register `GPIO_INT_MODE_SET3` writer"]
pub type W = crate::W<GpioIntModeSet3Spec>;
#[doc = "Interrupt trigger mode register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio20triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio20triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio20triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio20triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio20triggerMode {}
#[doc = "Field `reg_gpio_20_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO20."]
pub type RegGpio20InterruptTriggerModeR = crate::FieldReader<Gpio20triggerMode>;
impl RegGpio20InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20triggerMode {
        match self.bits {
            0 => Gpio20triggerMode::NegativePulse,
            1 => Gpio20triggerMode::PositivePulse,
            2 => Gpio20triggerMode::NegativeLevel,
            3 => Gpio20triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio20triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio20triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio20triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio20triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_20_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO20."]
pub type RegGpio20InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio20triggerMode, crate::Safe>;
impl<'a, REG> RegGpio20InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio20controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio20controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_interrupt_control_mode` reader - Interrupt control mode register for GPIO20."]
pub type RegGpio20InterruptControlModeR = crate::BitReader<Gpio20controlMode>;
impl RegGpio20InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20controlMode {
        match self.bits {
            false => Gpio20controlMode::Synchronous,
            true => Gpio20controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio20controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio20controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_20_interrupt_control_mode` writer - Interrupt control mode register for GPIO20."]
pub type RegGpio20InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio20controlMode>;
impl<'a, REG> RegGpio20InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio21triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio21triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio21triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio21triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio21triggerMode {}
#[doc = "Field `reg_gpio_21_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO21."]
pub type RegGpio21InterruptTriggerModeR = crate::FieldReader<Gpio21triggerMode>;
impl RegGpio21InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21triggerMode {
        match self.bits {
            0 => Gpio21triggerMode::NegativePulse,
            1 => Gpio21triggerMode::PositivePulse,
            2 => Gpio21triggerMode::NegativeLevel,
            3 => Gpio21triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio21triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio21triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio21triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio21triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_21_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO21."]
pub type RegGpio21InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio21triggerMode, crate::Safe>;
impl<'a, REG> RegGpio21InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio21controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio21controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_interrupt_control_mode` reader - Interrupt control mode register for GPIO21."]
pub type RegGpio21InterruptControlModeR = crate::BitReader<Gpio21controlMode>;
impl RegGpio21InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21controlMode {
        match self.bits {
            false => Gpio21controlMode::Synchronous,
            true => Gpio21controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio21controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio21controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_21_interrupt_control_mode` writer - Interrupt control mode register for GPIO21."]
pub type RegGpio21InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio21controlMode>;
impl<'a, REG> RegGpio21InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio22triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio22triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio22triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio22triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio22triggerMode {}
#[doc = "Field `reg_gpio_22_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO22."]
pub type RegGpio22InterruptTriggerModeR = crate::FieldReader<Gpio22triggerMode>;
impl RegGpio22InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22triggerMode {
        match self.bits {
            0 => Gpio22triggerMode::NegativePulse,
            1 => Gpio22triggerMode::PositivePulse,
            2 => Gpio22triggerMode::NegativeLevel,
            3 => Gpio22triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio22triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio22triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio22triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio22triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_22_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO22."]
pub type RegGpio22InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio22triggerMode, crate::Safe>;
impl<'a, REG> RegGpio22InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio22controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio22controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_interrupt_control_mode` reader - Interrupt control mode register for GPIO22."]
pub type RegGpio22InterruptControlModeR = crate::BitReader<Gpio22controlMode>;
impl RegGpio22InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22controlMode {
        match self.bits {
            false => Gpio22controlMode::Synchronous,
            true => Gpio22controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio22controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio22controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_22_interrupt_control_mode` writer - Interrupt control mode register for GPIO22."]
pub type RegGpio22InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio22controlMode>;
impl<'a, REG> RegGpio22InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio23triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio23triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio23triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio23triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio23triggerMode {}
#[doc = "Field `reg_gpio_23_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO23."]
pub type RegGpio23InterruptTriggerModeR = crate::FieldReader<Gpio23triggerMode>;
impl RegGpio23InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio23triggerMode {
        match self.bits {
            0 => Gpio23triggerMode::NegativePulse,
            1 => Gpio23triggerMode::PositivePulse,
            2 => Gpio23triggerMode::NegativeLevel,
            3 => Gpio23triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio23triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio23triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio23triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio23triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_23_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO23."]
pub type RegGpio23InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio23triggerMode, crate::Safe>;
impl<'a, REG> RegGpio23InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio23controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio23controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio23controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_interrupt_control_mode` reader - Interrupt control mode register for GPIO23."]
pub type RegGpio23InterruptControlModeR = crate::BitReader<Gpio23controlMode>;
impl RegGpio23InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio23controlMode {
        match self.bits {
            false => Gpio23controlMode::Synchronous,
            true => Gpio23controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio23controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio23controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_23_interrupt_control_mode` writer - Interrupt control mode register for GPIO23."]
pub type RegGpio23InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio23controlMode>;
impl<'a, REG> RegGpio23InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio24triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio24triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio24triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio24triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio24triggerMode {}
#[doc = "Field `reg_gpio_24_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO24."]
pub type RegGpio24InterruptTriggerModeR = crate::FieldReader<Gpio24triggerMode>;
impl RegGpio24InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio24triggerMode {
        match self.bits {
            0 => Gpio24triggerMode::NegativePulse,
            1 => Gpio24triggerMode::PositivePulse,
            2 => Gpio24triggerMode::NegativeLevel,
            3 => Gpio24triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio24triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio24triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio24triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio24triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_24_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO24."]
pub type RegGpio24InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio24triggerMode, crate::Safe>;
impl<'a, REG> RegGpio24InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio24controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio24controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio24controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_interrupt_control_mode` reader - Interrupt control mode register for GPIO24."]
pub type RegGpio24InterruptControlModeR = crate::BitReader<Gpio24controlMode>;
impl RegGpio24InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio24controlMode {
        match self.bits {
            false => Gpio24controlMode::Synchronous,
            true => Gpio24controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio24controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio24controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_24_interrupt_control_mode` writer - Interrupt control mode register for GPIO24."]
pub type RegGpio24InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio24controlMode>;
impl<'a, REG> RegGpio24InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio25triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio25triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio25triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio25triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio25triggerMode {}
#[doc = "Field `reg_gpio_25_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO25."]
pub type RegGpio25InterruptTriggerModeR = crate::FieldReader<Gpio25triggerMode>;
impl RegGpio25InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio25triggerMode {
        match self.bits {
            0 => Gpio25triggerMode::NegativePulse,
            1 => Gpio25triggerMode::PositivePulse,
            2 => Gpio25triggerMode::NegativeLevel,
            3 => Gpio25triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio25triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio25triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio25triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio25triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_25_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO25."]
pub type RegGpio25InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio25triggerMode, crate::Safe>;
impl<'a, REG> RegGpio25InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio25controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio25controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio25controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_interrupt_control_mode` reader - Interrupt control mode register for GPIO25."]
pub type RegGpio25InterruptControlModeR = crate::BitReader<Gpio25controlMode>;
impl RegGpio25InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio25controlMode {
        match self.bits {
            false => Gpio25controlMode::Synchronous,
            true => Gpio25controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio25controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio25controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_25_interrupt_control_mode` writer - Interrupt control mode register for GPIO25."]
pub type RegGpio25InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio25controlMode>;
impl<'a, REG> RegGpio25InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio26triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio26triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio26triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio26triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio26triggerMode {}
#[doc = "Field `reg_gpio_26_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO26."]
pub type RegGpio26InterruptTriggerModeR = crate::FieldReader<Gpio26triggerMode>;
impl RegGpio26InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio26triggerMode {
        match self.bits {
            0 => Gpio26triggerMode::NegativePulse,
            1 => Gpio26triggerMode::PositivePulse,
            2 => Gpio26triggerMode::NegativeLevel,
            3 => Gpio26triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio26triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio26triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio26triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio26triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_26_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO26."]
pub type RegGpio26InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio26triggerMode, crate::Safe>;
impl<'a, REG> RegGpio26InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio26controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio26controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio26controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_interrupt_control_mode` reader - Interrupt control mode register for GPIO26."]
pub type RegGpio26InterruptControlModeR = crate::BitReader<Gpio26controlMode>;
impl RegGpio26InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio26controlMode {
        match self.bits {
            false => Gpio26controlMode::Synchronous,
            true => Gpio26controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio26controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio26controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_26_interrupt_control_mode` writer - Interrupt control mode register for GPIO26."]
pub type RegGpio26InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio26controlMode>;
impl<'a, REG> RegGpio26InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio27triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio27triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio27triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio27triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio27triggerMode {}
#[doc = "Field `reg_gpio_27_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO27."]
pub type RegGpio27InterruptTriggerModeR = crate::FieldReader<Gpio27triggerMode>;
impl RegGpio27InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio27triggerMode {
        match self.bits {
            0 => Gpio27triggerMode::NegativePulse,
            1 => Gpio27triggerMode::PositivePulse,
            2 => Gpio27triggerMode::NegativeLevel,
            3 => Gpio27triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio27triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio27triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio27triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio27triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_27_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO27."]
pub type RegGpio27InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio27triggerMode, crate::Safe>;
impl<'a, REG> RegGpio27InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio27controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio27controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio27controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_interrupt_control_mode` reader - Interrupt control mode register for GPIO27."]
pub type RegGpio27InterruptControlModeR = crate::BitReader<Gpio27controlMode>;
impl RegGpio27InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio27controlMode {
        match self.bits {
            false => Gpio27controlMode::Synchronous,
            true => Gpio27controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio27controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio27controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_27_interrupt_control_mode` writer - Interrupt control mode register for GPIO27."]
pub type RegGpio27InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio27controlMode>;
impl<'a, REG> RegGpio27InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio28triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio28triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio28triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio28triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio28triggerMode {}
#[doc = "Field `reg_gpio_28_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO28."]
pub type RegGpio28InterruptTriggerModeR = crate::FieldReader<Gpio28triggerMode>;
impl RegGpio28InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio28triggerMode {
        match self.bits {
            0 => Gpio28triggerMode::NegativePulse,
            1 => Gpio28triggerMode::PositivePulse,
            2 => Gpio28triggerMode::NegativeLevel,
            3 => Gpio28triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio28triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio28triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio28triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio28triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_28_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO28."]
pub type RegGpio28InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio28triggerMode, crate::Safe>;
impl<'a, REG> RegGpio28InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio28controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio28controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio28controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_interrupt_control_mode` reader - Interrupt control mode register for GPIO28."]
pub type RegGpio28InterruptControlModeR = crate::BitReader<Gpio28controlMode>;
impl RegGpio28InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio28controlMode {
        match self.bits {
            false => Gpio28controlMode::Synchronous,
            true => Gpio28controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio28controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio28controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_28_interrupt_control_mode` writer - Interrupt control mode register for GPIO28."]
pub type RegGpio28InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio28controlMode>;
impl<'a, REG> RegGpio28InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio29triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio29triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio29triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio29triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio29triggerMode {}
#[doc = "Field `reg_gpio_29_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO29."]
pub type RegGpio29InterruptTriggerModeR = crate::FieldReader<Gpio29triggerMode>;
impl RegGpio29InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio29triggerMode {
        match self.bits {
            0 => Gpio29triggerMode::NegativePulse,
            1 => Gpio29triggerMode::PositivePulse,
            2 => Gpio29triggerMode::NegativeLevel,
            3 => Gpio29triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio29triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio29triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio29triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio29triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_29_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO29."]
pub type RegGpio29InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio29triggerMode, crate::Safe>;
impl<'a, REG> RegGpio29InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio29triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio29triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio29triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio29triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio29controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio29controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio29controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_29_interrupt_control_mode` reader - Interrupt control mode register for GPIO29."]
pub type RegGpio29InterruptControlModeR = crate::BitReader<Gpio29controlMode>;
impl RegGpio29InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio29controlMode {
        match self.bits {
            false => Gpio29controlMode::Synchronous,
            true => Gpio29controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio29controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio29controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_29_interrupt_control_mode` writer - Interrupt control mode register for GPIO29."]
pub type RegGpio29InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio29controlMode>;
impl<'a, REG> RegGpio29InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio29controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio29controlMode::Asynchronous)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_trigger_mode(&self) -> RegGpio20InterruptTriggerModeR {
        RegGpio20InterruptTriggerModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_control_mode(&self) -> RegGpio20InterruptControlModeR {
        RegGpio20InterruptControlModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_trigger_mode(&self) -> RegGpio21InterruptTriggerModeR {
        RegGpio21InterruptTriggerModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_control_mode(&self) -> RegGpio21InterruptControlModeR {
        RegGpio21InterruptControlModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_trigger_mode(&self) -> RegGpio22InterruptTriggerModeR {
        RegGpio22InterruptTriggerModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_control_mode(&self) -> RegGpio22InterruptControlModeR {
        RegGpio22InterruptControlModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_trigger_mode(&self) -> RegGpio23InterruptTriggerModeR {
        RegGpio23InterruptTriggerModeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_control_mode(&self) -> RegGpio23InterruptControlModeR {
        RegGpio23InterruptControlModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_trigger_mode(&self) -> RegGpio24InterruptTriggerModeR {
        RegGpio24InterruptTriggerModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_control_mode(&self) -> RegGpio24InterruptControlModeR {
        RegGpio24InterruptControlModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_trigger_mode(&self) -> RegGpio25InterruptTriggerModeR {
        RegGpio25InterruptTriggerModeR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_control_mode(&self) -> RegGpio25InterruptControlModeR {
        RegGpio25InterruptControlModeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_trigger_mode(&self) -> RegGpio26InterruptTriggerModeR {
        RegGpio26InterruptTriggerModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_control_mode(&self) -> RegGpio26InterruptControlModeR {
        RegGpio26InterruptControlModeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_trigger_mode(&self) -> RegGpio27InterruptTriggerModeR {
        RegGpio27InterruptTriggerModeR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_control_mode(&self) -> RegGpio27InterruptControlModeR {
        RegGpio27InterruptControlModeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_trigger_mode(&self) -> RegGpio28InterruptTriggerModeR {
        RegGpio28InterruptTriggerModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_control_mode(&self) -> RegGpio28InterruptControlModeR {
        RegGpio28InterruptControlModeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_trigger_mode(&self) -> RegGpio29InterruptTriggerModeR {
        RegGpio29InterruptTriggerModeR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_control_mode(&self) -> RegGpio29InterruptControlModeR {
        RegGpio29InterruptControlModeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio20InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio20InterruptTriggerModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_control_mode(
        &mut self,
    ) -> RegGpio20InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio20InterruptControlModeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio21InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio21InterruptTriggerModeW::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_control_mode(
        &mut self,
    ) -> RegGpio21InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio21InterruptControlModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio22InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio22InterruptTriggerModeW::new(self, 6)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_control_mode(
        &mut self,
    ) -> RegGpio22InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio22InterruptControlModeW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio23InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio23InterruptTriggerModeW::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_control_mode(
        &mut self,
    ) -> RegGpio23InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio23InterruptControlModeW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio24InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio24InterruptTriggerModeW::new(self, 12)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_control_mode(
        &mut self,
    ) -> RegGpio24InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio24InterruptControlModeW::new(self, 14)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio25InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio25InterruptTriggerModeW::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_control_mode(
        &mut self,
    ) -> RegGpio25InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio25InterruptControlModeW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio26InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio26InterruptTriggerModeW::new(self, 18)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_control_mode(
        &mut self,
    ) -> RegGpio26InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio26InterruptControlModeW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio27InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio27InterruptTriggerModeW::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_control_mode(
        &mut self,
    ) -> RegGpio27InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio27InterruptControlModeW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio28InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio28InterruptTriggerModeW::new(self, 24)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_control_mode(
        &mut self,
    ) -> RegGpio28InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio28InterruptControlModeW::new(self, 26)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio29InterruptTriggerModeW<'_, GpioIntModeSet3Spec> {
        RegGpio29InterruptTriggerModeW::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_control_mode(
        &mut self,
    ) -> RegGpio29InterruptControlModeW<'_, GpioIntModeSet3Spec> {
        RegGpio29InterruptControlModeW::new(self, 29)
    }
}
#[doc = "GPIO interrupt trigger and control register for GPIO20-GPIO29.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mode_set3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mode_set3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntModeSet3Spec;
impl crate::RegisterSpec for GpioIntModeSet3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_mode_set3::R`](R) reader structure"]
impl crate::Readable for GpioIntModeSet3Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_mode_set3::W`](W) writer structure"]
impl crate::Writable for GpioIntModeSet3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET3 to value 0"]
impl crate::Resettable for GpioIntModeSet3Spec {}
