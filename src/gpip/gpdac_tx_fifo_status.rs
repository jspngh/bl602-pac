#[doc = "Register `gpdac_tx_fifo_status` reader"]
pub type R = crate::R<GpdacTxFifoStatusSpec>;
#[doc = "Field `tx_fifo_empty` reader - "]
pub type TxFifoEmptyR = crate::BitReader;
#[doc = "Field `tx_fifo_full` reader - "]
pub type TxFifoFullR = crate::BitReader;
#[doc = "Field `tx_cs` reader - "]
pub type TxCsR = crate::FieldReader;
#[doc = "Field `TxFifoRdPtr` reader - "]
pub type TxFifoRdPtrR = crate::FieldReader;
#[doc = "Field `TxFifoWrPtr` reader - "]
pub type TxFifoWrPtrR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TxFifoEmptyR {
        TxFifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TxFifoFullR {
        TxFifoFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_cs(&self) -> TxCsR {
        TxCsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_fifo_rd_ptr(&self) -> TxFifoRdPtrR {
        TxFifoRdPtrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_fifo_wr_ptr(&self) -> TxFifoWrPtrR {
        TxFifoWrPtrR::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "gpdac_tx_fifo_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_tx_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacTxFifoStatusSpec;
impl crate::RegisterSpec for GpdacTxFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_tx_fifo_status::R`](R) reader structure"]
impl crate::Readable for GpdacTxFifoStatusSpec {}
#[doc = "`reset()` method sets gpdac_tx_fifo_status to value 0x40"]
impl crate::Resettable for GpdacTxFifoStatusSpec {
    const RESET_VALUE: u32 = 0x40;
}
