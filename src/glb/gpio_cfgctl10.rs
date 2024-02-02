#[doc = "Register `GPIO_CFGCTL10` reader"]
pub type R = crate::R<GpioCfgctl10Spec>;
#[doc = "Register `GPIO_CFGCTL10` writer"]
pub type W = crate::W<GpioCfgctl10Spec>;
#[doc = "Input enable for GPIO20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio20inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio20inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_ie` reader - Input enable for GPIO20."]
pub type RegGpio20IeR = crate::BitReader<Gpio20inputEnabled>;
impl RegGpio20IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20inputEnabled {
        match self.bits {
            false => Gpio20inputEnabled::Disabled,
            true => Gpio20inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio20inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio20inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_20_ie` writer - Input enable for GPIO20."]
pub type RegGpio20IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio20inputEnabled>;
impl<'a, REG> RegGpio20IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio20schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio20schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_smt` reader - Schmitt trigger enabled for GPIO20."]
pub type RegGpio20SmtR = crate::BitReader<Gpio20schmitt>;
impl RegGpio20SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20schmitt {
        match self.bits {
            false => Gpio20schmitt::Disabled,
            true => Gpio20schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio20schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio20schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_20_smt` writer - Schmitt trigger enabled for GPIO20."]
pub type RegGpio20SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio20schmitt>;
impl<'a, REG> RegGpio20SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio20driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio20driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio20driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio20driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio20driving {}
#[doc = "Field `reg_gpio_20_drv` reader - Driving control enabled for GPIO20."]
pub type RegGpio20DrvR = crate::FieldReader<Gpio20driving>;
impl RegGpio20DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio20driving> {
        match self.bits {
            0 => Some(Gpio20driving::Disabled),
            1 => Some(Gpio20driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio20driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio20driving::Enabled
    }
}
#[doc = "Field `reg_gpio_20_drv` writer - Driving control enabled for GPIO20."]
pub type RegGpio20DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio20driving>;
impl<'a, REG> RegGpio20DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio20pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio20pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_pu` reader - Pull Up Resistor for GPIO20."]
pub type RegGpio20PuR = crate::BitReader<Gpio20pullUpResistor>;
impl RegGpio20PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20pullUpResistor {
        match self.bits {
            false => Gpio20pullUpResistor::Disabled,
            true => Gpio20pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio20pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio20pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_20_pu` writer - Pull Up Resistor for GPIO20."]
pub type RegGpio20PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio20pullUpResistor>;
impl<'a, REG> RegGpio20PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio20pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio20pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio20pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_pd` reader - Pull Down Resistor for GPIO20."]
pub type RegGpio20PdR = crate::BitReader<Gpio20pullDownResistor>;
impl RegGpio20PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio20pullDownResistor {
        match self.bits {
            false => Gpio20pullDownResistor::Disabled,
            true => Gpio20pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio20pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio20pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_20_pd` writer - Pull Down Resistor for GPIO20."]
pub type RegGpio20PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio20pullDownResistor>;
impl<'a, REG> RegGpio20PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO20.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio20functionSelect {
    #[doc = "2: `10`"]
    SfD0 = 2,
    #[doc = "4: `100`"]
    SpiMisoSpiMosi = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig4 = 7,
    #[doc = "8: `1000`"]
    PwmCh0 = 8,
    #[doc = "9: `1001`"]
    FemGpio0 = 9,
    #[doc = "11: `1011`"]
    Swgpio20 = 11,
    #[doc = "14: `1110`"]
    E21Tms = 14,
}
impl From<Gpio20functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio20functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio20functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio20functionSelect {}
#[doc = "Field `reg_gpio_20_func_sel` reader - Function select for GPIO20."]
pub type RegGpio20FuncSelR = crate::FieldReader<Gpio20functionSelect>;
impl RegGpio20FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio20functionSelect> {
        match self.bits {
            2 => Some(Gpio20functionSelect::SfD0),
            4 => Some(Gpio20functionSelect::SpiMisoSpiMosi),
            6 => Some(Gpio20functionSelect::I2cScl),
            7 => Some(Gpio20functionSelect::UartSig4),
            8 => Some(Gpio20functionSelect::PwmCh0),
            9 => Some(Gpio20functionSelect::FemGpio0),
            11 => Some(Gpio20functionSelect::Swgpio20),
            14 => Some(Gpio20functionSelect::E21Tms),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sf_d0(&self) -> bool {
        *self == Gpio20functionSelect::SfD0
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_miso_spi_mosi(&self) -> bool {
        *self == Gpio20functionSelect::SpiMisoSpiMosi
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio20functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig4(&self) -> bool {
        *self == Gpio20functionSelect::UartSig4
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        *self == Gpio20functionSelect::PwmCh0
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == Gpio20functionSelect::FemGpio0
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_20(&self) -> bool {
        *self == Gpio20functionSelect::Swgpio20
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == Gpio20functionSelect::E21Tms
    }
}
#[doc = "Field `reg_gpio_20_func_sel` writer - Function select for GPIO20."]
pub type RegGpio20FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio20functionSelect>;
impl<'a, REG> RegGpio20FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20functionSelect::SfD0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_miso_spi_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20functionSelect::SpiMisoSpiMosi)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig4(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20functionSelect::UartSig4)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20functionSelect::PwmCh0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20functionSelect::FemGpio0)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_20(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20functionSelect::Swgpio20)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio20functionSelect::E21Tms)
    }
}
#[doc = "Input enable for GPIO21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio21inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio21inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_ie` reader - Input enable for GPIO21."]
pub type RegGpio21IeR = crate::BitReader<Gpio21inputEnabled>;
impl RegGpio21IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21inputEnabled {
        match self.bits {
            false => Gpio21inputEnabled::Disabled,
            true => Gpio21inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio21inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio21inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_21_ie` writer - Input enable for GPIO21."]
pub type RegGpio21IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio21inputEnabled>;
impl<'a, REG> RegGpio21IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio21schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio21schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_smt` reader - Schmitt trigger enabled for GPIO21."]
pub type RegGpio21SmtR = crate::BitReader<Gpio21schmitt>;
impl RegGpio21SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21schmitt {
        match self.bits {
            false => Gpio21schmitt::Disabled,
            true => Gpio21schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio21schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio21schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_21_smt` writer - Schmitt trigger enabled for GPIO21."]
pub type RegGpio21SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio21schmitt>;
impl<'a, REG> RegGpio21SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio21driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio21driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio21driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio21driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio21driving {}
#[doc = "Field `reg_gpio_21_drv` reader - Driving control enabled for GPIO21."]
pub type RegGpio21DrvR = crate::FieldReader<Gpio21driving>;
impl RegGpio21DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio21driving> {
        match self.bits {
            0 => Some(Gpio21driving::Disabled),
            1 => Some(Gpio21driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio21driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio21driving::Enabled
    }
}
#[doc = "Field `reg_gpio_21_drv` writer - Driving control enabled for GPIO21."]
pub type RegGpio21DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio21driving>;
impl<'a, REG> RegGpio21DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio21pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio21pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_pu` reader - Pull Up Resistor for GPIO21."]
pub type RegGpio21PuR = crate::BitReader<Gpio21pullUpResistor>;
impl RegGpio21PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21pullUpResistor {
        match self.bits {
            false => Gpio21pullUpResistor::Disabled,
            true => Gpio21pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio21pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio21pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_21_pu` writer - Pull Up Resistor for GPIO21."]
pub type RegGpio21PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio21pullUpResistor>;
impl<'a, REG> RegGpio21PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio21pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio21pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio21pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_pd` reader - Pull Down Resistor for GPIO21."]
pub type RegGpio21PdR = crate::BitReader<Gpio21pullDownResistor>;
impl RegGpio21PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio21pullDownResistor {
        match self.bits {
            false => Gpio21pullDownResistor::Disabled,
            true => Gpio21pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio21pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio21pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_21_pd` writer - Pull Down Resistor for GPIO21."]
pub type RegGpio21PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio21pullDownResistor>;
impl<'a, REG> RegGpio21PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO21.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio21functionSelect {
    #[doc = "2: `10`"]
    SfCs = 2,
    #[doc = "4: `100`"]
    SpiMosiSpiMiso = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig5 = 7,
    #[doc = "8: `1000`"]
    PwmCh1 = 8,
    #[doc = "9: `1001`"]
    FemGpio1 = 9,
    #[doc = "11: `1011`"]
    Swgpio21 = 11,
    #[doc = "14: `1110`"]
    E21Tdi = 14,
}
impl From<Gpio21functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio21functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio21functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio21functionSelect {}
#[doc = "Field `reg_gpio_21_func_sel` reader - Function select for GPIO21."]
pub type RegGpio21FuncSelR = crate::FieldReader<Gpio21functionSelect>;
impl RegGpio21FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio21functionSelect> {
        match self.bits {
            2 => Some(Gpio21functionSelect::SfCs),
            4 => Some(Gpio21functionSelect::SpiMosiSpiMiso),
            6 => Some(Gpio21functionSelect::I2cSda),
            7 => Some(Gpio21functionSelect::UartSig5),
            8 => Some(Gpio21functionSelect::PwmCh1),
            9 => Some(Gpio21functionSelect::FemGpio1),
            11 => Some(Gpio21functionSelect::Swgpio21),
            14 => Some(Gpio21functionSelect::E21Tdi),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sf_cs(&self) -> bool {
        *self == Gpio21functionSelect::SfCs
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == Gpio21functionSelect::SpiMosiSpiMiso
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio21functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig5(&self) -> bool {
        *self == Gpio21functionSelect::UartSig5
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        *self == Gpio21functionSelect::PwmCh1
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == Gpio21functionSelect::FemGpio1
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_21(&self) -> bool {
        *self == Gpio21functionSelect::Swgpio21
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == Gpio21functionSelect::E21Tdi
    }
}
#[doc = "Field `reg_gpio_21_func_sel` writer - Function select for GPIO21."]
pub type RegGpio21FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio21functionSelect>;
impl<'a, REG> RegGpio21FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_cs(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21functionSelect::SfCs)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21functionSelect::SpiMosiSpiMiso)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig5(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21functionSelect::UartSig5)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21functionSelect::PwmCh1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21functionSelect::FemGpio1)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_21(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21functionSelect::Swgpio21)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio21functionSelect::E21Tdi)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_ie(&self) -> RegGpio20IeR {
        RegGpio20IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_smt(&self) -> RegGpio20SmtR {
        RegGpio20SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_drv(&self) -> RegGpio20DrvR {
        RegGpio20DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_pu(&self) -> RegGpio20PuR {
        RegGpio20PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_pd(&self) -> RegGpio20PdR {
        RegGpio20PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_func_sel(&self) -> RegGpio20FuncSelR {
        RegGpio20FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_ie(&self) -> RegGpio21IeR {
        RegGpio21IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_smt(&self) -> RegGpio21SmtR {
        RegGpio21SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_drv(&self) -> RegGpio21DrvR {
        RegGpio21DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_pu(&self) -> RegGpio21PuR {
        RegGpio21PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_pd(&self) -> RegGpio21PdR {
        RegGpio21PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_func_sel(&self) -> RegGpio21FuncSelR {
        RegGpio21FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_ie(&mut self) -> RegGpio20IeW<'_, GpioCfgctl10Spec> {
        RegGpio20IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_smt(&mut self) -> RegGpio20SmtW<'_, GpioCfgctl10Spec> {
        RegGpio20SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_drv(&mut self) -> RegGpio20DrvW<'_, GpioCfgctl10Spec> {
        RegGpio20DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_pu(&mut self) -> RegGpio20PuW<'_, GpioCfgctl10Spec> {
        RegGpio20PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_pd(&mut self) -> RegGpio20PdW<'_, GpioCfgctl10Spec> {
        RegGpio20PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_func_sel(&mut self) -> RegGpio20FuncSelW<'_, GpioCfgctl10Spec> {
        RegGpio20FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_ie(&mut self) -> RegGpio21IeW<'_, GpioCfgctl10Spec> {
        RegGpio21IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_smt(&mut self) -> RegGpio21SmtW<'_, GpioCfgctl10Spec> {
        RegGpio21SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_drv(&mut self) -> RegGpio21DrvW<'_, GpioCfgctl10Spec> {
        RegGpio21DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_pu(&mut self) -> RegGpio21PuW<'_, GpioCfgctl10Spec> {
        RegGpio21PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_pd(&mut self) -> RegGpio21PdW<'_, GpioCfgctl10Spec> {
        RegGpio21PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_func_sel(&mut self) -> RegGpio21FuncSelW<'_, GpioCfgctl10Spec> {
        RegGpio21FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO20, GPIO21 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl10Spec;
impl crate::RegisterSpec for GpioCfgctl10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl10::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl10Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl10::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL10 to value 0x0b03_0b03"]
impl crate::Resettable for GpioCfgctl10Spec {
    const RESET_VALUE: u32 = 0x0b03_0b03;
}
