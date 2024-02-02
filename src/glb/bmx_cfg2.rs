#[doc = "Register `bmx_cfg2` reader"]
pub type R = crate::R<BmxCfg2Spec>;
#[doc = "Register `bmx_cfg2` writer"]
pub type W = crate::W<BmxCfg2Spec>;
#[doc = "Field `bmx_err_addr_dis` reader - BMX address monitor disable"]
pub type BmxErrAddrDisR = crate::BitReader;
#[doc = "Field `bmx_err_addr_dis` writer - BMX address monitor disable"]
pub type BmxErrAddrDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bmx_err_dec` reader - "]
pub type BmxErrDecR = crate::BitReader;
#[doc = "Field `bmx_err_tz` reader - "]
pub type BmxErrTzR = crate::BitReader;
#[doc = "Field `bmx_dbg_sel` reader - "]
pub type BmxDbgSelR = crate::FieldReader;
#[doc = "Field `bmx_dbg_sel` writer - "]
pub type BmxDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - BMX address monitor disable"]
    #[inline(always)]
    pub fn bmx_err_addr_dis(&self) -> BmxErrAddrDisR {
        BmxErrAddrDisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bmx_err_dec(&self) -> BmxErrDecR {
        BmxErrDecR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bmx_err_tz(&self) -> BmxErrTzR {
        BmxErrTzR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&self) -> BmxDbgSelR {
        BmxDbgSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BMX address monitor disable"]
    #[inline(always)]
    pub fn bmx_err_addr_dis(&mut self) -> BmxErrAddrDisW<'_, BmxCfg2Spec> {
        BmxErrAddrDisW::new(self, 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&mut self) -> BmxDbgSelW<'_, BmxCfg2Spec> {
        BmxDbgSelW::new(self, 28)
    }
}
#[doc = "bmx_cfg2.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmx_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxCfg2Spec;
impl crate::RegisterSpec for BmxCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg2::R`](R) reader structure"]
impl crate::Readable for BmxCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg2::W`](W) writer structure"]
impl crate::Writable for BmxCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets bmx_cfg2 to value 0"]
impl crate::Resettable for BmxCfg2Spec {}
