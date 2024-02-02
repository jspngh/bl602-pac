#[doc = "Register `rxiq_ctrl_hw2` reader"]
pub type R = crate::R<RxiqCtrlHw2Spec>;
#[doc = "Register `rxiq_ctrl_hw2` writer"]
pub type W = crate::W<RxiqCtrlHw2Spec>;
#[doc = "Field `rx_iq_phase_comp_gc1` reader - "]
pub type RxIqPhaseCompGc1R = crate::FieldReader<u16>;
#[doc = "Field `rx_iq_phase_comp_gc1` writer - "]
pub type RxIqPhaseCompGc1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_iq_gain_comp_gc1` reader - "]
pub type RxIqGainCompGc1R = crate::FieldReader<u16>;
#[doc = "Field `rx_iq_gain_comp_gc1` writer - "]
pub type RxIqGainCompGc1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc1(&self) -> RxIqPhaseCompGc1R {
        RxIqPhaseCompGc1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc1(&self) -> RxIqGainCompGc1R {
        RxIqGainCompGc1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc1(&mut self) -> RxIqPhaseCompGc1W<'_, RxiqCtrlHw2Spec> {
        RxIqPhaseCompGc1W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc1(&mut self) -> RxIqGainCompGc1W<'_, RxiqCtrlHw2Spec> {
        RxIqGainCompGc1W::new(self, 16)
    }
}
#[doc = "rxiq_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxiq_ctrl_hw2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxiq_ctrl_hw2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxiqCtrlHw2Spec;
impl crate::RegisterSpec for RxiqCtrlHw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxiq_ctrl_hw2::R`](R) reader structure"]
impl crate::Readable for RxiqCtrlHw2Spec {}
#[doc = "`write(|w| ..)` method takes [`rxiq_ctrl_hw2::W`](W) writer structure"]
impl crate::Writable for RxiqCtrlHw2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rxiq_ctrl_hw2 to value 0"]
impl crate::Resettable for RxiqCtrlHw2Spec {}
