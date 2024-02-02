#[doc = "Register `DMA_EnbldChns` reader"]
pub type R = crate::R<DmaEnbldChnsSpec>;
#[doc = "Register `DMA_EnbldChns` writer"]
pub type W = crate::W<DmaEnbldChnsSpec>;
#[doc = "Field `EnabledChannels` reader - "]
pub type EnabledChannelsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn enabled_channels(&self) -> EnabledChannelsR {
        EnabledChannelsR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "DMA_EnbldChns.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enbld_chns::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enbld_chns::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEnbldChnsSpec;
impl crate::RegisterSpec for DmaEnbldChnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_enbld_chns::R`](R) reader structure"]
impl crate::Readable for DmaEnbldChnsSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_enbld_chns::W`](W) writer structure"]
impl crate::Writable for DmaEnbldChnsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_EnbldChns to value 0"]
impl crate::Resettable for DmaEnbldChnsSpec {}
