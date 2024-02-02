#[doc = "Register `clk_cfg3` reader"]
pub type R = crate::R<ClkCfg3Spec>;
#[doc = "Register `clk_cfg3` writer"]
pub type W = crate::W<ClkCfg3Spec>;
#[doc = "Field `spi_clk_div` reader - "]
pub type SpiClkDivR = crate::FieldReader;
#[doc = "Field `spi_clk_div` writer - "]
pub type SpiClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `spi_clk_en` reader - "]
pub type SpiClkEnR = crate::BitReader;
#[doc = "Field `spi_clk_en` writer - "]
pub type SpiClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c_clk_div` reader - "]
pub type I2cClkDivR = crate::FieldReader;
#[doc = "Field `i2c_clk_div` writer - "]
pub type I2cClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `i2c_clk_en` reader - "]
pub type I2cClkEnR = crate::BitReader;
#[doc = "Field `i2c_clk_en` writer - "]
pub type I2cClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&self) -> SpiClkDivR {
        SpiClkDivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SpiClkEnR {
        SpiClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&self) -> I2cClkDivR {
        I2cClkDivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2cClkEnR {
        I2cClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&mut self) -> SpiClkDivW<'_, ClkCfg3Spec> {
        SpiClkDivW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&mut self) -> SpiClkEnW<'_, ClkCfg3Spec> {
        SpiClkEnW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&mut self) -> I2cClkDivW<'_, ClkCfg3Spec> {
        I2cClkDivW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2cClkEnW<'_, ClkCfg3Spec> {
        I2cClkEnW::new(self, 24)
    }
}
#[doc = "Clock configuration for I2C and SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCfg3Spec;
impl crate::RegisterSpec for ClkCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cfg3::R`](R) reader structure"]
impl crate::Readable for ClkCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_cfg3::W`](W) writer structure"]
impl crate::Writable for ClkCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clk_cfg3 to value 0x01ff_0103"]
impl crate::Resettable for ClkCfg3Spec {
    const RESET_VALUE: u32 = 0x01ff_0103;
}
