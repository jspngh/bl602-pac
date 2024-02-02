#[doc = "Register `rf_data_temp_0` reader"]
pub type R = crate::R<RfDataTemp0Spec>;
#[doc = "Register `rf_data_temp_0` writer"]
pub type W = crate::W<RfDataTemp0Spec>;
#[doc = "Field `rf_data_temp_0` reader - "]
pub type RfDataTemp0R = crate::FieldReader<u32>;
#[doc = "Field `rf_data_temp_0` writer - "]
pub type RfDataTemp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_0(&self) -> RfDataTemp0R {
        RfDataTemp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_0(&mut self) -> RfDataTemp0W<'_, RfDataTemp0Spec> {
        RfDataTemp0W::new(self, 0)
    }
}
#[doc = "rf_data_temp_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_data_temp_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_data_temp_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfDataTemp0Spec;
impl crate::RegisterSpec for RfDataTemp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_data_temp_0::R`](R) reader structure"]
impl crate::Readable for RfDataTemp0Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_data_temp_0::W`](W) writer structure"]
impl crate::Writable for RfDataTemp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_data_temp_0 to value 0"]
impl crate::Resettable for RfDataTemp0Spec {}
