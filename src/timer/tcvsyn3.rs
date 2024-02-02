#[doc = "Register `TCVSYN3` reader"]
pub type R = crate::R<Tcvsyn3Spec>;
#[doc = "Field `tcvsyn3` reader - "]
pub type Tcvsyn3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn3(&self) -> Tcvsyn3R {
        Tcvsyn3R::new(self.bits)
    }
}
#[doc = "TCVSYN3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvsyn3Spec;
impl crate::RegisterSpec for Tcvsyn3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvsyn3::R`](R) reader structure"]
impl crate::Readable for Tcvsyn3Spec {}
#[doc = "`reset()` method sets TCVSYN3 to value 0"]
impl crate::Resettable for Tcvsyn3Spec {}
