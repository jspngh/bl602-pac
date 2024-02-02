#[doc = "Register `lo_cal_ctrl_hw7` reader"]
pub type R = crate::R<LoCalCtrlHw7Spec>;
#[doc = "Register `lo_cal_ctrl_hw7` writer"]
pub type W = crate::W<LoCalCtrlHw7Spec>;
#[doc = "Field `lo_vco_idac_cw_2452` reader - "]
pub type LoVcoIdacCw2452R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2452` writer - "]
pub type LoVcoIdacCw2452W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2452` reader - "]
pub type LoVcoFreqCw2452R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2452` writer - "]
pub type LoVcoFreqCw2452W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lo_vco_idac_cw_2456` reader - "]
pub type LoVcoIdacCw2456R = crate::FieldReader;
#[doc = "Field `lo_vco_idac_cw_2456` writer - "]
pub type LoVcoIdacCw2456W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lo_vco_freq_cw_2456` reader - "]
pub type LoVcoFreqCw2456R = crate::FieldReader;
#[doc = "Field `lo_vco_freq_cw_2456` writer - "]
pub type LoVcoFreqCw2456W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2452(&self) -> LoVcoIdacCw2452R {
        LoVcoIdacCw2452R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2452(&self) -> LoVcoFreqCw2452R {
        LoVcoFreqCw2452R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2456(&self) -> LoVcoIdacCw2456R {
        LoVcoIdacCw2456R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2456(&self) -> LoVcoFreqCw2456R {
        LoVcoFreqCw2456R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2452(&mut self) -> LoVcoIdacCw2452W<'_, LoCalCtrlHw7Spec> {
        LoVcoIdacCw2452W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2452(&mut self) -> LoVcoFreqCw2452W<'_, LoCalCtrlHw7Spec> {
        LoVcoFreqCw2452W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lo_vco_idac_cw_2456(&mut self) -> LoVcoIdacCw2456W<'_, LoCalCtrlHw7Spec> {
        LoVcoIdacCw2456W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn lo_vco_freq_cw_2456(&mut self) -> LoVcoFreqCw2456W<'_, LoCalCtrlHw7Spec> {
        LoVcoFreqCw2456W::new(self, 24)
    }
}
#[doc = "lo_cal_ctrl_hw7.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoCalCtrlHw7Spec;
impl crate::RegisterSpec for LoCalCtrlHw7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_cal_ctrl_hw7::R`](R) reader structure"]
impl crate::Readable for LoCalCtrlHw7Spec {}
#[doc = "`write(|w| ..)` method takes [`lo_cal_ctrl_hw7::W`](W) writer structure"]
impl crate::Writable for LoCalCtrlHw7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lo_cal_ctrl_hw7 to value 0"]
impl crate::Resettable for LoCalCtrlHw7Spec {}
