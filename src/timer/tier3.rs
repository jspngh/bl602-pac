#[doc = "Register `TIER3` reader"]
pub type R = crate::R<Tier3Spec>;
#[doc = "Register `TIER3` writer"]
pub type W = crate::W<Tier3Spec>;
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
    pub fn tier_0(&mut self) -> Tier0W<'_, Tier3Spec> {
        Tier0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier_1(&mut self) -> Tier1W<'_, Tier3Spec> {
        Tier1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tier_2(&mut self) -> Tier2W<'_, Tier3Spec> {
        Tier2W::new(self, 2)
    }
}
#[doc = "TIER3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tier3Spec;
impl crate::RegisterSpec for Tier3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tier3::R`](R) reader structure"]
impl crate::Readable for Tier3Spec {}
#[doc = "`write(|w| ..)` method takes [`tier3::W`](W) writer structure"]
impl crate::Writable for Tier3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIER3 to value 0"]
impl crate::Resettable for Tier3Spec {}
