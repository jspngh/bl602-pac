#[doc = "Register `lo_cal_ctrl_hw4` reader"]
pub type R = crate::R<LoCalCtrlHw4Spec>;
#[doc = "Register `lo_cal_ctrl_hw4` writer"]
pub type W = crate::W<LoCalCtrlHw4Spec>;
#[doc = "Field `lo_vco_idac_cw_2428` reader - "]
pub type LoVcoIdacCw2428R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2428` writer - "]
pub type LoVcoIdacCw2428W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2428` reader - "]
pub type LoVcoFreqCw2428R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2428` writer - "]
pub type LoVcoFreqCw2428W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2432` reader - "]
pub type LoVcoIdacCw2432R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2432` writer - "]
pub type LoVcoIdacCw2432W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2432` reader - "]
pub type LoVcoFreqCw2432R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2432` writer - "]
pub type LoVcoFreqCw2432W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2428(&self) -> LoVcoIdacCw2428R {
        LoVcoIdacCw2428R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2428(&self) -> LoVcoFreqCw2428R {
        LoVcoFreqCw2428R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2432(&self) -> LoVcoIdacCw2432R {
        LoVcoIdacCw2432R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2432(&self) -> LoVcoFreqCw2432R {
        LoVcoFreqCw2432R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2428(&mut self) -> LoVcoIdacCw2428W<'_, LoCalCtrlHw4Spec> {
        LoVcoIdacCw2428W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2428(&mut self) -> LoVcoFreqCw2428W<'_, LoCalCtrlHw4Spec> {
        LoVcoFreqCw2428W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2432(&mut self) -> LoVcoIdacCw2432W<'_, LoCalCtrlHw4Spec> {
        LoVcoIdacCw2432W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2432(&mut self) -> LoVcoFreqCw2432W<'_, LoCalCtrlHw4Spec> {
        LoVcoFreqCw2432W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw4Spec;
impl crate::RegisterSpec for LoCalCtrlHw4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw4::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw4Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw4::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw4 to value 0"]
impl crate::Resettable for LoCalCtrlHw4Spec {}
