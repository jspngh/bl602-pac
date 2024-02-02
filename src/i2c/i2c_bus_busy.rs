#[doc = "Register `i2c_bus_busy` reader"]
pub type R = crate::R<I2cBusBusySpec>;
#[doc = "Register `i2c_bus_busy` writer"]
pub type W = crate::W<I2cBusBusySpec>;
#[doc = "Field `sts_i2c_bus_busy` reader - "]
pub type StsI2cBusBusyR = crate::BitReader;
#[doc = "Field `cr_i2c_bus_busy_clr` reader - "]
pub type CrI2cBusBusyClrR = crate::BitReader;
#[doc = "Field `cr_i2c_bus_busy_clr` writer - "]
pub type CrI2cBusBusyClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_i2c_bus_busy(&self) -> StsI2cBusBusyR {
        StsI2cBusBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_bus_busy_clr(&self) -> CrI2cBusBusyClrR {
        CrI2cBusBusyClrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_bus_busy_clr(&mut self) -> CrI2cBusBusyClrW<'_, I2cBusBusySpec> {
        CrI2cBusBusyClrW::new(self, 1)
    }
}
#[doc = "i2c_bus_busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_bus_busy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_bus_busy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cBusBusySpec;
impl crate::RegisterSpec for I2cBusBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_bus_busy::R`](R) reader structure"]
impl crate::Readable for I2cBusBusySpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_bus_busy::W`](W) writer structure"]
impl crate::Writable for I2cBusBusySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets i2c_bus_busy to value 0"]
impl crate::Resettable for I2cBusBusySpec {}
