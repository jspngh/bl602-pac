#[doc = "Register `ef_ana_trim_0` reader"]
pub type R = crate::R<EfAnaTrim0Spec>;
#[doc = "Register `ef_ana_trim_0` writer"]
pub type W = crate::W<EfAnaTrim0Spec>;
#[doc = "Field `ef_ana_trim_0` reader - "]
pub type EfAnaTrim0R = crate::FieldReader<u32>;
#[doc = "Field `ef_ana_trim_0` writer - "]
pub type EfAnaTrim0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_ana_trim_0(&self) -> EfAnaTrim0R {
        EfAnaTrim0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_ana_trim_0(&mut self) -> EfAnaTrim0W<'_, EfAnaTrim0Spec> {
        EfAnaTrim0W::new(self, 0)
    }
}
#[doc = "ef_ana_trim_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ana_trim_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ana_trim_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfAnaTrim0Spec;
impl crate::RegisterSpec for EfAnaTrim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ana_trim_0::R`](R) reader structure"]
impl crate::Readable for EfAnaTrim0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ana_trim_0::W`](W) writer structure"]
impl crate::Writable for EfAnaTrim0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_ana_trim_0 to value 0"]
impl crate::Resettable for EfAnaTrim0Spec {}
