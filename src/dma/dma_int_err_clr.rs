#[doc = "Register `DMA_IntErrClr` reader"]
pub type R = crate::R<DmaIntErrClrSpec>;
#[doc = "Register `DMA_IntErrClr` writer"]
pub type W = crate::W<DmaIntErrClrSpec>;
#[doc = "Field `IntErrClr` writer - "]
pub type IntErrClrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_err_clr(&mut self) -> IntErrClrW<'_, DmaIntErrClrSpec> {
        IntErrClrW::new(self, 0)
    }
}
#[doc = "DMA_IntErrClr.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_err_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_err_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntErrClrSpec;
impl crate::RegisterSpec for DmaIntErrClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_err_clr::R`](R) reader structure"]
impl crate::Readable for DmaIntErrClrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_int_err_clr::W`](W) writer structure"]
impl crate::Writable for DmaIntErrClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_IntErrClr to value 0"]
impl crate::Resettable for DmaIntErrClrSpec {}
