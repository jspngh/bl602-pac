#[doc = "Register `lo_cal_ctrl_hw6` reader"]
pub type R = crate::R<LoCalCtrlHw6Spec>;
#[doc = "Register `lo_cal_ctrl_hw6` writer"]
pub type W = crate::W<LoCalCtrlHw6Spec>;
#[doc = "Field `lo_vco_idac_cw_2444` reader - "]
pub type LoVcoIdacCw2444R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2444` writer - "]
pub type LoVcoIdacCw2444W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2444` reader - "]
pub type LoVcoFreqCw2444R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2444` writer - "]
pub type LoVcoFreqCw2444W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2448` reader - "]
pub type LoVcoIdacCw2448R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2448` writer - "]
pub type LoVcoIdacCw2448W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2448` reader - "]
pub type LoVcoFreqCw2448R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2448` writer - "]
pub type LoVcoFreqCw2448W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2444(&self) -> LoVcoIdacCw2444R {
        LoVcoIdacCw2444R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2444(&self) -> LoVcoFreqCw2444R {
        LoVcoFreqCw2444R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2448(&self) -> LoVcoIdacCw2448R {
        LoVcoIdacCw2448R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2448(&self) -> LoVcoFreqCw2448R {
        LoVcoFreqCw2448R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2444(&mut self) -> LoVcoIdacCw2444W<'_, LoCalCtrlHw6Spec> {
        LoVcoIdacCw2444W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2444(&mut self) -> LoVcoFreqCw2444W<'_, LoCalCtrlHw6Spec> {
        LoVcoFreqCw2444W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2448(&mut self) -> LoVcoIdacCw2448W<'_, LoCalCtrlHw6Spec> {
        LoVcoIdacCw2448W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2448(&mut self) -> LoVcoFreqCw2448W<'_, LoCalCtrlHw6Spec> {
        LoVcoFreqCw2448W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw6.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw6Spec;
impl crate::RegisterSpec for LoCalCtrlHw6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw6::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw6Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw6::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw6 to value 0"]
impl crate::Resettable for LoCalCtrlHw6Spec {}
