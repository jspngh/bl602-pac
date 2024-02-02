#[doc = "Register `GPIO_INT_MODE_SET2` reader"]
pub type R = crate::R<GpioIntModeSet2Spec>;
#[doc = "Register `GPIO_INT_MODE_SET2` writer"]
pub type W = crate::W<GpioIntModeSet2Spec>;
#[doc = "Interrupt trigger mode register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio10triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio10triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio10triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio10triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio10triggerMode {}
#[doc = "Field `reg_gpio_10_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO10."]
pub type RegGpio10InterruptTriggerModeR = crate::FieldReader<Gpio10triggerMode>;
impl RegGpio10InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10triggerMode {
        match self.bits {
            0 => Gpio10triggerMode::NegativePulse,
            1 => Gpio10triggerMode::PositivePulse,
            2 => Gpio10triggerMode::NegativeLevel,
            3 => Gpio10triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio10triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio10triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio10triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio10triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_10_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO10."]
pub type RegGpio10InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio10triggerMode, crate::Safe>;
impl<'a, REG> RegGpio10InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio10controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio10controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_interrupt_control_mode` reader - Interrupt control mode register for GPIO10."]
pub type RegGpio10InterruptControlModeR = crate::BitReader<Gpio10controlMode>;
impl RegGpio10InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10controlMode {
        match self.bits {
            false => Gpio10controlMode::Synchronous,
            true => Gpio10controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio10controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio10controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_10_interrupt_control_mode` writer - Interrupt control mode register for GPIO10."]
pub type RegGpio10InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio10controlMode>;
impl<'a, REG> RegGpio10InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio11triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio11triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio11triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio11triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio11triggerMode {}
#[doc = "Field `reg_gpio_11_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO11."]
pub type RegGpio11InterruptTriggerModeR = crate::FieldReader<Gpio11triggerMode>;
impl RegGpio11InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11triggerMode {
        match self.bits {
            0 => Gpio11triggerMode::NegativePulse,
            1 => Gpio11triggerMode::PositivePulse,
            2 => Gpio11triggerMode::NegativeLevel,
            3 => Gpio11triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio11triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio11triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio11triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio11triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_11_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO11."]
pub type RegGpio11InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio11triggerMode, crate::Safe>;
impl<'a, REG> RegGpio11InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio11controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio11controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_interrupt_control_mode` reader - Interrupt control mode register for GPIO11."]
pub type RegGpio11InterruptControlModeR = crate::BitReader<Gpio11controlMode>;
impl RegGpio11InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11controlMode {
        match self.bits {
            false => Gpio11controlMode::Synchronous,
            true => Gpio11controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio11controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio11controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_11_interrupt_control_mode` writer - Interrupt control mode register for GPIO11."]
pub type RegGpio11InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio11controlMode>;
impl<'a, REG> RegGpio11InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio12triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio12triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio12triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio12triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio12triggerMode {}
#[doc = "Field `reg_gpio_12_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO12."]
pub type RegGpio12InterruptTriggerModeR = crate::FieldReader<Gpio12triggerMode>;
impl RegGpio12InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12triggerMode {
        match self.bits {
            0 => Gpio12triggerMode::NegativePulse,
            1 => Gpio12triggerMode::PositivePulse,
            2 => Gpio12triggerMode::NegativeLevel,
            3 => Gpio12triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio12triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio12triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio12triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio12triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_12_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO12."]
pub type RegGpio12InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio12triggerMode, crate::Safe>;
impl<'a, REG> RegGpio12InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio12controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio12controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_interrupt_control_mode` reader - Interrupt control mode register for GPIO12."]
pub type RegGpio12InterruptControlModeR = crate::BitReader<Gpio12controlMode>;
impl RegGpio12InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12controlMode {
        match self.bits {
            false => Gpio12controlMode::Synchronous,
            true => Gpio12controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio12controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio12controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_12_interrupt_control_mode` writer - Interrupt control mode register for GPIO12."]
pub type RegGpio12InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio12controlMode>;
impl<'a, REG> RegGpio12InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio13triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio13triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio13triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio13triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio13triggerMode {}
#[doc = "Field `reg_gpio_13_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO13."]
pub type RegGpio13InterruptTriggerModeR = crate::FieldReader<Gpio13triggerMode>;
impl RegGpio13InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13triggerMode {
        match self.bits {
            0 => Gpio13triggerMode::NegativePulse,
            1 => Gpio13triggerMode::PositivePulse,
            2 => Gpio13triggerMode::NegativeLevel,
            3 => Gpio13triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio13triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio13triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio13triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio13triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_13_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO13."]
pub type RegGpio13InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio13triggerMode, crate::Safe>;
impl<'a, REG> RegGpio13InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio13controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio13controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_interrupt_control_mode` reader - Interrupt control mode register for GPIO13."]
pub type RegGpio13InterruptControlModeR = crate::BitReader<Gpio13controlMode>;
impl RegGpio13InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13controlMode {
        match self.bits {
            false => Gpio13controlMode::Synchronous,
            true => Gpio13controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio13controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio13controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_13_interrupt_control_mode` writer - Interrupt control mode register for GPIO13."]
pub type RegGpio13InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio13controlMode>;
impl<'a, REG> RegGpio13InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio14triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio14triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio14triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio14triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio14triggerMode {}
#[doc = "Field `reg_gpio_14_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO14."]
pub type RegGpio14InterruptTriggerModeR = crate::FieldReader<Gpio14triggerMode>;
impl RegGpio14InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14triggerMode {
        match self.bits {
            0 => Gpio14triggerMode::NegativePulse,
            1 => Gpio14triggerMode::PositivePulse,
            2 => Gpio14triggerMode::NegativeLevel,
            3 => Gpio14triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio14triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio14triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio14triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio14triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_14_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO14."]
pub type RegGpio14InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio14triggerMode, crate::Safe>;
impl<'a, REG> RegGpio14InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio14controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio14controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_interrupt_control_mode` reader - Interrupt control mode register for GPIO14."]
pub type RegGpio14InterruptControlModeR = crate::BitReader<Gpio14controlMode>;
impl RegGpio14InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14controlMode {
        match self.bits {
            false => Gpio14controlMode::Synchronous,
            true => Gpio14controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio14controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio14controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_14_interrupt_control_mode` writer - Interrupt control mode register for GPIO14."]
pub type RegGpio14InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio14controlMode>;
impl<'a, REG> RegGpio14InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio15triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio15triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio15triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio15triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio15triggerMode {}
#[doc = "Field `reg_gpio_15_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO15."]
pub type RegGpio15InterruptTriggerModeR = crate::FieldReader<Gpio15triggerMode>;
impl RegGpio15InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15triggerMode {
        match self.bits {
            0 => Gpio15triggerMode::NegativePulse,
            1 => Gpio15triggerMode::PositivePulse,
            2 => Gpio15triggerMode::NegativeLevel,
            3 => Gpio15triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio15triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio15triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio15triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio15triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_15_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO15."]
pub type RegGpio15InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio15triggerMode, crate::Safe>;
impl<'a, REG> RegGpio15InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio15controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio15controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_interrupt_control_mode` reader - Interrupt control mode register for GPIO15."]
pub type RegGpio15InterruptControlModeR = crate::BitReader<Gpio15controlMode>;
impl RegGpio15InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15controlMode {
        match self.bits {
            false => Gpio15controlMode::Synchronous,
            true => Gpio15controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio15controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio15controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_15_interrupt_control_mode` writer - Interrupt control mode register for GPIO15."]
pub type RegGpio15InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio15controlMode>;
impl<'a, REG> RegGpio15InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio16triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio16triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio16triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio16triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio16triggerMode {}
#[doc = "Field `reg_gpio_16_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO16."]
pub type RegGpio16InterruptTriggerModeR = crate::FieldReader<Gpio16triggerMode>;
impl RegGpio16InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16triggerMode {
        match self.bits {
            0 => Gpio16triggerMode::NegativePulse,
            1 => Gpio16triggerMode::PositivePulse,
            2 => Gpio16triggerMode::NegativeLevel,
            3 => Gpio16triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio16triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio16triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio16triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio16triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_16_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO16."]
pub type RegGpio16InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio16triggerMode, crate::Safe>;
impl<'a, REG> RegGpio16InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio16controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio16controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_interrupt_control_mode` reader - Interrupt control mode register for GPIO16."]
pub type RegGpio16InterruptControlModeR = crate::BitReader<Gpio16controlMode>;
impl RegGpio16InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16controlMode {
        match self.bits {
            false => Gpio16controlMode::Synchronous,
            true => Gpio16controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio16controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio16controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_16_interrupt_control_mode` writer - Interrupt control mode register for GPIO16."]
pub type RegGpio16InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio16controlMode>;
impl<'a, REG> RegGpio16InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio17triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio17triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio17triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio17triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio17triggerMode {}
#[doc = "Field `reg_gpio_17_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO17."]
pub type RegGpio17InterruptTriggerModeR = crate::FieldReader<Gpio17triggerMode>;
impl RegGpio17InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17triggerMode {
        match self.bits {
            0 => Gpio17triggerMode::NegativePulse,
            1 => Gpio17triggerMode::PositivePulse,
            2 => Gpio17triggerMode::NegativeLevel,
            3 => Gpio17triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio17triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio17triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio17triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio17triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_17_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO17."]
pub type RegGpio17InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio17triggerMode, crate::Safe>;
impl<'a, REG> RegGpio17InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio17controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio17controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_interrupt_control_mode` reader - Interrupt control mode register for GPIO17."]
pub type RegGpio17InterruptControlModeR = crate::BitReader<Gpio17controlMode>;
impl RegGpio17InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17controlMode {
        match self.bits {
            false => Gpio17controlMode::Synchronous,
            true => Gpio17controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio17controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio17controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_17_interrupt_control_mode` writer - Interrupt control mode register for GPIO17."]
pub type RegGpio17InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio17controlMode>;
impl<'a, REG> RegGpio17InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio18triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio18triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio18triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio18triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio18triggerMode {}
#[doc = "Field `reg_gpio_18_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO18."]
pub type RegGpio18InterruptTriggerModeR = crate::FieldReader<Gpio18triggerMode>;
impl RegGpio18InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18triggerMode {
        match self.bits {
            0 => Gpio18triggerMode::NegativePulse,
            1 => Gpio18triggerMode::PositivePulse,
            2 => Gpio18triggerMode::NegativeLevel,
            3 => Gpio18triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio18triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio18triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio18triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio18triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_18_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO18."]
pub type RegGpio18InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio18triggerMode, crate::Safe>;
impl<'a, REG> RegGpio18InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio18controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio18controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_interrupt_control_mode` reader - Interrupt control mode register for GPIO18."]
pub type RegGpio18InterruptControlModeR = crate::BitReader<Gpio18controlMode>;
impl RegGpio18InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18controlMode {
        match self.bits {
            false => Gpio18controlMode::Synchronous,
            true => Gpio18controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio18controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio18controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_18_interrupt_control_mode` writer - Interrupt control mode register for GPIO18."]
pub type RegGpio18InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio18controlMode>;
impl<'a, REG> RegGpio18InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18controlMode::Asynchronous)
    }
}
#[doc = "Interrupt trigger mode register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio19triggerMode {
    #[doc = "0: `0`"]
    NegativePulse = 0,
    #[doc = "1: `1`"]
    PositivePulse = 1,
    #[doc = "2: `10`"]
    NegativeLevel = 2,
    #[doc = "3: `11`"]
    PositiveLevel = 3,
}
impl From<Gpio19triggerMode> for u8 {
    #[inline(always)]
    fn from(variant: Gpio19triggerMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio19triggerMode {
    type Ux = u8;
}
impl crate::IsEnum for Gpio19triggerMode {}
#[doc = "Field `reg_gpio_19_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO19."]
pub type RegGpio19InterruptTriggerModeR = crate::FieldReader<Gpio19triggerMode>;
impl RegGpio19InterruptTriggerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19triggerMode {
        match self.bits {
            0 => Gpio19triggerMode::NegativePulse,
            1 => Gpio19triggerMode::PositivePulse,
            2 => Gpio19triggerMode::NegativeLevel,
            3 => Gpio19triggerMode::PositiveLevel,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == Gpio19triggerMode::NegativePulse
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == Gpio19triggerMode::PositivePulse
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == Gpio19triggerMode::NegativeLevel
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == Gpio19triggerMode::PositiveLevel
    }
}
#[doc = "Field `reg_gpio_19_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO19."]
pub type RegGpio19InterruptTriggerModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Gpio19triggerMode, crate::Safe>;
impl<'a, REG> RegGpio19InterruptTriggerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19triggerMode::NegativePulse)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19triggerMode::PositivePulse)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19triggerMode::NegativeLevel)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19triggerMode::PositiveLevel)
    }
}
#[doc = "Interrupt control mode register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19controlMode {
    #[doc = "0: `0`"]
    Synchronous = 0,
    #[doc = "1: `1`"]
    Asynchronous = 1,
}
impl From<Gpio19controlMode> for bool {
    #[inline(always)]
    fn from(variant: Gpio19controlMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_interrupt_control_mode` reader - Interrupt control mode register for GPIO19."]
pub type RegGpio19InterruptControlModeR = crate::BitReader<Gpio19controlMode>;
impl RegGpio19InterruptControlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19controlMode {
        match self.bits {
            false => Gpio19controlMode::Synchronous,
            true => Gpio19controlMode::Asynchronous,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Gpio19controlMode::Synchronous
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Gpio19controlMode::Asynchronous
    }
}
#[doc = "Field `reg_gpio_19_interrupt_control_mode` writer - Interrupt control mode register for GPIO19."]
pub type RegGpio19InterruptControlModeW<'a, REG> = crate::BitWriter<'a, REG, Gpio19controlMode>;
impl<'a, REG> RegGpio19InterruptControlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19controlMode::Synchronous)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19controlMode::Asynchronous)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_trigger_mode(&self) -> RegGpio10InterruptTriggerModeR {
        RegGpio10InterruptTriggerModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_control_mode(&self) -> RegGpio10InterruptControlModeR {
        RegGpio10InterruptControlModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_trigger_mode(&self) -> RegGpio11InterruptTriggerModeR {
        RegGpio11InterruptTriggerModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_control_mode(&self) -> RegGpio11InterruptControlModeR {
        RegGpio11InterruptControlModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_trigger_mode(&self) -> RegGpio12InterruptTriggerModeR {
        RegGpio12InterruptTriggerModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_control_mode(&self) -> RegGpio12InterruptControlModeR {
        RegGpio12InterruptControlModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_trigger_mode(&self) -> RegGpio13InterruptTriggerModeR {
        RegGpio13InterruptTriggerModeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_control_mode(&self) -> RegGpio13InterruptControlModeR {
        RegGpio13InterruptControlModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_trigger_mode(&self) -> RegGpio14InterruptTriggerModeR {
        RegGpio14InterruptTriggerModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_control_mode(&self) -> RegGpio14InterruptControlModeR {
        RegGpio14InterruptControlModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_trigger_mode(&self) -> RegGpio15InterruptTriggerModeR {
        RegGpio15InterruptTriggerModeR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_control_mode(&self) -> RegGpio15InterruptControlModeR {
        RegGpio15InterruptControlModeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_trigger_mode(&self) -> RegGpio16InterruptTriggerModeR {
        RegGpio16InterruptTriggerModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_control_mode(&self) -> RegGpio16InterruptControlModeR {
        RegGpio16InterruptControlModeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_trigger_mode(&self) -> RegGpio17InterruptTriggerModeR {
        RegGpio17InterruptTriggerModeR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_control_mode(&self) -> RegGpio17InterruptControlModeR {
        RegGpio17InterruptControlModeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_trigger_mode(&self) -> RegGpio18InterruptTriggerModeR {
        RegGpio18InterruptTriggerModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_control_mode(&self) -> RegGpio18InterruptControlModeR {
        RegGpio18InterruptControlModeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_trigger_mode(&self) -> RegGpio19InterruptTriggerModeR {
        RegGpio19InterruptTriggerModeR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_control_mode(&self) -> RegGpio19InterruptControlModeR {
        RegGpio19InterruptControlModeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio10InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio10InterruptTriggerModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_control_mode(
        &mut self,
    ) -> RegGpio10InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio10InterruptControlModeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio11InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio11InterruptTriggerModeW::new(self, 3)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_control_mode(
        &mut self,
    ) -> RegGpio11InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio11InterruptControlModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio12InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio12InterruptTriggerModeW::new(self, 6)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_control_mode(
        &mut self,
    ) -> RegGpio12InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio12InterruptControlModeW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio13InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio13InterruptTriggerModeW::new(self, 9)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_control_mode(
        &mut self,
    ) -> RegGpio13InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio13InterruptControlModeW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio14InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio14InterruptTriggerModeW::new(self, 12)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_control_mode(
        &mut self,
    ) -> RegGpio14InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio14InterruptControlModeW::new(self, 14)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio15InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio15InterruptTriggerModeW::new(self, 15)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_control_mode(
        &mut self,
    ) -> RegGpio15InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio15InterruptControlModeW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio16InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio16InterruptTriggerModeW::new(self, 18)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_control_mode(
        &mut self,
    ) -> RegGpio16InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio16InterruptControlModeW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio17InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio17InterruptTriggerModeW::new(self, 21)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_control_mode(
        &mut self,
    ) -> RegGpio17InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio17InterruptControlModeW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio18InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio18InterruptTriggerModeW::new(self, 24)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_control_mode(
        &mut self,
    ) -> RegGpio18InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio18InterruptControlModeW::new(self, 26)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_trigger_mode(
        &mut self,
    ) -> RegGpio19InterruptTriggerModeW<'_, GpioIntModeSet2Spec> {
        RegGpio19InterruptTriggerModeW::new(self, 27)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_control_mode(
        &mut self,
    ) -> RegGpio19InterruptControlModeW<'_, GpioIntModeSet2Spec> {
        RegGpio19InterruptControlModeW::new(self, 29)
    }
}
#[doc = "GPIO interrupt trigger and control register for GPIO10-GPIO19.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mode_set2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mode_set2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntModeSet2Spec;
impl crate::RegisterSpec for GpioIntModeSet2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_mode_set2::R`](R) reader structure"]
impl crate::Readable for GpioIntModeSet2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_mode_set2::W`](W) writer structure"]
impl crate::Writable for GpioIntModeSet2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET2 to value 0"]
impl crate::Resettable for GpioIntModeSet2Spec {}
