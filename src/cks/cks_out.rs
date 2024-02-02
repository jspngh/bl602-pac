#[doc = "Register `cks_out` reader"]
pub type R = crate::R<CksOutSpec>;
#[doc = "Register `cks_out` writer"]
pub type W = crate::W<CksOutSpec>;
#[doc = "Field `cks_out` reader - "]
pub type CksOutR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cks_out(&self) -> CksOutR {
        CksOutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Checksum value out\n\nYou can [`read`](crate::Reg::read) this register and get [`cks_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cks_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CksOutSpec;
impl crate::RegisterSpec for CksOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cks_out::R`](R) reader structure"]
impl crate::Readable for CksOutSpec {}
#[doc = "`write(|w| ..)` method takes [`cks_out::W`](W) writer structure"]
impl crate::Writable for CksOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets cks_out to value 0xffff"]
impl crate::Resettable for CksOutSpec {
    const RESET_VALUE: u32 = 0xffff;
}
