#[doc = "Register `l1c_bmx_err_addr_en` reader"]
pub type R = crate::R<L1cBmxErrAddrEnSpec>;
#[doc = "Register `l1c_bmx_err_addr_en` writer"]
pub type W = crate::W<L1cBmxErrAddrEnSpec>;
#[doc = "Field `l1c_bmx_err_addr_dis` reader - "]
pub type L1cBmxErrAddrDisR = crate::BitReader;
#[doc = "Field `l1c_bmx_err_addr_dis` writer - "]
pub type L1cBmxErrAddrDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l1c_bmx_err_dec` reader - "]
pub type L1cBmxErrDecR = crate::BitReader;
#[doc = "Field `l1c_bmx_err_tz` reader - "]
pub type L1cBmxErrTzR = crate::BitReader;
#[doc = "Field `l1c_hsel_option` reader - "]
pub type L1cHselOptionR = crate::FieldReader;
#[doc = "Field `l1c_hsel_option` writer - "]
pub type L1cHselOptionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr_dis(&self) -> L1cBmxErrAddrDisR {
        L1cBmxErrAddrDisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn l1c_bmx_err_dec(&self) -> L1cBmxErrDecR {
        L1cBmxErrDecR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn l1c_bmx_err_tz(&self) -> L1cBmxErrTzR {
        L1cBmxErrTzR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn l1c_hsel_option(&self) -> L1cHselOptionR {
        L1cHselOptionR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr_dis(&mut self) -> L1cBmxErrAddrDisW<'_, L1cBmxErrAddrEnSpec> {
        L1cBmxErrAddrDisW::new(self, 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn l1c_hsel_option(&mut self) -> L1cHselOptionW<'_, L1cBmxErrAddrEnSpec> {
        L1cHselOptionW::new(self, 16)
    }
}
#[doc = "l1c_bmx_err_addr_en.\n\nYou can [`read`](crate::Reg::read) this register and get [`l1c_bmx_err_addr_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1c_bmx_err_addr_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1cBmxErrAddrEnSpec;
impl crate::RegisterSpec for L1cBmxErrAddrEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1c_bmx_err_addr_en::R`](R) reader structure"]
impl crate::Readable for L1cBmxErrAddrEnSpec {}
#[doc = "`write(|w| ..)` method takes [`l1c_bmx_err_addr_en::W`](W) writer structure"]
impl crate::Writable for L1cBmxErrAddrEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets l1c_bmx_err_addr_en to value 0"]
impl crate::Resettable for L1cBmxErrAddrEnSpec {}
