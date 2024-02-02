#[doc = "Register `DMA_IntTCClear` reader"]
pub type R = crate::R<DmaIntTcclearSpec>;
#[doc = "Register `DMA_IntTCClear` writer"]
pub type W = crate::W<DmaIntTcclearSpec>;
#[doc = "Field `IntTCClear` writer - "]
pub type IntTcclearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_tcclear(&mut self) -> IntTcclearW<'_, DmaIntTcclearSpec> {
        IntTcclearW::new(self, 0)
    }
}
#[doc = "DMA_IntTCClear.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_tcclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_tcclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntTcclearSpec;
impl crate::RegisterSpec for DmaIntTcclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_tcclear::R`](R) reader structure"]
impl crate::Readable for DmaIntTcclearSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_int_tcclear::W`](W) writer structure"]
impl crate::Writable for DmaIntTcclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_IntTCClear to value 0"]
impl crate::Resettable for DmaIntTcclearSpec {}
