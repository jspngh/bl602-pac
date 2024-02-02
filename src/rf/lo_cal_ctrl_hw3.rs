#[doc = "Register `lo_cal_ctrl_hw3` reader"]
pub type R = crate::R<LoCalCtrlHw3Spec>;
#[doc = "Register `lo_cal_ctrl_hw3` writer"]
pub type W = crate::W<LoCalCtrlHw3Spec>;
#[doc = "Field `lo_vco_idac_cw_2420` reader - "]
pub type LoVcoIdacCw2420R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2420` writer - "]
pub type LoVcoIdacCw2420W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2420` reader - "]
pub type LoVcoFreqCw2420R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2420` writer - "]
pub type LoVcoFreqCw2420W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2424` reader - "]
pub type LoVcoIdacCw2424R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2424` writer - "]
pub type LoVcoIdacCw2424W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2424` reader - "]
pub type LoVcoFreqCw2424R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2424` writer - "]
pub type LoVcoFreqCw2424W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2420(&self) -> LoVcoIdacCw2420R {
        LoVcoIdacCw2420R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2420(&self) -> LoVcoFreqCw2420R {
        LoVcoFreqCw2420R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2424(&self) -> LoVcoIdacCw2424R {
        LoVcoIdacCw2424R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2424(&self) -> LoVcoFreqCw2424R {
        LoVcoFreqCw2424R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2420(&mut self) -> LoVcoIdacCw2420W<'_, LoCalCtrlHw3Spec> {
        LoVcoIdacCw2420W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2420(&mut self) -> LoVcoFreqCw2420W<'_, LoCalCtrlHw3Spec> {
        LoVcoFreqCw2420W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2424(&mut self) -> LoVcoIdacCw2424W<'_, LoCalCtrlHw3Spec> {
        LoVcoIdacCw2424W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2424(&mut self) -> LoVcoFreqCw2424W<'_, LoCalCtrlHw3Spec> {
        LoVcoFreqCw2424W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw3.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw3Spec;
impl crate::RegisterSpec for LoCalCtrlHw3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw3::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw3Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw3::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw3 to value 0"]
impl crate::Resettable for LoCalCtrlHw3Spec {}
