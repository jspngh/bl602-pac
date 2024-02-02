#[doc = "Register `lo_cal_ctrl_hw11` reader"]
pub type R = crate::R<LoCalCtrlHw11Spec>;
#[doc = "Register `lo_cal_ctrl_hw11` writer"]
pub type W = crate::W<LoCalCtrlHw11Spec>;
#[doc = "Field `lo_vco_idac_cw_2484` reader - "]
pub type LoVcoIdacCw2484R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2484` writer - "]
pub type LoVcoIdacCw2484W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2484` reader - "]
pub type LoVcoFreqCw2484R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2484` writer - "]
pub type LoVcoFreqCw2484W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2484(&self) -> LoVcoIdacCw2484R {
        LoVcoIdacCw2484R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2484(&self) -> LoVcoFreqCw2484R {
        LoVcoFreqCw2484R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2484(&mut self) -> LoVcoIdacCw2484W<'_, LoCalCtrlHw11Spec> {
        LoVcoIdacCw2484W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2484(&mut self) -> LoVcoFreqCw2484W<'_, LoCalCtrlHw11Spec> {
        LoVcoFreqCw2484W::new(self, 8)
    }
}
#[doc = "lo_cal_ctrl_hw11.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw11Spec;
impl crate::RegisterSpec for LoCalCtrlHw11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw11::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw11Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw11::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw11 to value 0"]
impl crate::Resettable for LoCalCtrlHw11Spec {}
