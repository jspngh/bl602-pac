#[doc = "Register `vco1` reader"]
pub type R = crate::R<Vco1Spec>;
#[doc = "Register `vco1` writer"]
pub type W = crate::W<Vco1Spec>;
#[doc = "Field `lo_vco_freq_cw` reader - "]
pub type LoVcoFreqCwR = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw` writer - "]
pub type LoVcoFreqCwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_freq_cw_hw` reader - "]
pub type LoVcoFreqCwHwR = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_hw` writer - "]
pub type LoVcoFreqCwHwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw` reader - "]
pub type LoVcoIdacCwR = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw` writer - "]
pub type LoVcoIdacCwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_idac_cw_hw` reader - "]
pub type LoVcoIdacCwHwR = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_hw` writer - "]
pub type LoVcoIdacCwHwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lo_vco_freq_cw(&self) -> LoVcoFreqCwR {
        LoVcoFreqCwR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_hw(&self) -> LoVcoFreqCwHwR {
        LoVcoFreqCwHwR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw(&self) -> LoVcoIdacCwR {
        LoVcoIdacCwR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_hw(&self) -> LoVcoIdacCwHwR {
        LoVcoIdacCwHwR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lo_vco_freq_cw(&mut self) -> LoVcoFreqCwW<'_, Vco1Spec> {
        LoVcoFreqCwW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_hw(&mut self) -> LoVcoFreqCwHwW<'_, Vco1Spec> {
        LoVcoFreqCwHwW::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw(&mut self) -> LoVcoIdacCwW<'_, Vco1Spec> {
        LoVcoIdacCwW::new(self, 16)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_hw(&mut self) -> LoVcoIdacCwHwW<'_, Vco1Spec> {
        LoVcoIdacCwHwW::new(self, 24)
    }
}
#[doc = "vco1.\n\nYou can [`read`](crate::Reg::read) this register and get [`vco1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vco1Spec;
impl crate::RegisterSpec for Vco1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vco1::R`](R) reader structure"]
impl crate::Readable for Vco1Spec {}
#[doc = "`write(|w| ..)` method takes [`vco1::W`](W) writer structure"]
impl crate::Writable for Vco1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets vco1 to value 0"]
impl crate::Resettable for Vco1Spec {}
