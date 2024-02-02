#[doc = "Register `lodist` reader"]
pub type R = crate::R<LodistSpec>;
#[doc = "Register `lodist` writer"]
pub type W = crate::W<LodistSpec>;
#[doc = "Field `lo_osmx_xgm_boost` reader - "]
pub type LoOsmxXgmBoostR = crate::BitReader;
#[doc = "Field `lo_osmx_xgm_boost` writer - "]
pub type LoOsmxXgmBoostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_osmx_en_xgm` reader - "]
pub type LoOsmxEnXgmR = crate::BitReader;
#[doc = "Field `lo_osmx_en_xgm` writer - "]
pub type LoOsmxEnXgmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_osmx_fix_cap` reader - "]
pub type LoOsmxFixCapR = crate::BitReader;
#[doc = "Field `lo_osmx_fix_cap` writer - "]
pub type LoOsmxFixCapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_osmx_vbuf_stre` reader - "]
pub type LoOsmxVbufStreR = crate::BitReader;
#[doc = "Field `lo_osmx_vbuf_stre` writer - "]
pub type LoOsmxVbufStreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_osmx_capbank_bias` reader - "]
pub type LoOsmxCapbankBiasR = crate::FieldReader;
#[doc = "Field `lo_osmx_capbank_bias` writer - "]
pub type LoOsmxCapbankBiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_osmx_cap` reader - "]
pub type LoOsmxCapR = crate::FieldReader;
#[doc = "Field `lo_osmx_cap` writer - "]
pub type LoOsmxCapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lo_lodist_txbuf_stre` reader - "]
pub type LoLodistTxbufStreR = crate::BitReader;
#[doc = "Field `lo_lodist_txbuf_stre` writer - "]
pub type LoLodistTxbufStreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_lodist_rxbuf_stre` reader - "]
pub type LoLodistRxbufStreR = crate::BitReader;
#[doc = "Field `lo_lodist_rxbuf_stre` writer - "]
pub type LoLodistRxbufStreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_osmx_xgm_boost(&self) -> LoOsmxXgmBoostR {
        LoOsmxXgmBoostR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_osmx_en_xgm(&self) -> LoOsmxEnXgmR {
        LoOsmxEnXgmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_osmx_fix_cap(&self) -> LoOsmxFixCapR {
        LoOsmxFixCapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_osmx_vbuf_stre(&self) -> LoOsmxVbufStreR {
        LoOsmxVbufStreR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_osmx_capbank_bias(&self) -> LoOsmxCapbankBiasR {
        LoOsmxCapbankBiasR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lo_osmx_cap(&self) -> LoOsmxCapR {
        LoOsmxCapR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_lodist_txbuf_stre(&self) -> LoLodistTxbufStreR {
        LoLodistTxbufStreR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_lodist_rxbuf_stre(&self) -> LoLodistRxbufStreR {
        LoLodistRxbufStreR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_osmx_xgm_boost(&mut self) -> LoOsmxXgmBoostW<'_, LodistSpec> {
        LoOsmxXgmBoostW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_osmx_en_xgm(&mut self) -> LoOsmxEnXgmW<'_, LodistSpec> {
        LoOsmxEnXgmW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_osmx_fix_cap(&mut self) -> LoOsmxFixCapW<'_, LodistSpec> {
        LoOsmxFixCapW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_osmx_vbuf_stre(&mut self) -> LoOsmxVbufStreW<'_, LodistSpec> {
        LoOsmxVbufStreW::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_osmx_capbank_bias(&mut self) -> LoOsmxCapbankBiasW<'_, LodistSpec> {
        LoOsmxCapbankBiasW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lo_osmx_cap(&mut self) -> LoOsmxCapW<'_, LodistSpec> {
        LoOsmxCapW::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_lodist_txbuf_stre(&mut self) -> LoLodistTxbufStreW<'_, LodistSpec> {
        LoLodistTxbufStreW::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_lodist_rxbuf_stre(&mut self) -> LoLodistRxbufStreW<'_, LodistSpec> {
        LoLodistRxbufStreW::new(self, 28)
    }
}
#[doc = "lodist.\n\nYou can [`read`](crate::Reg::read) this register and get [`lodist::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lodist::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LodistSpec;
impl crate::RegisterSpec for LodistSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lodist::R`](R) reader structure"]
impl crate::Readable for LodistSpec {}
#[doc = "`write(|w| ..)` method takes [`lodist::W`](W) writer structure"]
impl crate::Writable for LodistSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets lodist to value 0"]
impl crate::Resettable for LodistSpec {}
