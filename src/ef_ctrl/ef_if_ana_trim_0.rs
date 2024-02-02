#[doc = "Register `ef_if_ana_trim_0` reader"]
pub type R = crate::R<EfIfAnaTrim0Spec>;
#[doc = "Register `ef_if_ana_trim_0` writer"]
pub type W = crate::W<EfIfAnaTrim0Spec>;
#[doc = "Field `ef_if_ana_trim_0` reader - "]
pub type EfIfAnaTrim0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_ana_trim_0(&self) -> EfIfAnaTrim0R {
        EfIfAnaTrim0R::new(self.bits)
    }
}
impl W {}
#[doc = "ef_if_ana_trim_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_if_ana_trim_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_if_ana_trim_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfIfAnaTrim0Spec;
impl crate::RegisterSpec for EfIfAnaTrim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_if_ana_trim_0::R`](R) reader structure"]
impl crate::Readable for EfIfAnaTrim0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_if_ana_trim_0::W`](W) writer structure"]
impl crate::Writable for EfIfAnaTrim0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_if_ana_trim_0 to value 0"]
impl crate::Resettable for EfIfAnaTrim0Spec {}
