#[doc = "Register `rfcal_status2` reader"]
pub type R = crate::R<RfcalStatus2Spec>;
#[doc = "Register `rfcal_status2` writer"]
pub type W = crate::W<RfcalStatus2Spec>;
#[doc = "Field `dl_rfcal_table_status` reader - "]
pub type DlRfcalTableStatusR = crate::FieldReader;
#[doc = "Field `dl_rfcal_table_status` writer - "]
pub type DlRfcalTableStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&self) -> DlRfcalTableStatusR {
        DlRfcalTableStatusR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&mut self) -> DlRfcalTableStatusW<'_, RfcalStatus2Spec> {
        DlRfcalTableStatusW::new(self, 0)
    }
}
#[doc = "rfcal_status2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcal_status2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcal_status2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcalStatus2Spec;
impl crate::RegisterSpec for RfcalStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcal_status2::R`](R) reader structure"]
impl crate::Readable for RfcalStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`rfcal_status2::W`](W) writer structure"]
impl crate::Writable for RfcalStatus2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rfcal_status2 to value 0"]
impl crate::Resettable for RfcalStatus2Spec {}
