#[doc = "Register `TICR2` writer"]
pub type W = crate::W<Ticr2Spec>;
#[doc = "Field `tclr_0` writer - "]
pub type Tclr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tclr_1` writer - "]
pub type Tclr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tclr_2` writer - "]
pub type Tclr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr_0(&mut self) -> Tclr0W<'_, Ticr2Spec> {
        Tclr0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr_1(&mut self) -> Tclr1W<'_, Ticr2Spec> {
        Tclr1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tclr_2(&mut self) -> Tclr2W<'_, Ticr2Spec> {
        Tclr2W::new(self, 2)
    }
}
#[doc = "TICR2.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ticr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ticr2Spec;
impl crate::RegisterSpec for Ticr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ticr2::W`](W) writer structure"]
impl crate::Writable for Ticr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TICR2 to value 0"]
impl crate::Resettable for Ticr2Spec {}
