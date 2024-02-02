#[doc = "Register `sram_slp` reader"]
pub type R = crate::R<SramSlpSpec>;
#[doc = "Register `sram_slp` writer"]
pub type W = crate::W<SramSlpSpec>;
#[doc = "Field `reg_sram_slp` reader - "]
pub type RegSramSlpR = crate::FieldReader<u32>;
#[doc = "Field `reg_sram_slp` writer - "]
pub type RegSramSlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_sram_slp(&self) -> RegSramSlpR {
        RegSramSlpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_sram_slp(&mut self) -> RegSramSlpW<'_, SramSlpSpec> {
        RegSramSlpW::new(self, 0)
    }
}
#[doc = "sram_slp.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_slp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_slp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramSlpSpec;
impl crate::RegisterSpec for SramSlpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_slp::R`](R) reader structure"]
impl crate::Readable for SramSlpSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_slp::W`](W) writer structure"]
impl crate::Writable for SramSlpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sram_slp to value 0"]
impl crate::Resettable for SramSlpSpec {}
