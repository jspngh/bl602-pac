#[doc = "Register `tmx` reader"]
pub type R = crate::R<TmxSpec>;
#[doc = "Register `tmx` writer"]
pub type W = crate::W<TmxSpec>;
#[doc = "Field `tmx_cs` reader - "]
pub type TmxCsR = crate::FieldReader;
#[doc = "Field `tmx_cs` writer - "]
pub type TmxCsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tmx_bm_sw` reader - "]
pub type TmxBmSwR = crate::FieldReader;
#[doc = "Field `tmx_bm_sw` writer - "]
pub type TmxBmSwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tmx_bm_cas` reader - "]
pub type TmxBmCasR = crate::FieldReader;
#[doc = "Field `tmx_bm_cas` writer - "]
pub type TmxBmCasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tmx_bm_cas_bulk` reader - "]
pub type TmxBmCasBulkR = crate::FieldReader;
#[doc = "Field `tmx_bm_cas_bulk` writer - "]
pub type TmxBmCasBulkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tx_tsense_en` reader - "]
pub type TxTsenseEnR = crate::BitReader;
#[doc = "Field `tx_tsense_en` writer - "]
pub type TxTsenseEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmx_cs(&self) -> TmxCsR {
        TmxCsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmx_bm_sw(&self) -> TmxBmSwR {
        TmxBmSwR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tmx_bm_cas(&self) -> TmxBmCasR {
        TmxBmCasR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn tmx_bm_cas_bulk(&self) -> TmxBmCasBulkR {
        TmxBmCasBulkR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_tsense_en(&self) -> TxTsenseEnR {
        TxTsenseEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmx_cs(&mut self) -> TmxCsW<'_, TmxSpec> {
        TmxCsW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmx_bm_sw(&mut self) -> TmxBmSwW<'_, TmxSpec> {
        TmxBmSwW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn tmx_bm_cas(&mut self) -> TmxBmCasW<'_, TmxSpec> {
        TmxBmCasW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn tmx_bm_cas_bulk(&mut self) -> TmxBmCasBulkW<'_, TmxSpec> {
        TmxBmCasBulkW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_tsense_en(&mut self) -> TxTsenseEnW<'_, TmxSpec> {
        TxTsenseEnW::new(self, 16)
    }
}
#[doc = "tmx.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmxSpec;
impl crate::RegisterSpec for TmxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmx::R`](R) reader structure"]
impl crate::Readable for TmxSpec {}
#[doc = "`write(|w| ..)` method takes [`tmx::W`](W) writer structure"]
impl crate::Writable for TmxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tmx to value 0"]
impl crate::Resettable for TmxSpec {}
