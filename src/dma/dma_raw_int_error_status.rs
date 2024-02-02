#[doc = "Register `DMA_RawIntErrorStatus` reader"]
pub type R = crate::R<DmaRawIntErrorStatusSpec>;
#[doc = "Register `DMA_RawIntErrorStatus` writer"]
pub type W = crate::W<DmaRawIntErrorStatusSpec>;
#[doc = "Field `RawIntErrorStatus` reader - "]
pub type RawIntErrorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn raw_int_error_status(&self) -> RawIntErrorStatusR {
        RawIntErrorStatusR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "DMA_RawIntErrorStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_raw_int_error_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_raw_int_error_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRawIntErrorStatusSpec;
impl crate::RegisterSpec for DmaRawIntErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_raw_int_error_status::R`](R) reader structure"]
impl crate::Readable for DmaRawIntErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_raw_int_error_status::W`](W) writer structure"]
impl crate::Writable for DmaRawIntErrorStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RawIntErrorStatus to value 0"]
impl crate::Resettable for DmaRawIntErrorStatusSpec {}
