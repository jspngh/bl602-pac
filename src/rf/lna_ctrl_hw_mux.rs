#[doc = "Register `lna_ctrl_hw_mux` reader"]
pub type R = crate::R<LnaCtrlHwMuxSpec>;
#[doc = "Register `lna_ctrl_hw_mux` writer"]
pub type W = crate::W<LnaCtrlHwMuxSpec>;
#[doc = "Field `lna_bm_hg` reader - "]
pub type LnaBmHgR = crate::FieldReader;
#[doc = "Field `lna_bm_hg` writer - "]
pub type LnaBmHgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lna_bm_lg` reader - "]
pub type LnaBmLgR = crate::FieldReader;
#[doc = "Field `lna_bm_lg` writer - "]
pub type LnaBmLgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lna_load_csw_hg` reader - "]
pub type LnaLoadCswHgR = crate::FieldReader;
#[doc = "Field `lna_load_csw_hg` writer - "]
pub type LnaLoadCswHgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lna_load_csw_lg` reader - "]
pub type LnaLoadCswLgR = crate::FieldReader;
#[doc = "Field `lna_load_csw_lg` writer - "]
pub type LnaLoadCswLgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm_hg(&self) -> LnaBmHgR {
        LnaBmHgR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_lg(&self) -> LnaBmLgR {
        LnaBmLgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw_hg(&self) -> LnaLoadCswHgR {
        LnaLoadCswHgR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_lg(&self) -> LnaLoadCswLgR {
        LnaLoadCswLgR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm_hg(&mut self) -> LnaBmHgW<'_, LnaCtrlHwMuxSpec> {
        LnaBmHgW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_lg(&mut self) -> LnaBmLgW<'_, LnaCtrlHwMuxSpec> {
        LnaBmLgW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw_hg(&mut self) -> LnaLoadCswHgW<'_, LnaCtrlHwMuxSpec> {
        LnaLoadCswHgW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_lg(&mut self) -> LnaLoadCswLgW<'_, LnaCtrlHwMuxSpec> {
        LnaLoadCswLgW::new(self, 12)
    }
}
#[doc = "lna_ctrl_hw_mux.\n\nYou can [`read`](crate::Reg::read) this register and get [`lna_ctrl_hw_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lna_ctrl_hw_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LnaCtrlHwMuxSpec;
impl crate::RegisterSpec for LnaCtrlHwMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lna_ctrl_hw_mux::R`](R) reader structure"]
impl crate::Readable for LnaCtrlHwMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`lna_ctrl_hw_mux::W`](W) writer structure"]
impl crate::Writable for LnaCtrlHwMuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lna_ctrl_hw_mux to value 0"]
impl crate::Resettable for LnaCtrlHwMuxSpec {}
