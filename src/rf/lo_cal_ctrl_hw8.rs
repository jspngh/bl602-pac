#[doc = "Register `lo_cal_ctrl_hw8` reader"]
pub type R = crate::R<LoCalCtrlHw8Spec>;
#[doc = "Register `lo_cal_ctrl_hw8` writer"]
pub type W = crate::W<LoCalCtrlHw8Spec>;
#[doc = "Field `lo_vco_idac_cw_2460` reader - "]
pub type LoVcoIdacCw2460R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2460` writer - "]
pub type LoVcoIdacCw2460W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2460` reader - "]
pub type LoVcoFreqCw2460R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2460` writer - "]
pub type LoVcoFreqCw2460W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2464` reader - "]
pub type LoVcoIdacCw2464R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2464` writer - "]
pub type LoVcoIdacCw2464W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2464` reader - "]
pub type LoVcoFreqCw2464R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2464` writer - "]
pub type LoVcoFreqCw2464W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2460(&self) -> LoVcoIdacCw2460R {
        LoVcoIdacCw2460R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2460(&self) -> LoVcoFreqCw2460R {
        LoVcoFreqCw2460R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2464(&self) -> LoVcoIdacCw2464R {
        LoVcoIdacCw2464R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2464(&self) -> LoVcoFreqCw2464R {
        LoVcoFreqCw2464R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2460(&mut self) -> LoVcoIdacCw2460W<'_, LoCalCtrlHw8Spec> {
        LoVcoIdacCw2460W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2460(&mut self) -> LoVcoFreqCw2460W<'_, LoCalCtrlHw8Spec> {
        LoVcoFreqCw2460W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2464(&mut self) -> LoVcoIdacCw2464W<'_, LoCalCtrlHw8Spec> {
        LoVcoIdacCw2464W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2464(&mut self) -> LoVcoFreqCw2464W<'_, LoCalCtrlHw8Spec> {
        LoVcoFreqCw2464W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw8.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw8Spec;
impl crate::RegisterSpec for LoCalCtrlHw8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw8::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw8Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw8::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw8 to value 0"]
impl crate::Resettable for LoCalCtrlHw8Spec {}
