#[doc = "Register `rf_data_temp_3` reader"]
pub type R = crate::R<RfDataTemp3Spec>;
#[doc = "Register `rf_data_temp_3` writer"]
pub type W = crate::W<RfDataTemp3Spec>;
#[doc = "Field `rf_data_temp_3` reader - "]
pub type RfDataTemp3R = crate::FieldReader<u32>;
#[doc = "Field `rf_data_temp_3` writer - "]
pub type RfDataTemp3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_3(&self) -> RfDataTemp3R {
        RfDataTemp3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_3(&mut self) -> RfDataTemp3W<'_, RfDataTemp3Spec> {
        RfDataTemp3W::new(self, 0)
    }
}
#[doc = "rf_data_temp_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_data_temp_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_data_temp_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfDataTemp3Spec;
impl crate::RegisterSpec for RfDataTemp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_data_temp_3::R`](R) reader structure"]
impl crate::Readable for RfDataTemp3Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_data_temp_3::W`](W) writer structure"]
impl crate::Writable for RfDataTemp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_data_temp_3 to value 0"]
impl crate::Resettable for RfDataTemp3Spec {}
