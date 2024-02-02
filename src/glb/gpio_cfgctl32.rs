#[doc = "Register `GPIO_CFGCTL32` reader"]
pub type R = crate::R<GpioCfgctl32Spec>;
#[doc = "Register `GPIO_CFGCTL32` writer"]
pub type W = crate::W<GpioCfgctl32Spec>;
#[doc = "Output register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio0output> for bool {
    #[inline(always)]
    fn from(variant: Gpio0output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_o` reader - Output register for GPIO0."]
pub type RegGpio0OR = crate::BitReader<Gpio0output>;
impl RegGpio0OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0output {
        match self.bits {
            false => Gpio0output::Disabled,
            true => Gpio0output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio0output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio0output::Enabled
    }
}
#[doc = "Field `reg_gpio_0_o` writer - Output register for GPIO0."]
pub type RegGpio0OW<'a, REG> = crate::BitWriter<'a, REG, Gpio0output>;
impl<'a, REG> RegGpio0OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0output::Enabled)
    }
}
#[doc = "Output register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio1output> for bool {
    #[inline(always)]
    fn from(variant: Gpio1output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_o` reader - Output register for GPIO1."]
pub type RegGpio1OR = crate::BitReader<Gpio1output>;
impl RegGpio1OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1output {
        match self.bits {
            false => Gpio1output::Disabled,
            true => Gpio1output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio1output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio1output::Enabled
    }
}
#[doc = "Field `reg_gpio_1_o` writer - Output register for GPIO1."]
pub type RegGpio1OW<'a, REG> = crate::BitWriter<'a, REG, Gpio1output>;
impl<'a, REG> RegGpio1OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1output::Enabled)
    }
}
#[doc = "Output register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio2output> for bool {
    #[inline(always)]
    fn from(variant: Gpio2output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_o` reader - Output register for GPIO2."]
pub type RegGpio2OR = crate::BitReader<Gpio2output>;
impl RegGpio2OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2output {
        match self.bits {
            false => Gpio2output::Disabled,
            true => Gpio2output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio2output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio2output::Enabled
    }
}
#[doc = "Field `reg_gpio_2_o` writer - Output register for GPIO2."]
pub type RegGpio2OW<'a, REG> = crate::BitWriter<'a, REG, Gpio2output>;
impl<'a, REG> RegGpio2OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2output::Enabled)
    }
}
#[doc = "Output register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio3output> for bool {
    #[inline(always)]
    fn from(variant: Gpio3output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_o` reader - Output register for GPIO3."]
pub type RegGpio3OR = crate::BitReader<Gpio3output>;
impl RegGpio3OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3output {
        match self.bits {
            false => Gpio3output::Disabled,
            true => Gpio3output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio3output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio3output::Enabled
    }
}
#[doc = "Field `reg_gpio_3_o` writer - Output register for GPIO3."]
pub type RegGpio3OW<'a, REG> = crate::BitWriter<'a, REG, Gpio3output>;
impl<'a, REG> RegGpio3OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3output::Enabled)
    }
}
#[doc = "Output register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio4output> for bool {
    #[inline(always)]
    fn from(variant: Gpio4output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_o` reader - Output register for GPIO4."]
pub type RegGpio4OR = crate::BitReader<Gpio4output>;
impl RegGpio4OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4output {
        match self.bits {
            false => Gpio4output::Disabled,
            true => Gpio4output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio4output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio4output::Enabled
    }
}
#[doc = "Field `reg_gpio_4_o` writer - Output register for GPIO4."]
pub type RegGpio4OW<'a, REG> = crate::BitWriter<'a, REG, Gpio4output>;
impl<'a, REG> RegGpio4OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4output::Enabled)
    }
}
#[doc = "Output register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio5output> for bool {
    #[inline(always)]
    fn from(variant: Gpio5output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_o` reader - Output register for GPIO5."]
pub type RegGpio5OR = crate::BitReader<Gpio5output>;
impl RegGpio5OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5output {
        match self.bits {
            false => Gpio5output::Disabled,
            true => Gpio5output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio5output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio5output::Enabled
    }
}
#[doc = "Field `reg_gpio_5_o` writer - Output register for GPIO5."]
pub type RegGpio5OW<'a, REG> = crate::BitWriter<'a, REG, Gpio5output>;
impl<'a, REG> RegGpio5OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5output::Enabled)
    }
}
#[doc = "Output register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio6output> for bool {
    #[inline(always)]
    fn from(variant: Gpio6output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_o` reader - Output register for GPIO6."]
pub type RegGpio6OR = crate::BitReader<Gpio6output>;
impl RegGpio6OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6output {
        match self.bits {
            false => Gpio6output::Disabled,
            true => Gpio6output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio6output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio6output::Enabled
    }
}
#[doc = "Field `reg_gpio_6_o` writer - Output register for GPIO6."]
pub type RegGpio6OW<'a, REG> = crate::BitWriter<'a, REG, Gpio6output>;
impl<'a, REG> RegGpio6OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6output::Enabled)
    }
}
#[doc = "Output register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio7output> for bool {
    #[inline(always)]
    fn from(variant: Gpio7output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_o` reader - Output register for GPIO7."]
pub type RegGpio7OR = crate::BitReader<Gpio7output>;
impl RegGpio7OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7output {
        match self.bits {
            false => Gpio7output::Disabled,
            true => Gpio7output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio7output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio7output::Enabled
    }
}
#[doc = "Field `reg_gpio_7_o` writer - Output register for GPIO7."]
pub type RegGpio7OW<'a, REG> = crate::BitWriter<'a, REG, Gpio7output>;
impl<'a, REG> RegGpio7OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7output::Enabled)
    }
}
#[doc = "Output register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio8output> for bool {
    #[inline(always)]
    fn from(variant: Gpio8output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_o` reader - Output register for GPIO8."]
pub type RegGpio8OR = crate::BitReader<Gpio8output>;
impl RegGpio8OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8output {
        match self.bits {
            false => Gpio8output::Disabled,
            true => Gpio8output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio8output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio8output::Enabled
    }
}
#[doc = "Field `reg_gpio_8_o` writer - Output register for GPIO8."]
pub type RegGpio8OW<'a, REG> = crate::BitWriter<'a, REG, Gpio8output>;
impl<'a, REG> RegGpio8OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8output::Enabled)
    }
}
#[doc = "Output register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio9output> for bool {
    #[inline(always)]
    fn from(variant: Gpio9output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_o` reader - Output register for GPIO9."]
pub type RegGpio9OR = crate::BitReader<Gpio9output>;
impl RegGpio9OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9output {
        match self.bits {
            false => Gpio9output::Disabled,
            true => Gpio9output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio9output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio9output::Enabled
    }
}
#[doc = "Field `reg_gpio_9_o` writer - Output register for GPIO9."]
pub type RegGpio9OW<'a, REG> = crate::BitWriter<'a, REG, Gpio9output>;
impl<'a, REG> RegGpio9OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9output::Enabled)
    }
}
#[doc = "Output register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio10output> for bool {
    #[inline(always)]
    fn from(variant: Gpio10output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_o` reader - Output register for GPIO10."]
pub type RegGpio10OR = crate::BitReader<Gpio10output>;
impl RegGpio10OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10output {
        match self.bits {
            false => Gpio10output::Disabled,
            true => Gpio10output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio10output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio10output::Enabled
    }
}
#[doc = "Field `reg_gpio_10_o` writer - Output register for GPIO10."]
pub type RegGpio10OW<'a, REG> = crate::BitWriter<'a, REG, Gpio10output>;
impl<'a, REG> RegGpio10OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10output::Enabled)
    }
}
#[doc = "Output register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio11output> for bool {
    #[inline(always)]
    fn from(variant: Gpio11output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_o` reader - Output register for GPIO11."]
pub type RegGpio11OR = crate::BitReader<Gpio11output>;
impl RegGpio11OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11output {
        match self.bits {
            false => Gpio11output::Disabled,
            true => Gpio11output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio11output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio11output::Enabled
    }
}
#[doc = "Field `reg_gpio_11_o` writer - Output register for GPIO11."]
pub type RegGpio11OW<'a, REG> = crate::BitWriter<'a, REG, Gpio11output>;
impl<'a, REG> RegGpio11OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11output::Enabled)
    }
}
#[doc = "Output register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio12output> for bool {
    #[inline(always)]
    fn from(variant: Gpio12output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_o` reader - Output register for GPIO12."]
pub type RegGpio12OR = crate::BitReader<Gpio12output>;
impl RegGpio12OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12output {
        match self.bits {
            false => Gpio12output::Disabled,
            true => Gpio12output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio12output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio12output::Enabled
    }
}
#[doc = "Field `reg_gpio_12_o` writer - Output register for GPIO12."]
pub type RegGpio12OW<'a, REG> = crate::BitWriter<'a, REG, Gpio12output>;
impl<'a, REG> RegGpio12OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12output::Enabled)
    }
}
#[doc = "Output register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio13output> for bool {
    #[inline(always)]
    fn from(variant: Gpio13output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_o` reader - Output register for GPIO13."]
pub type RegGpio13OR = crate::BitReader<Gpio13output>;
impl RegGpio13OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13output {
        match self.bits {
            false => Gpio13output::Disabled,
            true => Gpio13output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio13output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio13output::Enabled
    }
}
#[doc = "Field `reg_gpio_13_o` writer - Output register for GPIO13."]
pub type RegGpio13OW<'a, REG> = crate::BitWriter<'a, REG, Gpio13output>;
impl<'a, REG> RegGpio13OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13output::Enabled)
    }
}
#[doc = "Output register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio14output> for bool {
    #[inline(always)]
    fn from(variant: Gpio14output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_o` reader - Output register for GPIO14."]
pub type RegGpio14OR = crate::BitReader<Gpio14output>;
impl RegGpio14OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14output {
        match self.bits {
            false => Gpio14output::Disabled,
            true => Gpio14output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio14output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio14output::Enabled
    }
}
#[doc = "Field `reg_gpio_14_o` writer - Output register for GPIO14."]
pub type RegGpio14OW<'a, REG> = crate::BitWriter<'a, REG, Gpio14output>;
impl<'a, REG> RegGpio14OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14output::Enabled)
    }
}
#[doc = "Output register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio15output> for bool {
    #[inline(always)]
    fn from(variant: Gpio15output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_o` reader - Output register for GPIO15."]
pub type RegGpio15OR = crate::BitReader<Gpio15output>;
impl RegGpio15OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15output {
        match self.bits {
            false => Gpio15output::Disabled,
            true => Gpio15output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio15output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio15output::Enabled
    }
}
#[doc = "Field `reg_gpio_15_o` writer - Output register for GPIO15."]
pub type RegGpio15OW<'a, REG> = crate::BitWriter<'a, REG, Gpio15output>;
impl<'a, REG> RegGpio15OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15output::Enabled)
    }
}
#[doc = "Output register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio16output> for bool {
    #[inline(always)]
    fn from(variant: Gpio16output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_o` reader - Output register for GPIO16."]
pub type RegGpio16OR = crate::BitReader<Gpio16output>;
impl RegGpio16OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16output {
        match self.bits {
            false => Gpio16output::Disabled,
            true => Gpio16output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio16output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio16output::Enabled
    }
}
#[doc = "Field `reg_gpio_16_o` writer - Output register for GPIO16."]
pub type RegGpio16OW<'a, REG> = crate::BitWriter<'a, REG, Gpio16output>;
impl<'a, REG> RegGpio16OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16output::Enabled)
    }
}
#[doc = "Output register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio17output> for bool {
    #[inline(always)]
    fn from(variant: Gpio17output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_o` reader - Output register for GPIO17."]
pub type RegGpio17OR = crate::BitReader<Gpio17output>;
impl RegGpio17OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17output {
        match self.bits {
            false => Gpio17output::Disabled,
            true => Gpio17output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio17output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio17output::Enabled
    }
}
#[doc = "Field `reg_gpio_17_o` writer - Output register for GPIO17."]
pub type RegGpio17OW<'a, REG> = crate::BitWriter<'a, REG, Gpio17output>;
impl<'a, REG> RegGpio17OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17output::Enabled)
    }
}
#[doc = "Output register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio18output> for bool {
    #[inline(always)]
    fn from(variant: Gpio18output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_o` reader - Output register for GPIO18."]
pub type RegGpio18OR = crate::BitReader<Gpio18output>;
impl RegGpio18OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18output {
        match self.bits {
            false => Gpio18output::Disabled,
            true => Gpio18output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio18output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio18output::Enabled
    }
}
#[doc = "Field `reg_gpio_18_o` writer - Output register for GPIO18."]
pub type RegGpio18OW<'a, REG> = crate::BitWriter<'a, REG, Gpio18output>;
impl<'a, REG> RegGpio18OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18output::Enabled)
    }
}
#[doc = "Output register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio19output> for bool {
    #[inline(always)]
    fn from(variant: Gpio19output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_o` reader - Output register for GPIO19."]
pub type RegGpio19OR = crate::BitReader<Gpio19output>;
impl RegGpio19OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19output {
        match self.bits {
            false => Gpio19output::Disabled,
            true => Gpio19output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio19output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio19output::Enabled
    }
}
#[doc = "Field `reg_gpio_19_o` writer - Output register for GPIO19."]
pub type RegGpio19OW<'a, REG> = crate::BitWriter<'a, REG, Gpio19output>;
impl<'a, REG> RegGpio19OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19output::Enabled)
    }
}
#[doc = "Output register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio20output> for bool {
    #[inline(always)]
    fn from(variant: Gpio20output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_o` reader - Output register for GPIO20."]
pub type RegGpio20OR = crate::BitReader<Gpio20output>;
impl RegGpio20OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20output {
        match self.bits {
            false => Gpio20output::Disabled,
            true => Gpio20output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio20output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio20output::Enabled
    }
}
#[doc = "Field `reg_gpio_20_o` writer - Output register for GPIO20."]
pub type RegGpio20OW<'a, REG> = crate::BitWriter<'a, REG, Gpio20output>;
impl<'a, REG> RegGpio20OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20output::Enabled)
    }
}
#[doc = "Output register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio21output> for bool {
    #[inline(always)]
    fn from(variant: Gpio21output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_o` reader - Output register for GPIO21."]
pub type RegGpio21OR = crate::BitReader<Gpio21output>;
impl RegGpio21OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21output {
        match self.bits {
            false => Gpio21output::Disabled,
            true => Gpio21output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio21output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio21output::Enabled
    }
}
#[doc = "Field `reg_gpio_21_o` writer - Output register for GPIO21."]
pub type RegGpio21OW<'a, REG> = crate::BitWriter<'a, REG, Gpio21output>;
impl<'a, REG> RegGpio21OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21output::Enabled)
    }
}
#[doc = "Output register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22output {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio22output> for bool {
    #[inline(always)]
    fn from(variant: Gpio22output) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_o` reader - Output register for GPIO22."]
pub type RegGpio22OR = crate::BitReader<Gpio22output>;
impl RegGpio22OR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22output {
        match self.bits {
            false => Gpio22output::Disabled,
            true => Gpio22output::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio22output::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio22output::Enabled
    }
}
#[doc = "Field `reg_gpio_22_o` writer - Output register for GPIO22."]
pub type RegGpio22OW<'a, REG> = crate::BitWriter<'a, REG, Gpio22output>;
impl<'a, REG> RegGpio22OW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22output::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22output::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Output register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_o(&self) -> RegGpio0OR {
        RegGpio0OR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_o(&self) -> RegGpio1OR {
        RegGpio1OR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_o(&self) -> RegGpio2OR {
        RegGpio2OR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_o(&self) -> RegGpio3OR {
        RegGpio3OR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_o(&self) -> RegGpio4OR {
        RegGpio4OR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_o(&self) -> RegGpio5OR {
        RegGpio5OR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_o(&self) -> RegGpio6OR {
        RegGpio6OR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_o(&self) -> RegGpio7OR {
        RegGpio7OR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_o(&self) -> RegGpio8OR {
        RegGpio8OR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_o(&self) -> RegGpio9OR {
        RegGpio9OR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_o(&self) -> RegGpio10OR {
        RegGpio10OR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_o(&self) -> RegGpio11OR {
        RegGpio11OR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_o(&self) -> RegGpio12OR {
        RegGpio12OR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_o(&self) -> RegGpio13OR {
        RegGpio13OR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_o(&self) -> RegGpio14OR {
        RegGpio14OR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_o(&self) -> RegGpio15OR {
        RegGpio15OR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_o(&self) -> RegGpio16OR {
        RegGpio16OR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_o(&self) -> RegGpio17OR {
        RegGpio17OR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_o(&self) -> RegGpio18OR {
        RegGpio18OR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_o(&self) -> RegGpio19OR {
        RegGpio19OR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_o(&self) -> RegGpio20OR {
        RegGpio20OR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_o(&self) -> RegGpio21OR {
        RegGpio21OR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_o(&self) -> RegGpio22OR {
        RegGpio22OR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_o(&mut self) -> RegGpio0OW<'_, GpioCfgctl32Spec> {
        RegGpio0OW::new(self, 0)
    }
    #[doc = "Bit 1 - Output register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_o(&mut self) -> RegGpio1OW<'_, GpioCfgctl32Spec> {
        RegGpio1OW::new(self, 1)
    }
    #[doc = "Bit 2 - Output register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_o(&mut self) -> RegGpio2OW<'_, GpioCfgctl32Spec> {
        RegGpio2OW::new(self, 2)
    }
    #[doc = "Bit 3 - Output register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_o(&mut self) -> RegGpio3OW<'_, GpioCfgctl32Spec> {
        RegGpio3OW::new(self, 3)
    }
    #[doc = "Bit 4 - Output register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_o(&mut self) -> RegGpio4OW<'_, GpioCfgctl32Spec> {
        RegGpio4OW::new(self, 4)
    }
    #[doc = "Bit 5 - Output register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_o(&mut self) -> RegGpio5OW<'_, GpioCfgctl32Spec> {
        RegGpio5OW::new(self, 5)
    }
    #[doc = "Bit 6 - Output register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_o(&mut self) -> RegGpio6OW<'_, GpioCfgctl32Spec> {
        RegGpio6OW::new(self, 6)
    }
    #[doc = "Bit 7 - Output register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_o(&mut self) -> RegGpio7OW<'_, GpioCfgctl32Spec> {
        RegGpio7OW::new(self, 7)
    }
    #[doc = "Bit 8 - Output register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_o(&mut self) -> RegGpio8OW<'_, GpioCfgctl32Spec> {
        RegGpio8OW::new(self, 8)
    }
    #[doc = "Bit 9 - Output register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_o(&mut self) -> RegGpio9OW<'_, GpioCfgctl32Spec> {
        RegGpio9OW::new(self, 9)
    }
    #[doc = "Bit 10 - Output register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_o(&mut self) -> RegGpio10OW<'_, GpioCfgctl32Spec> {
        RegGpio10OW::new(self, 10)
    }
    #[doc = "Bit 11 - Output register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_o(&mut self) -> RegGpio11OW<'_, GpioCfgctl32Spec> {
        RegGpio11OW::new(self, 11)
    }
    #[doc = "Bit 12 - Output register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_o(&mut self) -> RegGpio12OW<'_, GpioCfgctl32Spec> {
        RegGpio12OW::new(self, 12)
    }
    #[doc = "Bit 13 - Output register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_o(&mut self) -> RegGpio13OW<'_, GpioCfgctl32Spec> {
        RegGpio13OW::new(self, 13)
    }
    #[doc = "Bit 14 - Output register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_o(&mut self) -> RegGpio14OW<'_, GpioCfgctl32Spec> {
        RegGpio14OW::new(self, 14)
    }
    #[doc = "Bit 15 - Output register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_o(&mut self) -> RegGpio15OW<'_, GpioCfgctl32Spec> {
        RegGpio15OW::new(self, 15)
    }
    #[doc = "Bit 16 - Output register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_o(&mut self) -> RegGpio16OW<'_, GpioCfgctl32Spec> {
        RegGpio16OW::new(self, 16)
    }
    #[doc = "Bit 17 - Output register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_o(&mut self) -> RegGpio17OW<'_, GpioCfgctl32Spec> {
        RegGpio17OW::new(self, 17)
    }
    #[doc = "Bit 18 - Output register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_o(&mut self) -> RegGpio18OW<'_, GpioCfgctl32Spec> {
        RegGpio18OW::new(self, 18)
    }
    #[doc = "Bit 19 - Output register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_o(&mut self) -> RegGpio19OW<'_, GpioCfgctl32Spec> {
        RegGpio19OW::new(self, 19)
    }
    #[doc = "Bit 20 - Output register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_o(&mut self) -> RegGpio20OW<'_, GpioCfgctl32Spec> {
        RegGpio20OW::new(self, 20)
    }
    #[doc = "Bit 21 - Output register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_o(&mut self) -> RegGpio21OW<'_, GpioCfgctl32Spec> {
        RegGpio21OW::new(self, 21)
    }
    #[doc = "Bit 22 - Output register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_o(&mut self) -> RegGpio22OW<'_, GpioCfgctl32Spec> {
        RegGpio22OW::new(self, 22)
    }
}
#[doc = "Output register for all GPIO pins. Output Enabled bit must be set in Output Enable register to work.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl32Spec;
impl crate::RegisterSpec for GpioCfgctl32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl32::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl32Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl32::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL32 to value 0"]
impl crate::Resettable for GpioCfgctl32Spec {}
