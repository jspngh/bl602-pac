#[doc = "Register `DMA_Top_Config` reader"]
pub type R = crate::R<DmaTopConfigSpec>;
#[doc = "Register `DMA_Top_Config` writer"]
pub type W = crate::W<DmaTopConfigSpec>;
#[doc = "Field `E` reader - "]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - "]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M` reader - "]
pub type MR = crate::BitReader;
#[doc = "Field `M` writer - "]
pub type MW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&mut self) -> EW<'_, DmaTopConfigSpec> {
        EW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, DmaTopConfigSpec> {
        MW::new(self, 1)
    }
}
#[doc = "DMA_Top_Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_top_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_top_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaTopConfigSpec;
impl crate::RegisterSpec for DmaTopConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_top_config::R`](R) reader structure"]
impl crate::Readable for DmaTopConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_top_config::W`](W) writer structure"]
impl crate::Writable for DmaTopConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_Top_Config to value 0"]
impl crate::Resettable for DmaTopConfigSpec {}
