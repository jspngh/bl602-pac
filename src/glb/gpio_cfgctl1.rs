#[doc = "Register `GPIO_CFGCTL1` reader"]
pub type R = crate::R<GpioCfgctl1Spec>;
#[doc = "Register `GPIO_CFGCTL1` writer"]
pub type W = crate::W<GpioCfgctl1Spec>;
#[doc = "Input enable for GPIO2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio2inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio2inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_ie` reader - Input enable for GPIO2."]
pub type RegGpio2IeR = crate::BitReader<Gpio2inputEnabled>;
impl RegGpio2IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2inputEnabled {
        match self.bits {
            false => Gpio2inputEnabled::Disabled,
            true => Gpio2inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio2inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio2inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_2_ie` writer - Input enable for GPIO2."]
pub type RegGpio2IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio2inputEnabled>;
impl<'a, REG> RegGpio2IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio2schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio2schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_smt` reader - Schmitt trigger enabled for GPIO2."]
pub type RegGpio2SmtR = crate::BitReader<Gpio2schmitt>;
impl RegGpio2SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2schmitt {
        match self.bits {
            false => Gpio2schmitt::Disabled,
            true => Gpio2schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio2schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio2schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_2_smt` writer - Schmitt trigger enabled for GPIO2."]
pub type RegGpio2SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio2schmitt>;
impl<'a, REG> RegGpio2SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio2driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio2driving {}
#[doc = "Field `reg_gpio_2_drv` reader - Driving control enabled for GPIO2."]
pub type RegGpio2DrvR = crate::FieldReader<Gpio2driving>;
impl RegGpio2DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio2driving> {
        match self.bits {
            0 => Some(Gpio2driving::Disabled),
            1 => Some(Gpio2driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio2driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio2driving::Enabled
    }
}
#[doc = "Field `reg_gpio_2_drv` writer - Driving control enabled for GPIO2."]
pub type RegGpio2DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio2driving>;
impl<'a, REG> RegGpio2DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio2pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio2pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_pu` reader - Pull Up Resistor for GPIO2."]
pub type RegGpio2PuR = crate::BitReader<Gpio2pullUpResistor>;
impl RegGpio2PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2pullUpResistor {
        match self.bits {
            false => Gpio2pullUpResistor::Disabled,
            true => Gpio2pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio2pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio2pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_2_pu` writer - Pull Up Resistor for GPIO2."]
pub type RegGpio2PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio2pullUpResistor>;
impl<'a, REG> RegGpio2PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio2pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio2pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_pd` reader - Pull Down Resistor for GPIO2."]
pub type RegGpio2PdR = crate::BitReader<Gpio2pullDownResistor>;
impl RegGpio2PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2pullDownResistor {
        match self.bits {
            false => Gpio2pullDownResistor::Disabled,
            true => Gpio2pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio2pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio2pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_2_pd` writer - Pull Down Resistor for GPIO2."]
pub type RegGpio2PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio2pullDownResistor>;
impl<'a, REG> RegGpio2PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2functionSelect {
    #[doc = "1: `1`"]
    SdioDat0 = 1,
    #[doc = "2: `10`"]
    SfD3 = 2,
    #[doc = "4: `100`"]
    SpiSs = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig2 = 7,
    #[doc = "8: `1000`"]
    PwmCh2 = 8,
    #[doc = "9: `1001`"]
    FemGpio2 = 9,
    #[doc = "10: `1010`"]
    AtestQn = 10,
    #[doc = "11: `1011`"]
    Swgpio2 = 11,
    #[doc = "14: `1110`"]
    E21Tck = 14,
}
impl From<Gpio2functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio2functionSelect {}
#[doc = "Field `reg_gpio_2_func_sel` reader - Function select for GPIO2."]
pub type RegGpio2FuncSelR = crate::FieldReader<Gpio2functionSelect>;
impl RegGpio2FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio2functionSelect> {
        match self.bits {
            1 => Some(Gpio2functionSelect::SdioDat0),
            2 => Some(Gpio2functionSelect::SfD3),
            4 => Some(Gpio2functionSelect::SpiSs),
            6 => Some(Gpio2functionSelect::I2cScl),
            7 => Some(Gpio2functionSelect::UartSig2),
            8 => Some(Gpio2functionSelect::PwmCh2),
            9 => Some(Gpio2functionSelect::FemGpio2),
            10 => Some(Gpio2functionSelect::AtestQn),
            11 => Some(Gpio2functionSelect::Swgpio2),
            14 => Some(Gpio2functionSelect::E21Tck),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sdio_dat0(&self) -> bool {
        *self == Gpio2functionSelect::SdioDat0
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sf_d3(&self) -> bool {
        *self == Gpio2functionSelect::SfD3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == Gpio2functionSelect::SpiSs
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio2functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig2(&self) -> bool {
        *self == Gpio2functionSelect::UartSig2
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        *self == Gpio2functionSelect::PwmCh2
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == Gpio2functionSelect::FemGpio2
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_atest_qn(&self) -> bool {
        *self == Gpio2functionSelect::AtestQn
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_2(&self) -> bool {
        *self == Gpio2functionSelect::Swgpio2
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == Gpio2functionSelect::E21Tck
    }
}
#[doc = "Field `reg_gpio_2_func_sel` writer - Function select for GPIO2."]
pub type RegGpio2FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio2functionSelect>;
impl<'a, REG> RegGpio2FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_dat0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::SdioDat0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::SfD3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::SpiSs)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::UartSig2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::PwmCh2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::FemGpio2)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_qn(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::AtestQn)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::Swgpio2)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2functionSelect::E21Tck)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2realFunctionSelect {
    #[doc = "0: Function select is reg_gpio_2_func_sel"]
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
impl From<Gpio2realFunctionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2realFunctionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2realFunctionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio2realFunctionSelect {}
#[doc = "Field `real_gpio_2_func_sel` reader - "]
pub type RealGpio2FuncSelR = crate::FieldReader<Gpio2realFunctionSelect>;
impl RealGpio2FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio2realFunctionSelect> {
        match self.bits {
            0 => Some(Gpio2realFunctionSelect::GlbGpioRealModeReg),
            1 => Some(Gpio2realFunctionSelect::GlbGpioRealModeSdio),
            12 => Some(Gpio2realFunctionSelect::GlbGpioRealModeRf),
            14 => Some(Gpio2realFunctionSelect::GlbGpioRealModeJtag),
            15 => Some(Gpio2realFunctionSelect::GlbGpioRealModeCci),
            _ => None,
        }
    }
    #[doc = "Function select is reg_gpio_2_func_sel"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == Gpio2realFunctionSelect::GlbGpioRealModeReg
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == Gpio2realFunctionSelect::GlbGpioRealModeSdio
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == Gpio2realFunctionSelect::GlbGpioRealModeRf
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == Gpio2realFunctionSelect::GlbGpioRealModeJtag
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == Gpio2realFunctionSelect::GlbGpioRealModeCci
    }
}
#[doc = "Input enable for GPIO3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio3inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio3inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_ie` reader - Input enable for GPIO3."]
pub type RegGpio3IeR = crate::BitReader<Gpio3inputEnabled>;
impl RegGpio3IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3inputEnabled {
        match self.bits {
            false => Gpio3inputEnabled::Disabled,
            true => Gpio3inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio3inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio3inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_3_ie` writer - Input enable for GPIO3."]
pub type RegGpio3IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio3inputEnabled>;
impl<'a, REG> RegGpio3IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio3schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio3schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_smt` reader - Schmitt trigger enabled for GPIO3."]
pub type RegGpio3SmtR = crate::BitReader<Gpio3schmitt>;
impl RegGpio3SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3schmitt {
        match self.bits {
            false => Gpio3schmitt::Disabled,
            true => Gpio3schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio3schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio3schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_3_smt` writer - Schmitt trigger enabled for GPIO3."]
pub type RegGpio3SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio3schmitt>;
impl<'a, REG> RegGpio3SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio3driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio3driving {}
#[doc = "Field `reg_gpio_3_drv` reader - Driving control enabled for GPIO3."]
pub type RegGpio3DrvR = crate::FieldReader<Gpio3driving>;
impl RegGpio3DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3driving> {
        match self.bits {
            0 => Some(Gpio3driving::Disabled),
            1 => Some(Gpio3driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio3driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio3driving::Enabled
    }
}
#[doc = "Field `reg_gpio_3_drv` writer - Driving control enabled for GPIO3."]
pub type RegGpio3DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio3driving>;
impl<'a, REG> RegGpio3DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio3pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio3pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_pu` reader - Pull Up Resistor for GPIO3."]
pub type RegGpio3PuR = crate::BitReader<Gpio3pullUpResistor>;
impl RegGpio3PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3pullUpResistor {
        match self.bits {
            false => Gpio3pullUpResistor::Disabled,
            true => Gpio3pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio3pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio3pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_3_pu` writer - Pull Up Resistor for GPIO3."]
pub type RegGpio3PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio3pullUpResistor>;
impl<'a, REG> RegGpio3PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio3pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio3pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio3pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_pd` reader - Pull Down Resistor for GPIO3."]
pub type RegGpio3PdR = crate::BitReader<Gpio3pullDownResistor>;
impl RegGpio3PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3pullDownResistor {
        match self.bits {
            false => Gpio3pullDownResistor::Disabled,
            true => Gpio3pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio3pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio3pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_3_pd` writer - Pull Down Resistor for GPIO3."]
pub type RegGpio3PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio3pullDownResistor>;
impl<'a, REG> RegGpio3PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3functionSelect {
    #[doc = "1: `1`"]
    SdioDat1 = 1,
    #[doc = "4: `100`"]
    SpiSclk = 4,
    #[doc = "6: `110`"]
    I2cSda = 6,
    #[doc = "7: `111`"]
    UartSig3 = 7,
    #[doc = "8: `1000`"]
    PwmCh3 = 8,
    #[doc = "9: `1001`"]
    FemGpio3 = 9,
    #[doc = "10: `1010`"]
    AtestQp = 10,
    #[doc = "11: `1011`"]
    Swgpio3 = 11,
    #[doc = "14: `1110`"]
    E21Tdo = 14,
}
impl From<Gpio3functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio3functionSelect {}
#[doc = "Field `reg_gpio_3_func_sel` reader - Function select for GPIO3."]
pub type RegGpio3FuncSelR = crate::FieldReader<Gpio3functionSelect>;
impl RegGpio3FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3functionSelect> {
        match self.bits {
            1 => Some(Gpio3functionSelect::SdioDat1),
            4 => Some(Gpio3functionSelect::SpiSclk),
            6 => Some(Gpio3functionSelect::I2cSda),
            7 => Some(Gpio3functionSelect::UartSig3),
            8 => Some(Gpio3functionSelect::PwmCh3),
            9 => Some(Gpio3functionSelect::FemGpio3),
            10 => Some(Gpio3functionSelect::AtestQp),
            11 => Some(Gpio3functionSelect::Swgpio3),
            14 => Some(Gpio3functionSelect::E21Tdo),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sdio_dat1(&self) -> bool {
        *self == Gpio3functionSelect::SdioDat1
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        *self == Gpio3functionSelect::SpiSclk
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        *self == Gpio3functionSelect::I2cSda
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig3(&self) -> bool {
        *self == Gpio3functionSelect::UartSig3
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch3(&self) -> bool {
        *self == Gpio3functionSelect::PwmCh3
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        *self == Gpio3functionSelect::FemGpio3
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_atest_qp(&self) -> bool {
        *self == Gpio3functionSelect::AtestQp
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_3(&self) -> bool {
        *self == Gpio3functionSelect::Swgpio3
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        *self == Gpio3functionSelect::E21Tdo
    }
}
#[doc = "Field `reg_gpio_3_func_sel` writer - Function select for GPIO3."]
pub type RegGpio3FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio3functionSelect>;
impl<'a, REG> RegGpio3FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_dat1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3functionSelect::SdioDat1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3functionSelect::SpiSclk)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3functionSelect::I2cSda)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3functionSelect::UartSig3)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3functionSelect::PwmCh3)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3functionSelect::FemGpio3)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_qp(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3functionSelect::AtestQp)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_3(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3functionSelect::Swgpio3)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3functionSelect::E21Tdo)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3realFunctionSelect {
    #[doc = "0: Function select is reg_gpio_3_func_sel"]
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
impl From<Gpio3realFunctionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3realFunctionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3realFunctionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio3realFunctionSelect {}
#[doc = "Field `real_gpio_3_func_sel` reader - "]
pub type RealGpio3FuncSelR = crate::FieldReader<Gpio3realFunctionSelect>;
impl RealGpio3FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3realFunctionSelect> {
        match self.bits {
            0 => Some(Gpio3realFunctionSelect::GlbGpioRealModeReg),
            1 => Some(Gpio3realFunctionSelect::GlbGpioRealModeSdio),
            12 => Some(Gpio3realFunctionSelect::GlbGpioRealModeRf),
            14 => Some(Gpio3realFunctionSelect::GlbGpioRealModeJtag),
            15 => Some(Gpio3realFunctionSelect::GlbGpioRealModeCci),
            _ => None,
        }
    }
    #[doc = "Function select is reg_gpio_3_func_sel"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        *self == Gpio3realFunctionSelect::GlbGpioRealModeReg
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        *self == Gpio3realFunctionSelect::GlbGpioRealModeSdio
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        *self == Gpio3realFunctionSelect::GlbGpioRealModeRf
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        *self == Gpio3realFunctionSelect::GlbGpioRealModeJtag
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        *self == Gpio3realFunctionSelect::GlbGpioRealModeCci
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_ie(&self) -> RegGpio2IeR {
        RegGpio2IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_smt(&self) -> RegGpio2SmtR {
        RegGpio2SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_drv(&self) -> RegGpio2DrvR {
        RegGpio2DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_pu(&self) -> RegGpio2PuR {
        RegGpio2PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_pd(&self) -> RegGpio2PdR {
        RegGpio2PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_func_sel(&self) -> RegGpio2FuncSelR {
        RegGpio2FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_2_func_sel(&self) -> RealGpio2FuncSelR {
        RealGpio2FuncSelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_ie(&self) -> RegGpio3IeR {
        RegGpio3IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_smt(&self) -> RegGpio3SmtR {
        RegGpio3SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_drv(&self) -> RegGpio3DrvR {
        RegGpio3DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_pu(&self) -> RegGpio3PuR {
        RegGpio3PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_pd(&self) -> RegGpio3PdR {
        RegGpio3PdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Function select for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_func_sel(&self) -> RegGpio3FuncSelR {
        RegGpio3FuncSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_3_func_sel(&self) -> RealGpio3FuncSelR {
        RealGpio3FuncSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_ie(&mut self) -> RegGpio2IeW<'_, GpioCfgctl1Spec> {
        RegGpio2IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_smt(&mut self) -> RegGpio2SmtW<'_, GpioCfgctl1Spec> {
        RegGpio2SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_drv(&mut self) -> RegGpio2DrvW<'_, GpioCfgctl1Spec> {
        RegGpio2DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_pu(&mut self) -> RegGpio2PuW<'_, GpioCfgctl1Spec> {
        RegGpio2PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_pd(&mut self) -> RegGpio2PdW<'_, GpioCfgctl1Spec> {
        RegGpio2PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_func_sel(&mut self) -> RegGpio2FuncSelW<'_, GpioCfgctl1Spec> {
        RegGpio2FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_ie(&mut self) -> RegGpio3IeW<'_, GpioCfgctl1Spec> {
        RegGpio3IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_smt(&mut self) -> RegGpio3SmtW<'_, GpioCfgctl1Spec> {
        RegGpio3SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_drv(&mut self) -> RegGpio3DrvW<'_, GpioCfgctl1Spec> {
        RegGpio3DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_pu(&mut self) -> RegGpio3PuW<'_, GpioCfgctl1Spec> {
        RegGpio3PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_pd(&mut self) -> RegGpio3PdW<'_, GpioCfgctl1Spec> {
        RegGpio3PdW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Function select for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_func_sel(&mut self) -> RegGpio3FuncSelW<'_, GpioCfgctl1Spec> {
        RegGpio3FuncSelW::new(self, 24)
    }
}
#[doc = "GPIO2, GPIO3 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl1Spec;
impl crate::RegisterSpec for GpioCfgctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl1::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl1::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL1 to value 0x1103_1103"]
impl crate::Resettable for GpioCfgctl1Spec {
    const RESET_VALUE: u32 = 0x1103_1103;
}
