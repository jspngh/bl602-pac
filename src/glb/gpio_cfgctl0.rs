#[doc = "Register `GPIO_CFGCTL0` reader"]
pub type R = crate::R<GpioCfgctl0Spec>;
#[doc = "Register `GPIO_CFGCTL0` writer"]
pub type W = crate::W<GpioCfgctl0Spec>;
#[doc = "GPIO0 input enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio0inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio0inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_ie` reader - GPIO0 input enable."]
pub type RegGpio0IeR = crate::BitReader<Gpio0inputEnabled>;
impl RegGpio0IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0inputEnabled {
        match self.bits {
            false => Gpio0inputEnabled::Disabled,
            true => Gpio0inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio0inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio0inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_0_ie` writer - GPIO0 input enable."]
pub type RegGpio0IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio0inputEnabled>;
impl<'a, REG> RegGpio0IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio0schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio0schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_smt` reader - Schmitt trigger enabled for GPIO0."]
pub type RegGpio0SmtR = crate::BitReader<Gpio0schmitt>;
impl RegGpio0SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0schmitt {
        match self.bits {
            false => Gpio0schmitt::Disabled,
            true => Gpio0schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio0schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio0schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_0_smt` writer - Schmitt trigger enabled for GPIO0."]
pub type RegGpio0SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio0schmitt>;
impl<'a, REG> RegGpio0SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0schmitt::Enabled)
    }
}
#[doc = "Driving control for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio0driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio0driving {}
#[doc = "Field `reg_gpio_0_drv` reader - Driving control for GPIO0."]
pub type RegGpio0DrvR = crate::FieldReader<Gpio0driving>;
impl RegGpio0DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0driving> {
        match self.bits {
            0 => Some(Gpio0driving::Disabled),
            1 => Some(Gpio0driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio0driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio0driving::Enabled
    }
}
#[doc = "Field `reg_gpio_0_drv` writer - Driving control for GPIO0."]
pub type RegGpio0DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0driving>;
impl<'a, REG> RegGpio0DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio0pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio0pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_pu` reader - Pull Up Resistor for GPIO0."]
pub type RegGpio0PuR = crate::BitReader<Gpio0pullUpResistor>;
impl RegGpio0PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0pullUpResistor {
        match self.bits {
            false => Gpio0pullUpResistor::Disabled,
            true => Gpio0pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio0pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio0pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_0_pu` writer - Pull Up Resistor for GPIO0."]
pub type RegGpio0PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio0pullUpResistor>;
impl<'a, REG> RegGpio0PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio0pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio0pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_pd` reader - Pull Down Resistor for GPIO0."]
pub type RegGpio0PdR = crate::BitReader<Gpio0pullDownResistor>;
impl RegGpio0PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0pullDownResistor {
        match self.bits {
            false => Gpio0pullDownResistor::Disabled,
            true => Gpio0pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio0pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio0pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_0_pd` writer - Pull Down Resistor for GPIO0."]
pub type RegGpio0PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio0pullDownResistor>;
impl<'a, REG> RegGpio0PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0functionSelect {
    #[doc = "1: `1`"]
    SdioClk = 1,
    #[doc = "2: `10`"]
    SfD1 = 2,
    #[doc = "4: `100`"]
    SpiMosiSpiMiso = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig0 = 7,
    #[doc = "8: `1000`"]
    PwmCh0 = 8,
    #[doc = "9: `1001`"]
    FemGpio0 = 9,
    #[doc = "10: `1010`"]
    AtestIn = 10,
    #[doc = "11: `1011`"]
    Swgpio0 = 11,
    #[doc = "14: `1110`"]
    E21Tms = 14,
}
impl From<Gpio0functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio0functionSelect {}
#[doc = "Field `reg_gpio_0_func_sel` reader - Function select for GPIO0."]
pub type RegGpio0FuncSelR = crate::FieldReader<Gpio0functionSelect>;
impl RegGpio0FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0functionSelect> {
        match self.bits {
            1 => Some(Gpio0functionSelect::SdioClk),
            2 => Some(Gpio0functionSelect::SfD1),
            4 => Some(Gpio0functionSelect::SpiMosiSpiMiso),
            6 => Some(Gpio0functionSelect::I2cScl),
            7 => Some(Gpio0functionSelect::UartSig0),
            8 => Some(Gpio0functionSelect::PwmCh0),
            9 => Some(Gpio0functionSelect::FemGpio0),
            10 => Some(Gpio0functionSelect::AtestIn),
            11 => Some(Gpio0functionSelect::Swgpio0),
            14 => Some(Gpio0functionSelect::E21Tms),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sdio_clk(&self) -> bool {
        *self == Gpio0functionSelect::SdioClk
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sf_d1(&self) -> bool {
        *self == Gpio0functionSelect::SfD1
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == Gpio0functionSelect::SpiMosiSpiMiso
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio0functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig0(&self) -> bool {
        *self == Gpio0functionSelect::UartSig0
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        *self == Gpio0functionSelect::PwmCh0
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        *self == Gpio0functionSelect::FemGpio0
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_atest_in(&self) -> bool {
        *self == Gpio0functionSelect::AtestIn
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_0(&self) -> bool {
        *self == Gpio0functionSelect::Swgpio0
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        *self == Gpio0functionSelect::E21Tms
    }
}
#[doc = "Field `reg_gpio_0_func_sel` writer - Function select for GPIO0."]
pub type RegGpio0FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio0functionSelect>;
impl<'a, REG> RegGpio0FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::SdioClk)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::SfD1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::SpiMosiSpiMiso)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::UartSig0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::PwmCh0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::FemGpio0)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_in(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::AtestIn)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::Swgpio0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0functionSelect::E21Tms)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0realFunctionSelect {
    #[doc = "0: Function select is reg_gpio_0_func_sel"]
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
impl From<Gpio0realFunctionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0realFunctionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0realFunctionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio0realFunctionSelect {}
#[doc = "Field `real_gpio_0_func_sel` reader - "]
pub type RealGpio0FuncSelR = crate::FieldReader<Gpio0realFunctionSelect>;
impl RealGpio0FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0realFunctionSelect> {
        match self.bits {
            0 => Some(Gpio0realFunctionSelect::GlbGpioRealModeReg),
            1 => Some(Gpio0realFunctionSelect::GlbGpioRealModeSdio),
            12 => Some(Gpio0realFunctionSelect::GlbGpioRealModeRf),
            14 => Some(Gpio0realFunctionSelect::GlbGpioRealModeJtag),
            15 => Some(Gpio0realFunctionSelect::GlbGpioRealModeCci),
            _ => None,
        }
    }
    #[doc = "Function select is reg_gpio_0_func_sel"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == Gpio0realFunctionSelect::GlbGpioRealModeReg
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == Gpio0realFunctionSelect::GlbGpioRealModeSdio
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == Gpio0realFunctionSelect::GlbGpioRealModeRf
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == Gpio0realFunctionSelect::GlbGpioRealModeJtag
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == Gpio0realFunctionSelect::GlbGpioRealModeCci
    }
}
#[doc = "Input enable for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio1inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio1inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_ie` reader - Input enable for GPIO1."]
pub type RegGpio1IeR = crate::BitReader<Gpio1inputEnabled>;
impl RegGpio1IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1inputEnabled {
        match self.bits {
            false => Gpio1inputEnabled::Disabled,
            true => Gpio1inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio1inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio1inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_1_ie` writer - Input enable for GPIO1."]
pub type RegGpio1IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio1inputEnabled>;
impl<'a, REG> RegGpio1IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio1schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio1schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_smt` reader - Schmitt trigger enabled for GPIO1."]
pub type RegGpio1SmtR = crate::BitReader<Gpio1schmitt>;
impl RegGpio1SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1schmitt {
        match self.bits {
            false => Gpio1schmitt::Disabled,
            true => Gpio1schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio1schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio1schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_1_smt` writer - Schmitt trigger enabled for GPIO1."]
pub type RegGpio1SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio1schmitt>;
impl<'a, REG> RegGpio1SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio1driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio1driving {}
#[doc = "Field `reg_gpio_1_drv` reader - Driving control enabled for GPIO1."]
pub type RegGpio1DrvR = crate::FieldReader<Gpio1driving>;
impl RegGpio1DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1driving> {
        match self.bits {
            0 => Some(Gpio1driving::Disabled),
            1 => Some(Gpio1driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio1driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio1driving::Enabled
    }
}
#[doc = "Field `reg_gpio_1_drv` writer - Driving control enabled for GPIO1."]
pub type RegGpio1DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1driving>;
impl<'a, REG> RegGpio1DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio1pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio1pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_pu` reader - Pull Up Resistor for GPIO1."]
pub type RegGpio1PuR = crate::BitReader<Gpio1pullUpResistor>;
impl RegGpio1PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1pullUpResistor {
        match self.bits {
            false => Gpio1pullUpResistor::Disabled,
            true => Gpio1pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio1pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio1pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_1_pu` writer - Pull Up Resistor for GPIO1."]
pub type RegGpio1PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio1pullUpResistor>;
impl<'a, REG> RegGpio1PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio1pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio1pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_pd` reader - Pull Down Resistor for GPIO1."]
pub type RegGpio1PdR = crate::BitReader<Gpio1pullDownResistor>;
impl RegGpio1PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1pullDownResistor {
        match self.bits {
            false => Gpio1pullDownResistor::Disabled,
            true => Gpio1pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio1pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio1pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_1_pd` writer - Pull Down Resistor for GPIO1."]
pub type RegGpio1PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio1pullDownResistor>;
impl<'a, REG> RegGpio1PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1functionSelect {
    #[doc = "1: `1`"]
    SdioCmd = 1,
    #[doc = "2: `10`"]
    SfD2 = 2,
    #[doc = "4: `100`"]
    SpiMosiSpiMiso = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig1 = 7,
    #[doc = "8: `1000`"]
    PwmCh1 = 8,
    #[doc = "9: `1001`"]
    FemGpio1 = 9,
    #[doc = "10: `1010`"]
    AtestIp = 10,
    #[doc = "11: `1011`"]
    Swgpio1 = 11,
    #[doc = "14: `1110`"]
    E21Tdi = 14,
}
impl From<Gpio1functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio1functionSelect {}
#[doc = "Field `reg_gpio_1_func_sel` reader - Function select for GPIO1."]
pub type RegGpio1FuncSelR = crate::FieldReader<Gpio1functionSelect>;
impl RegGpio1FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1functionSelect> {
        match self.bits {
            1 => Some(Gpio1functionSelect::SdioCmd),
            2 => Some(Gpio1functionSelect::SfD2),
            4 => Some(Gpio1functionSelect::SpiMosiSpiMiso),
            6 => Some(Gpio1functionSelect::I2cSda),
            7 => Some(Gpio1functionSelect::UartSig1),
            8 => Some(Gpio1functionSelect::PwmCh1),
            9 => Some(Gpio1functionSelect::FemGpio1),
            10 => Some(Gpio1functionSelect::AtestIp),
            11 => Some(Gpio1functionSelect::Swgpio1),
            14 => Some(Gpio1functionSelect::E21Tdi),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sdio_cmd(&self) -> bool {
        *self == Gpio1functionSelect::SdioCmd
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sf_d2(&self) -> bool {
        *self == Gpio1functionSelect::SfD2
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        *self == Gpio1functionSelect::SpiMosiSpiMiso
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio1functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig1(&self) -> bool {
        *self == Gpio1functionSelect::UartSig1
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        *self == Gpio1functionSelect::PwmCh1
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        *self == Gpio1functionSelect::FemGpio1
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_atest_ip(&self) -> bool {
        *self == Gpio1functionSelect::AtestIp
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_1(&self) -> bool {
        *self == Gpio1functionSelect::Swgpio1
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        *self == Gpio1functionSelect::E21Tdi
    }
}
#[doc = "Field `reg_gpio_1_func_sel` writer - Function select for GPIO1."]
pub type RegGpio1FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio1functionSelect>;
impl<'a, REG> RegGpio1FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::SdioCmd)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::SfD2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::SpiMosiSpiMiso)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::UartSig1)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::PwmCh1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::FemGpio1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_ip(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::AtestIp)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::Swgpio1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1functionSelect::E21Tdi)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1realFunctionSelect {
    #[doc = "0: Function select is reg_gpio_1_func_sel"]
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
impl From<Gpio1realFunctionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1realFunctionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1realFunctionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio1realFunctionSelect {}
#[doc = "Field `real_gpio_1_func_sel` reader - "]
pub type RealGpio1FuncSelR = crate::FieldReader<Gpio1realFunctionSelect>;
impl RealGpio1FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1realFunctionSelect> {
        match self.bits {
            0 => Some(Gpio1realFunctionSelect::GlbGpioRealModeReg),
            1 => Some(Gpio1realFunctionSelect::GlbGpioRealModeSdio),
            12 => Some(Gpio1realFunctionSelect::GlbGpioRealModeRf),
            14 => Some(Gpio1realFunctionSelect::GlbGpioRealModeJtag),
            15 => Some(Gpio1realFunctionSelect::GlbGpioRealModeCci),
            _ => None,
        }
    }
    #[doc = "Function select is reg_gpio_1_func_sel"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == Gpio1realFunctionSelect::GlbGpioRealModeReg
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == Gpio1realFunctionSelect::GlbGpioRealModeSdio
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == Gpio1realFunctionSelect::GlbGpioRealModeRf
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == Gpio1realFunctionSelect::GlbGpioRealModeJtag
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == Gpio1realFunctionSelect::GlbGpioRealModeCci
    }
}
impl R {
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline(always)]
    pub fn reg_gpio_0_ie(&self) -> RegGpio0IeR {
        RegGpio0IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_smt(&self) -> RegGpio0SmtR {
        RegGpio0SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_drv(&self) -> RegGpio0DrvR {
        RegGpio0DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pu(&self) -> RegGpio0PuR {
        RegGpio0PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pd(&self) -> RegGpio0PdR {
        RegGpio0PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_func_sel(&self) -> RegGpio0FuncSelR {
        RegGpio0FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_0_func_sel(&self) -> RealGpio0FuncSelR {
        RealGpio0FuncSelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_ie(&self) -> RegGpio1IeR {
        RegGpio1IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_smt(&self) -> RegGpio1SmtR {
        RegGpio1SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_drv(&self) -> RegGpio1DrvR {
        RegGpio1DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pu(&self) -> RegGpio1PuR {
        RegGpio1PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pd(&self) -> RegGpio1PdR {
        RegGpio1PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_func_sel(&self) -> RegGpio1FuncSelR {
        RegGpio1FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_1_func_sel(&self) -> RealGpio1FuncSelR {
        RealGpio1FuncSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline(always)]
    pub fn reg_gpio_0_ie(&mut self) -> RegGpio0IeW<'_, GpioCfgctl0Spec> {
        RegGpio0IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_smt(&mut self) -> RegGpio0SmtW<'_, GpioCfgctl0Spec> {
        RegGpio0SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_drv(&mut self) -> RegGpio0DrvW<'_, GpioCfgctl0Spec> {
        RegGpio0DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pu(&mut self) -> RegGpio0PuW<'_, GpioCfgctl0Spec> {
        RegGpio0PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pd(&mut self) -> RegGpio0PdW<'_, GpioCfgctl0Spec> {
        RegGpio0PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_func_sel(&mut self) -> RegGpio0FuncSelW<'_, GpioCfgctl0Spec> {
        RegGpio0FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_ie(&mut self) -> RegGpio1IeW<'_, GpioCfgctl0Spec> {
        RegGpio1IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_smt(&mut self) -> RegGpio1SmtW<'_, GpioCfgctl0Spec> {
        RegGpio1SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_drv(&mut self) -> RegGpio1DrvW<'_, GpioCfgctl0Spec> {
        RegGpio1DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pu(&mut self) -> RegGpio1PuW<'_, GpioCfgctl0Spec> {
        RegGpio1PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pd(&mut self) -> RegGpio1PdW<'_, GpioCfgctl0Spec> {
        RegGpio1PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_func_sel(&mut self) -> RegGpio1FuncSelW<'_, GpioCfgctl0Spec> {
        RegGpio1FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO0, GPIO1 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl0Spec;
impl crate::RegisterSpec for GpioCfgctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl0::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl0::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL0 to value 0x1103_1103"]
impl crate::Resettable for GpioCfgctl0Spec {
    const RESET_VALUE: u32 = 0x1103_1103;
}
