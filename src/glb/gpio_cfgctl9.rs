#[doc = "Register `GPIO_CFGCTL9` reader"]
pub type R = crate::R<GpioCfgctl9Spec>;
#[doc = "Register `GPIO_CFGCTL9` writer"]
pub type W = crate::W<GpioCfgctl9Spec>;
#[doc = "Input enable for GPIO18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio18inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio18inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_ie` reader - Input enable for GPIO18."]
pub type RegGpio18IeR = crate::BitReader<Gpio18inputEnabled>;
impl RegGpio18IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18inputEnabled {
        match self.bits {
            false => Gpio18inputEnabled::Disabled,
            true => Gpio18inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio18inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio18inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_18_ie` writer - Input enable for GPIO18."]
pub type RegGpio18IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio18inputEnabled>;
impl<'a, REG> RegGpio18IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio18schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio18schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_smt` reader - Schmitt trigger enabled for GPIO18."]
pub type RegGpio18SmtR = crate::BitReader<Gpio18schmitt>;
impl RegGpio18SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18schmitt {
        match self.bits {
            false => Gpio18schmitt::Disabled,
            true => Gpio18schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio18schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio18schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_18_smt` writer - Schmitt trigger enabled for GPIO18."]
pub type RegGpio18SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio18schmitt>;
impl<'a, REG> RegGpio18SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio18driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio18driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio18driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio18driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio18driving {}
#[doc = "Field `reg_gpio_18_drv` reader - Driving control enabled for GPIO18."]
pub type RegGpio18DrvR = crate::FieldReader<Gpio18driving>;
impl RegGpio18DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio18driving> {
        match self.bits {
            0 => Some(Gpio18driving::Disabled),
            1 => Some(Gpio18driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio18driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio18driving::Enabled
    }
}
#[doc = "Field `reg_gpio_18_drv` writer - Driving control enabled for GPIO18."]
pub type RegGpio18DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio18driving>;
impl<'a, REG> RegGpio18DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio18pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio18pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_pu` reader - Pull Up Resistor for GPIO18."]
pub type RegGpio18PuR = crate::BitReader<Gpio18pullUpResistor>;
impl RegGpio18PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18pullUpResistor {
        match self.bits {
            false => Gpio18pullUpResistor::Disabled,
            true => Gpio18pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio18pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio18pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_18_pu` writer - Pull Up Resistor for GPIO18."]
pub type RegGpio18PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio18pullUpResistor>;
impl<'a, REG> RegGpio18PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio18pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio18pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio18pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_pd` reader - Pull Down Resistor for GPIO18."]
pub type RegGpio18PdR = crate::BitReader<Gpio18pullDownResistor>;
impl RegGpio18PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio18pullDownResistor {
        match self.bits {
            false => Gpio18pullDownResistor::Disabled,
            true => Gpio18pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio18pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio18pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_18_pd` writer - Pull Down Resistor for GPIO18."]
pub type RegGpio18PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio18pullDownResistor>;
impl<'a, REG> RegGpio18PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO18.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio18functionSelect {
    #[doc = "2: `10`"]
    SfD2 = 2,
    #[doc = "4: `100`"]
    SpiSs = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig2 = 7,
    #[doc = "8: `1000`"]
    PwmCh3 = 8,
    #[doc = "9: `1001`"]
    FemGpio2 = 9,
    #[doc = "11: `1011`"]
    Swgpio18 = 11,
    #[doc = "14: `1110`"]
    E21Tck = 14,
}
impl From<Gpio18functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio18functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio18functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio18functionSelect {}
#[doc = "Field `reg_gpio_18_func_sel` reader - Function select for GPIO18."]
pub type RegGpio18FuncSelR = crate::FieldReader<Gpio18functionSelect>;
impl RegGpio18FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio18functionSelect> {
        match self.bits {
            2 => Some(Gpio18functionSelect::SfD2),
            4 => Some(Gpio18functionSelect::SpiSs),
            6 => Some(Gpio18functionSelect::I2cScl),
            7 => Some(Gpio18functionSelect::UartSig2),
            8 => Some(Gpio18functionSelect::PwmCh3),
            9 => Some(Gpio18functionSelect::FemGpio2),
            11 => Some(Gpio18functionSelect::Swgpio18),
            14 => Some(Gpio18functionSelect::E21Tck),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sf_d2(&self) -> bool {
        *self == Gpio18functionSelect::SfD2
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == Gpio18functionSelect::SpiSs
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio18functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig2(&self) -> bool {
        *self == Gpio18functionSelect::UartSig2
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch3(&self) -> bool {
        *self == Gpio18functionSelect::PwmCh3
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == Gpio18functionSelect::FemGpio2
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_18(&self) -> bool {
        *self == Gpio18functionSelect::Swgpio18
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == Gpio18functionSelect::E21Tck
    }
}
#[doc = "Field `reg_gpio_18_func_sel` writer - Function select for GPIO18."]
pub type RegGpio18FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio18functionSelect>;
impl<'a, REG> RegGpio18FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18functionSelect::SfD2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18functionSelect::SpiSs)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18functionSelect::UartSig2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18functionSelect::PwmCh3)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18functionSelect::FemGpio2)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_18(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18functionSelect::Swgpio18)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio18functionSelect::E21Tck)
    }
}
#[doc = "Input enable for GPIO19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio19inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio19inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_ie` reader - Input enable for GPIO19."]
pub type RegGpio19IeR = crate::BitReader<Gpio19inputEnabled>;
impl RegGpio19IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19inputEnabled {
        match self.bits {
            false => Gpio19inputEnabled::Disabled,
            true => Gpio19inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio19inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio19inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_19_ie` writer - Input enable for GPIO19."]
pub type RegGpio19IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio19inputEnabled>;
impl<'a, REG> RegGpio19IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio19schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio19schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_smt` reader - Schmitt trigger enabled for GPIO19."]
pub type RegGpio19SmtR = crate::BitReader<Gpio19schmitt>;
impl RegGpio19SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19schmitt {
        match self.bits {
            false => Gpio19schmitt::Disabled,
            true => Gpio19schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio19schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio19schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_19_smt` writer - Schmitt trigger enabled for GPIO19."]
pub type RegGpio19SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio19schmitt>;
impl<'a, REG> RegGpio19SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio19driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio19driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio19driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio19driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio19driving {}
#[doc = "Field `reg_gpio_19_drv` reader - Driving control enabled for GPIO19."]
pub type RegGpio19DrvR = crate::FieldReader<Gpio19driving>;
impl RegGpio19DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio19driving> {
        match self.bits {
            0 => Some(Gpio19driving::Disabled),
            1 => Some(Gpio19driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio19driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio19driving::Enabled
    }
}
#[doc = "Field `reg_gpio_19_drv` writer - Driving control enabled for GPIO19."]
pub type RegGpio19DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio19driving>;
impl<'a, REG> RegGpio19DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio19pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio19pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_pu` reader - Pull Up Resistor for GPIO19."]
pub type RegGpio19PuR = crate::BitReader<Gpio19pullUpResistor>;
impl RegGpio19PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19pullUpResistor {
        match self.bits {
            false => Gpio19pullUpResistor::Disabled,
            true => Gpio19pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio19pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio19pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_19_pu` writer - Pull Up Resistor for GPIO19."]
pub type RegGpio19PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio19pullUpResistor>;
impl<'a, REG> RegGpio19PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio19pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio19pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio19pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_pd` reader - Pull Down Resistor for GPIO19."]
pub type RegGpio19PdR = crate::BitReader<Gpio19pullDownResistor>;
impl RegGpio19PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio19pullDownResistor {
        match self.bits {
            false => Gpio19pullDownResistor::Disabled,
            true => Gpio19pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio19pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio19pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_19_pd` writer - Pull Down Resistor for GPIO19."]
pub type RegGpio19PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio19pullDownResistor>;
impl<'a, REG> RegGpio19PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO19.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio19functionSelect {
    #[doc = "2: `10`"]
    SfD1 = 2,
    #[doc = "4: `100`"]
    SpiSclk = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig3 = 7,
    #[doc = "8: `1000`"]
    PwmCh4 = 8,
    #[doc = "9: `1001`"]
    FemGpio3 = 9,
    #[doc = "11: `1011`"]
    Swgpio19 = 11,
    #[doc = "14: `1110`"]
    E21Tdo = 14,
}
impl From<Gpio19functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio19functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio19functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio19functionSelect {}
#[doc = "Field `reg_gpio_19_func_sel` reader - Function select for GPIO19."]
pub type RegGpio19FuncSelR = crate::FieldReader<Gpio19functionSelect>;
impl RegGpio19FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio19functionSelect> {
        match self.bits {
            2 => Some(Gpio19functionSelect::SfD1),
            4 => Some(Gpio19functionSelect::SpiSclk),
            6 => Some(Gpio19functionSelect::I2cSda),
            7 => Some(Gpio19functionSelect::UartSig3),
            8 => Some(Gpio19functionSelect::PwmCh4),
            9 => Some(Gpio19functionSelect::FemGpio3),
            11 => Some(Gpio19functionSelect::Swgpio19),
            14 => Some(Gpio19functionSelect::E21Tdo),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sf_d1(&self) -> bool {
        *self == Gpio19functionSelect::SfD1
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        *self == Gpio19functionSelect::SpiSclk
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio19functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig3(&self) -> bool {
        *self == Gpio19functionSelect::UartSig3
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch4(&self) -> bool {
        *self == Gpio19functionSelect::PwmCh4
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        *self == Gpio19functionSelect::FemGpio3
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_19(&self) -> bool {
        *self == Gpio19functionSelect::Swgpio19
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        *self == Gpio19functionSelect::E21Tdo
    }
}
#[doc = "Field `reg_gpio_19_func_sel` writer - Function select for GPIO19."]
pub type RegGpio19FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio19functionSelect>;
impl<'a, REG> RegGpio19FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19functionSelect::SfD1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19functionSelect::SpiSclk)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19functionSelect::UartSig3)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19functionSelect::PwmCh4)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19functionSelect::FemGpio3)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_19(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19functionSelect::Swgpio19)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio19functionSelect::E21Tdo)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_ie(&self) -> RegGpio18IeR {
        RegGpio18IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_smt(&self) -> RegGpio18SmtR {
        RegGpio18SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_drv(&self) -> RegGpio18DrvR {
        RegGpio18DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_pu(&self) -> RegGpio18PuR {
        RegGpio18PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_pd(&self) -> RegGpio18PdR {
        RegGpio18PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_func_sel(&self) -> RegGpio18FuncSelR {
        RegGpio18FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_ie(&self) -> RegGpio19IeR {
        RegGpio19IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_smt(&self) -> RegGpio19SmtR {
        RegGpio19SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_drv(&self) -> RegGpio19DrvR {
        RegGpio19DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_pu(&self) -> RegGpio19PuR {
        RegGpio19PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_pd(&self) -> RegGpio19PdR {
        RegGpio19PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_func_sel(&self) -> RegGpio19FuncSelR {
        RegGpio19FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_ie(&mut self) -> RegGpio18IeW<'_, GpioCfgctl9Spec> {
        RegGpio18IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_smt(&mut self) -> RegGpio18SmtW<'_, GpioCfgctl9Spec> {
        RegGpio18SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_drv(&mut self) -> RegGpio18DrvW<'_, GpioCfgctl9Spec> {
        RegGpio18DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_pu(&mut self) -> RegGpio18PuW<'_, GpioCfgctl9Spec> {
        RegGpio18PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_pd(&mut self) -> RegGpio18PdW<'_, GpioCfgctl9Spec> {
        RegGpio18PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_func_sel(&mut self) -> RegGpio18FuncSelW<'_, GpioCfgctl9Spec> {
        RegGpio18FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_ie(&mut self) -> RegGpio19IeW<'_, GpioCfgctl9Spec> {
        RegGpio19IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_smt(&mut self) -> RegGpio19SmtW<'_, GpioCfgctl9Spec> {
        RegGpio19SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_drv(&mut self) -> RegGpio19DrvW<'_, GpioCfgctl9Spec> {
        RegGpio19DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_pu(&mut self) -> RegGpio19PuW<'_, GpioCfgctl9Spec> {
        RegGpio19PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_pd(&mut self) -> RegGpio19PdW<'_, GpioCfgctl9Spec> {
        RegGpio19PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_func_sel(&mut self) -> RegGpio19FuncSelW<'_, GpioCfgctl9Spec> {
        RegGpio19FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO18, GPIO19 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl9Spec;
impl crate::RegisterSpec for GpioCfgctl9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl9::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl9Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl9::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL9 to value 0x0b03_0b03"]
impl crate::Resettable for GpioCfgctl9Spec {
    const RESET_VALUE: u32 = 0x0b03_0b03;
}
