#[doc = "Register `led_driver` reader"]
pub type R = crate::R<LedDriverSpec>;
#[doc = "Register `led_driver` writer"]
pub type W = crate::W<LedDriverSpec>;
#[doc = "Field `led_din_reg` reader - "]
pub type LedDinRegR = crate::BitReader;
#[doc = "Field `led_din_reg` writer - "]
pub type LedDinRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `led_din_sel` reader - "]
pub type LedDinSelR = crate::BitReader;
#[doc = "Field `led_din_sel` writer - "]
pub type LedDinSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `led_din_polarity_sel` reader - "]
pub type LedDinPolaritySelR = crate::BitReader;
#[doc = "Field `led_din_polarity_sel` writer - "]
pub type LedDinPolaritySelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `leddrv_ibias` reader - "]
pub type LeddrvIbiasR = crate::FieldReader;
#[doc = "Field `leddrv_ibias` writer - "]
pub type LeddrvIbiasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ir_rx_gpio_sel` reader - "]
pub type IrRxGpioSelR = crate::FieldReader;
#[doc = "Field `ir_rx_gpio_sel` writer - "]
pub type IrRxGpioSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pu_leddrv` reader - "]
pub type PuLeddrvR = crate::BitReader;
#[doc = "Field `pu_leddrv` writer - "]
pub type PuLeddrvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_din_reg(&self) -> LedDinRegR {
        LedDinRegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_din_sel(&self) -> LedDinSelR {
        LedDinSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_din_polarity_sel(&self) -> LedDinPolaritySelR {
        LedDinPolaritySelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn leddrv_ibias(&self) -> LeddrvIbiasR {
        LeddrvIbiasR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ir_rx_gpio_sel(&self) -> IrRxGpioSelR {
        IrRxGpioSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_leddrv(&self) -> PuLeddrvR {
        PuLeddrvR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_din_reg(&mut self) -> LedDinRegW<'_, LedDriverSpec> {
        LedDinRegW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_din_sel(&mut self) -> LedDinSelW<'_, LedDriverSpec> {
        LedDinSelW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_din_polarity_sel(&mut self) -> LedDinPolaritySelW<'_, LedDriverSpec> {
        LedDinPolaritySelW::new(self, 2)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn leddrv_ibias(&mut self) -> LeddrvIbiasW<'_, LedDriverSpec> {
        LeddrvIbiasW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ir_rx_gpio_sel(&mut self) -> IrRxGpioSelW<'_, LedDriverSpec> {
        IrRxGpioSelW::new(self, 8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_leddrv(&mut self) -> PuLeddrvW<'_, LedDriverSpec> {
        PuLeddrvW::new(self, 31)
    }
}
#[doc = "led_driver.\n\nYou can [`read`](crate::Reg::read) this register and get [`led_driver::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`led_driver::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LedDriverSpec;
impl crate::RegisterSpec for LedDriverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`led_driver::R`](R) reader structure"]
impl crate::Readable for LedDriverSpec {}
#[doc = "`write(|w| ..)` method takes [`led_driver::W`](W) writer structure"]
impl crate::Writable for LedDriverSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets led_driver to value 0x80"]
impl crate::Resettable for LedDriverSpec {
    const RESET_VALUE: u32 = 0x80;
}
