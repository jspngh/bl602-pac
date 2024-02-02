#[doc = "Register `rf_data_temp_1` reader"]
pub type R = crate::R<RfDataTemp1Spec>;
#[doc = "Register `rf_data_temp_1` writer"]
pub type W = crate::W<RfDataTemp1Spec>;
#[doc = "Field `rf_data_temp_1` reader - "]
pub type RfDataTemp1R = crate::FieldReader<u32>;
#[doc = "Field `rf_data_temp_1` writer - "]
pub type RfDataTemp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_1(&self) -> RfDataTemp1R {
        RfDataTemp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_1(&mut self) -> RfDataTemp1W<'_, RfDataTemp1Spec> {
        RfDataTemp1W::new(self, 0)
    }
}
#[doc = "rf_data_temp_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_data_temp_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_data_temp_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfDataTemp1Spec;
impl crate::RegisterSpec for RfDataTemp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_data_temp_1::R`](R) reader structure"]
impl crate::Readable for RfDataTemp1Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_data_temp_1::W`](W) writer structure"]
impl crate::Writable for RfDataTemp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_data_temp_1 to value 0"]
impl crate::Resettable for RfDataTemp1Spec {}
