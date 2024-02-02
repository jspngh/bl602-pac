#[doc = "Register `rsv2` reader"]
pub type R = crate::R<Rsv2Spec>;
#[doc = "Register `rsv2` writer"]
pub type W = crate::W<Rsv2Spec>;
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
    pub fn rsvd_31_0(&mut self) -> Rsvd31_0W<'_, Rsv2Spec> {
        Rsvd31_0W::new(self, 0)
    }
}
#[doc = "rsv2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rsv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsv2Spec;
impl crate::RegisterSpec for Rsv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsv2::R`](R) reader structure"]
impl crate::Readable for Rsv2Spec {}
#[doc = "`write(|w| ..)` method takes [`rsv2::W`](W) writer structure"]
impl crate::Writable for Rsv2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rsv2 to value 0"]
impl crate::Resettable for Rsv2Spec {}
