#[doc = "Register `lo_cal_ctrl_hw9` reader"]
pub type R = crate::R<LoCalCtrlHw9Spec>;
#[doc = "Register `lo_cal_ctrl_hw9` writer"]
pub type W = crate::W<LoCalCtrlHw9Spec>;
#[doc = "Field `lo_vco_idac_cw_2468` reader - "]
pub type LoVcoIdacCw2468R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2468` writer - "]
pub type LoVcoIdacCw2468W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2468` reader - "]
pub type LoVcoFreqCw2468R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2468` writer - "]
pub type LoVcoFreqCw2468W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2472` reader - "]
pub type LoVcoIdacCw2472R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2472` writer - "]
pub type LoVcoIdacCw2472W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2472` reader - "]
pub type LoVcoFreqCw2472R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2472` writer - "]
pub type LoVcoFreqCw2472W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2468(&self) -> LoVcoIdacCw2468R {
        LoVcoIdacCw2468R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2468(&self) -> LoVcoFreqCw2468R {
        LoVcoFreqCw2468R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2472(&self) -> LoVcoIdacCw2472R {
        LoVcoIdacCw2472R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2472(&self) -> LoVcoFreqCw2472R {
        LoVcoFreqCw2472R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2468(&mut self) -> LoVcoIdacCw2468W<'_, LoCalCtrlHw9Spec> {
        LoVcoIdacCw2468W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2468(&mut self) -> LoVcoFreqCw2468W<'_, LoCalCtrlHw9Spec> {
        LoVcoFreqCw2468W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2472(&mut self) -> LoVcoIdacCw2472W<'_, LoCalCtrlHw9Spec> {
        LoVcoIdacCw2472W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2472(&mut self) -> LoVcoFreqCw2472W<'_, LoCalCtrlHw9Spec> {
        LoVcoFreqCw2472W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw9.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw9Spec;
impl crate::RegisterSpec for LoCalCtrlHw9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw9::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw9Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw9::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw9 to value 0"]
impl crate::Resettable for LoCalCtrlHw9Spec {}
