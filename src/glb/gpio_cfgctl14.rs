#[doc = "Register `GPIO_CFGCTL14` reader"]
pub type R = crate::R<GpioCfgctl14Spec>;
#[doc = "Register `GPIO_CFGCTL14` writer"]
pub type W = crate::W<GpioCfgctl14Spec>;
#[doc = "Input enable for GPIO28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio28inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio28inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio28inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_ie` reader - Input enable for GPIO28."]
pub type RegGpio28IeR = crate::BitReader<Gpio28inputEnabled>;
impl RegGpio28IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio28inputEnabled {
        match self.bits {
            false => Gpio28inputEnabled::Disabled,
            true => Gpio28inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio28inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio28inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_28_ie` writer - Input enable for GPIO28."]
pub type RegGpio28IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio28inputEnabled>;
impl<'a, REG> RegGpio28IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio28schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio28schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio28schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_smt` reader - Schmitt trigger enabled for GPIO28."]
pub type RegGpio28SmtR = crate::BitReader<Gpio28schmitt>;
impl RegGpio28SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio28schmitt {
        match self.bits {
            false => Gpio28schmitt::Disabled,
            true => Gpio28schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio28schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio28schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_28_smt` writer - Schmitt trigger enabled for GPIO28."]
pub type RegGpio28SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio28schmitt>;
impl<'a, REG> RegGpio28SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio28driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio28driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio28driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio28driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio28driving {}
#[doc = "Field `reg_gpio_28_drv` reader - Driving control enabled for GPIO28."]
pub type RegGpio28DrvR = crate::FieldReader<Gpio28driving>;
impl RegGpio28DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio28driving> {
        match self.bits {
            0 => Some(Gpio28driving::Disabled),
            1 => Some(Gpio28driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio28driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio28driving::Enabled
    }
}
#[doc = "Field `reg_gpio_28_drv` writer - Driving control enabled for GPIO28."]
pub type RegGpio28DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio28driving>;
impl<'a, REG> RegGpio28DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio28pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio28pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio28pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_pu` reader - Pull Up Resistor for GPIO28."]
pub type RegGpio28PuR = crate::BitReader<Gpio28pullUpResistor>;
impl RegGpio28PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio28pullUpResistor {
        match self.bits {
            false => Gpio28pullUpResistor::Disabled,
            true => Gpio28pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio28pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio28pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_28_pu` writer - Pull Up Resistor for GPIO28."]
pub type RegGpio28PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio28pullUpResistor>;
impl<'a, REG> RegGpio28PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio28pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio28pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio28pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_pd` reader - Pull Down Resistor for GPIO28."]
pub type RegGpio28PdR = crate::BitReader<Gpio28pullDownResistor>;
impl RegGpio28PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio28pullDownResistor {
        match self.bits {
            false => Gpio28pullDownResistor::Disabled,
            true => Gpio28pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio28pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio28pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_28_pd` writer - Pull Down Resistor for GPIO28."]
pub type RegGpio28PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio28pullDownResistor>;
impl<'a, REG> RegGpio28PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio28pullDownResistor::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_ie(&self) -> RegGpio28IeR {
        RegGpio28IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_smt(&self) -> RegGpio28SmtR {
        RegGpio28SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_drv(&self) -> RegGpio28DrvR {
        RegGpio28DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_pu(&self) -> RegGpio28PuR {
        RegGpio28PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_pd(&self) -> RegGpio28PdR {
        RegGpio28PdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_ie(&mut self) -> RegGpio28IeW<'_, GpioCfgctl14Spec> {
        RegGpio28IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_smt(&mut self) -> RegGpio28SmtW<'_, GpioCfgctl14Spec> {
        RegGpio28SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_drv(&mut self) -> RegGpio28DrvW<'_, GpioCfgctl14Spec> {
        RegGpio28DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_pu(&mut self) -> RegGpio28PuW<'_, GpioCfgctl14Spec> {
        RegGpio28PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_pd(&mut self) -> RegGpio28PdW<'_, GpioCfgctl14Spec> {
        RegGpio28PdW::new(self, 5)
    }
}
#[doc = "GPIO28 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl14Spec;
impl crate::RegisterSpec for GpioCfgctl14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl14::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl14Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl14::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL14 to value 0x03"]
impl crate::Resettable for GpioCfgctl14Spec {
    const RESET_VALUE: u32 = 0x03;
}
