#[doc = "Register `rmxgm` reader"]
pub type R = crate::R<RmxgmSpec>;
#[doc = "Register `rmxgm` writer"]
pub type W = crate::W<RmxgmSpec>;
#[doc = "Field `rmx_bm` reader - "]
pub type RmxBmR = crate::FieldReader;
#[doc = "Field `rmx_bm` writer - "]
pub type RmxBmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rmxgm_bm` reader - "]
pub type RmxgmBmR = crate::FieldReader;
#[doc = "Field `rmxgm_bm` writer - "]
pub type RmxgmBmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rmxgm_10m_mode_en` reader - "]
pub type Rmxgm10mModeEnR = crate::BitReader;
#[doc = "Field `rmxgm_10m_mode_en` writer - "]
pub type Rmxgm10mModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&self) -> RmxBmR {
        RmxBmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rmxgm_bm(&self) -> RmxgmBmR {
        RmxgmBmR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmxgm_10m_mode_en(&self) -> Rmxgm10mModeEnR {
        Rmxgm10mModeEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&mut self) -> RmxBmW<'_, RmxgmSpec> {
        RmxBmW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rmxgm_bm(&mut self) -> RmxgmBmW<'_, RmxgmSpec> {
        RmxgmBmW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmxgm_10m_mode_en(&mut self) -> Rmxgm10mModeEnW<'_, RmxgmSpec> {
        Rmxgm10mModeEnW::new(self, 8)
    }
}
#[doc = "rmxgm.\n\nYou can [`read`](crate::Reg::read) this register and get [`rmxgm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmxgm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmxgmSpec;
impl crate::RegisterSpec for RmxgmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmxgm::R`](R) reader structure"]
impl crate::Readable for RmxgmSpec {}
#[doc = "`write(|w| ..)` method takes [`rmxgm::W`](W) writer structure"]
impl crate::Writable for RmxgmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rmxgm to value 0"]
impl crate::Resettable for RmxgmSpec {}
