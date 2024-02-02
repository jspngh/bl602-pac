#[doc = "Register `cci_ctl` reader"]
pub type R = crate::R<CciCtlSpec>;
#[doc = "Register `cci_ctl` writer"]
pub type W = crate::W<CciCtlSpec>;
#[doc = "Field `cci_write_flag` reader - "]
pub type CciWriteFlagR = crate::BitReader;
#[doc = "Field `cci_read_flag` reader - "]
pub type CciReadFlagR = crate::BitReader;
#[doc = "Field `ahb_state` reader - "]
pub type AhbStateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_write_flag(&self) -> CciWriteFlagR {
        CciWriteFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_read_flag(&self) -> CciReadFlagR {
        CciReadFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ahb_state(&self) -> AhbStateR {
        AhbStateR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {}
#[doc = "cci_ctl.\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciCtlSpec;
impl crate::RegisterSpec for CciCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_ctl::R`](R) reader structure"]
impl crate::Readable for CciCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cci_ctl::W`](W) writer structure"]
impl crate::Writable for CciCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cci_ctl to value 0"]
impl crate::Resettable for CciCtlSpec {}
