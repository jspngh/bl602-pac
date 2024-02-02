#[doc = "Register `rf_ical_ctrl0` reader"]
pub type R = crate::R<RfIcalCtrl0Spec>;
#[doc = "Register `rf_ical_ctrl0` writer"]
pub type W = crate::W<RfIcalCtrl0Spec>;
#[doc = "Field `rf_ical_r_cnt_n` reader - "]
pub type RfIcalRCntNR = crate::FieldReader<u16>;
#[doc = "Field `rf_ical_r_cnt_n` writer - "]
pub type RfIcalRCntNW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rf_ical_a_cnt_n` reader - "]
pub type RfIcalACntNR = crate::FieldReader<u16>;
#[doc = "Field `rf_ical_a_cnt_n` writer - "]
pub type RfIcalACntNW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rf_ical_f_cnt_n` reader - "]
pub type RfIcalFCntNR = crate::FieldReader<u16>;
#[doc = "Field `rf_ical_f_cnt_n` writer - "]
pub type RfIcalFCntNW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rf_ical_a_ud_inv_en` reader - "]
pub type RfIcalAUdInvEnR = crate::BitReader;
#[doc = "Field `rf_ical_a_ud_inv_en` writer - "]
pub type RfIcalAUdInvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rf_ical_f_ud_inv_en` reader - "]
pub type RfIcalFUdInvEnR = crate::BitReader;
#[doc = "Field `rf_ical_f_ud_inv_en` writer - "]
pub type RfIcalFUdInvEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rf_ical_r_cnt_n(&self) -> RfIcalRCntNR {
        RfIcalRCntNR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_a_cnt_n(&self) -> RfIcalACntNR {
        RfIcalACntNR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_f_cnt_n(&self) -> RfIcalFCntNR {
        RfIcalFCntNR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_ical_a_ud_inv_en(&self) -> RfIcalAUdInvEnR {
        RfIcalAUdInvEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_ical_f_ud_inv_en(&self) -> RfIcalFUdInvEnR {
        RfIcalFUdInvEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rf_ical_r_cnt_n(&mut self) -> RfIcalRCntNW<'_, RfIcalCtrl0Spec> {
        RfIcalRCntNW::new(self, 0)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_ical_a_cnt_n(&mut self) -> RfIcalACntNW<'_, RfIcalCtrl0Spec> {
        RfIcalACntNW::new(self, 10)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_ical_f_cnt_n(&mut self) -> RfIcalFCntNW<'_, RfIcalCtrl0Spec> {
        RfIcalFCntNW::new(self, 20)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_ical_a_ud_inv_en(&mut self) -> RfIcalAUdInvEnW<'_, RfIcalCtrl0Spec> {
        RfIcalAUdInvEnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_ical_f_ud_inv_en(&mut self) -> RfIcalFUdInvEnW<'_, RfIcalCtrl0Spec> {
        RfIcalFUdInvEnW::new(self, 31)
    }
}
#[doc = "rf_ical_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_ical_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_ical_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfIcalCtrl0Spec;
impl crate::RegisterSpec for RfIcalCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_ical_ctrl0::R`](R) reader structure"]
impl crate::Readable for RfIcalCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_ical_ctrl0::W`](W) writer structure"]
impl crate::Writable for RfIcalCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_ical_ctrl0 to value 0"]
impl crate::Resettable for RfIcalCtrl0Spec {}
