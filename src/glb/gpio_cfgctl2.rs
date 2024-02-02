#[doc = "Register `GPIO_CFGCTL2` reader"]
pub type R = crate::R<GpioCfgctl2Spec>;
#[doc = "Register `GPIO_CFGCTL2` writer"]
pub type W = crate::W<GpioCfgctl2Spec>;
#[doc = "Input enable for GPIO4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio4inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio4inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_ie` reader - Input enable for GPIO4."]
pub type RegGpio4IeR = crate::BitReader<Gpio4inputEnabled>;
impl RegGpio4IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4inputEnabled {
        match self.bits {
            false => Gpio4inputEnabled::Disabled,
            true => Gpio4inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio4inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio4inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_4_ie` writer - Input enable for GPIO4."]
pub type RegGpio4IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio4inputEnabled>;
impl<'a, REG> RegGpio4IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio4schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio4schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_smt` reader - Schmitt trigger enabled for GPIO4."]
pub type RegGpio4SmtR = crate::BitReader<Gpio4schmitt>;
impl RegGpio4SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4schmitt {
        match self.bits {
            false => Gpio4schmitt::Disabled,
            true => Gpio4schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio4schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio4schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_4_smt` writer - Schmitt trigger enabled for GPIO4."]
pub type RegGpio4SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio4schmitt>;
impl<'a, REG> RegGpio4SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio4driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio4driving {}
#[doc = "Field `reg_gpio_4_drv` reader - Driving control enabled for GPIO4."]
pub type RegGpio4DrvR = crate::FieldReader<Gpio4driving>;
impl RegGpio4DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4driving> {
        match self.bits {
            0 => Some(Gpio4driving::Disabled),
            1 => Some(Gpio4driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio4driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio4driving::Enabled
    }
}
#[doc = "Field `reg_gpio_4_drv` writer - Driving control enabled for GPIO4."]
pub type RegGpio4DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio4driving>;
impl<'a, REG> RegGpio4DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio4pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio4pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_pu` reader - Pull Up Resistor for GPIO4."]
pub type RegGpio4PuR = crate::BitReader<Gpio4pullUpResistor>;
impl RegGpio4PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4pullUpResistor {
        match self.bits {
            false => Gpio4pullUpResistor::Disabled,
            true => Gpio4pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio4pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio4pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_4_pu` writer - Pull Up Resistor for GPIO4."]
pub type RegGpio4PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio4pullUpResistor>;
impl<'a, REG> RegGpio4PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio4pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio4pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio4pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_pd` reader - Pull Down Resistor for GPIO4."]
pub type RegGpio4PdR = crate::BitReader<Gpio4pullDownResistor>;
impl RegGpio4PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4pullDownResistor {
        match self.bits {
            false => Gpio4pullDownResistor::Disabled,
            true => Gpio4pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio4pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio4pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_4_pd` writer - Pull Down Resistor for GPIO4."]
pub type RegGpio4PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio4pullDownResistor>;
impl<'a, REG> RegGpio4PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4functionSelect {
    #[doc = "1: `1`"]
    SdioDat2 = 1,
    #[doc = "4: `100`"]
    SpiMisoSpiMosi = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig4 = 7,
    #[doc = "8: `1000`"]
    PwmCh4 = 8,
    #[doc = "9: `1001`"]
    FemGpio0 = 9,
    #[doc = "10: `1010`"]
    GpipCh1 = 10,
    #[doc = "11: `1011`"]
    Swgpio4 = 11,
    #[doc = "14: `1110`"]
    E21Tms = 14,
}
impl From<Gpio4functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio4functionSelect {}
#[doc = "Field `reg_gpio_4_func_sel` reader - Function select for GPIO4."]
pub type RegGpio4FuncSelR = crate::FieldReader<Gpio4functionSelect>;
impl RegGpio4FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4functionSelect> {
        match self.bits {
            1 => Some(Gpio4functionSelect::SdioDat2),
            4 => Some(Gpio4functionSelect::SpiMisoSpiMosi),
            6 => Some(Gpio4functionSelect::I2cScl),
            7 => Some(Gpio4functionSelect::UartSig4),
            8 => Some(Gpio4functionSelect::PwmCh4),
            9 => Some(Gpio4functionSelect::FemGpio0),
            10 => Some(Gpio4functionSelect::GpipCh1),
            11 => Some(Gpio4functionSelect::Swgpio4),
            14 => Some(Gpio4functionSelect::E21Tms),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sdio_dat2(&self) -> bool {
        *self == Gpio4functionSelect::SdioDat2
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_miso_spi_mosi(&self) -> bool {
        *self == Gpio4functionSelect::SpiMisoSpiMosi
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio4functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig4(&self) -> bool {
        *self == Gpio4functionSelect::UartSig4
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch4(&self) -> bool {
        *self == Gpio4functionSelect::PwmCh4
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == Gpio4functionSelect::FemGpio0
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_gpip_ch1(&self) -> bool {
        *self == Gpio4functionSelect::GpipCh1
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_4(&self) -> bool {
        *self == Gpio4functionSelect::Swgpio4
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == Gpio4functionSelect::E21Tms
    }
}
#[doc = "Field `reg_gpio_4_func_sel` writer - Function select for GPIO4."]
pub type RegGpio4FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio4functionSelect>;
impl<'a, REG> RegGpio4FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_dat2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4functionSelect::SdioDat2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_miso_spi_mosi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4functionSelect::SpiMisoSpiMosi)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig4(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4functionSelect::UartSig4)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4functionSelect::PwmCh4)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4functionSelect::FemGpio0)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4functionSelect::GpipCh1)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_4(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4functionSelect::Swgpio4)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4functionSelect::E21Tms)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4realFunctionSelect {
    #[doc = "0: Function select is reg_gpio_4_func_sel"]
    GlbGpioRealModeReg = 0,
    #[doc = "1: `1`"]
    GlbGpioRealModeSdio = 1,
    #[doc = "12: `1100`"]
    GlbGpioRealModeRf = 12,
    #[doc = "14: `1110`"]
    GlbGpioRealModeJtag = 14,
    #[doc = "15: `1111`"]
    GlbGpioRealModeCci = 15,
}
impl From<Gpio4realFunctionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4realFunctionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4realFunctionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio4realFunctionSelect {}
#[doc = "Field `real_gpio_4_func_sel` reader - "]
pub type RealGpio4FuncSelR = crate::FieldReader<Gpio4realFunctionSelect>;
impl RealGpio4FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4realFunctionSelect> {
        match self.bits {
            0 => Some(Gpio4realFunctionSelect::GlbGpioRealModeReg),
            1 => Some(Gpio4realFunctionSelect::GlbGpioRealModeSdio),
            12 => Some(Gpio4realFunctionSelect::GlbGpioRealModeRf),
            14 => Some(Gpio4realFunctionSelect::GlbGpioRealModeJtag),
            15 => Some(Gpio4realFunctionSelect::GlbGpioRealModeCci),
            _ => None,
        }
    }
    #[doc = "Function select is reg_gpio_4_func_sel"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == Gpio4realFunctionSelect::GlbGpioRealModeReg
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == Gpio4realFunctionSelect::GlbGpioRealModeSdio
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == Gpio4realFunctionSelect::GlbGpioRealModeRf
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == Gpio4realFunctionSelect::GlbGpioRealModeJtag
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == Gpio4realFunctionSelect::GlbGpioRealModeCci
    }
}
#[doc = "Input enable for GPIO5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio5inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio5inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_ie` reader - Input enable for GPIO5."]
pub type RegGpio5IeR = crate::BitReader<Gpio5inputEnabled>;
impl RegGpio5IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5inputEnabled {
        match self.bits {
            false => Gpio5inputEnabled::Disabled,
            true => Gpio5inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio5inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio5inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_5_ie` writer - Input enable for GPIO5."]
pub type RegGpio5IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio5inputEnabled>;
impl<'a, REG> RegGpio5IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio5schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio5schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_smt` reader - Schmitt trigger enabled for GPIO5."]
pub type RegGpio5SmtR = crate::BitReader<Gpio5schmitt>;
impl RegGpio5SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5schmitt {
        match self.bits {
            false => Gpio5schmitt::Disabled,
            true => Gpio5schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio5schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio5schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_5_smt` writer - Schmitt trigger enabled for GPIO5."]
pub type RegGpio5SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio5schmitt>;
impl<'a, REG> RegGpio5SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio5driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio5driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio5driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio5driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio5driving {}
#[doc = "Field `reg_gpio_5_drv` reader - Driving control enabled for GPIO5."]
pub type RegGpio5DrvR = crate::FieldReader<Gpio5driving>;
impl RegGpio5DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio5driving> {
        match self.bits {
            0 => Some(Gpio5driving::Disabled),
            1 => Some(Gpio5driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio5driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio5driving::Enabled
    }
}
#[doc = "Field `reg_gpio_5_drv` writer - Driving control enabled for GPIO5."]
pub type RegGpio5DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio5driving>;
impl<'a, REG> RegGpio5DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio5pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio5pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_pu` reader - Pull Up Resistor for GPIO5."]
pub type RegGpio5PuR = crate::BitReader<Gpio5pullUpResistor>;
impl RegGpio5PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5pullUpResistor {
        match self.bits {
            false => Gpio5pullUpResistor::Disabled,
            true => Gpio5pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio5pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio5pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_5_pu` writer - Pull Up Resistor for GPIO5."]
pub type RegGpio5PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio5pullUpResistor>;
impl<'a, REG> RegGpio5PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio5pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio5pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio5pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_pd` reader - Pull Down Resistor for GPIO5."]
pub type RegGpio5PdR = crate::BitReader<Gpio5pullDownResistor>;
impl RegGpio5PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio5pullDownResistor {
        match self.bits {
            false => Gpio5pullDownResistor::Disabled,
            true => Gpio5pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio5pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio5pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_5_pd` writer - Pull Down Resistor for GPIO5."]
pub type RegGpio5PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio5pullDownResistor>;
impl<'a, REG> RegGpio5PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio5functionSelect {
    #[doc = "1: `1`"]
    SdioDat3 = 1,
    #[doc = "4: `100`"]
    SpiMosiSpiMiso = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig5 = 7,
    #[doc = "8: `1000`"]
    PwmCh0 = 8,
    #[doc = "9: `1001`"]
    FemGpio1 = 9,
    #[doc = "10: `1010`"]
    GpipCh4 = 10,
    #[doc = "11: `1011`"]
    Swgpio5 = 11,
    #[doc = "14: `1110`"]
    E21Tdi = 14,
}
impl From<Gpio5functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio5functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio5functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio5functionSelect {}
#[doc = "Field `reg_gpio_5_func_sel` reader - Function select for GPIO5."]
pub type RegGpio5FuncSelR = crate::FieldReader<Gpio5functionSelect>;
impl RegGpio5FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio5functionSelect> {
        match self.bits {
            1 => Some(Gpio5functionSelect::SdioDat3),
            4 => Some(Gpio5functionSelect::SpiMosiSpiMiso),
            6 => Some(Gpio5functionSelect::I2cSda),
            7 => Some(Gpio5functionSelect::UartSig5),
            8 => Some(Gpio5functionSelect::PwmCh0),
            9 => Some(Gpio5functionSelect::FemGpio1),
            10 => Some(Gpio5functionSelect::GpipCh4),
            11 => Some(Gpio5functionSelect::Swgpio5),
            14 => Some(Gpio5functionSelect::E21Tdi),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sdio_dat3(&self) -> bool {
        *self == Gpio5functionSelect::SdioDat3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == Gpio5functionSelect::SpiMosiSpiMiso
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio5functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig5(&self) -> bool {
        *self == Gpio5functionSelect::UartSig5
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        *self == Gpio5functionSelect::PwmCh0
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == Gpio5functionSelect::FemGpio1
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_gpip_ch4(&self) -> bool {
        *self == Gpio5functionSelect::GpipCh4
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_5(&self) -> bool {
        *self == Gpio5functionSelect::Swgpio5
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == Gpio5functionSelect::E21Tdi
    }
}
#[doc = "Field `reg_gpio_5_func_sel` writer - Function select for GPIO5."]
pub type RegGpio5FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio5functionSelect>;
impl<'a, REG> RegGpio5FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_dat3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5functionSelect::SdioDat3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5functionSelect::SpiMosiSpiMiso)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig5(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5functionSelect::UartSig5)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5functionSelect::PwmCh0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5functionSelect::FemGpio1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn gpip_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5functionSelect::GpipCh4)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_5(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5functionSelect::Swgpio5)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio5functionSelect::E21Tdi)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio5realFunctionSelect {
    #[doc = "0: Function select is reg_gpio_5_func_sel"]
    GlbGpioRealModeReg = 0,
    #[doc = "1: `1`"]
    GlbGpioRealModeSdio = 1,
    #[doc = "12: `1100`"]
    GlbGpioRealModeRf = 12,
    #[doc = "14: `1110`"]
    GlbGpioRealModeJtag = 14,
    #[doc = "15: `1111`"]
    GlbGpioRealModeCci = 15,
}
impl From<Gpio5realFunctionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio5realFunctionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio5realFunctionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio5realFunctionSelect {}
#[doc = "Field `real_gpio_5_func_sel` reader - "]
pub type RealGpio5FuncSelR = crate::FieldReader<Gpio5realFunctionSelect>;
impl RealGpio5FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio5realFunctionSelect> {
        match self.bits {
            0 => Some(Gpio5realFunctionSelect::GlbGpioRealModeReg),
            1 => Some(Gpio5realFunctionSelect::GlbGpioRealModeSdio),
            12 => Some(Gpio5realFunctionSelect::GlbGpioRealModeRf),
            14 => Some(Gpio5realFunctionSelect::GlbGpioRealModeJtag),
            15 => Some(Gpio5realFunctionSelect::GlbGpioRealModeCci),
            _ => None,
        }
    }
    #[doc = "Function select is reg_gpio_5_func_sel"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == Gpio5realFunctionSelect::GlbGpioRealModeReg
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == Gpio5realFunctionSelect::GlbGpioRealModeSdio
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == Gpio5realFunctionSelect::GlbGpioRealModeRf
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == Gpio5realFunctionSelect::GlbGpioRealModeJtag
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == Gpio5realFunctionSelect::GlbGpioRealModeCci
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_ie(&self) -> RegGpio4IeR {
        RegGpio4IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_smt(&self) -> RegGpio4SmtR {
        RegGpio4SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_drv(&self) -> RegGpio4DrvR {
        RegGpio4DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_pu(&self) -> RegGpio4PuR {
        RegGpio4PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_pd(&self) -> RegGpio4PdR {
        RegGpio4PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_func_sel(&self) -> RegGpio4FuncSelR {
        RegGpio4FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_4_func_sel(&self) -> RealGpio4FuncSelR {
        RealGpio4FuncSelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_ie(&self) -> RegGpio5IeR {
        RegGpio5IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_smt(&self) -> RegGpio5SmtR {
        RegGpio5SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_drv(&self) -> RegGpio5DrvR {
        RegGpio5DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_pu(&self) -> RegGpio5PuR {
        RegGpio5PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_pd(&self) -> RegGpio5PdR {
        RegGpio5PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_func_sel(&self) -> RegGpio5FuncSelR {
        RegGpio5FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_5_func_sel(&self) -> RealGpio5FuncSelR {
        RealGpio5FuncSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_ie(&mut self) -> RegGpio4IeW<'_, GpioCfgctl2Spec> {
        RegGpio4IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_smt(&mut self) -> RegGpio4SmtW<'_, GpioCfgctl2Spec> {
        RegGpio4SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_drv(&mut self) -> RegGpio4DrvW<'_, GpioCfgctl2Spec> {
        RegGpio4DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_pu(&mut self) -> RegGpio4PuW<'_, GpioCfgctl2Spec> {
        RegGpio4PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_pd(&mut self) -> RegGpio4PdW<'_, GpioCfgctl2Spec> {
        RegGpio4PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_func_sel(&mut self) -> RegGpio4FuncSelW<'_, GpioCfgctl2Spec> {
        RegGpio4FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_ie(&mut self) -> RegGpio5IeW<'_, GpioCfgctl2Spec> {
        RegGpio5IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_smt(&mut self) -> RegGpio5SmtW<'_, GpioCfgctl2Spec> {
        RegGpio5SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_drv(&mut self) -> RegGpio5DrvW<'_, GpioCfgctl2Spec> {
        RegGpio5DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_pu(&mut self) -> RegGpio5PuW<'_, GpioCfgctl2Spec> {
        RegGpio5PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_pd(&mut self) -> RegGpio5PdW<'_, GpioCfgctl2Spec> {
        RegGpio5PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_func_sel(&mut self) -> RegGpio5FuncSelW<'_, GpioCfgctl2Spec> {
        RegGpio5FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO4, GPIO5 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl2Spec;
impl crate::RegisterSpec for GpioCfgctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl2::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl2::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL2 to value 0x1103_1103"]
impl crate::Resettable for GpioCfgctl2Spec {
    const RESET_VALUE: u32 = 0x1103_1103;
}
