#[doc = "Register `GPIO_INT_MASK1` reader"]
pub type R = crate::R<GpioIntMask1Spec>;
#[doc = "Register `GPIO_INT_MASK1` writer"]
pub type W = crate::W<GpioIntMask1Spec>;
#[doc = "Mask register for GPIO0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio0mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio0mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_mask` reader - Mask register for GPIO0."]
pub type RegGpio0MaskR = crate::BitReader<Gpio0mask>;
impl RegGpio0MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0mask {
        match self.bits {
            false => Gpio0mask::Unmasked,
            true => Gpio0mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio0mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio0mask::Masked
    }
}
#[doc = "Field `reg_gpio_0_mask` writer - Mask register for GPIO0."]
pub type RegGpio0MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio0mask>;
impl<'a, REG> RegGpio0MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0mask::Masked)
    }
}
#[doc = "Mask register for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio1mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio1mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_mask` reader - Mask register for GPIO1."]
pub type RegGpio1MaskR = crate::BitReader<Gpio1mask>;
impl RegGpio1MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1mask {
        match self.bits {
            false => Gpio1mask::Unmasked,
            true => Gpio1mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio1mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio1mask::Masked
    }
}
#[doc = "Field `reg_gpio_1_mask` writer - Mask register for GPIO1."]
pub type RegGpio1MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio1mask>;
impl<'a, REG> RegGpio1MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1mask::Masked)
    }
}
#[doc = "Mask register for GPIO2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio2mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio2mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_mask` reader - Mask register for GPIO2."]
pub type RegGpio2MaskR = crate::BitReader<Gpio2mask>;
impl RegGpio2MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2mask {
        match self.bits {
            false => Gpio2mask::Unmasked,
            true => Gpio2mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio2mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio2mask::Masked
    }
}
#[doc = "Field `reg_gpio_2_mask` writer - Mask register for GPIO2."]
pub type RegGpio2MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio2mask>;
impl<'a, REG> RegGpio2MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2mask::Masked)
    }
}
#[doc = "Mask register for GPIO3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio3mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio3mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_mask` reader - Mask register for GPIO3."]
pub type RegGpio3MaskR = crate::BitReader<Gpio3mask>;
impl RegGpio3MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3mask {
        match self.bits {
            false => Gpio3mask::Unmasked,
            true => Gpio3mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio3mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio3mask::Masked
    }
}
#[doc = "Field `reg_gpio_3_mask` writer - Mask register for GPIO3."]
pub type RegGpio3MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio3mask>;
impl<'a, REG> RegGpio3MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3mask::Masked)
    }
}
#[doc = "Mask register for GPIO4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio4mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio4mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_mask` reader - Mask register for GPIO4."]
pub type RegGpio4MaskR = crate::BitReader<Gpio4mask>;
impl RegGpio4MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4mask {
        match self.bits {
            false => Gpio4mask::Unmasked,
            true => Gpio4mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio4mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio4mask::Masked
    }
}
#[doc = "Field `reg_gpio_4_mask` writer - Mask register for GPIO4."]
pub type RegGpio4MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio4mask>;
impl<'a, REG> RegGpio4MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4mask::Masked)
    }
}
#[doc = "Mask register for GPIO5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio5mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio5mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_mask` reader - Mask register for GPIO5."]
pub type RegGpio5MaskR = crate::BitReader<Gpio5mask>;
impl RegGpio5MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5mask {
        match self.bits {
            false => Gpio5mask::Unmasked,
            true => Gpio5mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio5mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio5mask::Masked
    }
}
#[doc = "Field `reg_gpio_5_mask` writer - Mask register for GPIO5."]
pub type RegGpio5MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio5mask>;
impl<'a, REG> RegGpio5MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5mask::Masked)
    }
}
#[doc = "Mask register for GPIO6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio6mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio6mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_mask` reader - Mask register for GPIO6."]
pub type RegGpio6MaskR = crate::BitReader<Gpio6mask>;
impl RegGpio6MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6mask {
        match self.bits {
            false => Gpio6mask::Unmasked,
            true => Gpio6mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio6mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio6mask::Masked
    }
}
#[doc = "Field `reg_gpio_6_mask` writer - Mask register for GPIO6."]
pub type RegGpio6MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio6mask>;
impl<'a, REG> RegGpio6MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6mask::Masked)
    }
}
#[doc = "Mask register for GPIO7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio7mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio7mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_mask` reader - Mask register for GPIO7."]
pub type RegGpio7MaskR = crate::BitReader<Gpio7mask>;
impl RegGpio7MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7mask {
        match self.bits {
            false => Gpio7mask::Unmasked,
            true => Gpio7mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio7mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio7mask::Masked
    }
}
#[doc = "Field `reg_gpio_7_mask` writer - Mask register for GPIO7."]
pub type RegGpio7MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio7mask>;
impl<'a, REG> RegGpio7MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7mask::Masked)
    }
}
#[doc = "Mask register for GPIO8.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio8mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio8mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_mask` reader - Mask register for GPIO8."]
pub type RegGpio8MaskR = crate::BitReader<Gpio8mask>;
impl RegGpio8MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8mask {
        match self.bits {
            false => Gpio8mask::Unmasked,
            true => Gpio8mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio8mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio8mask::Masked
    }
}
#[doc = "Field `reg_gpio_8_mask` writer - Mask register for GPIO8."]
pub type RegGpio8MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio8mask>;
impl<'a, REG> RegGpio8MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8mask::Masked)
    }
}
#[doc = "Mask register for GPIO9.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio9mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio9mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_mask` reader - Mask register for GPIO9."]
pub type RegGpio9MaskR = crate::BitReader<Gpio9mask>;
impl RegGpio9MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9mask {
        match self.bits {
            false => Gpio9mask::Unmasked,
            true => Gpio9mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio9mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio9mask::Masked
    }
}
#[doc = "Field `reg_gpio_9_mask` writer - Mask register for GPIO9."]
pub type RegGpio9MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio9mask>;
impl<'a, REG> RegGpio9MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9mask::Masked)
    }
}
#[doc = "Mask register for GPIO10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio10mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio10mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_mask` reader - Mask register for GPIO10."]
pub type RegGpio10MaskR = crate::BitReader<Gpio10mask>;
impl RegGpio10MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10mask {
        match self.bits {
            false => Gpio10mask::Unmasked,
            true => Gpio10mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio10mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio10mask::Masked
    }
}
#[doc = "Field `reg_gpio_10_mask` writer - Mask register for GPIO10."]
pub type RegGpio10MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio10mask>;
impl<'a, REG> RegGpio10MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10mask::Masked)
    }
}
#[doc = "Mask register for GPIO11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio11mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio11mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_mask` reader - Mask register for GPIO11."]
pub type RegGpio11MaskR = crate::BitReader<Gpio11mask>;
impl RegGpio11MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11mask {
        match self.bits {
            false => Gpio11mask::Unmasked,
            true => Gpio11mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio11mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio11mask::Masked
    }
}
#[doc = "Field `reg_gpio_11_mask` writer - Mask register for GPIO11."]
pub type RegGpio11MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio11mask>;
impl<'a, REG> RegGpio11MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11mask::Masked)
    }
}
#[doc = "Mask register for GPIO12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio12mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio12mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_mask` reader - Mask register for GPIO12."]
pub type RegGpio12MaskR = crate::BitReader<Gpio12mask>;
impl RegGpio12MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12mask {
        match self.bits {
            false => Gpio12mask::Unmasked,
            true => Gpio12mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio12mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio12mask::Masked
    }
}
#[doc = "Field `reg_gpio_12_mask` writer - Mask register for GPIO12."]
pub type RegGpio12MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio12mask>;
impl<'a, REG> RegGpio12MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12mask::Masked)
    }
}
#[doc = "Mask register for GPIO13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio13mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio13mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_mask` reader - Mask register for GPIO13."]
pub type RegGpio13MaskR = crate::BitReader<Gpio13mask>;
impl RegGpio13MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13mask {
        match self.bits {
            false => Gpio13mask::Unmasked,
            true => Gpio13mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio13mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio13mask::Masked
    }
}
#[doc = "Field `reg_gpio_13_mask` writer - Mask register for GPIO13."]
pub type RegGpio13MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio13mask>;
impl<'a, REG> RegGpio13MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13mask::Masked)
    }
}
#[doc = "Mask register for GPIO14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio14mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio14mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_mask` reader - Mask register for GPIO14."]
pub type RegGpio14MaskR = crate::BitReader<Gpio14mask>;
impl RegGpio14MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14mask {
        match self.bits {
            false => Gpio14mask::Unmasked,
            true => Gpio14mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio14mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio14mask::Masked
    }
}
#[doc = "Field `reg_gpio_14_mask` writer - Mask register for GPIO14."]
pub type RegGpio14MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio14mask>;
impl<'a, REG> RegGpio14MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14mask::Masked)
    }
}
#[doc = "Mask register for GPIO15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio15mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio15mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_mask` reader - Mask register for GPIO15."]
pub type RegGpio15MaskR = crate::BitReader<Gpio15mask>;
impl RegGpio15MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15mask {
        match self.bits {
            false => Gpio15mask::Unmasked,
            true => Gpio15mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio15mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio15mask::Masked
    }
}
#[doc = "Field `reg_gpio_15_mask` writer - Mask register for GPIO15."]
pub type RegGpio15MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio15mask>;
impl<'a, REG> RegGpio15MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15mask::Masked)
    }
}
#[doc = "Mask register for GPIO16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio16mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio16mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_mask` reader - Mask register for GPIO16."]
pub type RegGpio16MaskR = crate::BitReader<Gpio16mask>;
impl RegGpio16MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16mask {
        match self.bits {
            false => Gpio16mask::Unmasked,
            true => Gpio16mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio16mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio16mask::Masked
    }
}
#[doc = "Field `reg_gpio_16_mask` writer - Mask register for GPIO16."]
pub type RegGpio16MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio16mask>;
impl<'a, REG> RegGpio16MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16mask::Masked)
    }
}
#[doc = "Mask register for GPIO17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio17mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio17mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_mask` reader - Mask register for GPIO17."]
pub type RegGpio17MaskR = crate::BitReader<Gpio17mask>;
impl RegGpio17MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17mask {
        match self.bits {
            false => Gpio17mask::Unmasked,
            true => Gpio17mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio17mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio17mask::Masked
    }
}
#[doc = "Field `reg_gpio_17_mask` writer - Mask register for GPIO17."]
pub type RegGpio17MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio17mask>;
impl<'a, REG> RegGpio17MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17mask::Masked)
    }
}
#[doc = "Mask register for GPIO18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio18mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio18mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_mask` reader - Mask register for GPIO18."]
pub type RegGpio18MaskR = crate::BitReader<Gpio18mask>;
impl RegGpio18MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18mask {
        match self.bits {
            false => Gpio18mask::Unmasked,
            true => Gpio18mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio18mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio18mask::Masked
    }
}
#[doc = "Field `reg_gpio_18_mask` writer - Mask register for GPIO18."]
pub type RegGpio18MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio18mask>;
impl<'a, REG> RegGpio18MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18mask::Masked)
    }
}
#[doc = "Mask register for GPIO19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio19mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio19mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_mask` reader - Mask register for GPIO19."]
pub type RegGpio19MaskR = crate::BitReader<Gpio19mask>;
impl RegGpio19MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19mask {
        match self.bits {
            false => Gpio19mask::Unmasked,
            true => Gpio19mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio19mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio19mask::Masked
    }
}
#[doc = "Field `reg_gpio_19_mask` writer - Mask register for GPIO19."]
pub type RegGpio19MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio19mask>;
impl<'a, REG> RegGpio19MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19mask::Masked)
    }
}
#[doc = "Mask register for GPIO20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio20mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio20mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_mask` reader - Mask register for GPIO20."]
pub type RegGpio20MaskR = crate::BitReader<Gpio20mask>;
impl RegGpio20MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20mask {
        match self.bits {
            false => Gpio20mask::Unmasked,
            true => Gpio20mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio20mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio20mask::Masked
    }
}
#[doc = "Field `reg_gpio_20_mask` writer - Mask register for GPIO20."]
pub type RegGpio20MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio20mask>;
impl<'a, REG> RegGpio20MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20mask::Masked)
    }
}
#[doc = "Mask register for GPIO21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio21mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio21mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_mask` reader - Mask register for GPIO21."]
pub type RegGpio21MaskR = crate::BitReader<Gpio21mask>;
impl RegGpio21MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21mask {
        match self.bits {
            false => Gpio21mask::Unmasked,
            true => Gpio21mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio21mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio21mask::Masked
    }
}
#[doc = "Field `reg_gpio_21_mask` writer - Mask register for GPIO21."]
pub type RegGpio21MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio21mask>;
impl<'a, REG> RegGpio21MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21mask::Masked)
    }
}
#[doc = "Mask register for GPIO22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio22mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio22mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_mask` reader - Mask register for GPIO22."]
pub type RegGpio22MaskR = crate::BitReader<Gpio22mask>;
impl RegGpio22MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22mask {
        match self.bits {
            false => Gpio22mask::Unmasked,
            true => Gpio22mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio22mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio22mask::Masked
    }
}
#[doc = "Field `reg_gpio_22_mask` writer - Mask register for GPIO22."]
pub type RegGpio22MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio22mask>;
impl<'a, REG> RegGpio22MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22mask::Masked)
    }
}
#[doc = "Mask register for GPIO23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio23mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio23mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio23mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_mask` reader - Mask register for GPIO23."]
pub type RegGpio23MaskR = crate::BitReader<Gpio23mask>;
impl RegGpio23MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio23mask {
        match self.bits {
            false => Gpio23mask::Unmasked,
            true => Gpio23mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio23mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio23mask::Masked
    }
}
#[doc = "Field `reg_gpio_23_mask` writer - Mask register for GPIO23."]
pub type RegGpio23MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio23mask>;
impl<'a, REG> RegGpio23MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23mask::Masked)
    }
}
#[doc = "Mask register for GPIO24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio24mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio24mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio24mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_mask` reader - Mask register for GPIO24."]
pub type RegGpio24MaskR = crate::BitReader<Gpio24mask>;
impl RegGpio24MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio24mask {
        match self.bits {
            false => Gpio24mask::Unmasked,
            true => Gpio24mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio24mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio24mask::Masked
    }
}
#[doc = "Field `reg_gpio_24_mask` writer - Mask register for GPIO24."]
pub type RegGpio24MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio24mask>;
impl<'a, REG> RegGpio24MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24mask::Masked)
    }
}
#[doc = "Mask register for GPIO25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio25mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio25mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio25mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_mask` reader - Mask register for GPIO25."]
pub type RegGpio25MaskR = crate::BitReader<Gpio25mask>;
impl RegGpio25MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio25mask {
        match self.bits {
            false => Gpio25mask::Unmasked,
            true => Gpio25mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio25mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio25mask::Masked
    }
}
#[doc = "Field `reg_gpio_25_mask` writer - Mask register for GPIO25."]
pub type RegGpio25MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio25mask>;
impl<'a, REG> RegGpio25MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25mask::Masked)
    }
}
#[doc = "Mask register for GPIO26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio26mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio26mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio26mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_mask` reader - Mask register for GPIO26."]
pub type RegGpio26MaskR = crate::BitReader<Gpio26mask>;
impl RegGpio26MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio26mask {
        match self.bits {
            false => Gpio26mask::Unmasked,
            true => Gpio26mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio26mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio26mask::Masked
    }
}
#[doc = "Field `reg_gpio_26_mask` writer - Mask register for GPIO26."]
pub type RegGpio26MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio26mask>;
impl<'a, REG> RegGpio26MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26mask::Masked)
    }
}
#[doc = "Mask register for GPIO27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio27mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio27mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio27mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_mask` reader - Mask register for GPIO27."]
pub type RegGpio27MaskR = crate::BitReader<Gpio27mask>;
impl RegGpio27MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio27mask {
        match self.bits {
            false => Gpio27mask::Unmasked,
            true => Gpio27mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio27mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio27mask::Masked
    }
}
#[doc = "Field `reg_gpio_27_mask` writer - Mask register for GPIO27."]
pub type RegGpio27MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio27mask>;
impl<'a, REG> RegGpio27MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27mask::Masked)
    }
}
#[doc = "Mask register for GPIO28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio28mask {
    #[doc = "0: `0`"]
    Unmasked = 0,
    #[doc = "1: `1`"]
    Masked = 1,
}
impl From<Gpio28mask> for bool {
    #[inline(always)]
    fn from(variant: Gpio28mask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_mask` reader - Mask register for GPIO28."]
pub type RegGpio28MaskR = crate::BitReader<Gpio28mask>;
impl RegGpio28MaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio28mask {
        match self.bits {
            false => Gpio28mask::Unmasked,
            true => Gpio28mask::Masked,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Gpio28mask::Unmasked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Gpio28mask::Masked
    }
}
#[doc = "Field `reg_gpio_28_mask` writer - Mask register for GPIO28."]
pub type RegGpio28MaskW<'a, REG> = crate::BitWriter<'a, REG, Gpio28mask>;
impl<'a, REG> RegGpio28MaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28mask::Unmasked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28mask::Masked)
    }
}
impl R {
    #[doc = "Bit 0 - Mask register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_mask(&self) -> RegGpio0MaskR {
        RegGpio0MaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_mask(&self) -> RegGpio1MaskR {
        RegGpio1MaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_mask(&self) -> RegGpio2MaskR {
        RegGpio2MaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_mask(&self) -> RegGpio3MaskR {
        RegGpio3MaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_mask(&self) -> RegGpio4MaskR {
        RegGpio4MaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_mask(&self) -> RegGpio5MaskR {
        RegGpio5MaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_mask(&self) -> RegGpio6MaskR {
        RegGpio6MaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_mask(&self) -> RegGpio7MaskR {
        RegGpio7MaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_mask(&self) -> RegGpio8MaskR {
        RegGpio8MaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_mask(&self) -> RegGpio9MaskR {
        RegGpio9MaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_mask(&self) -> RegGpio10MaskR {
        RegGpio10MaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_mask(&self) -> RegGpio11MaskR {
        RegGpio11MaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_mask(&self) -> RegGpio12MaskR {
        RegGpio12MaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_mask(&self) -> RegGpio13MaskR {
        RegGpio13MaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_mask(&self) -> RegGpio14MaskR {
        RegGpio14MaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_mask(&self) -> RegGpio15MaskR {
        RegGpio15MaskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_mask(&self) -> RegGpio16MaskR {
        RegGpio16MaskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_mask(&self) -> RegGpio17MaskR {
        RegGpio17MaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_mask(&self) -> RegGpio18MaskR {
        RegGpio18MaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mask register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_mask(&self) -> RegGpio19MaskR {
        RegGpio19MaskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Mask register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_mask(&self) -> RegGpio20MaskR {
        RegGpio20MaskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Mask register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_mask(&self) -> RegGpio21MaskR {
        RegGpio21MaskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mask register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_mask(&self) -> RegGpio22MaskR {
        RegGpio22MaskR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mask register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_mask(&self) -> RegGpio23MaskR {
        RegGpio23MaskR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_mask(&self) -> RegGpio24MaskR {
        RegGpio24MaskR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_mask(&self) -> RegGpio25MaskR {
        RegGpio25MaskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mask register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_mask(&self) -> RegGpio26MaskR {
        RegGpio26MaskR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Mask register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_mask(&self) -> RegGpio27MaskR {
        RegGpio27MaskR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Mask register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_mask(&self) -> RegGpio28MaskR {
        RegGpio28MaskR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_mask(&mut self) -> RegGpio0MaskW<'_, GpioIntMask1Spec> {
        RegGpio0MaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_mask(&mut self) -> RegGpio1MaskW<'_, GpioIntMask1Spec> {
        RegGpio1MaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_mask(&mut self) -> RegGpio2MaskW<'_, GpioIntMask1Spec> {
        RegGpio2MaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_mask(&mut self) -> RegGpio3MaskW<'_, GpioIntMask1Spec> {
        RegGpio3MaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_mask(&mut self) -> RegGpio4MaskW<'_, GpioIntMask1Spec> {
        RegGpio4MaskW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_mask(&mut self) -> RegGpio5MaskW<'_, GpioIntMask1Spec> {
        RegGpio5MaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Mask register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_mask(&mut self) -> RegGpio6MaskW<'_, GpioIntMask1Spec> {
        RegGpio6MaskW::new(self, 6)
    }
    #[doc = "Bit 7 - Mask register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_mask(&mut self) -> RegGpio7MaskW<'_, GpioIntMask1Spec> {
        RegGpio7MaskW::new(self, 7)
    }
    #[doc = "Bit 8 - Mask register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_mask(&mut self) -> RegGpio8MaskW<'_, GpioIntMask1Spec> {
        RegGpio8MaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_mask(&mut self) -> RegGpio9MaskW<'_, GpioIntMask1Spec> {
        RegGpio9MaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_mask(&mut self) -> RegGpio10MaskW<'_, GpioIntMask1Spec> {
        RegGpio10MaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Mask register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_mask(&mut self) -> RegGpio11MaskW<'_, GpioIntMask1Spec> {
        RegGpio11MaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Mask register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_mask(&mut self) -> RegGpio12MaskW<'_, GpioIntMask1Spec> {
        RegGpio12MaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Mask register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_mask(&mut self) -> RegGpio13MaskW<'_, GpioIntMask1Spec> {
        RegGpio13MaskW::new(self, 13)
    }
    #[doc = "Bit 14 - Mask register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_mask(&mut self) -> RegGpio14MaskW<'_, GpioIntMask1Spec> {
        RegGpio14MaskW::new(self, 14)
    }
    #[doc = "Bit 15 - Mask register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_mask(&mut self) -> RegGpio15MaskW<'_, GpioIntMask1Spec> {
        RegGpio15MaskW::new(self, 15)
    }
    #[doc = "Bit 16 - Mask register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_mask(&mut self) -> RegGpio16MaskW<'_, GpioIntMask1Spec> {
        RegGpio16MaskW::new(self, 16)
    }
    #[doc = "Bit 17 - Mask register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_mask(&mut self) -> RegGpio17MaskW<'_, GpioIntMask1Spec> {
        RegGpio17MaskW::new(self, 17)
    }
    #[doc = "Bit 18 - Mask register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_mask(&mut self) -> RegGpio18MaskW<'_, GpioIntMask1Spec> {
        RegGpio18MaskW::new(self, 18)
    }
    #[doc = "Bit 19 - Mask register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_mask(&mut self) -> RegGpio19MaskW<'_, GpioIntMask1Spec> {
        RegGpio19MaskW::new(self, 19)
    }
    #[doc = "Bit 20 - Mask register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_mask(&mut self) -> RegGpio20MaskW<'_, GpioIntMask1Spec> {
        RegGpio20MaskW::new(self, 20)
    }
    #[doc = "Bit 21 - Mask register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_mask(&mut self) -> RegGpio21MaskW<'_, GpioIntMask1Spec> {
        RegGpio21MaskW::new(self, 21)
    }
    #[doc = "Bit 22 - Mask register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_mask(&mut self) -> RegGpio22MaskW<'_, GpioIntMask1Spec> {
        RegGpio22MaskW::new(self, 22)
    }
    #[doc = "Bit 23 - Mask register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_mask(&mut self) -> RegGpio23MaskW<'_, GpioIntMask1Spec> {
        RegGpio23MaskW::new(self, 23)
    }
    #[doc = "Bit 24 - Mask register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_mask(&mut self) -> RegGpio24MaskW<'_, GpioIntMask1Spec> {
        RegGpio24MaskW::new(self, 24)
    }
    #[doc = "Bit 25 - Mask register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_mask(&mut self) -> RegGpio25MaskW<'_, GpioIntMask1Spec> {
        RegGpio25MaskW::new(self, 25)
    }
    #[doc = "Bit 26 - Mask register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_mask(&mut self) -> RegGpio26MaskW<'_, GpioIntMask1Spec> {
        RegGpio26MaskW::new(self, 26)
    }
    #[doc = "Bit 27 - Mask register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_mask(&mut self) -> RegGpio27MaskW<'_, GpioIntMask1Spec> {
        RegGpio27MaskW::new(self, 27)
    }
    #[doc = "Bit 28 - Mask register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_mask(&mut self) -> RegGpio28MaskW<'_, GpioIntMask1Spec> {
        RegGpio28MaskW::new(self, 28)
    }
}
#[doc = "Interrupt masking register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntMask1Spec;
impl crate::RegisterSpec for GpioIntMask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_mask1::R`](R) reader structure"]
impl crate::Readable for GpioIntMask1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_mask1::W`](W) writer structure"]
impl crate::Writable for GpioIntMask1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_MASK1 to value 0xffff_ffff"]
impl crate::Resettable for GpioIntMask1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
