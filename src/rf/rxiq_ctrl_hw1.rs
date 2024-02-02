#[doc = "Register `rxiq_ctrl_hw1` reader"]
pub type R = crate::R<RxiqCtrlHw1Spec>;
#[doc = "Register `rxiq_ctrl_hw1` writer"]
pub type W = crate::W<RxiqCtrlHw1Spec>;
#[doc = "Field `rx_iq_phase_comp_gc0` reader - "]
pub type RxIqPhaseCompGc0R = crate::FieldReader<u16>;
#[doc = "Field `rx_iq_phase_comp_gc0` writer - "]
pub type RxIqPhaseCompGc0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_iq_gain_comp_gc0` reader - "]
pub type RxIqGainCompGc0R = crate::FieldReader<u16>;
#[doc = "Field `rx_iq_gain_comp_gc0` writer - "]
pub type RxIqGainCompGc0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc0(&self) -> RxIqPhaseCompGc0R {
        RxIqPhaseCompGc0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc0(&self) -> RxIqGainCompGc0R {
        RxIqGainCompGc0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc0(&mut self) -> RxIqPhaseCompGc0W<'_, RxiqCtrlHw1Spec> {
        RxIqPhaseCompGc0W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc0(&mut self) -> RxIqGainCompGc0W<'_, RxiqCtrlHw1Spec> {
        RxIqGainCompGc0W::new(self, 16)
    }
}
#[doc = "rxiq_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxiq_ctrl_hw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxiq_ctrl_hw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxiqCtrlHw1Spec;
impl crate::RegisterSpec for RxiqCtrlHw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxiq_ctrl_hw1::R`](R) reader structure"]
impl crate::Readable for RxiqCtrlHw1Spec {}
#[doc = "`write(|w| ..)` method takes [`rxiq_ctrl_hw1::W`](W) writer structure"]
impl crate::Writable for RxiqCtrlHw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rxiq_ctrl_hw1 to value 0"]
impl crate::Resettable for RxiqCtrlHw1Spec {}
