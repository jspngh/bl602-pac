#[doc = "Register `data_in` reader"]
pub type R = crate::R<DataInSpec>;
#[doc = "Register `data_in` writer"]
pub type W = crate::W<DataInSpec>;
#[doc = "Field `data_in` writer - "]
pub type DataInW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data_in(&mut self) -> DataInW<'_, DataInSpec> {
        DataInW::new(self, 0)
    }
}
#[doc = "Checksum data in\n\nYou can [`read`](crate::Reg::read) this register and get [`data_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataInSpec;
impl crate::RegisterSpec for DataInSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_in::R`](R) reader structure"]
impl crate::Readable for DataInSpec {}
#[doc = "`write(|w| ..)` method takes [`data_in::W`](W) writer structure"]
impl crate::Writable for DataInSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets data_in to value 0"]
impl crate::Resettable for DataInSpec {}
