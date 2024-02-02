#[doc = "Register `vco2` reader"]
pub type R = crate::R<Vco2Spec>;
#[doc = "Register `vco2` writer"]
pub type W = crate::W<Vco2Spec>;
#[doc = "Field `lo_vco_vbias_cw` reader - "]
pub type LoVcoVbiasCwR = crate::FieldReader;
#[doc = "Field `lo_vco_vbias_cw` writer - "]
pub type LoVcoVbiasCwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_vco_idac_boot` reader - "]
pub type LoVcoIdacBootR = crate::BitReader;
#[doc = "Field `lo_vco_idac_boot` writer - "]
pub type LoVcoIdacBootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_vco_short_vbias_filter` reader - "]
pub type LoVcoShortVbiasFilterR = crate::BitReader;
#[doc = "Field `lo_vco_short_vbias_filter` writer - "]
pub type LoVcoShortVbiasFilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_vco_short_idac_filter` reader - "]
pub type LoVcoShortIdacFilterR = crate::BitReader;
#[doc = "Field `lo_vco_short_idac_filter` writer - "]
pub type LoVcoShortIdacFilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acal_vref_cw` reader - "]
pub type AcalVrefCwR = crate::FieldReader;
#[doc = "Field `acal_vref_cw` writer - "]
pub type AcalVrefCwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `acal_vco_ud` reader - "]
pub type AcalVcoUdR = crate::BitReader;
#[doc = "Field `acal_vco_ud` writer - "]
pub type AcalVcoUdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acal_inc_en_hw` reader - "]
pub type AcalIncEnHwR = crate::BitReader;
#[doc = "Field `acal_inc_en_hw` writer - "]
pub type AcalIncEnHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_vco_vbias_cw(&self) -> LoVcoVbiasCwR {
        LoVcoVbiasCwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_vco_idac_boot(&self) -> LoVcoIdacBootR {
        LoVcoIdacBootR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lo_vco_short_vbias_filter(&self) -> LoVcoShortVbiasFilterR {
        LoVcoShortVbiasFilterR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_vco_short_idac_filter(&self) -> LoVcoShortIdacFilterR {
        LoVcoShortIdacFilterR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn acal_vref_cw(&self) -> AcalVrefCwR {
        AcalVrefCwR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn acal_vco_ud(&self) -> AcalVcoUdR {
        AcalVcoUdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn acal_inc_en_hw(&self) -> AcalIncEnHwR {
        AcalIncEnHwR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_vco_vbias_cw(&mut self) -> LoVcoVbiasCwW<'_, Vco2Spec> {
        LoVcoVbiasCwW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_vco_idac_boot(&mut self) -> LoVcoIdacBootW<'_, Vco2Spec> {
        LoVcoIdacBootW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lo_vco_short_vbias_filter(&mut self) -> LoVcoShortVbiasFilterW<'_, Vco2Spec> {
        LoVcoShortVbiasFilterW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_vco_short_idac_filter(&mut self) -> LoVcoShortIdacFilterW<'_, Vco2Spec> {
        LoVcoShortIdacFilterW::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn acal_vref_cw(&mut self) -> AcalVrefCwW<'_, Vco2Spec> {
        AcalVrefCwW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn acal_vco_ud(&mut self) -> AcalVcoUdW<'_, Vco2Spec> {
        AcalVcoUdW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn acal_inc_en_hw(&mut self) -> AcalIncEnHwW<'_, Vco2Spec> {
        AcalIncEnHwW::new(self, 16)
    }
}
#[doc = "vco2.\n\nYou can [`read`](crate::Reg::read) this register and get [`vco2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vco2Spec;
impl crate::RegisterSpec for Vco2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vco2::R`](R) reader structure"]
impl crate::Readable for Vco2Spec {}
#[doc = "`write(|w| ..)` method takes [`vco2::W`](W) writer structure"]
impl crate::Writable for Vco2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets vco2 to value 0"]
impl crate::Resettable for Vco2Spec {}
