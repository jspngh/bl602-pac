#[doc = "Register `TIER2` reader"]
pub type R = crate::R<Tier2Spec>;
#[doc = "Register `TIER2` writer"]
pub type W = crate::W<Tier2Spec>;
#[doc = "Field `tier_0` reader - "]
pub type Tier0R = crate::BitReader;
#[doc = "Field `tier_0` writer - "]
pub type Tier0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tier_1` reader - "]
pub type Tier1R = crate::BitReader;
#[doc = "Field `tier_1` writer - "]
pub type Tier1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tier_2` reader - "]
pub type Tier2R = crate::BitReader;
#[doc = "Field `tier_2` writer - "]
pub type Tier2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tier_0(&self) -> Tier0R {
        Tier0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier_1(&self) -> Tier1R {
        Tier1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tier_2(&self) -> Tier2R {
        Tier2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tier_0(&mut self) -> Tier0W<'_, Tier2Spec> {
        Tier0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier_1(&mut self) -> Tier1W<'_, Tier2Spec> {
        Tier1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tier_2(&mut self) -> Tier2W<'_, Tier2Spec> {
        Tier2W::new(self, 2)
    }
}
#[doc = "TIER2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tier2Spec;
impl crate::RegisterSpec for Tier2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tier2::R`](R) reader structure"]
impl crate::Readable for Tier2Spec {}
#[doc = "`write(|w| ..)` method takes [`tier2::W`](W) writer structure"]
impl crate::Writable for Tier2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIER2 to value 0"]
impl crate::Resettable for Tier2Spec {}
