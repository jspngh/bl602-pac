#[doc = "Register `GPIO_CFGCTL11` reader"]
pub type R = crate::R<GpioCfgctl11Spec>;
#[doc = "Register `GPIO_CFGCTL11` writer"]
pub type W = crate::W<GpioCfgctl11Spec>;
#[doc = "Input enable for GPIO22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio22inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio22inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_ie` reader - Input enable for GPIO22."]
pub type RegGpio22IeR = crate::BitReader<Gpio22inputEnabled>;
impl RegGpio22IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22inputEnabled {
        match self.bits {
            false => Gpio22inputEnabled::Disabled,
            true => Gpio22inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio22inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio22inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_22_ie` writer - Input enable for GPIO22."]
pub type RegGpio22IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio22inputEnabled>;
impl<'a, REG> RegGpio22IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio22schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio22schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_smt` reader - Schmitt trigger enabled for GPIO22."]
pub type RegGpio22SmtR = crate::BitReader<Gpio22schmitt>;
impl RegGpio22SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22schmitt {
        match self.bits {
            false => Gpio22schmitt::Disabled,
            true => Gpio22schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio22schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio22schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_22_smt` writer - Schmitt trigger enabled for GPIO22."]
pub type RegGpio22SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio22schmitt>;
impl<'a, REG> RegGpio22SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio22driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio22driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio22driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio22driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio22driving {}
#[doc = "Field `reg_gpio_22_drv` reader - Driving control enabled for GPIO22."]
pub type RegGpio22DrvR = crate::FieldReader<Gpio22driving>;
impl RegGpio22DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio22driving> {
        match self.bits {
            0 => Some(Gpio22driving::Disabled),
            1 => Some(Gpio22driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio22driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio22driving::Enabled
    }
}
#[doc = "Field `reg_gpio_22_drv` writer - Driving control enabled for GPIO22."]
pub type RegGpio22DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio22driving>;
impl<'a, REG> RegGpio22DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio22pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio22pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_pu` reader - Pull Up Resistor for GPIO22."]
pub type RegGpio22PuR = crate::BitReader<Gpio22pullUpResistor>;
impl RegGpio22PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22pullUpResistor {
        match self.bits {
            false => Gpio22pullUpResistor::Disabled,
            true => Gpio22pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio22pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio22pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_22_pu` writer - Pull Up Resistor for GPIO22."]
pub type RegGpio22PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio22pullUpResistor>;
impl<'a, REG> RegGpio22PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio22pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio22pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio22pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_pd` reader - Pull Down Resistor for GPIO22."]
pub type RegGpio22PdR = crate::BitReader<Gpio22pullDownResistor>;
impl RegGpio22PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio22pullDownResistor {
        match self.bits {
            false => Gpio22pullDownResistor::Disabled,
            true => Gpio22pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio22pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio22pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_22_pd` writer - Pull Down Resistor for GPIO22."]
pub type RegGpio22PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio22pullDownResistor>;
impl<'a, REG> RegGpio22PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22pullDownResistor::Enabled)
    }
}
#[doc = "Function select for GPIO22.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio22functionSelect {
    #[doc = "2: `10`"]
    SfClkOut = 2,
    #[doc = "4: `100`"]
    SpiSs = 4,
    #[doc = "6: `110`"]
    I2cScl = 6,
    #[doc = "7: `111`"]
    UartSig6 = 7,
    #[doc = "8: `1000`"]
    PwmCh2 = 8,
    #[doc = "9: `1001`"]
    FemGpio2 = 9,
    #[doc = "11: `1011`"]
    Swgpio22 = 11,
    #[doc = "14: `1110`"]
    E21Tck = 14,
}
impl From<Gpio22functionSelect> for u8 {
    #[inline(always)]
    fn from(variant: Gpio22functionSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio22functionSelect {
    type Ux = u8;
}
impl crate::IsEnum for Gpio22functionSelect {}
#[doc = "Field `reg_gpio_22_func_sel` reader - Function select for GPIO22."]
pub type RegGpio22FuncSelR = crate::FieldReader<Gpio22functionSelect>;
impl RegGpio22FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio22functionSelect> {
        match self.bits {
            2 => Some(Gpio22functionSelect::SfClkOut),
            4 => Some(Gpio22functionSelect::SpiSs),
            6 => Some(Gpio22functionSelect::I2cScl),
            7 => Some(Gpio22functionSelect::UartSig6),
            8 => Some(Gpio22functionSelect::PwmCh2),
            9 => Some(Gpio22functionSelect::FemGpio2),
            11 => Some(Gpio22functionSelect::Swgpio22),
            14 => Some(Gpio22functionSelect::E21Tck),
            _ => None,
        }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sf_clk_out(&self) -> bool {
        *self == Gpio22functionSelect::SfClkOut
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        *self == Gpio22functionSelect::SpiSs
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        *self == Gpio22functionSelect::I2cScl
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart_sig6(&self) -> bool {
        *self == Gpio22functionSelect::UartSig6
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        *self == Gpio22functionSelect::PwmCh2
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        *self == Gpio22functionSelect::FemGpio2
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_swgpio_22(&self) -> bool {
        *self == Gpio22functionSelect::Swgpio22
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        *self == Gpio22functionSelect::E21Tck
    }
}
#[doc = "Field `reg_gpio_22_func_sel` writer - Function select for GPIO22."]
pub type RegGpio22FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Gpio22functionSelect>;
impl<'a, REG> RegGpio22FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_clk_out(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22functionSelect::SfClkOut)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22functionSelect::SpiSs)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22functionSelect::I2cScl)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig6(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22functionSelect::UartSig6)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22functionSelect::PwmCh2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22functionSelect::FemGpio2)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_22(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22functionSelect::Swgpio22)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio22functionSelect::E21Tck)
    }
}
#[doc = "Input enable for GPIO23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio23inputEnabled {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio23inputEnabled> for bool {
    #[inline(always)]
    fn from(variant: Gpio23inputEnabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_ie` reader - Input enable for GPIO23."]
pub type RegGpio23IeR = crate::BitReader<Gpio23inputEnabled>;
impl RegGpio23IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio23inputEnabled {
        match self.bits {
            false => Gpio23inputEnabled::Disabled,
            true => Gpio23inputEnabled::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio23inputEnabled::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio23inputEnabled::Enabled
    }
}
#[doc = "Field `reg_gpio_23_ie` writer - Input enable for GPIO23."]
pub type RegGpio23IeW<'a, REG> = crate::BitWriter<'a, REG, Gpio23inputEnabled>;
impl<'a, REG> RegGpio23IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23inputEnabled::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23inputEnabled::Enabled)
    }
}
#[doc = "Schmitt trigger enabled for GPIO23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio23schmitt {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio23schmitt> for bool {
    #[inline(always)]
    fn from(variant: Gpio23schmitt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_smt` reader - Schmitt trigger enabled for GPIO23."]
pub type RegGpio23SmtR = crate::BitReader<Gpio23schmitt>;
impl RegGpio23SmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio23schmitt {
        match self.bits {
            false => Gpio23schmitt::Disabled,
            true => Gpio23schmitt::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio23schmitt::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio23schmitt::Enabled
    }
}
#[doc = "Field `reg_gpio_23_smt` writer - Schmitt trigger enabled for GPIO23."]
pub type RegGpio23SmtW<'a, REG> = crate::BitWriter<'a, REG, Gpio23schmitt>;
impl<'a, REG> RegGpio23SmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23schmitt::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23schmitt::Enabled)
    }
}
#[doc = "Driving control enabled for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio23driving {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio23driving> for u8 {
    #[inline(always)]
    fn from(variant: Gpio23driving) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio23driving {
    type Ux = u8;
}
impl crate::IsEnum for Gpio23driving {}
#[doc = "Field `reg_gpio_23_drv` reader - Driving control enabled for GPIO23."]
pub type RegGpio23DrvR = crate::FieldReader<Gpio23driving>;
impl RegGpio23DrvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio23driving> {
        match self.bits {
            0 => Some(Gpio23driving::Disabled),
            1 => Some(Gpio23driving::Enabled),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio23driving::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio23driving::Enabled
    }
}
#[doc = "Field `reg_gpio_23_drv` writer - Driving control enabled for GPIO23."]
pub type RegGpio23DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio23driving>;
impl<'a, REG> RegGpio23DrvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23driving::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23driving::Enabled)
    }
}
#[doc = "Pull Up Resistor for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio23pullUpResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio23pullUpResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio23pullUpResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_pu` reader - Pull Up Resistor for GPIO23."]
pub type RegGpio23PuR = crate::BitReader<Gpio23pullUpResistor>;
impl RegGpio23PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio23pullUpResistor {
        match self.bits {
            false => Gpio23pullUpResistor::Disabled,
            true => Gpio23pullUpResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio23pullUpResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio23pullUpResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_23_pu` writer - Pull Up Resistor for GPIO23."]
pub type RegGpio23PuW<'a, REG> = crate::BitWriter<'a, REG, Gpio23pullUpResistor>;
impl<'a, REG> RegGpio23PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23pullUpResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23pullUpResistor::Enabled)
    }
}
#[doc = "Pull Down Resistor for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio23pullDownResistor {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: `1`"]
    Enabled = 1,
}
impl From<Gpio23pullDownResistor> for bool {
    #[inline(always)]
    fn from(variant: Gpio23pullDownResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_pd` reader - Pull Down Resistor for GPIO23."]
pub type RegGpio23PdR = crate::BitReader<Gpio23pullDownResistor>;
impl RegGpio23PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio23pullDownResistor {
        match self.bits {
            false => Gpio23pullDownResistor::Disabled,
            true => Gpio23pullDownResistor::Enabled,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gpio23pullDownResistor::Disabled
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gpio23pullDownResistor::Enabled
    }
}
#[doc = "Field `reg_gpio_23_pd` writer - Pull Down Resistor for GPIO23."]
pub type RegGpio23PdW<'a, REG> = crate::BitWriter<'a, REG, Gpio23pullDownResistor>;
impl<'a, REG> RegGpio23PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23pullDownResistor::Disabled)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio23pullDownResistor::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Input enable for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_ie(&self) -> RegGpio22IeR {
        RegGpio22IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_smt(&self) -> RegGpio22SmtR {
        RegGpio22SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_drv(&self) -> RegGpio22DrvR {
        RegGpio22DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_pu(&self) -> RegGpio22PuR {
        RegGpio22PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_pd(&self) -> RegGpio22PdR {
        RegGpio22PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_func_sel(&self) -> RegGpio22FuncSelR {
        RegGpio22FuncSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input enable for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_ie(&self) -> RegGpio23IeR {
        RegGpio23IeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_smt(&self) -> RegGpio23SmtR {
        RegGpio23SmtR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_drv(&self) -> RegGpio23DrvR {
        RegGpio23DrvR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_pu(&self) -> RegGpio23PuR {
        RegGpio23PuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_pd(&self) -> RegGpio23PdR {
        RegGpio23PdR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input enable for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_ie(&mut self) -> RegGpio22IeW<'_, GpioCfgctl11Spec> {
        RegGpio22IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_smt(&mut self) -> RegGpio22SmtW<'_, GpioCfgctl11Spec> {
        RegGpio22SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_drv(&mut self) -> RegGpio22DrvW<'_, GpioCfgctl11Spec> {
        RegGpio22DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_pu(&mut self) -> RegGpio22PuW<'_, GpioCfgctl11Spec> {
        RegGpio22PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_pd(&mut self) -> RegGpio22PdW<'_, GpioCfgctl11Spec> {
        RegGpio22PdW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Function select for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_func_sel(&mut self) -> RegGpio22FuncSelW<'_, GpioCfgctl11Spec> {
        RegGpio22FuncSelW::new(self, 8)
    }
    #[doc = "Bit 16 - Input enable for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_ie(&mut self) -> RegGpio23IeW<'_, GpioCfgctl11Spec> {
        RegGpio23IeW::new(self, 16)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_smt(&mut self) -> RegGpio23SmtW<'_, GpioCfgctl11Spec> {
        RegGpio23SmtW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_drv(&mut self) -> RegGpio23DrvW<'_, GpioCfgctl11Spec> {
        RegGpio23DrvW::new(self, 18)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_pu(&mut self) -> RegGpio23PuW<'_, GpioCfgctl11Spec> {
        RegGpio23PuW::new(self, 20)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_pd(&mut self) -> RegGpio23PdW<'_, GpioCfgctl11Spec> {
        RegGpio23PdW::new(self, 21)
    }
}
#[doc = "GPIO22, GPIO23 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfgctl11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfgctl11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfgctl11Spec;
impl crate::RegisterSpec for GpioCfgctl11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfgctl11::R`](R) reader structure"]
impl crate::Readable for GpioCfgctl11Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfgctl11::W`](W) writer structure"]
impl crate::Writable for GpioCfgctl11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFGCTL11 to value 0x0003_0b03"]
impl crate::Resettable for GpioCfgctl11Spec {
    const RESET_VALUE: u32 = 0x0003_0b03;
}
