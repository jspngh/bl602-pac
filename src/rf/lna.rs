#[doc = "Register `lna` reader"]
pub type R = crate::R<LnaSpec>;
#[doc = "Register `lna` writer"]
pub type W = crate::W<LnaSpec>;
#[doc = "Field `lna_bm` reader - "]
pub type LnaBmR = crate::FieldReader;
#[doc = "Field `lna_bm` writer - "]
pub type LnaBmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lna_bm_hw` reader - "]
pub type LnaBmHwR = crate::FieldReader;
#[doc = "Field `lna_bm_hw` writer - "]
pub type LnaBmHwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lna_load_csw` reader - "]
pub type LnaLoadCswR = crate::FieldReader;
#[doc = "Field `lna_load_csw` writer - "]
pub type LnaLoadCswW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lna_load_csw_hw` reader - "]
pub type LnaLoadCswHwR = crate::FieldReader;
#[doc = "Field `lna_load_csw_hw` writer - "]
pub type LnaLoadCswHwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lna_rfb_match` reader - "]
pub type LnaRfbMatchR = crate::FieldReader;
#[doc = "Field `lna_rfb_match` writer - "]
pub type LnaRfbMatchW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `lna_cap_lg` reader - "]
pub type LnaCapLgR = crate::FieldReader;
#[doc = "Field `lna_cap_lg` writer - "]
pub type LnaCapLgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lna_lg_gsel` reader - "]
pub type LnaLgGselR = crate::FieldReader;
#[doc = "Field `lna_lg_gsel` writer - "]
pub type LnaLgGselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm(&self) -> LnaBmR {
        LnaBmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_hw(&self) -> LnaBmHwR {
        LnaBmHwR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&self) -> LnaLoadCswR {
        LnaLoadCswR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_hw(&self) -> LnaLoadCswHwR {
        LnaLoadCswHwR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn lna_rfb_match(&self) -> LnaRfbMatchR {
        LnaRfbMatchR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lna_cap_lg(&self) -> LnaCapLgR {
        LnaCapLgR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn lna_lg_gsel(&self) -> LnaLgGselR {
        LnaLgGselR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lna_bm(&mut self) -> LnaBmW<'_, LnaSpec> {
        LnaBmW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lna_bm_hw(&mut self) -> LnaBmHwW<'_, LnaSpec> {
        LnaBmHwW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&mut self) -> LnaLoadCswW<'_, LnaSpec> {
        LnaLoadCswW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn lna_load_csw_hw(&mut self) -> LnaLoadCswHwW<'_, LnaSpec> {
        LnaLoadCswHwW::new(self, 12)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn lna_rfb_match(&mut self) -> LnaRfbMatchW<'_, LnaSpec> {
        LnaRfbMatchW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lna_cap_lg(&mut self) -> LnaCapLgW<'_, LnaSpec> {
        LnaCapLgW::new(self, 20)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn lna_lg_gsel(&mut self) -> LnaLgGselW<'_, LnaSpec> {
        LnaLgGselW::new(self, 24)
    }
}
#[doc = "lna.\n\nYou can [`read`](crate::Reg::read) this register and get [`lna::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lna::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LnaSpec;
impl crate::RegisterSpec for LnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lna::R`](R) reader structure"]
impl crate::Readable for LnaSpec {}
#[doc = "`write(|w| ..)` method takes [`lna::W`](W) writer structure"]
impl crate::Writable for LnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lna to value 0"]
impl crate::Resettable for LnaSpec {}
