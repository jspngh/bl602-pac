#[doc = "Register `sram_ret` reader"]
pub type R = crate::R<SramRetSpec>;
#[doc = "Register `sram_ret` writer"]
pub type W = crate::W<SramRetSpec>;
#[doc = "Field `reg_sram_ret` reader - "]
pub type RegSramRetR = crate::FieldReader<u32>;
#[doc = "Field `reg_sram_ret` writer - "]
pub type RegSramRetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_sram_ret(&self) -> RegSramRetR {
        RegSramRetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_sram_ret(&mut self) -> RegSramRetW<'_, SramRetSpec> {
        RegSramRetW::new(self, 0)
    }
}
#[doc = "sram_ret.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_ret::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_ret::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramRetSpec;
impl crate::RegisterSpec for SramRetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ret::R`](R) reader structure"]
impl crate::Readable for SramRetSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_ret::W`](W) writer structure"]
impl crate::Writable for SramRetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sram_ret to value 0"]
impl crate::Resettable for SramRetSpec {}
