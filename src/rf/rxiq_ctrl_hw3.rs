#[doc = "Register `rxiq_ctrl_hw3` reader"]
pub type R = crate::R<RxiqCtrlHw3Spec>;
#[doc = "Register `rxiq_ctrl_hw3` writer"]
pub type W = crate::W<RxiqCtrlHw3Spec>;
#[doc = "Field `rx_iq_phase_comp_gc2` reader - "]
pub type RxIqPhaseCompGc2R = crate::FieldReader<u16>;
#[doc = "Field `rx_iq_phase_comp_gc2` writer - "]
pub type RxIqPhaseCompGc2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_iq_gain_comp_gc2` reader - "]
pub type RxIqGainCompGc2R = crate::FieldReader<u16>;
#[doc = "Field `rx_iq_gain_comp_gc2` writer - "]
pub type RxIqGainCompGc2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc2(&self) -> RxIqPhaseCompGc2R {
        RxIqPhaseCompGc2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc2(&self) -> RxIqGainCompGc2R {
        RxIqGainCompGc2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc2(&mut self) -> RxIqPhaseCompGc2W<'_, RxiqCtrlHw3Spec> {
        RxIqPhaseCompGc2W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc2(&mut self) -> RxIqGainCompGc2W<'_, RxiqCtrlHw3Spec> {
        RxIqGainCompGc2W::new(self, 16)
    }
}
#[doc = "rxiq_ctrl_hw3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxiq_ctrl_hw3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxiq_ctrl_hw3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxiqCtrlHw3Spec;
impl crate::RegisterSpec for RxiqCtrlHw3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxiq_ctrl_hw3::R`](R) reader structure"]
impl crate::Readable for RxiqCtrlHw3Spec {}
#[doc = "`write(|w| ..)` method takes [`rxiq_ctrl_hw3::W`](W) writer structure"]
impl crate::Writable for RxiqCtrlHw3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rxiq_ctrl_hw3 to value 0"]
impl crate::Resettable for RxiqCtrlHw3Spec {}
