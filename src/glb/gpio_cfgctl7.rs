#[doc = "Register `GPIO_CFGCTL7` reader"]
pub type R = crate::R<GpioCfgctl7Spec>;
#[doc = "Register `GPIO_CFGCTL7` writer"]
pub type W = crate::W<GpioCfgctl7Spec>;
#[doc = "Input enable for GPIO14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio14inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio14inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_ie` reader - Input enable for GPIO14."]
pub type RegGpio14IeR = crate::BitReader<Gpio14inputEnabled>;
impl RegGpio14IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14inputEnabled {
        match self.bits {
            false => Gpio14inputEnabled::Disabled,
            true => Gpio14inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio14inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio14inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_14_ie` writer - Input enable for GPIO14."]
pub type RegGpio14IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio14inputEnabled>;
impl<'a, REG> RegGpio14IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio14schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio14schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_smt` reader - Schmitt trigger enabled for GPIO14."]
pub type RegGpio14SmtR = crate::BitReader<Gpio14schmitt>;
impl RegGpio14SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14schmitt {
        match self.bits {
            false => Gpio14schmitt::Disabled,
            true => Gpio14schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio14schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio14schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_14_smt` writer - Schmitt trigger enabled for GPIO14."]
pub type RegGpio14SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio14schmitt>;
impl<'a, REG> RegGpio14SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio14driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio14driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio14driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio14driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio14driving {}
#[doc = "Field `reg_gpio_14_drv` reader - Driving control enabled for GPIO14."]
pub type RegGpio14DrvR = crate::FieldReader<Gpio14driving>;
impl RegGpio14DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio14driving> {
        match self.bits {
            0 => Some(Gpio14driving::Disabled),
            1 => Some(Gpio14driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio14driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio14driving::Enabled
    }
}
#[doc = "Field `reg_gpio_14_drv` writer - Driving control enabled for GPIO14."]
pub type RegGpio14DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio14driving>;
impl<'a, REG> RegGpio14DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio14pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio14pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_pu` reader - Pull Up Resistor for GPIO14."]
pub type RegGpio14PuR = crate::BitReader<Gpio14pullUpResistor>;
impl RegGpio14PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14pullUpResistor {
        match self.bits {
            false => Gpio14pullUpResistor::Disabled,
            true => Gpio14pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio14pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio14pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_14_pu` writer - Pull Up Resistor for GPIO14."]
pub type RegGpio14PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio14pullUpResistor>;
impl<'a, REG> RegGpio14PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio14pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio14pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio14pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_pd` reader - Pull Down Resistor for GPIO14."]
pub type RegGpio14PdR = crate::BitReader<Gpio14pullDownResistor>;
impl RegGpio14PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio14pullDownResistor {
        match self.bits {
            false => Gpio14pullDownResistor::Disabled,
            true => Gpio14pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio14pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio14pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_14_pd` writer - Pull Down Resistor for GPIO14."]
pub type RegGpio14PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio14pullDownResistor>;
impl<'a, REG> RegGpio14PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO14.\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio14functionSelect {
    #[doc = "4: `100`"]
    SpiSs = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig6 = 7,
    #[doc = "8: `1000`"]
    PwmCh4 = 8,
    #[doc = "9: `1001`"]
    FemGpio2 = 9,
    #[doc = "10: `1010`"]
    GpipCh2 = 10,
    #[doc = "11: `1011`"]
    Swgpio14 = 11,
    #[doc = "14: `1110`"]
    E21Tck = 14,
}
impl From<Gpio14functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio14functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio14functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio14functionSelect {}
#[doc = "Field `reg_gpio_14_func_sel` reader - Function select for GPIO14."]
pub type RegGpio14FuncSelR = crate::FieldReader<Gpio14functionSelect>;
impl RegGpio14FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio14functionSelect> {
        match self.bits {
            4 => Some(Gpio14functionSelect::SpiSs),
            6 => Some(Gpio14functionSelect::I2cScl),
            7 => Some(Gpio14functionSelect::UartSig6),
            8 => Some(Gpio14functionSelect::PwmCh4),
            9 => Some(Gpio14functionSelect::FemGpio2),
            10 => Some(Gpio14functionSelect::GpipCh2),
            11 => Some(Gpio14functionSelect::Swgpio14),
            14 => Some(Gpio14functionSelect::E21Tck),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == Gpio14functionSelect::SpiSs
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio14functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig6(&self) -> bool {
        *self == Gpio14functionSelect::UartSig6
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch4(&self) -> bool {
        *self == Gpio14functionSelect::PwmCh4
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == Gpio14functionSelect::FemGpio2
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_gpip_ch2(&self) -> bool {
        *self == Gpio14functionSelect::GpipCh2
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_14(&self) -> bool {
        *self == Gpio14functionSelect::Swgpio14
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == Gpio14functionSelect::E21Tck
    }
}
#[doc = "Field `reg_gpio_14_func_sel` writer - Function select for GPIO14."]
pub type RegGpio14FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio14functionSelect>;
impl<'a, REG> RegGpio14FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14functionSelect::SpiSs)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig6(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14functionSelect::UartSig6)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14functionSelect::PwmCh4)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14functionSelect::FemGpio2)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14functionSelect::GpipCh2)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_14(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14functionSelect::Swgpio14)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio14functionSelect::E21Tck)
    }
}
#[doc = "Input enable for GPIO15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio15inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio15inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_ie` reader - Input enable for GPIO15."]
pub type RegGpio15IeR = crate::BitReader<Gpio15inputEnabled>;
impl RegGpio15IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15inputEnabled {
        match self.bits {
            false => Gpio15inputEnabled::Disabled,
            true => Gpio15inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio15inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio15inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_15_ie` writer - Input enable for GPIO15."]
pub type RegGpio15IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio15inputEnabled>;
impl<'a, REG> RegGpio15IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio15schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio15schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_smt` reader - Schmitt trigger enabled for GPIO15."]
pub type RegGpio15SmtR = crate::BitReader<Gpio15schmitt>;
impl RegGpio15SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15schmitt {
        match self.bits {
            false => Gpio15schmitt::Disabled,
            true => Gpio15schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio15schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio15schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_15_smt` writer - Schmitt trigger enabled for GPIO15."]
pub type RegGpio15SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio15schmitt>;
impl<'a, REG> RegGpio15SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio15driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio15driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio15driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio15driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio15driving {}
#[doc = "Field `reg_gpio_15_drv` reader - Driving control enabled for GPIO15."]
pub type RegGpio15DrvR = crate::FieldReader<Gpio15driving>;
impl RegGpio15DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio15driving> {
        match self.bits {
            0 => Some(Gpio15driving::Disabled),
            1 => Some(Gpio15driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio15driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio15driving::Enabled
    }
}
#[doc = "Field `reg_gpio_15_drv` writer - Driving control enabled for GPIO15."]
pub type RegGpio15DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio15driving>;
impl<'a, REG> RegGpio15DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio15pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio15pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_pu` reader - Pull Up Resistor for GPIO15."]
pub type RegGpio15PuR = crate::BitReader<Gpio15pullUpResistor>;
impl RegGpio15PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15pullUpResistor {
        match self.bits {
            false => Gpio15pullUpResistor::Disabled,
            true => Gpio15pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio15pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio15pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_15_pu` writer - Pull Up Resistor for GPIO15."]
pub type RegGpio15PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio15pullUpResistor>;
impl<'a, REG> RegGpio15PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio15pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio15pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio15pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_pd` reader - Pull Down Resistor for GPIO15."]
pub type RegGpio15PdR = crate::BitReader<Gpio15pullDownResistor>;
impl RegGpio15PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio15pullDownResistor {
        match self.bits {
            false => Gpio15pullDownResistor::Disabled,
            true => Gpio15pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio15pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio15pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_15_pd` writer - Pull Down Resistor for GPIO15."]
pub type RegGpio15PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio15pullDownResistor>;
impl<'a, REG> RegGpio15PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO15.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio15functionSelect {
    #[doc = "4: `100`"]
    SpiSclk = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig7 = 7,
    #[doc = "8: `1000`"]
    PwmCh0 = 8,
    #[doc = "9: `1001`"]
    FemGpio3 = 9,
    #[doc = "10: `1010`"]
    PswIrrcvOutGpipCh11 = 10,
    #[doc = "11: `1011`"]
    Swgpio15 = 11,
    #[doc = "14: `1110`"]
    E21Tdo = 14,
}
impl From<Gpio15functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio15functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio15functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio15functionSelect {}
#[doc = "Field `reg_gpio_15_func_sel` reader - Function select for GPIO15."]
pub type RegGpio15FuncSelR = crate::FieldReader<Gpio15functionSelect>;
impl RegGpio15FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio15functionSelect> {
        match self.bits {
            4 => Some(Gpio15functionSelect::SpiSclk),
            6 => Some(Gpio15functionSelect::I2cSda),
            7 => Some(Gpio15functionSelect::UartSig7),
            8 => Some(Gpio15functionSelect::PwmCh0),
            9 => Some(Gpio15functionSelect::FemGpio3),
            10 => Some(Gpio15functionSelect::PswIrrcvOutGpipCh11),
            11 => Some(Gpio15functionSelect::Swgpio15),
            14 => Some(Gpio15functionSelect::E21Tdo),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        *self == Gpio15functionSelect::SpiSclk
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio15functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig7(&self) -> bool {
        *self == Gpio15functionSelect::UartSig7
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        *self == Gpio15functionSelect::PwmCh0
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        *self == Gpio15functionSelect::FemGpio3
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_psw_irrcv_out_gpip_ch11(&self) -> bool {
        *self == Gpio15functionSelect::PswIrrcvOutGpipCh11
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_15(&self) -> bool {
        *self == Gpio15functionSelect::Swgpio15
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        *self == Gpio15functionSelect::E21Tdo
    }
}
#[doc = "Field `reg_gpio_15_func_sel` writer - Function select for GPIO15."]
pub type RegGpio15FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio15functionSelect>;
impl<'a, REG> RegGpio15FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15functionSelect::SpiSclk)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig7(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15functionSelect::UartSig7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15functionSelect::PwmCh0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15functionSelect::FemGpio3)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn psw_irrcv_out_gpip_ch11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15functionSelect::PswIrrcvOutGpipCh11)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_15(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15functionSelect::Swgpio15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio15functionSelect::E21Tdo)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_ie(&self) -> RegGpio14IeR {
        RegGpio14IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_smt(&self) -> RegGpio14SmtR {
        RegGpio14SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_drv(&self) -> RegGpio14DrvR {
        RegGpio14DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_pu(&self) -> RegGpio14PuR {
        RegGpio14PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_pd(&self) -> RegGpio14PdR {
        RegGpio14PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_func_sel(&self) -> RegGpio14FuncSelR {
        RegGpio14FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_ie(&self) -> RegGpio15IeR {
        RegGpio15IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_smt(&self) -> RegGpio15SmtR {
        RegGpio15SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_drv(&self) -> RegGpio15DrvR {
        RegGpio15DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_pu(&self) -> RegGpio15PuR {
        RegGpio15PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_pd(&self) -> RegGpio15PdR {
        RegGpio15PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_func_sel(&self) -> RegGpio15FuncSelR {
        RegGpio15FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_ie(&mut self) -> RegGpio14IeW<'_, GpioCfgctl7Spec> {
        RegGpio14IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_smt(&mut self) -> RegGpio14SmtW<'_, GpioCfgctl7Spec> {
        RegGpio14SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_drv(&mut self) -> RegGpio14DrvW<'_, GpioCfgctl7Spec> {
        RegGpio14DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_pu(&mut self) -> RegGpio14PuW<'_, GpioCfgctl7Spec> {
        RegGpio14PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_pd(&mut self) -> RegGpio14PdW<'_, GpioCfgctl7Spec> {
        RegGpio14PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_func_sel(&mut self) -> RegGpio14FuncSelW<'_, GpioCfgctl7Spec> {
        RegGpio14FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_ie(&mut self) -> RegGpio15IeW<'_, GpioCfgctl7Spec> {
        RegGpio15IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_smt(&mut self) -> RegGpio15SmtW<'_, GpioCfgctl7Spec> {
        RegGpio15SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_drv(&mut self) -> RegGpio15DrvW<'_, GpioCfgctl7Spec> {
        RegGpio15DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_pu(&mut self) -> RegGpio15PuW<'_, GpioCfgctl7Spec> {
        RegGpio15PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_pd(&mut self) -> RegGpio15PdW<'_, GpioCfgctl7Spec> {
        RegGpio15PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_func_sel(&mut self) -> RegGpio15FuncSelW<'_, GpioCfgctl7Spec> {
        RegGpio15FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO14, GPIO15 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl7Spec;
impl crate::RegisterSpec for GpioCfgctl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl7::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl7Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl7::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL7 to value 0x0b03_0e03"]
impl crate::Resettable for GpioCfgctl7Spec {
    const RESET_VALUE: u32 = 0x0b03_0e03;
}
