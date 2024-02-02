#[doc = "Register `lo_cal_ctrl_hw1` reader"]
pub type R = crate::R<LoCalCtrlHw1Spec>;
#[doc = "Register `lo_cal_ctrl_hw1` writer"]
pub type W = crate::W<LoCalCtrlHw1Spec>;
#[doc = "Field `lo_vco_idac_cw_2404` reader - "]
pub type LoVcoIdacCw2404R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2404` writer - "]
pub type LoVcoIdacCw2404W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2404` reader - "]
pub type LoVcoFreqCw2404R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2404` writer - "]
pub type LoVcoFreqCw2404W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2408` reader - "]
pub type LoVcoIdacCw2408R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2408` writer - "]
pub type LoVcoIdacCw2408W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2408` reader - "]
pub type LoVcoFreqCw2408R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2408` writer - "]
pub type LoVcoFreqCw2408W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2404(&self) -> LoVcoIdacCw2404R {
        LoVcoIdacCw2404R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2404(&self) -> LoVcoFreqCw2404R {
        LoVcoFreqCw2404R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2408(&self) -> LoVcoIdacCw2408R {
        LoVcoIdacCw2408R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2408(&self) -> LoVcoFreqCw2408R {
        LoVcoFreqCw2408R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2404(&mut self) -> LoVcoIdacCw2404W<'_, LoCalCtrlHw1Spec> {
        LoVcoIdacCw2404W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2404(&mut self) -> LoVcoFreqCw2404W<'_, LoCalCtrlHw1Spec> {
        LoVcoFreqCw2404W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2408(&mut self) -> LoVcoIdacCw2408W<'_, LoCalCtrlHw1Spec> {
        LoVcoIdacCw2408W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2408(&mut self) -> LoVcoFreqCw2408W<'_, LoCalCtrlHw1Spec> {
        LoVcoFreqCw2408W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw1Spec;
impl crate::RegisterSpec for LoCalCtrlHw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw1::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw1Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw1::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw1 to value 0"]
impl crate::Resettable for LoCalCtrlHw1Spec {}
