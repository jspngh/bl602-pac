#[doc = "Register `rfif_test_read` reader"]
pub type R = crate::R<RfifTestReadSpec>;
#[doc = "Register `rfif_test_read` writer"]
pub type W = crate::W<RfifTestReadSpec>;
#[doc = "Field `test_read` reader - "]
pub type TestReadR = crate::FieldReader<u32>;
#[doc = "Field `test_read` writer - "]
pub type TestReadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn test_read(&self) -> TestReadR {
        TestReadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn test_read(&mut self) -> TestReadW<'_, RfifTestReadSpec> {
        TestReadW::new(self, 0)
    }
}
#[doc = "rfif_test_read.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfif_test_read::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfif_test_read::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifTestReadSpec;
impl crate::RegisterSpec for RfifTestReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfif_test_read::R`](R) reader structure"]
impl crate::Readable for RfifTestReadSpec {}
#[doc = "`write(|w| ..)` method takes [`rfif_test_read::W`](W) writer structure"]
impl crate::Writable for RfifTestReadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rfif_test_read to value 0"]
impl crate::Resettable for RfifTestReadSpec {}
