#[doc = "Register `GPIO_CFGCTL5` reader"]
pub type R = crate::R<GpioCfgctl5Spec>;
#[doc = "Register `GPIO_CFGCTL5` writer"]
pub type W = crate::W<GpioCfgctl5Spec>;
#[doc = "Input enable for GPIO10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio10inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio10inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_ie` reader - Input enable for GPIO10."]
pub type RegGpio10IeR = crate::BitReader<Gpio10inputEnabled>;
impl RegGpio10IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10inputEnabled {
        match self.bits {
            false => Gpio10inputEnabled::Disabled,
            true => Gpio10inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio10inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio10inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_10_ie` writer - Input enable for GPIO10."]
pub type RegGpio10IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio10inputEnabled>;
impl<'a, REG> RegGpio10IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio10schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio10schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_smt` reader - Schmitt trigger enabled for GPIO10."]
pub type RegGpio10SmtR = crate::BitReader<Gpio10schmitt>;
impl RegGpio10SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10schmitt {
        match self.bits {
            false => Gpio10schmitt::Disabled,
            true => Gpio10schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio10schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio10schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_10_smt` writer - Schmitt trigger enabled for GPIO10."]
pub type RegGpio10SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio10schmitt>;
impl<'a, REG> RegGpio10SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio10driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio10driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio10driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio10driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio10driving {}
#[doc = "Field `reg_gpio_10_drv` reader - Driving control enabled for GPIO10."]
pub type RegGpio10DrvR = crate::FieldReader<Gpio10driving>;
impl RegGpio10DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio10driving> {
        match self.bits {
            0 => Some(Gpio10driving::Disabled),
            1 => Some(Gpio10driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio10driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio10driving::Enabled
    }
}
#[doc = "Field `reg_gpio_10_drv` writer - Driving control enabled for GPIO10."]
pub type RegGpio10DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio10driving>;
impl<'a, REG> RegGpio10DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio10pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio10pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_pu` reader - Pull Up Resistor for GPIO10."]
pub type RegGpio10PuR = crate::BitReader<Gpio10pullUpResistor>;
impl RegGpio10PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10pullUpResistor {
        match self.bits {
            false => Gpio10pullUpResistor::Disabled,
            true => Gpio10pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio10pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio10pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_10_pu` writer - Pull Up Resistor for GPIO10."]
pub type RegGpio10PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio10pullUpResistor>;
impl<'a, REG> RegGpio10PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio10pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio10pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio10pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_pd` reader - Pull Down Resistor for GPIO10."]
pub type RegGpio10PdR = crate::BitReader<Gpio10pullDownResistor>;
impl RegGpio10PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio10pullDownResistor {
        match self.bits {
            false => Gpio10pullDownResistor::Disabled,
            true => Gpio10pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio10pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio10pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_10_pd` writer - Pull Down Resistor for GPIO10."]
pub type RegGpio10PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio10pullDownResistor>;
impl<'a, REG> RegGpio10PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO10.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio10functionSelect {
    #[doc = "4: `100`"]
    SpiSs = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig2 = 7,
    #[doc = "8: `1000`"]
    PwmCh0 = 8,
    #[doc = "9: `1001`"]
    FemGpio2 = 9,
    #[doc = "10: `1010`"]
    MicbiasGpipCh8GpipCh9 = 10,
    #[doc = "11: `1011`"]
    Swgpio10 = 11,
    #[doc = "14: `1110`"]
    E21Tck = 14,
}
impl From<Gpio10functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio10functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio10functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio10functionSelect {}
#[doc = "Field `reg_gpio_10_func_sel` reader - Function select for GPIO10."]
pub type RegGpio10FuncSelR = crate::FieldReader<Gpio10functionSelect>;
impl RegGpio10FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio10functionSelect> {
        match self.bits {
            4 => Some(Gpio10functionSelect::SpiSs),
            6 => Some(Gpio10functionSelect::I2cScl),
            7 => Some(Gpio10functionSelect::UartSig2),
            8 => Some(Gpio10functionSelect::PwmCh0),
            9 => Some(Gpio10functionSelect::FemGpio2),
            10 => Some(Gpio10functionSelect::MicbiasGpipCh8GpipCh9),
            11 => Some(Gpio10functionSelect::Swgpio10),
            14 => Some(Gpio10functionSelect::E21Tck),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == Gpio10functionSelect::SpiSs
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio10functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig2(&self) -> bool {
        *self == Gpio10functionSelect::UartSig2
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        *self == Gpio10functionSelect::PwmCh0
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == Gpio10functionSelect::FemGpio2
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_micbias_gpip_ch8_gpip_ch9(&self) -> bool {
        *self == Gpio10functionSelect::MicbiasGpipCh8GpipCh9
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_10(&self) -> bool {
        *self == Gpio10functionSelect::Swgpio10
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == Gpio10functionSelect::E21Tck
    }
}
#[doc = "Field `reg_gpio_10_func_sel` writer - Function select for GPIO10."]
pub type RegGpio10FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio10functionSelect>;
impl<'a, REG> RegGpio10FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10functionSelect::SpiSs)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10functionSelect::UartSig2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10functionSelect::PwmCh0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10functionSelect::FemGpio2)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn micbias_gpip_ch8_gpip_ch9(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10functionSelect::MicbiasGpipCh8GpipCh9)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10functionSelect::Swgpio10)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio10functionSelect::E21Tck)
    }
}
#[doc = "Input enable for GPIO11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio11inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio11inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_ie` reader - Input enable for GPIO11."]
pub type RegGpio11IeR = crate::BitReader<Gpio11inputEnabled>;
impl RegGpio11IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11inputEnabled {
        match self.bits {
            false => Gpio11inputEnabled::Disabled,
            true => Gpio11inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio11inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio11inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_11_ie` writer - Input enable for GPIO11."]
pub type RegGpio11IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio11inputEnabled>;
impl<'a, REG> RegGpio11IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio11schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio11schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_smt` reader - Schmitt trigger enabled for GPIO11."]
pub type RegGpio11SmtR = crate::BitReader<Gpio11schmitt>;
impl RegGpio11SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11schmitt {
        match self.bits {
            false => Gpio11schmitt::Disabled,
            true => Gpio11schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio11schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio11schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_11_smt` writer - Schmitt trigger enabled for GPIO11."]
pub type RegGpio11SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio11schmitt>;
impl<'a, REG> RegGpio11SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio11driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio11driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio11driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio11driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio11driving {}
#[doc = "Field `reg_gpio_11_drv` reader - Driving control enabled for GPIO11."]
pub type RegGpio11DrvR = crate::FieldReader<Gpio11driving>;
impl RegGpio11DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio11driving> {
        match self.bits {
            0 => Some(Gpio11driving::Disabled),
            1 => Some(Gpio11driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio11driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio11driving::Enabled
    }
}
#[doc = "Field `reg_gpio_11_drv` writer - Driving control enabled for GPIO11."]
pub type RegGpio11DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio11driving>;
impl<'a, REG> RegGpio11DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio11pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio11pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_pu` reader - Pull Up Resistor for GPIO11."]
pub type RegGpio11PuR = crate::BitReader<Gpio11pullUpResistor>;
impl RegGpio11PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11pullUpResistor {
        match self.bits {
            false => Gpio11pullUpResistor::Disabled,
            true => Gpio11pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio11pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio11pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_11_pu` writer - Pull Up Resistor for GPIO11."]
pub type RegGpio11PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio11pullUpResistor>;
impl<'a, REG> RegGpio11PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio11pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio11pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio11pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_pd` reader - Pull Down Resistor for GPIO11."]
pub type RegGpio11PdR = crate::BitReader<Gpio11pullDownResistor>;
impl RegGpio11PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio11pullDownResistor {
        match self.bits {
            false => Gpio11pullDownResistor::Disabled,
            true => Gpio11pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio11pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio11pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_11_pd` writer - Pull Down Resistor for GPIO11."]
pub type RegGpio11PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio11pullDownResistor>;
impl<'a, REG> RegGpio11PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO11.\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio11functionSelect {
    #[doc = "4: `100`"]
    SpiSclk = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig3 = 7,
    #[doc = "8: `1000`"]
    PwmCh1 = 8,
    #[doc = "9: `1001`"]
    FemGpio3 = 9,
    #[doc = "10: `1010`"]
    IrledOutGpipCh10 = 10,
    #[doc = "11: `1011`"]
    Swgpio11 = 11,
    #[doc = "14: `1110`"]
    E21Tdo = 14,
}
impl From<Gpio11functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio11functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio11functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio11functionSelect {}
#[doc = "Field `reg_gpio_11_func_sel` reader - Function select for GPIO11."]
pub type RegGpio11FuncSelR = crate::FieldReader<Gpio11functionSelect>;
impl RegGpio11FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio11functionSelect> {
        match self.bits {
            4 => Some(Gpio11functionSelect::SpiSclk),
            6 => Some(Gpio11functionSelect::I2cSda),
            7 => Some(Gpio11functionSelect::UartSig3),
            8 => Some(Gpio11functionSelect::PwmCh1),
            9 => Some(Gpio11functionSelect::FemGpio3),
            10 => Some(Gpio11functionSelect::IrledOutGpipCh10),
            11 => Some(Gpio11functionSelect::Swgpio11),
            14 => Some(Gpio11functionSelect::E21Tdo),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        *self == Gpio11functionSelect::SpiSclk
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio11functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig3(&self) -> bool {
        *self == Gpio11functionSelect::UartSig3
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        *self == Gpio11functionSelect::PwmCh1
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        *self == Gpio11functionSelect::FemGpio3
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_irled_out_gpip_ch10(&self) -> bool {
        *self == Gpio11functionSelect::IrledOutGpipCh10
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_11(&self) -> bool {
        *self == Gpio11functionSelect::Swgpio11
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        *self == Gpio11functionSelect::E21Tdo
    }
}
#[doc = "Field `reg_gpio_11_func_sel` writer - Function select for GPIO11."]
pub type RegGpio11FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio11functionSelect>;
impl<'a, REG> RegGpio11FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11functionSelect::SpiSclk)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11functionSelect::UartSig3)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11functionSelect::PwmCh1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11functionSelect::FemGpio3)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn irled_out_gpip_ch10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11functionSelect::IrledOutGpipCh10)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11functionSelect::Swgpio11)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio11functionSelect::E21Tdo)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_ie(&self) -> RegGpio10IeR {
        RegGpio10IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_smt(&self) -> RegGpio10SmtR {
        RegGpio10SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_drv(&self) -> RegGpio10DrvR {
        RegGpio10DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_pu(&self) -> RegGpio10PuR {
        RegGpio10PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_pd(&self) -> RegGpio10PdR {
        RegGpio10PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_func_sel(&self) -> RegGpio10FuncSelR {
        RegGpio10FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_ie(&self) -> RegGpio11IeR {
        RegGpio11IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_smt(&self) -> RegGpio11SmtR {
        RegGpio11SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_drv(&self) -> RegGpio11DrvR {
        RegGpio11DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_pu(&self) -> RegGpio11PuR {
        RegGpio11PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_pd(&self) -> RegGpio11PdR {
        RegGpio11PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_func_sel(&self) -> RegGpio11FuncSelR {
        RegGpio11FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_ie(&mut self) -> RegGpio10IeW<'_, GpioCfgctl5Spec> {
        RegGpio10IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_smt(&mut self) -> RegGpio10SmtW<'_, GpioCfgctl5Spec> {
        RegGpio10SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_drv(&mut self) -> RegGpio10DrvW<'_, GpioCfgctl5Spec> {
        RegGpio10DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_pu(&mut self) -> RegGpio10PuW<'_, GpioCfgctl5Spec> {
        RegGpio10PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_pd(&mut self) -> RegGpio10PdW<'_, GpioCfgctl5Spec> {
        RegGpio10PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_func_sel(&mut self) -> RegGpio10FuncSelW<'_, GpioCfgctl5Spec> {
        RegGpio10FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_ie(&mut self) -> RegGpio11IeW<'_, GpioCfgctl5Spec> {
        RegGpio11IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_smt(&mut self) -> RegGpio11SmtW<'_, GpioCfgctl5Spec> {
        RegGpio11SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_drv(&mut self) -> RegGpio11DrvW<'_, GpioCfgctl5Spec> {
        RegGpio11DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_pu(&mut self) -> RegGpio11PuW<'_, GpioCfgctl5Spec> {
        RegGpio11PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_pd(&mut self) -> RegGpio11PdW<'_, GpioCfgctl5Spec> {
        RegGpio11PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_func_sel(&mut self) -> RegGpio11FuncSelW<'_, GpioCfgctl5Spec> {
        RegGpio11FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO10, GPIO11 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl5Spec;
impl crate::RegisterSpec for GpioCfgctl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl5::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl5Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl5::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL5 to value 0x0e03_0b03"]
impl crate::Resettable for GpioCfgctl5Spec {
    const RESET_VALUE: u32 = 0x0e03_0b03;
}
