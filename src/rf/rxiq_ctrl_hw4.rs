#[doc = "Register `rxiq_ctrl_hw4` reader"]
pub type R = crate::R<RxiqCtrlHw4Spec>;
#[doc = "Register `rxiq_ctrl_hw4` writer"]
pub type W = crate::W<RxiqCtrlHw4Spec>;
#[doc = "Field `rx_iq_phase_comp_gc3` reader - "]
pub type RxIqPhaseCompGc3R = crate::FieldReader<u16>;
#[doc = "Field `rx_iq_phase_comp_gc3` writer - "]
pub type RxIqPhaseCompGc3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rx_iq_gain_comp_gc3` reader - "]
pub type RxIqGainCompGc3R = crate::FieldReader<u16>;
#[doc = "Field `rx_iq_gain_comp_gc3` writer - "]
pub type RxIqGainCompGc3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc3(&self) -> RxIqPhaseCompGc3R {
        RxIqPhaseCompGc3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc3(&self) -> RxIqGainCompGc3R {
        RxIqGainCompGc3R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_iq_phase_comp_gc3(&mut self) -> RxIqPhaseCompGc3W<'_, RxiqCtrlHw4Spec> {
        RxIqPhaseCompGc3W::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn rx_iq_gain_comp_gc3(&mut self) -> RxIqGainCompGc3W<'_, RxiqCtrlHw4Spec> {
        RxIqGainCompGc3W::new(self, 16)
    }
}
#[doc = "rxiq_ctrl_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxiq_ctrl_hw4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxiq_ctrl_hw4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxiqCtrlHw4Spec;
impl crate::RegisterSpec for RxiqCtrlHw4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxiq_ctrl_hw4::R`](R) reader structure"]
impl crate::Readable for RxiqCtrlHw4Spec {}
#[doc = "`write(|w| ..)` method takes [`rxiq_ctrl_hw4::W`](W) writer structure"]
impl crate::Writable for RxiqCtrlHw4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rxiq_ctrl_hw4 to value 0"]
impl crate::Resettable for RxiqCtrlHw4Spec {}
