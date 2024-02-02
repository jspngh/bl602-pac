#[doc = "Register `GPIO_CFGCTL34` reader"]
pub type R = crate::R<GpioCfgctl34Spec>;
#[doc = "Register `GPIO_CFGCTL34` writer"]
pub type W = crate::W<GpioCfgctl34Spec>;
#[doc = "Output enable register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio0outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio0outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_oe` reader - Output enable register for GPIO0."]
pub type RegGpio0OeR = crate::BitReader<Gpio0outputEnable>;
impl RegGpio0OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0outputEnable {
        match self.bits {
            false => Gpio0outputEnable::Disabled,
            true => Gpio0outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio0outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio0outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_0_oe` writer - Output enable register for GPIO0."]
pub type RegGpio0OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio0outputEnable>;
impl<'a, REG> RegGpio0OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio1outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio1outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_oe` reader - Output enable register for GPIO1."]
pub type RegGpio1OeR = crate::BitReader<Gpio1outputEnable>;
impl RegGpio1OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1outputEnable {
        match self.bits {
            false => Gpio1outputEnable::Disabled,
            true => Gpio1outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio1outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio1outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_1_oe` writer - Output enable register for GPIO1."]
pub type RegGpio1OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio1outputEnable>;
impl<'a, REG> RegGpio1OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio2outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio2outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_oe` reader - Output enable register for GPIO2."]
pub type RegGpio2OeR = crate::BitReader<Gpio2outputEnable>;
impl RegGpio2OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2outputEnable {
        match self.bits {
            false => Gpio2outputEnable::Disabled,
            true => Gpio2outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio2outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio2outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_2_oe` writer - Output enable register for GPIO2."]
pub type RegGpio2OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio2outputEnable>;
impl<'a, REG> RegGpio2OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio3outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio3outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_oe` reader - Output enable register for GPIO3."]
pub type RegGpio3OeR = crate::BitReader<Gpio3outputEnable>;
impl RegGpio3OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3outputEnable {
        match self.bits {
            false => Gpio3outputEnable::Disabled,
            true => Gpio3outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio3outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio3outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_3_oe` writer - Output enable register for GPIO3."]
pub type RegGpio3OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio3outputEnable>;
impl<'a, REG> RegGpio3OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio4outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio4outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_oe` reader - Output enable register for GPIO4."]
pub type RegGpio4OeR = crate::BitReader<Gpio4outputEnable>;
impl RegGpio4OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4outputEnable {
        match self.bits {
            false => Gpio4outputEnable::Disabled,
            true => Gpio4outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio4outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio4outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_4_oe` writer - Output enable register for GPIO4."]
pub type RegGpio4OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio4outputEnable>;
impl<'a, REG> RegGpio4OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio5outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio5outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_oe` reader - Output enable register for GPIO5."]
pub type RegGpio5OeR = crate::BitReader<Gpio5outputEnable>;
impl RegGpio5OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5outputEnable {
        match self.bits {
            false => Gpio5outputEnable::Disabled,
            true => Gpio5outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio5outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio5outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_5_oe` writer - Output enable register for GPIO5."]
pub type RegGpio5OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio5outputEnable>;
impl<'a, REG> RegGpio5OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio6outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio6outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_oe` reader - Output enable register for GPIO6."]
pub type RegGpio6OeR = crate::BitReader<Gpio6outputEnable>;
impl RegGpio6OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6outputEnable {
        match self.bits {
            false => Gpio6outputEnable::Disabled,
            true => Gpio6outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio6outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio6outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_6_oe` writer - Output enable register for GPIO6."]
pub type RegGpio6OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio6outputEnable>;
impl<'a, REG> RegGpio6OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio7outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio7outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_oe` reader - Output enable register for GPIO7."]
pub type RegGpio7OeR = crate::BitReader<Gpio7outputEnable>;
impl RegGpio7OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7outputEnable {
        match self.bits {
            false => Gpio7outputEnable::Disabled,
            true => Gpio7outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio7outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio7outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_7_oe` writer - Output enable register for GPIO7."]
pub type RegGpio7OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio7outputEnable>;
impl<'a, REG> RegGpio7OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio8outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio8outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_oe` reader - Output enable register for GPIO8."]
pub type RegGpio8OeR = crate::BitReader<Gpio8outputEnable>;
impl RegGpio8OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8outputEnable {
        match self.bits {
            false => Gpio8outputEnable::Disabled,
            true => Gpio8outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio8outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio8outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_8_oe` writer - Output enable register for GPIO8."]
pub type RegGpio8OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio8outputEnable>;
impl<'a, REG> RegGpio8OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio9outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio9outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_oe` reader - Output enable register for GPIO9."]
pub type RegGpio9OeR = crate::BitReader<Gpio9outputEnable>;
impl RegGpio9OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9outputEnable {
        match self.bits {
            false => Gpio9outputEnable::Disabled,
            true => Gpio9outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio9outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio9outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_9_oe` writer - Output enable register for GPIO9."]
pub type RegGpio9OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio9outputEnable>;
impl<'a, REG> RegGpio9OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio10outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio10outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_oe` reader - Output enable register for GPIO10."]
pub type RegGpio10OeR = crate::BitReader<Gpio10outputEnable>;
impl RegGpio10OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10outputEnable {
        match self.bits {
            false => Gpio10outputEnable::Disabled,
            true => Gpio10outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio10outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio10outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_10_oe` writer - Output enable register for GPIO10."]
pub type RegGpio10OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio10outputEnable>;
impl<'a, REG> RegGpio10OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio11outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio11outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_oe` reader - Output enable register for GPIO11."]
pub type RegGpio11OeR = crate::BitReader<Gpio11outputEnable>;
impl RegGpio11OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11outputEnable {
        match self.bits {
            false => Gpio11outputEnable::Disabled,
            true => Gpio11outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio11outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio11outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_11_oe` writer - Output enable register for GPIO11."]
pub type RegGpio11OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio11outputEnable>;
impl<'a, REG> RegGpio11OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio12outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio12outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_oe` reader - Output enable register for GPIO12."]
pub type RegGpio12OeR = crate::BitReader<Gpio12outputEnable>;
impl RegGpio12OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12outputEnable {
        match self.bits {
            false => Gpio12outputEnable::Disabled,
            true => Gpio12outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio12outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio12outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_12_oe` writer - Output enable register for GPIO12."]
pub type RegGpio12OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio12outputEnable>;
impl<'a, REG> RegGpio12OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio13outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio13outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_oe` reader - Output enable register for GPIO13."]
pub type RegGpio13OeR = crate::BitReader<Gpio13outputEnable>;
impl RegGpio13OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13outputEnable {
        match self.bits {
            false => Gpio13outputEnable::Disabled,
            true => Gpio13outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio13outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio13outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_13_oe` writer - Output enable register for GPIO13."]
pub type RegGpio13OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio13outputEnable>;
impl<'a, REG> RegGpio13OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio14outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio14outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_oe` reader - Output enable register for GPIO14."]
pub type RegGpio14OeR = crate::BitReader<Gpio14outputEnable>;
impl RegGpio14OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14outputEnable {
        match self.bits {
            false => Gpio14outputEnable::Disabled,
            true => Gpio14outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio14outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio14outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_14_oe` writer - Output enable register for GPIO14."]
pub type RegGpio14OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio14outputEnable>;
impl<'a, REG> RegGpio14OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio15outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio15outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_oe` reader - Output enable register for GPIO15."]
pub type RegGpio15OeR = crate::BitReader<Gpio15outputEnable>;
impl RegGpio15OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15outputEnable {
        match self.bits {
            false => Gpio15outputEnable::Disabled,
            true => Gpio15outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio15outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio15outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_15_oe` writer - Output enable register for GPIO15."]
pub type RegGpio15OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio15outputEnable>;
impl<'a, REG> RegGpio15OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio16outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio16outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_oe` reader - Output enable register for GPIO16."]
pub type RegGpio16OeR = crate::BitReader<Gpio16outputEnable>;
impl RegGpio16OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16outputEnable {
        match self.bits {
            false => Gpio16outputEnable::Disabled,
            true => Gpio16outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio16outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio16outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_16_oe` writer - Output enable register for GPIO16."]
pub type RegGpio16OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio16outputEnable>;
impl<'a, REG> RegGpio16OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio17outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio17outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_oe` reader - Output enable register for GPIO17."]
pub type RegGpio17OeR = crate::BitReader<Gpio17outputEnable>;
impl RegGpio17OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17outputEnable {
        match self.bits {
            false => Gpio17outputEnable::Disabled,
            true => Gpio17outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio17outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio17outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_17_oe` writer - Output enable register for GPIO17."]
pub type RegGpio17OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio17outputEnable>;
impl<'a, REG> RegGpio17OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio18outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio18outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_oe` reader - Output enable register for GPIO18."]
pub type RegGpio18OeR = crate::BitReader<Gpio18outputEnable>;
impl RegGpio18OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18outputEnable {
        match self.bits {
            false => Gpio18outputEnable::Disabled,
            true => Gpio18outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio18outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio18outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_18_oe` writer - Output enable register for GPIO18."]
pub type RegGpio18OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio18outputEnable>;
impl<'a, REG> RegGpio18OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio19outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio19outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_oe` reader - Output enable register for GPIO19."]
pub type RegGpio19OeR = crate::BitReader<Gpio19outputEnable>;
impl RegGpio19OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19outputEnable {
        match self.bits {
            false => Gpio19outputEnable::Disabled,
            true => Gpio19outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio19outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio19outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_19_oe` writer - Output enable register for GPIO19."]
pub type RegGpio19OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio19outputEnable>;
impl<'a, REG> RegGpio19OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio20outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio20outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_oe` reader - Output enable register for GPIO20."]
pub type RegGpio20OeR = crate::BitReader<Gpio20outputEnable>;
impl RegGpio20OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20outputEnable {
        match self.bits {
            false => Gpio20outputEnable::Disabled,
            true => Gpio20outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio20outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio20outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_20_oe` writer - Output enable register for GPIO20."]
pub type RegGpio20OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio20outputEnable>;
impl<'a, REG> RegGpio20OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio21outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio21outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_oe` reader - Output enable register for GPIO21."]
pub type RegGpio21OeR = crate::BitReader<Gpio21outputEnable>;
impl RegGpio21OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21outputEnable {
        match self.bits {
            false => Gpio21outputEnable::Disabled,
            true => Gpio21outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio21outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio21outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_21_oe` writer - Output enable register for GPIO21."]
pub type RegGpio21OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio21outputEnable>;
impl<'a, REG> RegGpio21OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21outputEnable::Enabled)
    }
}
#[doc = "Output enable register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22outputEnable {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio22outputEnable> for bool {
    #[inline(always)]
    fn from(variant: Gpio22outputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_oe` reader - Output enable register for GPIO22."]
pub type RegGpio22OeR = crate::BitReader<Gpio22outputEnable>;
impl RegGpio22OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22outputEnable {
        match self.bits {
            false => Gpio22outputEnable::Disabled,
            true => Gpio22outputEnable::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio22outputEnable::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio22outputEnable::Enabled
    }
}
#[doc = "Field `reg_gpio_22_oe` writer - Output enable register for GPIO22."]
pub type RegGpio22OeW<'a, REG> = crate::BitWriter<'a, REG, Gpio22outputEnable>;
impl<'a, REG> RegGpio22OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22outputEnable::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22outputEnable::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Output enable register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_oe(&self) -> RegGpio0OeR {
        RegGpio0OeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output enable register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_oe(&self) -> RegGpio1OeR {
        RegGpio1OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output enable register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_oe(&self) -> RegGpio2OeR {
        RegGpio2OeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output enable register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_oe(&self) -> RegGpio3OeR {
        RegGpio3OeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output enable register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_oe(&self) -> RegGpio4OeR {
        RegGpio4OeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output enable register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_oe(&self) -> RegGpio5OeR {
        RegGpio5OeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output enable register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_oe(&self) -> RegGpio6OeR {
        RegGpio6OeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output enable register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_oe(&self) -> RegGpio7OeR {
        RegGpio7OeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output enable register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_oe(&self) -> RegGpio8OeR {
        RegGpio8OeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output enable register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_oe(&self) -> RegGpio9OeR {
        RegGpio9OeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output enable register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_oe(&self) -> RegGpio10OeR {
        RegGpio10OeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output enable register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_oe(&self) -> RegGpio11OeR {
        RegGpio11OeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output enable register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_oe(&self) -> RegGpio12OeR {
        RegGpio12OeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output enable register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_oe(&self) -> RegGpio13OeR {
        RegGpio13OeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_oe(&self) -> RegGpio14OeR {
        RegGpio14OeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output enable register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_oe(&self) -> RegGpio15OeR {
        RegGpio15OeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output enable register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_oe(&self) -> RegGpio16OeR {
        RegGpio16OeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output enable register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_oe(&self) -> RegGpio17OeR {
        RegGpio17OeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output enable register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_oe(&self) -> RegGpio18OeR {
        RegGpio18OeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output enable register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_oe(&self) -> RegGpio19OeR {
        RegGpio19OeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output enable register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_oe(&self) -> RegGpio20OeR {
        RegGpio20OeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output enable register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_oe(&self) -> RegGpio21OeR {
        RegGpio21OeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output enable register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_oe(&self) -> RegGpio22OeR {
        RegGpio22OeR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output enable register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_oe(&mut self) -> RegGpio0OeW<'_, GpioCfgctl34Spec> {
        RegGpio0OeW::new(self, 0)
    }
    #[doc = "Bit 1 - Output enable register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_oe(&mut self) -> RegGpio1OeW<'_, GpioCfgctl34Spec> {
        RegGpio1OeW::new(self, 1)
    }
    #[doc = "Bit 2 - Output enable register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_oe(&mut self) -> RegGpio2OeW<'_, GpioCfgctl34Spec> {
        RegGpio2OeW::new(self, 2)
    }
    #[doc = "Bit 3 - Output enable register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_oe(&mut self) -> RegGpio3OeW<'_, GpioCfgctl34Spec> {
        RegGpio3OeW::new(self, 3)
    }
    #[doc = "Bit 4 - Output enable register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_oe(&mut self) -> RegGpio4OeW<'_, GpioCfgctl34Spec> {
        RegGpio4OeW::new(self, 4)
    }
    #[doc = "Bit 5 - Output enable register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_oe(&mut self) -> RegGpio5OeW<'_, GpioCfgctl34Spec> {
        RegGpio5OeW::new(self, 5)
    }
    #[doc = "Bit 6 - Output enable register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_oe(&mut self) -> RegGpio6OeW<'_, GpioCfgctl34Spec> {
        RegGpio6OeW::new(self, 6)
    }
    #[doc = "Bit 7 - Output enable register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_oe(&mut self) -> RegGpio7OeW<'_, GpioCfgctl34Spec> {
        RegGpio7OeW::new(self, 7)
    }
    #[doc = "Bit 8 - Output enable register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_oe(&mut self) -> RegGpio8OeW<'_, GpioCfgctl34Spec> {
        RegGpio8OeW::new(self, 8)
    }
    #[doc = "Bit 9 - Output enable register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_oe(&mut self) -> RegGpio9OeW<'_, GpioCfgctl34Spec> {
        RegGpio9OeW::new(self, 9)
    }
    #[doc = "Bit 10 - Output enable register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_oe(&mut self) -> RegGpio10OeW<'_, GpioCfgctl34Spec> {
        RegGpio10OeW::new(self, 10)
    }
    #[doc = "Bit 11 - Output enable register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_oe(&mut self) -> RegGpio11OeW<'_, GpioCfgctl34Spec> {
        RegGpio11OeW::new(self, 11)
    }
    #[doc = "Bit 12 - Output enable register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_oe(&mut self) -> RegGpio12OeW<'_, GpioCfgctl34Spec> {
        RegGpio12OeW::new(self, 12)
    }
    #[doc = "Bit 13 - Output enable register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_oe(&mut self) -> RegGpio13OeW<'_, GpioCfgctl34Spec> {
        RegGpio13OeW::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_oe(&mut self) -> RegGpio14OeW<'_, GpioCfgctl34Spec> {
        RegGpio14OeW::new(self, 14)
    }
    #[doc = "Bit 15 - Output enable register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_oe(&mut self) -> RegGpio15OeW<'_, GpioCfgctl34Spec> {
        RegGpio15OeW::new(self, 15)
    }
    #[doc = "Bit 16 - Output enable register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_oe(&mut self) -> RegGpio16OeW<'_, GpioCfgctl34Spec> {
        RegGpio16OeW::new(self, 16)
    }
    #[doc = "Bit 17 - Output enable register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_oe(&mut self) -> RegGpio17OeW<'_, GpioCfgctl34Spec> {
        RegGpio17OeW::new(self, 17)
    }
    #[doc = "Bit 18 - Output enable register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_oe(&mut self) -> RegGpio18OeW<'_, GpioCfgctl34Spec> {
        RegGpio18OeW::new(self, 18)
    }
    #[doc = "Bit 19 - Output enable register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_oe(&mut self) -> RegGpio19OeW<'_, GpioCfgctl34Spec> {
        RegGpio19OeW::new(self, 19)
    }
    #[doc = "Bit 20 - Output enable register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_oe(&mut self) -> RegGpio20OeW<'_, GpioCfgctl34Spec> {
        RegGpio20OeW::new(self, 20)
    }
    #[doc = "Bit 21 - Output enable register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_oe(&mut self) -> RegGpio21OeW<'_, GpioCfgctl34Spec> {
        RegGpio21OeW::new(self, 21)
    }
    #[doc = "Bit 22 - Output enable register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_oe(&mut self) -> RegGpio22OeW<'_, GpioCfgctl34Spec> {
        RegGpio22OeW::new(self, 22)
    }
}
#[doc = "Output enable register for GPIO.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl34Spec;
impl crate::RegisterSpec for GpioCfgctl34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl34::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl34Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl34::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL34 to value 0"]
impl crate::Resettable for GpioCfgctl34Spec {}
