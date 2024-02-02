#[doc = "Register `CH_SRC_ADDR` reader"]
pub type R = crate::R<ChSrcAddrSpec>;
#[doc = "Register `CH_SRC_ADDR` writer"]
pub type W = crate::W<ChSrcAddrSpec>;
#[doc = "Field `SrcAddr` reader - "]
pub type SrcAddrR = crate::FieldReader<u32>;
#[doc = "Field `SrcAddr` writer - "]
pub type SrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_addr(&mut self) -> SrcAddrW<'_, ChSrcAddrSpec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "DMA channel source address\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_src_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_src_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChSrcAddrSpec;
impl crate::RegisterSpec for ChSrcAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_src_addr::R`](R) reader structure"]
impl crate::Readable for ChSrcAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_src_addr::W`](W) writer structure"]
impl crate::Writable for ChSrcAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_SRC_ADDR to value 0"]
impl crate::Resettable for ChSrcAddrSpec {}
