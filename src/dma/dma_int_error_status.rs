#[doc = "Register `DMA_IntErrorStatus` reader"]
pub type R = crate::R<DmaIntErrorStatusSpec>;
#[doc = "Register `DMA_IntErrorStatus` writer"]
pub type W = crate::W<DmaIntErrorStatusSpec>;
#[doc = "Field `IntErrorStatus` reader - "]
pub type IntErrorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_error_status(&self) -> IntErrorStatusR {
        IntErrorStatusR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "DMA_IntErrorStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_error_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_error_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntErrorStatusSpec;
impl crate::RegisterSpec for DmaIntErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_error_status::R`](R) reader structure"]
impl crate::Readable for DmaIntErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_int_error_status::W`](W) writer structure"]
impl crate::Writable for DmaIntErrorStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_IntErrorStatus to value 0"]
impl crate::Resettable for DmaIntErrorStatusSpec {}
