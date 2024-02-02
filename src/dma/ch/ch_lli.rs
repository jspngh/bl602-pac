#[doc = "Register `CH_LLI` reader"]
pub type R = crate::R<ChLliSpec>;
#[doc = "Register `CH_LLI` writer"]
pub type W = crate::W<ChLliSpec>;
#[doc = "Field `LLI` reader - "]
pub type LliR = crate::FieldReader<u32>;
#[doc = "Field `LLI` writer - "]
pub type LliW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lli(&self) -> LliR {
        LliR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lli(&mut self) -> LliW<'_, ChLliSpec> {
        LliW::new(self, 0)
    }
}
#[doc = "DMA channel linked list item\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_lli::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_lli::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChLliSpec;
impl crate::RegisterSpec for ChLliSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_lli::R`](R) reader structure"]
impl crate::Readable for ChLliSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_lli::W`](W) writer structure"]
impl crate::Writable for ChLliSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_LLI to value 0"]
impl crate::Resettable for ChLliSpec {}
