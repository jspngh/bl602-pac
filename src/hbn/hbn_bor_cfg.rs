#[doc = "Register `HBN_BOR_CFG` reader"]
pub type R = crate::R<HbnBorCfgSpec>;
#[doc = "Register `HBN_BOR_CFG` writer"]
pub type W = crate::W<HbnBorCfgSpec>;
#[doc = "Field `bor_sel` reader - "]
pub type BorSelR = crate::BitReader;
#[doc = "Field `bor_sel` writer - "]
pub type BorSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bor_vth` reader - "]
pub type BorVthR = crate::BitReader;
#[doc = "Field `bor_vth` writer - "]
pub type BorVthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_bor` reader - "]
pub type PuBorR = crate::BitReader;
#[doc = "Field `pu_bor` writer - "]
pub type PuBorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_bor_out` reader - "]
pub type RBorOutR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bor_sel(&self) -> BorSelR {
        BorSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor_vth(&self) -> BorVthR {
        BorVthR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_bor(&self) -> PuBorR {
        PuBorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_bor_out(&self) -> RBorOutR {
        RBorOutR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bor_sel(&mut self) -> BorSelW<'_, HbnBorCfgSpec> {
        BorSelW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor_vth(&mut self) -> BorVthW<'_, HbnBorCfgSpec> {
        BorVthW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_bor(&mut self) -> PuBorW<'_, HbnBorCfgSpec> {
        PuBorW::new(self, 2)
    }
}
#[doc = "HBN_BOR_CFG.\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_bor_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_bor_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnBorCfgSpec;
impl crate::RegisterSpec for HbnBorCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_bor_cfg::R`](R) reader structure"]
impl crate::Readable for HbnBorCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_bor_cfg::W`](W) writer structure"]
impl crate::Writable for HbnBorCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_BOR_CFG to value 0x02"]
impl crate::Resettable for HbnBorCfgSpec {
    const RESET_VALUE: u32 = 0x02;
}
