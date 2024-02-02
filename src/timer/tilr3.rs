#[doc = "Register `TILR3` reader"]
pub type R = crate::R<Tilr3Spec>;
#[doc = "Register `TILR3` writer"]
pub type W = crate::W<Tilr3Spec>;
#[doc = "Field `tilr_0` reader - "]
pub type Tilr0R = crate::BitReader;
#[doc = "Field `tilr_0` writer - "]
pub type Tilr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tilr_1` reader - "]
pub type Tilr1R = crate::BitReader;
#[doc = "Field `tilr_1` writer - "]
pub type Tilr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tilr_2` reader - "]
pub type Tilr2R = crate::BitReader;
#[doc = "Field `tilr_2` writer - "]
pub type Tilr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr_0(&self) -> Tilr0R {
        Tilr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr_1(&self) -> Tilr1R {
        Tilr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tilr_2(&self) -> Tilr2R {
        Tilr2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr_0(&mut self) -> Tilr0W<'_, Tilr3Spec> {
        Tilr0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr_1(&mut self) -> Tilr1W<'_, Tilr3Spec> {
        Tilr1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tilr_2(&mut self) -> Tilr2W<'_, Tilr3Spec> {
        Tilr2W::new(self, 2)
    }
}
#[doc = "TILR3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tilr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tilr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tilr3Spec;
impl crate::RegisterSpec for Tilr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tilr3::R`](R) reader structure"]
impl crate::Readable for Tilr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tilr3::W`](W) writer structure"]
impl crate::Writable for Tilr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TILR3 to value 0"]
impl crate::Resettable for Tilr3Spec {}
