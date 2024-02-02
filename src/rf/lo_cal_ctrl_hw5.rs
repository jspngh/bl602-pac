#[doc = "Register `lo_cal_ctrl_hw5` reader"]
pub type R = crate::R<LoCalCtrlHw5Spec>;
#[doc = "Register `lo_cal_ctrl_hw5` writer"]
pub type W = crate::W<LoCalCtrlHw5Spec>;
#[doc = "Field `lo_vco_idac_cw_2436` reader - "]
pub type LoVcoIdacCw2436R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2436` writer - "]
pub type LoVcoIdacCw2436W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2436` reader - "]
pub type LoVcoFreqCw2436R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2436` writer - "]
pub type LoVcoFreqCw2436W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2440` reader - "]
pub type LoVcoIdacCw2440R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2440` writer - "]
pub type LoVcoIdacCw2440W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2440` reader - "]
pub type LoVcoFreqCw2440R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2440` writer - "]
pub type LoVcoFreqCw2440W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2436(&self) -> LoVcoIdacCw2436R {
        LoVcoIdacCw2436R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2436(&self) -> LoVcoFreqCw2436R {
        LoVcoFreqCw2436R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2440(&self) -> LoVcoIdacCw2440R {
        LoVcoIdacCw2440R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2440(&self) -> LoVcoFreqCw2440R {
        LoVcoFreqCw2440R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2436(&mut self) -> LoVcoIdacCw2436W<'_, LoCalCtrlHw5Spec> {
        LoVcoIdacCw2436W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2436(&mut self) -> LoVcoFreqCw2436W<'_, LoCalCtrlHw5Spec> {
        LoVcoFreqCw2436W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2440(&mut self) -> LoVcoIdacCw2440W<'_, LoCalCtrlHw5Spec> {
        LoVcoIdacCw2440W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2440(&mut self) -> LoVcoFreqCw2440W<'_, LoCalCtrlHw5Spec> {
        LoVcoFreqCw2440W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw5.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw5Spec;
impl crate::RegisterSpec for LoCalCtrlHw5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw5::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw5Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw5::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw5 to value 0"]
impl crate::Resettable for LoCalCtrlHw5Spec {}
