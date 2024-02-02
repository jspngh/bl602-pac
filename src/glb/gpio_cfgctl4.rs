#[doc = "Register `GPIO_CFGCTL4` reader"]
pub type R = crate::R<GpioCfgctl4Spec>;
#[doc = "Register `GPIO_CFGCTL4` writer"]
pub type W = crate::W<GpioCfgctl4Spec>;
#[doc = "Input enable for GPIO8.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio8inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio8inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_ie` reader - Input enable for GPIO8."]
pub type RegGpio8IeR = crate::BitReader<Gpio8inputEnabled>;
impl RegGpio8IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8inputEnabled {
        match self.bits {
            false => Gpio8inputEnabled::Disabled,
            true => Gpio8inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio8inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio8inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_8_ie` writer - Input enable for GPIO8."]
pub type RegGpio8IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio8inputEnabled>;
impl<'a, REG> RegGpio8IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO8.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio8schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio8schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_smt` reader - Schmitt trigger enabled for GPIO8."]
pub type RegGpio8SmtR = crate::BitReader<Gpio8schmitt>;
impl RegGpio8SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8schmitt {
        match self.bits {
            false => Gpio8schmitt::Disabled,
            true => Gpio8schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio8schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio8schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_8_smt` writer - Schmitt trigger enabled for GPIO8."]
pub type RegGpio8SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio8schmitt>;
impl<'a, REG> RegGpio8SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio8driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio8driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio8driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio8driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio8driving {}
#[doc = "Field `reg_gpio_8_drv` reader - Driving control enabled for GPIO8."]
pub type RegGpio8DrvR = crate::FieldReader<Gpio8driving>;
impl RegGpio8DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio8driving> {
        match self.bits {
            0 => Some(Gpio8driving::Disabled),
            1 => Some(Gpio8driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio8driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio8driving::Enabled
    }
}
#[doc = "Field `reg_gpio_8_drv` writer - Driving control enabled for GPIO8."]
pub type RegGpio8DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio8driving>;
impl<'a, REG> RegGpio8DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio8pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio8pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_pu` reader - Pull Up Resistor for GPIO8."]
pub type RegGpio8PuR = crate::BitReader<Gpio8pullUpResistor>;
impl RegGpio8PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8pullUpResistor {
        match self.bits {
            false => Gpio8pullUpResistor::Disabled,
            true => Gpio8pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio8pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio8pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_8_pu` writer - Pull Up Resistor for GPIO8."]
pub type RegGpio8PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio8pullUpResistor>;
impl<'a, REG> RegGpio8PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio8pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio8pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio8pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_pd` reader - Pull Down Resistor for GPIO8."]
pub type RegGpio8PdR = crate::BitReader<Gpio8pullDownResistor>;
impl RegGpio8PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio8pullDownResistor {
        match self.bits {
            false => Gpio8pullDownResistor::Disabled,
            true => Gpio8pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio8pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio8pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_8_pd` writer - Pull Down Resistor for GPIO8."]
pub type RegGpio8PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio8pullDownResistor>;
impl<'a, REG> RegGpio8PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO8.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio8functionSelect {
    #[doc = "4: `100`"]
    SpiMisoSpiMosi = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig0 = 7,
    #[doc = "8: `1000`"]
    PwmCh3 = 8,
    #[doc = "9: `1001`"]
    FemGpio0 = 9,
    #[doc = "11: `1011`"]
    Swgpio8 = 11,
    #[doc = "14: `1110`"]
    E21Tms = 14,
}
impl From<Gpio8functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio8functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio8functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio8functionSelect {}
#[doc = "Field `reg_gpio_8_func_sel` reader - Function select for GPIO8."]
pub type RegGpio8FuncSelR = crate::FieldReader<Gpio8functionSelect>;
impl RegGpio8FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio8functionSelect> {
        match self.bits {
            4 => Some(Gpio8functionSelect::SpiMisoSpiMosi),
            6 => Some(Gpio8functionSelect::I2cScl),
            7 => Some(Gpio8functionSelect::UartSig0),
            8 => Some(Gpio8functionSelect::PwmCh3),
            9 => Some(Gpio8functionSelect::FemGpio0),
            11 => Some(Gpio8functionSelect::Swgpio8),
            14 => Some(Gpio8functionSelect::E21Tms),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_miso_spi_mosi(&self) -> bool {
        *self == Gpio8functionSelect::SpiMisoSpiMosi
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio8functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig0(&self) -> bool {
        *self == Gpio8functionSelect::UartSig0
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch3(&self) -> bool {
        *self == Gpio8functionSelect::PwmCh3
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == Gpio8functionSelect::FemGpio0
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_8(&self) -> bool {
        *self == Gpio8functionSelect::Swgpio8
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == Gpio8functionSelect::E21Tms
    }
}
#[doc = "Field `reg_gpio_8_func_sel` writer - Function select for GPIO8."]
pub type RegGpio8FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio8functionSelect>;
impl<'a, REG> RegGpio8FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_miso_spi_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8functionSelect::SpiMisoSpiMosi)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8functionSelect::UartSig0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8functionSelect::PwmCh3)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8functionSelect::FemGpio0)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_8(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8functionSelect::Swgpio8)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio8functionSelect::E21Tms)
    }
}
#[doc = "Input enable for GPIO9.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio9inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio9inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_ie` reader - Input enable for GPIO9."]
pub type RegGpio9IeR = crate::BitReader<Gpio9inputEnabled>;
impl RegGpio9IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9inputEnabled {
        match self.bits {
            false => Gpio9inputEnabled::Disabled,
            true => Gpio9inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio9inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio9inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_9_ie` writer - Input enable for GPIO9."]
pub type RegGpio9IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio9inputEnabled>;
impl<'a, REG> RegGpio9IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO9.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio9schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio9schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_smt` reader - Schmitt trigger enabled for GPIO9."]
pub type RegGpio9SmtR = crate::BitReader<Gpio9schmitt>;
impl RegGpio9SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9schmitt {
        match self.bits {
            false => Gpio9schmitt::Disabled,
            true => Gpio9schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio9schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio9schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_9_smt` writer - Schmitt trigger enabled for GPIO9."]
pub type RegGpio9SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio9schmitt>;
impl<'a, REG> RegGpio9SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio9driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio9driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio9driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio9driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio9driving {}
#[doc = "Field `reg_gpio_9_drv` reader - Driving control enabled for GPIO9."]
pub type RegGpio9DrvR = crate::FieldReader<Gpio9driving>;
impl RegGpio9DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio9driving> {
        match self.bits {
            0 => Some(Gpio9driving::Disabled),
            1 => Some(Gpio9driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio9driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio9driving::Enabled
    }
}
#[doc = "Field `reg_gpio_9_drv` writer - Driving control enabled for GPIO9."]
pub type RegGpio9DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio9driving>;
impl<'a, REG> RegGpio9DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio9pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio9pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_pu` reader - Pull Up Resistor for GPIO9."]
pub type RegGpio9PuR = crate::BitReader<Gpio9pullUpResistor>;
impl RegGpio9PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9pullUpResistor {
        match self.bits {
            false => Gpio9pullUpResistor::Disabled,
            true => Gpio9pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio9pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio9pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_9_pu` writer - Pull Up Resistor for GPIO9."]
pub type RegGpio9PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio9pullUpResistor>;
impl<'a, REG> RegGpio9PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio9pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio9pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio9pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_pd` reader - Pull Down Resistor for GPIO9."]
pub type RegGpio9PdR = crate::BitReader<Gpio9pullDownResistor>;
impl RegGpio9PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio9pullDownResistor {
        match self.bits {
            false => Gpio9pullDownResistor::Disabled,
            true => Gpio9pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio9pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio9pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_9_pd` writer - Pull Down Resistor for GPIO9."]
pub type RegGpio9PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio9pullDownResistor>;
impl<'a, REG> RegGpio9PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO9.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio9functionSelect {
    #[doc = "4: `100`"]
    SpiMosiSpiMiso = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig1 = 7,
    #[doc = "8: `1000`"]
    PwmCh4 = 8,
    #[doc = "9: `1001`"]
    FemGpio1 = 9,
    #[doc = "10: `1010`"]
    GpipCh6GpipCh7 = 10,
    #[doc = "11: `1011`"]
    Swgpio9 = 11,
    #[doc = "14: `1110`"]
    E21Tdi = 14,
}
impl From<Gpio9functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio9functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio9functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio9functionSelect {}
#[doc = "Field `reg_gpio_9_func_sel` reader - Function select for GPIO9."]
pub type RegGpio9FuncSelR = crate::FieldReader<Gpio9functionSelect>;
impl RegGpio9FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio9functionSelect> {
        match self.bits {
            4 => Some(Gpio9functionSelect::SpiMosiSpiMiso),
            6 => Some(Gpio9functionSelect::I2cSda),
            7 => Some(Gpio9functionSelect::UartSig1),
            8 => Some(Gpio9functionSelect::PwmCh4),
            9 => Some(Gpio9functionSelect::FemGpio1),
            10 => Some(Gpio9functionSelect::GpipCh6GpipCh7),
            11 => Some(Gpio9functionSelect::Swgpio9),
            14 => Some(Gpio9functionSelect::E21Tdi),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == Gpio9functionSelect::SpiMosiSpiMiso
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio9functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig1(&self) -> bool {
        *self == Gpio9functionSelect::UartSig1
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch4(&self) -> bool {
        *self == Gpio9functionSelect::PwmCh4
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == Gpio9functionSelect::FemGpio1
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_gpip_ch6_gpip_ch7(&self) -> bool {
        *self == Gpio9functionSelect::GpipCh6GpipCh7
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_9(&self) -> bool {
        *self == Gpio9functionSelect::Swgpio9
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == Gpio9functionSelect::E21Tdi
    }
}
#[doc = "Field `reg_gpio_9_func_sel` writer - Function select for GPIO9."]
pub type RegGpio9FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio9functionSelect>;
impl<'a, REG> RegGpio9FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9functionSelect::SpiMosiSpiMiso)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9functionSelect::UartSig1)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9functionSelect::PwmCh4)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9functionSelect::FemGpio1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch6_gpip_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9functionSelect::GpipCh6GpipCh7)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_9(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9functionSelect::Swgpio9)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio9functionSelect::E21Tdi)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_ie(&self) -> RegGpio8IeR {
        RegGpio8IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_smt(&self) -> RegGpio8SmtR {
        RegGpio8SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_drv(&self) -> RegGpio8DrvR {
        RegGpio8DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_pu(&self) -> RegGpio8PuR {
        RegGpio8PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_pd(&self) -> RegGpio8PdR {
        RegGpio8PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_func_sel(&self) -> RegGpio8FuncSelR {
        RegGpio8FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_ie(&self) -> RegGpio9IeR {
        RegGpio9IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_smt(&self) -> RegGpio9SmtR {
        RegGpio9SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_drv(&self) -> RegGpio9DrvR {
        RegGpio9DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_pu(&self) -> RegGpio9PuR {
        RegGpio9PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_pd(&self) -> RegGpio9PdR {
        RegGpio9PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_func_sel(&self) -> RegGpio9FuncSelR {
        RegGpio9FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_ie(&mut self) -> RegGpio8IeW<'_, GpioCfgctl4Spec> {
        RegGpio8IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_smt(&mut self) -> RegGpio8SmtW<'_, GpioCfgctl4Spec> {
        RegGpio8SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_drv(&mut self) -> RegGpio8DrvW<'_, GpioCfgctl4Spec> {
        RegGpio8DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_pu(&mut self) -> RegGpio8PuW<'_, GpioCfgctl4Spec> {
        RegGpio8PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_pd(&mut self) -> RegGpio8PdW<'_, GpioCfgctl4Spec> {
        RegGpio8PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_func_sel(&mut self) -> RegGpio8FuncSelW<'_, GpioCfgctl4Spec> {
        RegGpio8FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_ie(&mut self) -> RegGpio9IeW<'_, GpioCfgctl4Spec> {
        RegGpio9IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_smt(&mut self) -> RegGpio9SmtW<'_, GpioCfgctl4Spec> {
        RegGpio9SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_drv(&mut self) -> RegGpio9DrvW<'_, GpioCfgctl4Spec> {
        RegGpio9DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_pu(&mut self) -> RegGpio9PuW<'_, GpioCfgctl4Spec> {
        RegGpio9PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_pd(&mut self) -> RegGpio9PdW<'_, GpioCfgctl4Spec> {
        RegGpio9PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_func_sel(&mut self) -> RegGpio9FuncSelW<'_, GpioCfgctl4Spec> {
        RegGpio9FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO8, GPIO9 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl4Spec;
impl crate::RegisterSpec for GpioCfgctl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl4::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl4::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL4 to value 0x0b03_0b03"]
impl crate::Resettable for GpioCfgctl4Spec {
    const RESET_VALUE: u32 = 0x0b03_0b03;
}
