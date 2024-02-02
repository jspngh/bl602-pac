#[doc = "Register `TCVSYN2` reader"]
pub type R = crate::R<Tcvsyn2Spec>;
#[doc = "Field `tcvsyn2` reader - "]
pub type Tcvsyn2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn2(&self) -> Tcvsyn2R {
        Tcvsyn2R::new(self.bits)
    }
}
#[doc = "TCVSYN2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvsyn2Spec;
impl crate::RegisterSpec for Tcvsyn2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvsyn2::R`](R) reader structure"]
impl crate::Readable for Tcvsyn2Spec {}
#[doc = "`reset()` method sets TCVSYN2 to value 0"]
impl crate::Resettable for Tcvsyn2Spec {}
