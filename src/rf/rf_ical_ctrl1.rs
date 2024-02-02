#[doc = "Register `rf_ical_ctrl1` reader"]
pub type R = crate::R<RfIcalCtrl1Spec>;
#[doc = "Register `rf_ical_ctrl1` writer"]
pub type W = crate::W<RfIcalCtrl1Spec>;
#[doc = "Field `rf_ical_r_avg_n` reader - "]
pub type RfIcalRAvgNR = crate::FieldReader;
#[doc = "Field `rf_ical_r_avg_n` writer - "]
pub type RfIcalRAvgNW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `rf_ical_r_os_q` reader - "]
pub type RfIcalROsQR = crate::FieldReader<u16>;
#[doc = "Field `rf_ical_r_os_q` writer - "]
pub type RfIcalROsQW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rf_ical_r_os_i` reader - "]
pub type RfIcalROsIR = crate::FieldReader<u16>;
#[doc = "Field `rf_ical_r_os_i` writer - "]
pub type RfIcalROsIW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ical_r_avg_n(&self) -> RfIcalRAvgNR {
        RfIcalRAvgNR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_r_os_q(&self) -> RfIcalROsQR {
        RfIcalROsQR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_r_os_i(&self) -> RfIcalROsIR {
        RfIcalROsIR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ical_r_avg_n(&mut self) -> RfIcalRAvgNW<'_, RfIcalCtrl1Spec> {
        RfIcalRAvgNW::new(self, 0)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_r_os_q(&mut self) -> RfIcalROsQW<'_, RfIcalCtrl1Spec> {
        RfIcalROsQW::new(self, 10)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_r_os_i(&mut self) -> RfIcalROsIW<'_, RfIcalCtrl1Spec> {
        RfIcalROsIW::new(self, 20)
    }
}
#[doc = "rf_ical_ctrl1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_ical_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_ical_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfIcalCtrl1Spec;
impl crate::RegisterSpec for RfIcalCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_ical_ctrl1::R`](R) reader structure"]
impl crate::Readable for RfIcalCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_ical_ctrl1::W`](W) writer structure"]
impl crate::Writable for RfIcalCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_ical_ctrl1 to value 0"]
impl crate::Resettable for RfIcalCtrl1Spec {}
