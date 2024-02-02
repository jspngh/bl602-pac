#[doc = "Register `ef_reserved` reader"]
pub type R = crate::R<EfReservedSpec>;
#[doc = "Register `ef_reserved` writer"]
pub type W = crate::W<EfReservedSpec>;
#[doc = "Field `ef_reserved` reader - "]
pub type EfReservedR = crate::FieldReader<u32>;
#[doc = "Field `ef_reserved` writer - "]
pub type EfReservedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_reserved(&self) -> EfReservedR {
        EfReservedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_reserved(&mut self) -> EfReservedW<'_, EfReservedSpec> {
        EfReservedW::new(self, 0)
    }
}
#[doc = "ef_reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_reserved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_reserved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfReservedSpec;
impl crate::RegisterSpec for EfReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_reserved::R`](R) reader structure"]
impl crate::Readable for EfReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_reserved::W`](W) writer structure"]
impl crate::Writable for EfReservedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_reserved to value 0"]
impl crate::Resettable for EfReservedSpec {}
