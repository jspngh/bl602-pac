#[doc = "Register `rf_resv_reg_1` reader"]
pub type R = crate::R<RfResvReg1Spec>;
#[doc = "Register `rf_resv_reg_1` writer"]
pub type W = crate::W<RfResvReg1Spec>;
#[doc = "Field `rf_reserved1` reader - "]
pub type RfReserved1R = crate::FieldReader<u32>;
#[doc = "Field `rf_reserved1` writer - "]
pub type RfReserved1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_reserved1(&self) -> RfReserved1R {
        RfReserved1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_reserved1(&mut self) -> RfReserved1W<'_, RfResvReg1Spec> {
        RfReserved1W::new(self, 0)
    }
}
#[doc = "rf_resv_reg_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_resv_reg_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_resv_reg_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfResvReg1Spec;
impl crate::RegisterSpec for RfResvReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_resv_reg_1::R`](R) reader structure"]
impl crate::Readable for RfResvReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_resv_reg_1::W`](W) writer structure"]
impl crate::Writable for RfResvReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rf_resv_reg_1 to value 0"]
impl crate::Resettable for RfResvReg1Spec {}
