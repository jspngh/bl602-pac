#[doc = "Register `irom1_misr_dataout_0` reader"]
pub type R = crate::R<Irom1MisrDataout0Spec>;
#[doc = "Field `irom1_misr_dataout_0` reader - "]
pub type Irom1MisrDataout0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irom1_misr_dataout_0(&self) -> Irom1MisrDataout0R {
        Irom1MisrDataout0R::new(self.bits)
    }
}
#[doc = "irom1_misr_dataout_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irom1_misr_dataout_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Irom1MisrDataout0Spec;
impl crate::RegisterSpec for Irom1MisrDataout0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irom1_misr_dataout_0::R`](R) reader structure"]
impl crate::Readable for Irom1MisrDataout0Spec {}
#[doc = "`reset()` method sets irom1_misr_dataout_0 to value 0"]
impl crate::Resettable for Irom1MisrDataout0Spec {}
