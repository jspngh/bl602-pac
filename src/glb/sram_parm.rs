#[doc = "Register `sram_parm` reader"]
pub type R = crate::R<SramParmSpec>;
#[doc = "Register `sram_parm` writer"]
pub type W = crate::W<SramParmSpec>;
#[doc = "Field `reg_sram_parm` reader - "]
pub type RegSramParmR = crate::FieldReader<u32>;
#[doc = "Field `reg_sram_parm` writer - "]
pub type RegSramParmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_sram_parm(&self) -> RegSramParmR {
        RegSramParmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_sram_parm(&mut self) -> RegSramParmW<'_, SramParmSpec> {
        RegSramParmW::new(self, 0)
    }
}
#[doc = "sram_parm.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_parm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_parm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramParmSpec;
impl crate::RegisterSpec for SramParmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_parm::R`](R) reader structure"]
impl crate::Readable for SramParmSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_parm::W`](W) writer structure"]
impl crate::Writable for SramParmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sram_parm to value 0x0c0c_0c0c"]
impl crate::Resettable for SramParmSpec {
    const RESET_VALUE: u32 = 0x0c0c_0c0c;
}
