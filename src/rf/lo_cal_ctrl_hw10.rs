#[doc = "Register `lo_cal_ctrl_hw10` reader"]
pub type R = crate::R<LoCalCtrlHw10Spec>;
#[doc = "Register `lo_cal_ctrl_hw10` writer"]
pub type W = crate::W<LoCalCtrlHw10Spec>;
#[doc = "Field `lo_vco_idac_cw_2476` reader - "]
pub type LoVcoIdacCw2476R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2476` writer - "]
pub type LoVcoIdacCw2476W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2476` reader - "]
pub type LoVcoFreqCw2476R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2476` writer - "]
pub type LoVcoFreqCw2476W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2480` reader - "]
pub type LoVcoIdacCw2480R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2480` writer - "]
pub type LoVcoIdacCw2480W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2480` reader - "]
pub type LoVcoFreqCw2480R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2480` writer - "]
pub type LoVcoFreqCw2480W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2476(&self) -> LoVcoIdacCw2476R {
        LoVcoIdacCw2476R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2476(&self) -> LoVcoFreqCw2476R {
        LoVcoFreqCw2476R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2480(&self) -> LoVcoIdacCw2480R {
        LoVcoIdacCw2480R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2480(&self) -> LoVcoFreqCw2480R {
        LoVcoFreqCw2480R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2476(&mut self) -> LoVcoIdacCw2476W<'_, LoCalCtrlHw10Spec> {
        LoVcoIdacCw2476W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2476(&mut self) -> LoVcoFreqCw2476W<'_, LoCalCtrlHw10Spec> {
        LoVcoFreqCw2476W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2480(&mut self) -> LoVcoIdacCw2480W<'_, LoCalCtrlHw10Spec> {
        LoVcoIdacCw2480W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2480(&mut self) -> LoVcoFreqCw2480W<'_, LoCalCtrlHw10Spec> {
        LoVcoFreqCw2480W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw10.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw10Spec;
impl crate::RegisterSpec for LoCalCtrlHw10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw10::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw10Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw10::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw10 to value 0"]
impl crate::Resettable for LoCalCtrlHw10Spec {}
