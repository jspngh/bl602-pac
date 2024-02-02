#[doc = "Register `TCR3` reader"]
pub type R = crate::R<Tcr3Spec>;
#[doc = "Field `tcr3_counter` reader - "]
pub type Tcr3CounterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr3_counter(&self) -> Tcr3CounterR {
        Tcr3CounterR::new(self.bits)
    }
}
#[doc = "TCR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr3Spec;
impl crate::RegisterSpec for Tcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr3::R`](R) reader structure"]
impl crate::Readable for Tcr3Spec {}
#[doc = "`reset()` method sets TCR3 to value 0"]
impl crate::Resettable for Tcr3Spec {}
