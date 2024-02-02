#[doc = "Register `DMA_IntStatus` reader"]
pub type R = crate::R<DmaIntStatusSpec>;
#[doc = "Register `DMA_IntStatus` writer"]
pub type W = crate::W<DmaIntStatusSpec>;
#[doc = "Field `IntStatus` reader - "]
pub type IntStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "DMA_IntStatus.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntStatusSpec;
impl crate::RegisterSpec for DmaIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_status::R`](R) reader structure"]
impl crate::Readable for DmaIntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_int_status::W`](W) writer structure"]
impl crate::Writable for DmaIntStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_IntStatus to value 0"]
impl crate::Resettable for DmaIntStatusSpec {}
