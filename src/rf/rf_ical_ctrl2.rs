#[doc = "Register `rf_ical_ctrl2` reader"]
pub type R = crate::R<RfIcalCtrl2Spec>;
#[doc = "Register `rf_ical_ctrl2` writer"]
pub type W = crate::W<RfIcalCtrl2Spec>;
#[doc = "Field `rf_ical_period_n` reader - "]
pub type RfIcalPeriodNR = crate::FieldReader<u16>;
#[doc = "Field `rf_ical_period_n` writer - "]
pub type RfIcalPeriodNW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_ical_period_n(&self) -> RfIcalPeriodNR {
        RfIcalPeriodNR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_ical_period_n(&mut self) -> RfIcalPeriodNW<'_, RfIcalCtrl2Spec> {
        RfIcalPeriodNW::new(self, 0)
    }
}
#[doc = "rf_ical_ctrl2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_ical_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_ical_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfIcalCtrl2Spec;
impl crate::RegisterSpec for RfIcalCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_ical_ctrl2::R`](R) reader structure"]
impl crate::Readable for RfIcalCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_ical_ctrl2::W`](W) writer structure"]
impl crate::Writable for RfIcalCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_ical_ctrl2 to value 0"]
impl crate::Resettable for RfIcalCtrl2Spec {}
