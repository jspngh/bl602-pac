#[doc = "Register `GPIO_CFGCTL6` reader"]
pub type R = crate::R<GpioCfgctl6Spec>;
#[doc = "Register `GPIO_CFGCTL6` writer"]
pub type W = crate::W<GpioCfgctl6Spec>;
#[doc = "Input enable for GPIO12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio12inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio12inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_ie` reader - Input enable for GPIO12."]
pub type RegGpio12IeR = crate::BitReader<Gpio12inputEnabled>;
impl RegGpio12IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12inputEnabled {
        match self.bits {
            false => Gpio12inputEnabled::Disabled,
            true => Gpio12inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio12inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio12inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_12_ie` writer - Input enable for GPIO12."]
pub type RegGpio12IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio12inputEnabled>;
impl<'a, REG> RegGpio12IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio12schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio12schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_smt` reader - Schmitt trigger enabled for GPIO12."]
pub type RegGpio12SmtR = crate::BitReader<Gpio12schmitt>;
impl RegGpio12SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12schmitt {
        match self.bits {
            false => Gpio12schmitt::Disabled,
            true => Gpio12schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio12schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio12schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_12_smt` writer - Schmitt trigger enabled for GPIO12."]
pub type RegGpio12SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio12schmitt>;
impl<'a, REG> RegGpio12SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio12driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio12driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio12driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio12driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio12driving {}
#[doc = "Field `reg_gpio_12_drv` reader - Driving control enabled for GPIO12."]
pub type RegGpio12DrvR = crate::FieldReader<Gpio12driving>;
impl RegGpio12DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio12driving> {
        match self.bits {
            0 => Some(Gpio12driving::Disabled),
            1 => Some(Gpio12driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio12driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio12driving::Enabled
    }
}
#[doc = "Field `reg_gpio_12_drv` writer - Driving control enabled for GPIO12."]
pub type RegGpio12DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio12driving>;
impl<'a, REG> RegGpio12DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio12pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio12pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_pu` reader - Pull Up Resistor for GPIO12."]
pub type RegGpio12PuR = crate::BitReader<Gpio12pullUpResistor>;
impl RegGpio12PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12pullUpResistor {
        match self.bits {
            false => Gpio12pullUpResistor::Disabled,
            true => Gpio12pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio12pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio12pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_12_pu` writer - Pull Up Resistor for GPIO12."]
pub type RegGpio12PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio12pullUpResistor>;
impl<'a, REG> RegGpio12PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio12pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio12pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio12pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_pd` reader - Pull Down Resistor for GPIO12."]
pub type RegGpio12PdR = crate::BitReader<Gpio12pullDownResistor>;
impl RegGpio12PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio12pullDownResistor {
        match self.bits {
            false => Gpio12pullDownResistor::Disabled,
            true => Gpio12pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio12pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio12pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_12_pd` writer - Pull Down Resistor for GPIO12."]
pub type RegGpio12PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio12pullDownResistor>;
impl<'a, REG> RegGpio12PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO12.\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio12functionSelect {
    #[doc = "4: `100`"]
    SpiMisoSpiMosi = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig4 = 7,
    #[doc = "8: `1000`"]
    PwmCh2 = 8,
    #[doc = "9: `1001`"]
    FemGpio0 = 9,
    #[doc = "10: `1010`"]
    GpipCh0GpadcVrefExt = 10,
    #[doc = "11: `1011`"]
    Swgpio12 = 11,
    #[doc = "14: `1110`"]
    E21Tms = 14,
}
impl From<Gpio12functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio12functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio12functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio12functionSelect {}
#[doc = "Field `reg_gpio_12_func_sel` reader - Function select for GPIO12."]
pub type RegGpio12FuncSelR = crate::FieldReader<Gpio12functionSelect>;
impl RegGpio12FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio12functionSelect> {
        match self.bits {
            4 => Some(Gpio12functionSelect::SpiMisoSpiMosi),
            6 => Some(Gpio12functionSelect::I2cScl),
            7 => Some(Gpio12functionSelect::UartSig4),
            8 => Some(Gpio12functionSelect::PwmCh2),
            9 => Some(Gpio12functionSelect::FemGpio0),
            10 => Some(Gpio12functionSelect::GpipCh0GpadcVrefExt),
            11 => Some(Gpio12functionSelect::Swgpio12),
            14 => Some(Gpio12functionSelect::E21Tms),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_miso_spi_mosi(&self) -> bool {
        *self == Gpio12functionSelect::SpiMisoSpiMosi
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio12functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig4(&self) -> bool {
        *self == Gpio12functionSelect::UartSig4
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        *self == Gpio12functionSelect::PwmCh2
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == Gpio12functionSelect::FemGpio0
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_gpip_ch0_gpadc_vref_ext(&self) -> bool {
        *self == Gpio12functionSelect::GpipCh0GpadcVrefExt
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_12(&self) -> bool {
        *self == Gpio12functionSelect::Swgpio12
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == Gpio12functionSelect::E21Tms
    }
}
#[doc = "Field `reg_gpio_12_func_sel` writer - Function select for GPIO12."]
pub type RegGpio12FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio12functionSelect>;
impl<'a, REG> RegGpio12FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_miso_spi_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12functionSelect::SpiMisoSpiMosi)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig4(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12functionSelect::UartSig4)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12functionSelect::PwmCh2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12functionSelect::FemGpio0)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch0_gpadc_vref_ext(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12functionSelect::GpipCh0GpadcVrefExt)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_12(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12functionSelect::Swgpio12)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio12functionSelect::E21Tms)
    }
}
#[doc = "Input enable for GPIO13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio13inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio13inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_ie` reader - Input enable for GPIO13."]
pub type RegGpio13IeR = crate::BitReader<Gpio13inputEnabled>;
impl RegGpio13IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13inputEnabled {
        match self.bits {
            false => Gpio13inputEnabled::Disabled,
            true => Gpio13inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio13inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio13inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_13_ie` writer - Input enable for GPIO13."]
pub type RegGpio13IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio13inputEnabled>;
impl<'a, REG> RegGpio13IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio13schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio13schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_smt` reader - Schmitt trigger enabled for GPIO13."]
pub type RegGpio13SmtR = crate::BitReader<Gpio13schmitt>;
impl RegGpio13SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13schmitt {
        match self.bits {
            false => Gpio13schmitt::Disabled,
            true => Gpio13schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio13schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio13schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_13_smt` writer - Schmitt trigger enabled for GPIO13."]
pub type RegGpio13SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio13schmitt>;
impl<'a, REG> RegGpio13SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio13driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio13driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio13driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio13driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio13driving {}
#[doc = "Field `reg_gpio_13_drv` reader - Driving control enabled for GPIO13."]
pub type RegGpio13DrvR = crate::FieldReader<Gpio13driving>;
impl RegGpio13DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio13driving> {
        match self.bits {
            0 => Some(Gpio13driving::Disabled),
            1 => Some(Gpio13driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio13driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio13driving::Enabled
    }
}
#[doc = "Field `reg_gpio_13_drv` writer - Driving control enabled for GPIO13."]
pub type RegGpio13DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio13driving>;
impl<'a, REG> RegGpio13DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio13pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio13pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_pu` reader - Pull Up Resistor for GPIO13."]
pub type RegGpio13PuR = crate::BitReader<Gpio13pullUpResistor>;
impl RegGpio13PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13pullUpResistor {
        match self.bits {
            false => Gpio13pullUpResistor::Disabled,
            true => Gpio13pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio13pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio13pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_13_pu` writer - Pull Up Resistor for GPIO13."]
pub type RegGpio13PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio13pullUpResistor>;
impl<'a, REG> RegGpio13PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio13pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio13pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio13pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_pd` reader - Pull Down Resistor for GPIO13."]
pub type RegGpio13PdR = crate::BitReader<Gpio13pullDownResistor>;
impl RegGpio13PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio13pullDownResistor {
        match self.bits {
            false => Gpio13pullDownResistor::Disabled,
            true => Gpio13pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio13pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio13pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_13_pd` writer - Pull Down Resistor for GPIO13."]
pub type RegGpio13PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio13pullDownResistor>;
impl<'a, REG> RegGpio13PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO13.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio13functionSelect {
    #[doc = "4: `100`"]
    SpiMosiSpiMiso = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig5 = 7,
    #[doc = "8: `1000`"]
    PwmCh3 = 8,
    #[doc = "9: `1001`"]
    FemGpio1 = 9,
    #[doc = "10: `1010`"]
    GpipCh3 = 10,
    #[doc = "11: `1011`"]
    Swgpio13 = 11,
    #[doc = "14: `1110`"]
    E21Tdi = 14,
}
impl From<Gpio13functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio13functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio13functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio13functionSelect {}
#[doc = "Field `reg_gpio_13_func_sel` reader - Function select for GPIO13."]
pub type RegGpio13FuncSelR = crate::FieldReader<Gpio13functionSelect>;
impl RegGpio13FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio13functionSelect> {
        match self.bits {
            4 => Some(Gpio13functionSelect::SpiMosiSpiMiso),
            6 => Some(Gpio13functionSelect::I2cSda),
            7 => Some(Gpio13functionSelect::UartSig5),
            8 => Some(Gpio13functionSelect::PwmCh3),
            9 => Some(Gpio13functionSelect::FemGpio1),
            10 => Some(Gpio13functionSelect::GpipCh3),
            11 => Some(Gpio13functionSelect::Swgpio13),
            14 => Some(Gpio13functionSelect::E21Tdi),
            _ => None,
        }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == Gpio13functionSelect::SpiMosiSpiMiso
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio13functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig5(&self) -> bool {
        *self == Gpio13functionSelect::UartSig5
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch3(&self) -> bool {
        *self == Gpio13functionSelect::PwmCh3
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == Gpio13functionSelect::FemGpio1
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_gpip_ch3(&self) -> bool {
        *self == Gpio13functionSelect::GpipCh3
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_13(&self) -> bool {
        *self == Gpio13functionSelect::Swgpio13
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == Gpio13functionSelect::E21Tdi
    }
}
#[doc = "Field `reg_gpio_13_func_sel` writer - Function select for GPIO13."]
pub type RegGpio13FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio13functionSelect>;
impl<'a, REG> RegGpio13FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13functionSelect::SpiMosiSpiMiso)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig5(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13functionSelect::UartSig5)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13functionSelect::PwmCh3)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13functionSelect::FemGpio1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13functionSelect::GpipCh3)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_13(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13functionSelect::Swgpio13)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio13functionSelect::E21Tdi)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_ie(&self) -> RegGpio12IeR {
        RegGpio12IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_smt(&self) -> RegGpio12SmtR {
        RegGpio12SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_drv(&self) -> RegGpio12DrvR {
        RegGpio12DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_pu(&self) -> RegGpio12PuR {
        RegGpio12PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_pd(&self) -> RegGpio12PdR {
        RegGpio12PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_func_sel(&self) -> RegGpio12FuncSelR {
        RegGpio12FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_ie(&self) -> RegGpio13IeR {
        RegGpio13IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_smt(&self) -> RegGpio13SmtR {
        RegGpio13SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_drv(&self) -> RegGpio13DrvR {
        RegGpio13DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_pu(&self) -> RegGpio13PuR {
        RegGpio13PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_pd(&self) -> RegGpio13PdR {
        RegGpio13PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_func_sel(&self) -> RegGpio13FuncSelR {
        RegGpio13FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_ie(&mut self) -> RegGpio12IeW<'_, GpioCfgctl6Spec> {
        RegGpio12IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_smt(&mut self) -> RegGpio12SmtW<'_, GpioCfgctl6Spec> {
        RegGpio12SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_drv(&mut self) -> RegGpio12DrvW<'_, GpioCfgctl6Spec> {
        RegGpio12DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_pu(&mut self) -> RegGpio12PuW<'_, GpioCfgctl6Spec> {
        RegGpio12PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_pd(&mut self) -> RegGpio12PdW<'_, GpioCfgctl6Spec> {
        RegGpio12PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_func_sel(&mut self) -> RegGpio12FuncSelW<'_, GpioCfgctl6Spec> {
        RegGpio12FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_ie(&mut self) -> RegGpio13IeW<'_, GpioCfgctl6Spec> {
        RegGpio13IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_smt(&mut self) -> RegGpio13SmtW<'_, GpioCfgctl6Spec> {
        RegGpio13SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_drv(&mut self) -> RegGpio13DrvW<'_, GpioCfgctl6Spec> {
        RegGpio13DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_pu(&mut self) -> RegGpio13PuW<'_, GpioCfgctl6Spec> {
        RegGpio13PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_pd(&mut self) -> RegGpio13PdW<'_, GpioCfgctl6Spec> {
        RegGpio13PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_func_sel(&mut self) -> RegGpio13FuncSelW<'_, GpioCfgctl6Spec> {
        RegGpio13FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO12, GPIO13 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl6Spec;
impl crate::RegisterSpec for GpioCfgctl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl6::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl6Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl6::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL6 to value 0x0b03_0e03"]
impl crate::Resettable for GpioCfgctl6Spec {
    const RESET_VALUE: u32 = 0x0b03_0e03;
}
