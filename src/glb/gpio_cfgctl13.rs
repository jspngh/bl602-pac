#[doc = "Register `GPIO_CFGCTL13` reader"]
pub type R = crate::R<GpioCfgctl13Spec>;
#[doc = "Register `GPIO_CFGCTL13` writer"]
pub type W = crate::W<GpioCfgctl13Spec>;
#[doc = "Input enable for GPIO26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio26inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio26inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio26inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_ie` reader - Input enable for GPIO26."]
pub type RegGpio26IeR = crate::BitReader<Gpio26inputEnabled>;
impl RegGpio26IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio26inputEnabled {
        match self.bits {
            false => Gpio26inputEnabled::Disabled,
            true => Gpio26inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio26inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio26inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_26_ie` writer - Input enable for GPIO26."]
pub type RegGpio26IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio26inputEnabled>;
impl<'a, REG> RegGpio26IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio26schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio26schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio26schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_smt` reader - Schmitt trigger enabled for GPIO26."]
pub type RegGpio26SmtR = crate::BitReader<Gpio26schmitt>;
impl RegGpio26SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio26schmitt {
        match self.bits {
            false => Gpio26schmitt::Disabled,
            true => Gpio26schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio26schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio26schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_26_smt` writer - Schmitt trigger enabled for GPIO26."]
pub type RegGpio26SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio26schmitt>;
impl<'a, REG> RegGpio26SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio26driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio26driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio26driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio26driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio26driving {}
#[doc = "Field `reg_gpio_26_drv` reader - Driving control enabled for GPIO26."]
pub type RegGpio26DrvR = crate::FieldReader<Gpio26driving>;
impl RegGpio26DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio26driving> {
        match self.bits {
            0 => Some(Gpio26driving::Disabled),
            1 => Some(Gpio26driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio26driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio26driving::Enabled
    }
}
#[doc = "Field `reg_gpio_26_drv` writer - Driving control enabled for GPIO26."]
pub type RegGpio26DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio26driving>;
impl<'a, REG> RegGpio26DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio26pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio26pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio26pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_pu` reader - Pull Up Resistor for GPIO26."]
pub type RegGpio26PuR = crate::BitReader<Gpio26pullUpResistor>;
impl RegGpio26PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio26pullUpResistor {
        match self.bits {
            false => Gpio26pullUpResistor::Disabled,
            true => Gpio26pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio26pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio26pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_26_pu` writer - Pull Up Resistor for GPIO26."]
pub type RegGpio26PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio26pullUpResistor>;
impl<'a, REG> RegGpio26PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio26pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio26pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio26pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_pd` reader - Pull Down Resistor for GPIO26."]
pub type RegGpio26PdR = crate::BitReader<Gpio26pullDownResistor>;
impl RegGpio26PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio26pullDownResistor {
        match self.bits {
            false => Gpio26pullDownResistor::Disabled,
            true => Gpio26pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio26pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio26pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_26_pd` writer - Pull Down Resistor for GPIO26."]
pub type RegGpio26PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio26pullDownResistor>;
impl<'a, REG> RegGpio26PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio26pullDownResistor::Enabled)
    }
}
#[doc = "Input enable for GPIO27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio27inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio27inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio27inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_ie` reader - Input enable for GPIO27."]
pub type RegGpio27IeR = crate::BitReader<Gpio27inputEnabled>;
impl RegGpio27IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio27inputEnabled {
        match self.bits {
            false => Gpio27inputEnabled::Disabled,
            true => Gpio27inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio27inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio27inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_27_ie` writer - Input enable for GPIO27."]
pub type RegGpio27IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio27inputEnabled>;
impl<'a, REG> RegGpio27IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio27schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio27schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio27schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_smt` reader - Schmitt trigger enabled for GPIO27."]
pub type RegGpio27SmtR = crate::BitReader<Gpio27schmitt>;
impl RegGpio27SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio27schmitt {
        match self.bits {
            false => Gpio27schmitt::Disabled,
            true => Gpio27schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio27schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio27schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_27_smt` writer - Schmitt trigger enabled for GPIO27."]
pub type RegGpio27SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio27schmitt>;
impl<'a, REG> RegGpio27SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio27driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio27driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio27driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio27driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio27driving {}
#[doc = "Field `reg_gpio_27_drv` reader - Driving control enabled for GPIO27."]
pub type RegGpio27DrvR = crate::FieldReader<Gpio27driving>;
impl RegGpio27DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio27driving> {
        match self.bits {
            0 => Some(Gpio27driving::Disabled),
            1 => Some(Gpio27driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio27driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio27driving::Enabled
    }
}
#[doc = "Field `reg_gpio_27_drv` writer - Driving control enabled for GPIO27."]
pub type RegGpio27DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio27driving>;
impl<'a, REG> RegGpio27DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio27pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio27pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio27pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_pu` reader - Pull Up Resistor for GPIO27."]
pub type RegGpio27PuR = crate::BitReader<Gpio27pullUpResistor>;
impl RegGpio27PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio27pullUpResistor {
        match self.bits {
            false => Gpio27pullUpResistor::Disabled,
            true => Gpio27pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio27pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio27pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_27_pu` writer - Pull Up Resistor for GPIO27."]
pub type RegGpio27PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio27pullUpResistor>;
impl<'a, REG> RegGpio27PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio27pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio27pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio27pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_pd` reader - Pull Down Resistor for GPIO27."]
pub type RegGpio27PdR = crate::BitReader<Gpio27pullDownResistor>;
impl RegGpio27PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio27pullDownResistor {
        match self.bits {
            false => Gpio27pullDownResistor::Disabled,
            true => Gpio27pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio27pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio27pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_27_pd` writer - Pull Down Resistor for GPIO27."]
pub type RegGpio27PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio27pullDownResistor>;
impl<'a, REG> RegGpio27PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio27pullDownResistor::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_ie(&self) -> RegGpio26IeR {
        RegGpio26IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_smt(&self) -> RegGpio26SmtR {
        RegGpio26SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_drv(&self) -> RegGpio26DrvR {
        RegGpio26DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pu(&self) -> RegGpio26PuR {
        RegGpio26PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pd(&self) -> RegGpio26PdR {
        RegGpio26PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Input enable for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_ie(&self) -> RegGpio27IeR {
        RegGpio27IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_smt(&self) -> RegGpio27SmtR {
        RegGpio27SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_drv(&self) -> RegGpio27DrvR {
        RegGpio27DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pu(&self) -> RegGpio27PuR {
        RegGpio27PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pd(&self) -> RegGpio27PdR {
        RegGpio27PdR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_ie(&mut self) -> RegGpio26IeW<'_, GpioCfgctl13Spec> {
        RegGpio26IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_smt(&mut self) -> RegGpio26SmtW<'_, GpioCfgctl13Spec> {
        RegGpio26SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_drv(&mut self) -> RegGpio26DrvW<'_, GpioCfgctl13Spec> {
        RegGpio26DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pu(&mut self) -> RegGpio26PuW<'_, GpioCfgctl13Spec> {
        RegGpio26PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pd(&mut self) -> RegGpio26PdW<'_, GpioCfgctl13Spec> {
        RegGpio26PdW::new(self, 5)
    }
    #[doc = "Bit 16 - Input enable for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_ie(&mut self) -> RegGpio27IeW<'_, GpioCfgctl13Spec> {
        RegGpio27IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_smt(&mut self) -> RegGpio27SmtW<'_, GpioCfgctl13Spec> {
        RegGpio27SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_drv(&mut self) -> RegGpio27DrvW<'_, GpioCfgctl13Spec> {
        RegGpio27DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pu(&mut self) -> RegGpio27PuW<'_, GpioCfgctl13Spec> {
        RegGpio27PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pd(&mut self) -> RegGpio27PdW<'_, GpioCfgctl13Spec> {
        RegGpio27PdW::new(self, 21)
    }
}
#[doc = "GPIO26, GPIO27 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl13Spec;
impl crate::RegisterSpec for GpioCfgctl13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl13::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl13Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl13::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL13 to value 0x0003_0003"]
impl crate::Resettable for GpioCfgctl13Spec {
    const RESET_VALUE: u32 = 0x0003_0003;
}
