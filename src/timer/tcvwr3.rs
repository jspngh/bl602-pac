#[doc = "Register `TCVWR3` reader"]
pub type R = crate::R<Tcvwr3Spec>;
#[doc = "Field `tcvwr` reader - "]
pub type TcvwrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvwr(&self) -> TcvwrR {
        TcvwrR::new(self.bits)
    }
}
#[doc = "TCVWR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvwr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcvwr3Spec;
impl crate::RegisterSpec for Tcvwr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcvwr3::R`](R) reader structure"]
impl crate::Readable for Tcvwr3Spec {}
#[doc = "`reset()` method sets TCVWR3 to value 0"]
impl crate::Resettable for Tcvwr3Spec {}
