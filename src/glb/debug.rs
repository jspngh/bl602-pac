#[doc = "Register `debug` reader"]
pub type R = crate::R<DebugSpec>;
#[doc = "Register `debug` writer"]
pub type W = crate::W<DebugSpec>;
#[doc = "Field `debug_oe` reader - "]
pub type DebugOeR = crate::BitReader;
#[doc = "Field `debug_oe` writer - "]
pub type DebugOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `debug_i` reader - "]
pub type DebugIR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&self) -> DebugOeR {
        DebugOeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&self) -> DebugIR {
        DebugIR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&mut self) -> DebugOeW<'_, DebugSpec> {
        DebugOeW::new(self, 0)
    }
}
#[doc = "debug.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugSpec;
impl crate::RegisterSpec for DebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DebugSpec {}
#[doc = "`write(|w| ..)` method takes [`debug::W`](W) writer structure"]
impl crate::Writable for DebugSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets debug to value 0"]
impl crate::Resettable for DebugSpec {}
