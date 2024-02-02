#[doc = "Register `GPIO_CFGCTL3` reader"]
pub type R = crate::R<GpioCfgctl3Spec>;
#[doc = "Register `GPIO_CFGCTL3` writer"]
pub type W = crate::W<GpioCfgctl3Spec>;
#[doc = "Input enable for GPIO6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio6inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio6inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_ie` reader - Input enable for GPIO6."]
pub type RegGpio6IeR = crate::BitReader<Gpio6inputEnabled>;
impl RegGpio6IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6inputEnabled {
        match self.bits {
            false => Gpio6inputEnabled::Disabled,
            true => Gpio6inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio6inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio6inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_6_ie` writer - Input enable for GPIO6."]
pub type RegGpio6IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio6inputEnabled>;
impl<'a, REG> RegGpio6IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio6schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio6schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_smt` reader - Schmitt trigger enabled for GPIO6."]
pub type RegGpio6SmtR = crate::BitReader<Gpio6schmitt>;
impl RegGpio6SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6schmitt {
        match self.bits {
            false => Gpio6schmitt::Disabled,
            true => Gpio6schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio6schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio6schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_6_smt` writer - Schmitt trigger enabled for GPIO6."]
pub type RegGpio6SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio6schmitt>;
impl<'a, REG> RegGpio6SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio6driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio6driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio6driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio6driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio6driving {}
#[doc = "Field `reg_gpio_6_drv` reader - Driving control enabled for GPIO6."]
pub type RegGpio6DrvR = crate::FieldReader<Gpio6driving>;
impl RegGpio6DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio6driving> {
        match self.bits {
            0 => Some(Gpio6driving::Disabled),
            1 => Some(Gpio6driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio6driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio6driving::Enabled
    }
}
#[doc = "Field `reg_gpio_6_drv` writer - Driving control enabled for GPIO6."]
pub type RegGpio6DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio6driving>;
impl<'a, REG> RegGpio6DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio6pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio6pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_pu` reader - Pull Up Resistor for GPIO6."]
pub type RegGpio6PuR = crate::BitReader<Gpio6pullUpResistor>;
impl RegGpio6PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6pullUpResistor {
        match self.bits {
            false => Gpio6pullUpResistor::Disabled,
            true => Gpio6pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio6pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio6pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_6_pu` writer - Pull Up Resistor for GPIO6."]
pub type RegGpio6PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio6pullUpResistor>;
impl<'a, REG> RegGpio6PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio6pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio6pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio6pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_pd` reader - Pull Down Resistor for GPIO6."]
pub type RegGpio6PdR = crate::BitReader<Gpio6pullDownResistor>;
impl RegGpio6PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio6pullDownResistor {
        match self.bits {
            false => Gpio6pullDownResistor::Disabled,
            true => Gpio6pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio6pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio6pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_6_pd` writer - Pull Down Resistor for GPIO6."]
pub type RegGpio6PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio6pullDownResistor>;
impl<'a, REG> RegGpio6PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO6.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio6functionSelect {
    #[doc = "4: `100`"]
    SpiSs = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig6 = 7,
    #[doc = "8: `1000`"]
    PwmCh1 = 8,
    #[doc = "9: `1001`"]
    FemGpio2 = 9,
    #[doc = "10: `1010`"]
    GpipCh5 = 10,
    #[doc = "11: `1011`"]
    Swgpio6 = 11,
    #[doc = "14: `1110`"]
    E21Tck = 14,
}
impl From<Gpio6functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio6functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio6functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio6functionSelect {}
#[doc = "Field `reg_gpio_6_func_sel` reader - Function select for GPIO6."]
pub type RegGpio6FuncSelR = crate::FieldReader<Gpio6functionSelect>;
impl RegGpio6FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio6functionSelect> {
        match self.bits {
            4 => Some(Gpio6functionSelect::SpiSs),
            6 => Some(Gpio6functionSelect::I2cScl),
            7 => Some(Gpio6functionSelect::UartSig6),
            8 => Some(Gpio6functionSelect::PwmCh1),
            9 => Some(Gpio6functionSelect::FemGpio2),
            10 => Some(Gpio6functionSelect::GpipCh5),
            11 => Some(Gpio6functionSelect::Swgpio6),
            14 => Some(Gpio6functionSelect::E21Tck),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == Gpio6functionSelect::SpiSs
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio6functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig6(&self) -> bool {
        *self == Gpio6functionSelect::UartSig6
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        *self == Gpio6functionSelect::PwmCh1
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == Gpio6functionSelect::FemGpio2
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_gpip_ch5(&self) -> bool {
        *self == Gpio6functionSelect::GpipCh5
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_6(&self) -> bool {
        *self == Gpio6functionSelect::Swgpio6
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == Gpio6functionSelect::E21Tck
    }
}
#[doc = "Field `reg_gpio_6_func_sel` writer - Function select for GPIO6."]
pub type RegGpio6FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio6functionSelect>;
impl<'a, REG> RegGpio6FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6functionSelect::SpiSs)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig6(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6functionSelect::UartSig6)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6functionSelect::PwmCh1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6functionSelect::FemGpio2)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch5(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6functionSelect::GpipCh5)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_6(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6functionSelect::Swgpio6)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio6functionSelect::E21Tck)
    }
}
#[doc = "Input enable for GPIO7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio7inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio7inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_ie` reader - Input enable for GPIO7."]
pub type RegGpio7IeR = crate::BitReader<Gpio7inputEnabled>;
impl RegGpio7IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7inputEnabled {
        match self.bits {
            false => Gpio7inputEnabled::Disabled,
            true => Gpio7inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio7inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio7inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_7_ie` writer - Input enable for GPIO7."]
pub type RegGpio7IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio7inputEnabled>;
impl<'a, REG> RegGpio7IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio7schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio7schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_smt` reader - Schmitt trigger enabled for GPIO7."]
pub type RegGpio7SmtR = crate::BitReader<Gpio7schmitt>;
impl RegGpio7SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7schmitt {
        match self.bits {
            false => Gpio7schmitt::Disabled,
            true => Gpio7schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio7schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio7schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_7_smt` writer - Schmitt trigger enabled for GPIO7."]
pub type RegGpio7SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio7schmitt>;
impl<'a, REG> RegGpio7SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio7driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio7driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio7driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio7driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio7driving {}
#[doc = "Field `reg_gpio_7_drv` reader - Driving control enabled for GPIO7."]
pub type RegGpio7DrvR = crate::FieldReader<Gpio7driving>;
impl RegGpio7DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio7driving> {
        match self.bits {
            0 => Some(Gpio7driving::Disabled),
            1 => Some(Gpio7driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio7driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio7driving::Enabled
    }
}
#[doc = "Field `reg_gpio_7_drv` writer - Driving control enabled for GPIO7."]
pub type RegGpio7DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio7driving>;
impl<'a, REG> RegGpio7DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio7pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio7pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_pu` reader - Pull Up Resistor for GPIO7."]
pub type RegGpio7PuR = crate::BitReader<Gpio7pullUpResistor>;
impl RegGpio7PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7pullUpResistor {
        match self.bits {
            false => Gpio7pullUpResistor::Disabled,
            true => Gpio7pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio7pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio7pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_7_pu` writer - Pull Up Resistor for GPIO7."]
pub type RegGpio7PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio7pullUpResistor>;
impl<'a, REG> RegGpio7PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio7pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio7pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio7pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_pd` reader - Pull Down Resistor for GPIO7."]
pub type RegGpio7PdR = crate::BitReader<Gpio7pullDownResistor>;
impl RegGpio7PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio7pullDownResistor {
        match self.bits {
            false => Gpio7pullDownResistor::Disabled,
            true => Gpio7pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio7pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio7pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_7_pd` writer - Pull Down Resistor for GPIO7."]
pub type RegGpio7PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio7pullDownResistor>;
impl<'a, REG> RegGpio7PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO7.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio7functionSelect {
    #[doc = "4: `100`"]
    SpiSclk = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig7 = 7,
    #[doc = "8: `1000`"]
    PwmCh2 = 8,
    #[doc = "9: `1001`"]
    FemGpio3 = 9,
    #[doc = "11: `1011`"]
    Swgpio7 = 11,
    #[doc = "14: `1110`"]
    E21Tdo = 14,
}
impl From<Gpio7functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio7functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio7functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio7functionSelect {}
#[doc = "Field `reg_gpio_7_func_sel` reader - Function select for GPIO7."]
pub type RegGpio7FuncSelR = crate::FieldReader<Gpio7functionSelect>;
impl RegGpio7FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio7functionSelect> {
        match self.bits {
            4 => Some(Gpio7functionSelect::SpiSclk),
            6 => Some(Gpio7functionSelect::I2cSda),
            7 => Some(Gpio7functionSelect::UartSig7),
            8 => Some(Gpio7functionSelect::PwmCh2),
            9 => Some(Gpio7functionSelect::FemGpio3),
            11 => Some(Gpio7functionSelect::Swgpio7),
            14 => Some(Gpio7functionSelect::E21Tdo),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        *self == Gpio7functionSelect::SpiSclk
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio7functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig7(&self) -> bool {
        *self == Gpio7functionSelect::UartSig7
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        *self == Gpio7functionSelect::PwmCh2
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        *self == Gpio7functionSelect::FemGpio3
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_7(&self) -> bool {
        *self == Gpio7functionSelect::Swgpio7
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        *self == Gpio7functionSelect::E21Tdo
    }
}
#[doc = "Field `reg_gpio_7_func_sel` writer - Function select for GPIO7."]
pub type RegGpio7FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio7functionSelect>;
impl<'a, REG> RegGpio7FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7functionSelect::SpiSclk)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig7(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7functionSelect::UartSig7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7functionSelect::PwmCh2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7functionSelect::FemGpio3)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_7(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7functionSelect::Swgpio7)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio7functionSelect::E21Tdo)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_ie(&self) -> RegGpio6IeR {
        RegGpio6IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_smt(&self) -> RegGpio6SmtR {
        RegGpio6SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_drv(&self) -> RegGpio6DrvR {
        RegGpio6DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_pu(&self) -> RegGpio6PuR {
        RegGpio6PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_pd(&self) -> RegGpio6PdR {
        RegGpio6PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_func_sel(&self) -> RegGpio6FuncSelR {
        RegGpio6FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_ie(&self) -> RegGpio7IeR {
        RegGpio7IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_smt(&self) -> RegGpio7SmtR {
        RegGpio7SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_drv(&self) -> RegGpio7DrvR {
        RegGpio7DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_pu(&self) -> RegGpio7PuR {
        RegGpio7PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_pd(&self) -> RegGpio7PdR {
        RegGpio7PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_func_sel(&self) -> RegGpio7FuncSelR {
        RegGpio7FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_ie(&mut self) -> RegGpio6IeW<'_, GpioCfgctl3Spec> {
        RegGpio6IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_smt(&mut self) -> RegGpio6SmtW<'_, GpioCfgctl3Spec> {
        RegGpio6SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_drv(&mut self) -> RegGpio6DrvW<'_, GpioCfgctl3Spec> {
        RegGpio6DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_pu(&mut self) -> RegGpio6PuW<'_, GpioCfgctl3Spec> {
        RegGpio6PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_pd(&mut self) -> RegGpio6PdW<'_, GpioCfgctl3Spec> {
        RegGpio6PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_func_sel(&mut self) -> RegGpio6FuncSelW<'_, GpioCfgctl3Spec> {
        RegGpio6FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_ie(&mut self) -> RegGpio7IeW<'_, GpioCfgctl3Spec> {
        RegGpio7IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_smt(&mut self) -> RegGpio7SmtW<'_, GpioCfgctl3Spec> {
        RegGpio7SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_drv(&mut self) -> RegGpio7DrvW<'_, GpioCfgctl3Spec> {
        RegGpio7DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_pu(&mut self) -> RegGpio7PuW<'_, GpioCfgctl3Spec> {
        RegGpio7PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_pd(&mut self) -> RegGpio7PdW<'_, GpioCfgctl3Spec> {
        RegGpio7PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_func_sel(&mut self) -> RegGpio7FuncSelW<'_, GpioCfgctl3Spec> {
        RegGpio7FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO6, GPIO7 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl3Spec;
impl crate::RegisterSpec for GpioCfgctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl3::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl3::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL3 to value 0x0b03_0b03"]
impl crate::Resettable for GpioCfgctl3Spec {
    const RESET_VALUE: u32 = 0x0b03_0b03;
}
