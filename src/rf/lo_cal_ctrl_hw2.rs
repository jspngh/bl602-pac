#[doc = "Register `lo_cal_ctrl_hw2` reader"]
pub type R = crate::R<LoCalCtrlHw2Spec>;
#[doc = "Register `lo_cal_ctrl_hw2` writer"]
pub type W = crate::W<LoCalCtrlHw2Spec>;
#[doc = "Field `lo_vco_idac_cw_2412` reader - "]
pub type LoVcoIdacCw2412R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2412` writer - "]
pub type LoVcoIdacCw2412W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2412` reader - "]
pub type LoVcoFreqCw2412R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2412` writer - "]
pub type LoVcoFreqCw2412W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2416` reader - "]
pub type LoVcoIdacCw2416R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2416` writer - "]
pub type LoVcoIdacCw2416W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2416` reader - "]
pub type LoVcoFreqCw2416R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2416` writer - "]
pub type LoVcoFreqCw2416W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2412(&self) -> LoVcoIdacCw2412R {
        LoVcoIdacCw2412R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2412(&self) -> LoVcoFreqCw2412R {
        LoVcoFreqCw2412R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2416(&self) -> LoVcoIdacCw2416R {
        LoVcoIdacCw2416R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2416(&self) -> LoVcoFreqCw2416R {
        LoVcoFreqCw2416R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2412(&mut self) -> LoVcoIdacCw2412W<'_, LoCalCtrlHw2Spec> {
        LoVcoIdacCw2412W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2412(&mut self) -> LoVcoFreqCw2412W<'_, LoCalCtrlHw2Spec> {
        LoVcoFreqCw2412W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2416(&mut self) -> LoVcoIdacCw2416W<'_, LoCalCtrlHw2Spec> {
        LoVcoIdacCw2416W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2416(&mut self) -> LoVcoFreqCw2416W<'_, LoCalCtrlHw2Spec> {
        LoVcoFreqCw2416W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw2Spec;
impl crate::RegisterSpec for LoCalCtrlHw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw2::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw2Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw2::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw2 to value 0"]
impl crate::Resettable for LoCalCtrlHw2Spec {}
