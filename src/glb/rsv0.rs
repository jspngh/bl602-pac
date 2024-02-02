#[doc = "Register `rsv0` reader"]
pub type R = crate::R<Rsv0Spec>;
#[doc = "Register `rsv0` writer"]
pub type W = crate::W<Rsv0Spec>;
#[doc = "Field `rsvd_31_0` reader - "]
pub type Rsvd31_0R = crate::FieldReader<u32>;
#[doc = "Field `rsvd_31_0` writer - "]
pub type Rsvd31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rsvd_31_0(&self) -> Rsvd31_0R {
        Rsvd31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rsvd_31_0(&mut self) -> Rsvd31_0W<'_, Rsv0Spec> {
        Rsvd31_0W::new(self, 0)
    }
}
#[doc = "rsv0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rsv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsv0Spec;
impl crate::RegisterSpec for Rsv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsv0::R`](R) reader structure"]
impl crate::Readable for Rsv0Spec {}
#[doc = "`write(|w| ..)` method takes [`rsv0::W`](W) writer structure"]
impl crate::Writable for Rsv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rsv0 to value 0"]
impl crate::Resettable for Rsv0Spec {}
