#[doc = "Register `rf_data_temp_2` reader"]
pub type R = crate::R<RfDataTemp2Spec>;
#[doc = "Register `rf_data_temp_2` writer"]
pub type W = crate::W<RfDataTemp2Spec>;
#[doc = "Field `rf_data_temp_2` reader - "]
pub type RfDataTemp2R = crate::FieldReader<u32>;
#[doc = "Field `rf_data_temp_2` writer - "]
pub type RfDataTemp2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_2(&self) -> RfDataTemp2R {
        RfDataTemp2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_2(&mut self) -> RfDataTemp2W<'_, RfDataTemp2Spec> {
        RfDataTemp2W::new(self, 0)
    }
}
#[doc = "rf_data_temp_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_data_temp_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_data_temp_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfDataTemp2Spec;
impl crate::RegisterSpec for RfDataTemp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_data_temp_2::R`](R) reader structure"]
impl crate::Readable for RfDataTemp2Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_data_temp_2::W`](W) writer structure"]
impl crate::Writable for RfDataTemp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_data_temp_2 to value 0"]
impl crate::Resettable for RfDataTemp2Spec {}
