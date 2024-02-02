#[doc = "Register `irrx_data_word0` reader"]
pub type R = crate::R<IrrxDataWord0Spec>;
#[doc = "Register `irrx_data_word0` writer"]
pub type W = crate::W<IrrxDataWord0Spec>;
#[doc = "Field `sts_irrx_data_word0` reader - "]
pub type StsIrrxDataWord0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_irrx_data_word0(&self) -> StsIrrxDataWord0R {
        StsIrrxDataWord0R::new(self.bits)
    }
}
impl W {}
#[doc = "irrx_data_word0.\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_word0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_data_word0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxDataWord0Spec;
impl crate::RegisterSpec for IrrxDataWord0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_data_word0::R`](R) reader structure"]
impl crate::Readable for IrrxDataWord0Spec {}
#[doc = "`write(|w| ..)` method takes [`irrx_data_word0::W`](W) writer structure"]
impl crate::Writable for IrrxDataWord0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets irrx_data_word0 to value 0"]
impl crate::Resettable for IrrxDataWord0Spec {}
