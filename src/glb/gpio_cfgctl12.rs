#[doc = "Register `GPIO_CFGCTL12` reader"]
pub type R = crate::R<GpioCfgctl12Spec>;
#[doc = "Register `GPIO_CFGCTL12` writer"]
pub type W = crate::W<GpioCfgctl12Spec>;
#[doc = "Input enable for GPIO24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio24inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio24inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio24inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_ie` reader - Input enable for GPIO24."]
pub type RegGpio24IeR = crate::BitReader<Gpio24inputEnabled>;
impl RegGpio24IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio24inputEnabled {
        match self.bits {
            false => Gpio24inputEnabled::Disabled,
            true => Gpio24inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio24inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio24inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_24_ie` writer - Input enable for GPIO24."]
pub type RegGpio24IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio24inputEnabled>;
impl<'a, REG> RegGpio24IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio24schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio24schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio24schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_smt` reader - Schmitt trigger enabled for GPIO24."]
pub type RegGpio24SmtR = crate::BitReader<Gpio24schmitt>;
impl RegGpio24SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio24schmitt {
        match self.bits {
            false => Gpio24schmitt::Disabled,
            true => Gpio24schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio24schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio24schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_24_smt` writer - Schmitt trigger enabled for GPIO24."]
pub type RegGpio24SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio24schmitt>;
impl<'a, REG> RegGpio24SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio24driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio24driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio24driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio24driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio24driving {}
#[doc = "Field `reg_gpio_24_drv` reader - Driving control enabled for GPIO24."]
pub type RegGpio24DrvR = crate::FieldReader<Gpio24driving>;
impl RegGpio24DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio24driving> {
        match self.bits {
            0 => Some(Gpio24driving::Disabled),
            1 => Some(Gpio24driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio24driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio24driving::Enabled
    }
}
#[doc = "Field `reg_gpio_24_drv` writer - Driving control enabled for GPIO24."]
pub type RegGpio24DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio24driving>;
impl<'a, REG> RegGpio24DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio24pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio24pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio24pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_pu` reader - Pull Up Resistor for GPIO24."]
pub type RegGpio24PuR = crate::BitReader<Gpio24pullUpResistor>;
impl RegGpio24PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio24pullUpResistor {
        match self.bits {
            false => Gpio24pullUpResistor::Disabled,
            true => Gpio24pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio24pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio24pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_24_pu` writer - Pull Up Resistor for GPIO24."]
pub type RegGpio24PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio24pullUpResistor>;
impl<'a, REG> RegGpio24PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio24pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio24pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio24pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_pd` reader - Pull Down Resistor for GPIO24."]
pub type RegGpio24PdR = crate::BitReader<Gpio24pullDownResistor>;
impl RegGpio24PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio24pullDownResistor {
        match self.bits {
            false => Gpio24pullDownResistor::Disabled,
            true => Gpio24pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio24pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio24pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_24_pd` writer - Pull Down Resistor for GPIO24."]
pub type RegGpio24PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio24pullDownResistor>;
impl<'a, REG> RegGpio24PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio24pullDownResistor::Enabled)
    }
}
#[doc = "Input enable for GPIO25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio25inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio25inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio25inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_ie` reader - Input enable for GPIO25."]
pub type RegGpio25IeR = crate::BitReader<Gpio25inputEnabled>;
impl RegGpio25IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio25inputEnabled {
        match self.bits {
            false => Gpio25inputEnabled::Disabled,
            true => Gpio25inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio25inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio25inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_25_ie` writer - Input enable for GPIO25."]
pub type RegGpio25IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio25inputEnabled>;
impl<'a, REG> RegGpio25IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio25schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio25schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio25schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_smt` reader - Schmitt trigger enabled for GPIO25."]
pub type RegGpio25SmtR = crate::BitReader<Gpio25schmitt>;
impl RegGpio25SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio25schmitt {
        match self.bits {
            false => Gpio25schmitt::Disabled,
            true => Gpio25schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio25schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio25schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_25_smt` writer - Schmitt trigger enabled for GPIO25."]
pub type RegGpio25SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio25schmitt>;
impl<'a, REG> RegGpio25SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio25driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio25driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio25driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio25driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio25driving {}
#[doc = "Field `reg_gpio_25_drv` reader - Driving control enabled for GPIO25."]
pub type RegGpio25DrvR = crate::FieldReader<Gpio25driving>;
impl RegGpio25DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio25driving> {
        match self.bits {
            0 => Some(Gpio25driving::Disabled),
            1 => Some(Gpio25driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio25driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio25driving::Enabled
    }
}
#[doc = "Field `reg_gpio_25_drv` writer - Driving control enabled for GPIO25."]
pub type RegGpio25DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio25driving>;
impl<'a, REG> RegGpio25DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio25pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio25pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio25pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_pu` reader - Pull Up Resistor for GPIO25."]
pub type RegGpio25PuR = crate::BitReader<Gpio25pullUpResistor>;
impl RegGpio25PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio25pullUpResistor {
        match self.bits {
            false => Gpio25pullUpResistor::Disabled,
            true => Gpio25pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio25pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio25pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_25_pu` writer - Pull Up Resistor for GPIO25."]
pub type RegGpio25PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio25pullUpResistor>;
impl<'a, REG> RegGpio25PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio25pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio25pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio25pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_pd` reader - Pull Down Resistor for GPIO25."]
pub type RegGpio25PdR = crate::BitReader<Gpio25pullDownResistor>;
impl RegGpio25PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio25pullDownResistor {
        match self.bits {
            false => Gpio25pullDownResistor::Disabled,
            true => Gpio25pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio25pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio25pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_25_pd` writer - Pull Down Resistor for GPIO25."]
pub type RegGpio25PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio25pullDownResistor>;
impl<'a, REG> RegGpio25PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio25pullDownResistor::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_ie(&self) -> RegGpio24IeR {
        RegGpio24IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_smt(&self) -> RegGpio24SmtR {
        RegGpio24SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_drv(&self) -> RegGpio24DrvR {
        RegGpio24DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_pu(&self) -> RegGpio24PuR {
        RegGpio24PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_pd(&self) -> RegGpio24PdR {
        RegGpio24PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Input enable for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_ie(&self) -> RegGpio25IeR {
        RegGpio25IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_smt(&self) -> RegGpio25SmtR {
        RegGpio25SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_drv(&self) -> RegGpio25DrvR {
        RegGpio25DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_pu(&self) -> RegGpio25PuR {
        RegGpio25PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_pd(&self) -> RegGpio25PdR {
        RegGpio25PdR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_ie(&mut self) -> RegGpio24IeW<'_, GpioCfgctl12Spec> {
        RegGpio24IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_smt(&mut self) -> RegGpio24SmtW<'_, GpioCfgctl12Spec> {
        RegGpio24SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_drv(&mut self) -> RegGpio24DrvW<'_, GpioCfgctl12Spec> {
        RegGpio24DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_pu(&mut self) -> RegGpio24PuW<'_, GpioCfgctl12Spec> {
        RegGpio24PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_pd(&mut self) -> RegGpio24PdW<'_, GpioCfgctl12Spec> {
        RegGpio24PdW::new(self, 5)
    }
    #[doc = "Bit 16 - Input enable for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_ie(&mut self) -> RegGpio25IeW<'_, GpioCfgctl12Spec> {
        RegGpio25IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_smt(&mut self) -> RegGpio25SmtW<'_, GpioCfgctl12Spec> {
        RegGpio25SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_drv(&mut self) -> RegGpio25DrvW<'_, GpioCfgctl12Spec> {
        RegGpio25DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_pu(&mut self) -> RegGpio25PuW<'_, GpioCfgctl12Spec> {
        RegGpio25PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_pd(&mut self) -> RegGpio25PdW<'_, GpioCfgctl12Spec> {
        RegGpio25PdW::new(self, 21)
    }
}
#[doc = "GPIO24, GPIO25 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl12Spec;
impl crate::RegisterSpec for GpioCfgctl12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl12::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl12Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl12::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL12 to value 0x0003_0023"]
impl crate::Resettable for GpioCfgctl12Spec {
    const RESET_VALUE: u32 = 0x0003_0023;
}
