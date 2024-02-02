#[doc = "Register `TCVWR2` reader"]
pub type R = crate::R<Tcvwr2Spec>;
#[doc = "Field `tcvwr` reader - "]
pub type TcvwrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvwr(&self) -> TcvwrR {
        TcvwrR::new(self.bits)
    }
}
#[doc = "TCVWR2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvwr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvwr2Spec;
impl crate::RegisterSpec for Tcvwr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvwr2::R`](R) reader structure"]
impl crate::Readable for Tcvwr2Spec {}
#[doc = "`reset()` method sets TCVWR2 to value 0"]
impl crate::Resettable for Tcvwr2Spec {}
