#[doc = "Register `GPIO_CFGCTL8` reader"]
pub type R = crate::R<GpioCfgctl8Spec>;
#[doc = "Register `GPIO_CFGCTL8` writer"]
pub type W = crate::W<GpioCfgctl8Spec>;
#[doc = "Input enable for GPIO16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio16inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio16inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_ie` reader - Input enable for GPIO16."]
pub type RegGpio16IeR = crate::BitReader<Gpio16inputEnabled>;
impl RegGpio16IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16inputEnabled {
        match self.bits {
            false => Gpio16inputEnabled::Disabled,
            true => Gpio16inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio16inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio16inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_16_ie` writer - Input enable for GPIO16."]
pub type RegGpio16IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio16inputEnabled>;
impl<'a, REG> RegGpio16IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio16schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio16schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_smt` reader - Schmitt trigger enabled for GPIO16."]
pub type RegGpio16SmtR = crate::BitReader<Gpio16schmitt>;
impl RegGpio16SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16schmitt {
        match self.bits {
            false => Gpio16schmitt::Disabled,
            true => Gpio16schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio16schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio16schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_16_smt` writer - Schmitt trigger enabled for GPIO16."]
pub type RegGpio16SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio16schmitt>;
impl<'a, REG> RegGpio16SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio16driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio16driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio16driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio16driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio16driving {}
#[doc = "Field `reg_gpio_16_drv` reader - Driving control enabled for GPIO16."]
pub type RegGpio16DrvR = crate::FieldReader<Gpio16driving>;
impl RegGpio16DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio16driving> {
        match self.bits {
            0 => Some(Gpio16driving::Disabled),
            1 => Some(Gpio16driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio16driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio16driving::Enabled
    }
}
#[doc = "Field `reg_gpio_16_drv` writer - Driving control enabled for GPIO16."]
pub type RegGpio16DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio16driving>;
impl<'a, REG> RegGpio16DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio16pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio16pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_pu` reader - Pull Up Resistor for GPIO16."]
pub type RegGpio16PuR = crate::BitReader<Gpio16pullUpResistor>;
impl RegGpio16PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16pullUpResistor {
        match self.bits {
            false => Gpio16pullUpResistor::Disabled,
            true => Gpio16pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio16pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio16pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_16_pu` writer - Pull Up Resistor for GPIO16."]
pub type RegGpio16PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio16pullUpResistor>;
impl<'a, REG> RegGpio16PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio16pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio16pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio16pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_pd` reader - Pull Down Resistor for GPIO16."]
pub type RegGpio16PdR = crate::BitReader<Gpio16pullDownResistor>;
impl RegGpio16PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio16pullDownResistor {
        match self.bits {
            false => Gpio16pullDownResistor::Disabled,
            true => Gpio16pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio16pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio16pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_16_pd` writer - Pull Down Resistor for GPIO16."]
pub type RegGpio16PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio16pullDownResistor>;
impl<'a, REG> RegGpio16PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO16.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio16functionSelect {
    #[doc = "4: `100`"]
    SpiMisoSpiMosi = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig0 = 7,
    #[doc = "8: `1000`"]
    PwmCh1 = 8,
    #[doc = "9: `1001`"]
    FemGpio0 = 9,
    #[doc = "11: `1011`"]
    Swgpio16 = 11,
    #[doc = "14: `1110`"]
    E21Tms = 14,
}
impl From<Gpio16functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio16functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio16functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio16functionSelect {}
#[doc = "Field `reg_gpio_16_func_sel` reader - Function select for GPIO16."]
pub type RegGpio16FuncSelR = crate::FieldReader<Gpio16functionSelect>;
impl RegGpio16FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio16functionSelect> {
        match self.bits {
            4 => Some(Gpio16functionSelect::SpiMisoSpiMosi),
            6 => Some(Gpio16functionSelect::I2cScl),
            7 => Some(Gpio16functionSelect::UartSig0),
            8 => Some(Gpio16functionSelect::PwmCh1),
            9 => Some(Gpio16functionSelect::FemGpio0),
            11 => Some(Gpio16functionSelect::Swgpio16),
            14 => Some(Gpio16functionSelect::E21Tms),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_miso_spi_mosi(&self) -> bool {
        *self == Gpio16functionSelect::SpiMisoSpiMosi
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio16functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig0(&self) -> bool {
        *self == Gpio16functionSelect::UartSig0
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        *self == Gpio16functionSelect::PwmCh1
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == Gpio16functionSelect::FemGpio0
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_16(&self) -> bool {
        *self == Gpio16functionSelect::Swgpio16
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == Gpio16functionSelect::E21Tms
    }
}
#[doc = "Field `reg_gpio_16_func_sel` writer - Function select for GPIO16."]
pub type RegGpio16FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio16functionSelect>;
impl<'a, REG> RegGpio16FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_miso_spi_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16functionSelect::SpiMisoSpiMosi)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16functionSelect::UartSig0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16functionSelect::PwmCh1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16functionSelect::FemGpio0)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_16(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16functionSelect::Swgpio16)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio16functionSelect::E21Tms)
    }
}
#[doc = "Input enable for GPIO17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio17inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio17inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_ie` reader - Input enable for GPIO17."]
pub type RegGpio17IeR = crate::BitReader<Gpio17inputEnabled>;
impl RegGpio17IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17inputEnabled {
        match self.bits {
            false => Gpio17inputEnabled::Disabled,
            true => Gpio17inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio17inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio17inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_17_ie` writer - Input enable for GPIO17."]
pub type RegGpio17IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio17inputEnabled>;
impl<'a, REG> RegGpio17IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio17schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio17schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_smt` reader - Schmitt trigger enabled for GPIO17."]
pub type RegGpio17SmtR = crate::BitReader<Gpio17schmitt>;
impl RegGpio17SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17schmitt {
        match self.bits {
            false => Gpio17schmitt::Disabled,
            true => Gpio17schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio17schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio17schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_17_smt` writer - Schmitt trigger enabled for GPIO17."]
pub type RegGpio17SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio17schmitt>;
impl<'a, REG> RegGpio17SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio17driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio17driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio17driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio17driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio17driving {}
#[doc = "Field `reg_gpio_17_drv` reader - Driving control enabled for GPIO17."]
pub type RegGpio17DrvR = crate::FieldReader<Gpio17driving>;
impl RegGpio17DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio17driving> {
        match self.bits {
            0 => Some(Gpio17driving::Disabled),
            1 => Some(Gpio17driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio17driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio17driving::Enabled
    }
}
#[doc = "Field `reg_gpio_17_drv` writer - Driving control enabled for GPIO17."]
pub type RegGpio17DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio17driving>;
impl<'a, REG> RegGpio17DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio17pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio17pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_pu` reader - Pull Up Resistor for GPIO17."]
pub type RegGpio17PuR = crate::BitReader<Gpio17pullUpResistor>;
impl RegGpio17PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17pullUpResistor {
        match self.bits {
            false => Gpio17pullUpResistor::Disabled,
            true => Gpio17pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio17pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio17pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_17_pu` writer - Pull Up Resistor for GPIO17."]
pub type RegGpio17PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio17pullUpResistor>;
impl<'a, REG> RegGpio17PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio17pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio17pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio17pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_pd` reader - Pull Down Resistor for GPIO17."]
pub type RegGpio17PdR = crate::BitReader<Gpio17pullDownResistor>;
impl RegGpio17PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio17pullDownResistor {
        match self.bits {
            false => Gpio17pullDownResistor::Disabled,
            true => Gpio17pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio17pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio17pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_17_pd` writer - Pull Down Resistor for GPIO17."]
pub type RegGpio17PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio17pullDownResistor>;
impl<'a, REG> RegGpio17PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO17.\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio17functionSelect {
    #[doc = "2: `10`"]
    SfD3 = 2,
    #[doc = "4: `100`"]
    SpiMosiSpiMiso = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig1 = 7,
    #[doc = "8: `1000`"]
    PwmCh2 = 8,
    #[doc = "9: `1001`"]
    FemGpio1 = 9,
    #[doc = "10: `1010`"]
    PmipDcTpOut = 10,
    #[doc = "11: `1011`"]
    Swgpio17 = 11,
    #[doc = "14: `1110`"]
    E21Tdi = 14,
}
impl From<Gpio17functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio17functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio17functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio17functionSelect {}
#[doc = "Field `reg_gpio_17_func_sel` reader - Function select for GPIO17."]
pub type RegGpio17FuncSelR = crate::FieldReader<Gpio17functionSelect>;
impl RegGpio17FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio17functionSelect> {
        match self.bits {
            2 => Some(Gpio17functionSelect::SfD3),
            4 => Some(Gpio17functionSelect::SpiMosiSpiMiso),
            6 => Some(Gpio17functionSelect::I2cSda),
            7 => Some(Gpio17functionSelect::UartSig1),
            8 => Some(Gpio17functionSelect::PwmCh2),
            9 => Some(Gpio17functionSelect::FemGpio1),
            10 => Some(Gpio17functionSelect::PmipDcTpOut),
            11 => Some(Gpio17functionSelect::Swgpio17),
            14 => Some(Gpio17functionSelect::E21Tdi),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sf_d3(&self) -> bool {
        *self == Gpio17functionSelect::SfD3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == Gpio17functionSelect::SpiMosiSpiMiso
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio17functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig1(&self) -> bool {
        *self == Gpio17functionSelect::UartSig1
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        *self == Gpio17functionSelect::PwmCh2
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == Gpio17functionSelect::FemGpio1
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_pmip_dc_tp_out(&self) -> bool {
        *self == Gpio17functionSelect::PmipDcTpOut
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_17(&self) -> bool {
        *self == Gpio17functionSelect::Swgpio17
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == Gpio17functionSelect::E21Tdi
    }
}
#[doc = "Field `reg_gpio_17_func_sel` writer - Function select for GPIO17."]
pub type RegGpio17FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio17functionSelect>;
impl<'a, REG> RegGpio17FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17functionSelect::SfD3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17functionSelect::SpiMosiSpiMiso)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17functionSelect::UartSig1)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17functionSelect::PwmCh2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17functionSelect::FemGpio1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn pmip_dc_tp_out(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17functionSelect::PmipDcTpOut)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_17(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17functionSelect::Swgpio17)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio17functionSelect::E21Tdi)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_ie(&self) -> RegGpio16IeR {
        RegGpio16IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_smt(&self) -> RegGpio16SmtR {
        RegGpio16SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_drv(&self) -> RegGpio16DrvR {
        RegGpio16DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_pu(&self) -> RegGpio16PuR {
        RegGpio16PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_pd(&self) -> RegGpio16PdR {
        RegGpio16PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_func_sel(&self) -> RegGpio16FuncSelR {
        RegGpio16FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_ie(&self) -> RegGpio17IeR {
        RegGpio17IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_smt(&self) -> RegGpio17SmtR {
        RegGpio17SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_drv(&self) -> RegGpio17DrvR {
        RegGpio17DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_pu(&self) -> RegGpio17PuR {
        RegGpio17PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_pd(&self) -> RegGpio17PdR {
        RegGpio17PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_func_sel(&self) -> RegGpio17FuncSelR {
        RegGpio17FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_ie(&mut self) -> RegGpio16IeW<'_, GpioCfgctl8Spec> {
        RegGpio16IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_smt(&mut self) -> RegGpio16SmtW<'_, GpioCfgctl8Spec> {
        RegGpio16SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_drv(&mut self) -> RegGpio16DrvW<'_, GpioCfgctl8Spec> {
        RegGpio16DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_pu(&mut self) -> RegGpio16PuW<'_, GpioCfgctl8Spec> {
        RegGpio16PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_pd(&mut self) -> RegGpio16PdW<'_, GpioCfgctl8Spec> {
        RegGpio16PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_func_sel(&mut self) -> RegGpio16FuncSelW<'_, GpioCfgctl8Spec> {
        RegGpio16FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_ie(&mut self) -> RegGpio17IeW<'_, GpioCfgctl8Spec> {
        RegGpio17IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_smt(&mut self) -> RegGpio17SmtW<'_, GpioCfgctl8Spec> {
        RegGpio17SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_drv(&mut self) -> RegGpio17DrvW<'_, GpioCfgctl8Spec> {
        RegGpio17DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_pu(&mut self) -> RegGpio17PuW<'_, GpioCfgctl8Spec> {
        RegGpio17PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_pd(&mut self) -> RegGpio17PdW<'_, GpioCfgctl8Spec> {
        RegGpio17PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_func_sel(&mut self) -> RegGpio17FuncSelW<'_, GpioCfgctl8Spec> {
        RegGpio17FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO16, GPIO17 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl8Spec;
impl crate::RegisterSpec for GpioCfgctl8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl8::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl8::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL8 to value 0x0e03_0b03"]
impl crate::Resettable for GpioCfgctl8Spec {
    const RESET_VALUE: u32 = 0x0e03_0b03;
}
