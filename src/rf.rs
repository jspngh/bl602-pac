#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rf_rev: RfRev,
    rf_fsm_ctrl_hw: RfFsmCtrlHw,
    rf_fsm_ctrl_sw: RfFsmCtrlSw,
    rfctrl_hw_en: RfctrlHwEn,
    temp_comp: TempComp,
    rfcal_status: RfcalStatus,
    rfcal_status2: RfcalStatus2,
    rfcal_ctrlen: RfcalCtrlen,
    rfcal_stateen: RfcalStateen,
    saradc_resv: SaradcResv,
    rf_base_ctrl1: RfBaseCtrl1,
    rf_base_ctrl2: RfBaseCtrl2,
    pucr1: Pucr1,
    pucr1_hw: Pucr1Hw,
    pucr2: Pucr2,
    pucr2_hw: Pucr2Hw,
    ppu_ctrl_hw: PpuCtrlHw,
    pud_ctrl_hw: PudCtrlHw,
    trx_gain1: TrxGain1,
    trx_gain_hw: TrxGainHw,
    ten_dc: TenDc,
    ten_dig: TenDig,
    ten_ac: TenAc,
    pmip_mv2aon: PmipMv2aon,
    cip: Cip,
    pa1: Pa1,
    pa2: Pa2,
    tmx: Tmx,
    tbb: Tbb,
    lna: Lna,
    rmxgm: Rmxgm,
    rbb1: Rbb1,
    rbb2: Rbb2,
    rbb3: Rbb3,
    rbb4: Rbb4,
    adda1: Adda1,
    adda2: Adda2,
    _reserved37: [u8; 0x0c],
    vco1: Vco1,
    vco2: Vco2,
    vco3: Vco3,
    vco4: Vco4,
    pfdcp: Pfdcp,
    lo: Lo,
    fbdv: Fbdv,
    lodist: Lodist,
    sdm1: Sdm1,
    sdm2: Sdm2,
    sdm3: Sdm3,
    _reserved48: [u8; 0x20],
    rf_resv_reg_0: RfResvReg0,
    rf_resv_reg_1: RfResvReg1,
    rf_resv_reg_2: RfResvReg2,
    rrf_gain_index1: RrfGainIndex1,
    rrf_gain_index2: RrfGainIndex2,
    lna_ctrl_hw_mux: LnaCtrlHwMux,
    rbb_gain_index1: RbbGainIndex1,
    rbb_gain_index2: RbbGainIndex2,
    rbb_gain_index3: RbbGainIndex3,
    rbb_gain_index4: RbbGainIndex4,
    rbb_gain_index5: RbbGainIndex5,
    tbb_gain_index1: TbbGainIndex1,
    tbb_gain_index2: TbbGainIndex2,
    tbb_gain_index3: TbbGainIndex3,
    tbb_gain_index4: TbbGainIndex4,
    pa_reg_ctrl_hw1: PaRegCtrlHw1,
    pa_reg_ctrl_hw2: PaRegCtrlHw2,
    pa_reg_wifi_ctrl_hw: PaRegWifiCtrlHw,
    adda_reg_ctrl_hw: AddaRegCtrlHw,
    lo_reg_ctrl_hw1: LoRegCtrlHw1,
    lo_cal_ctrl_hw1: LoCalCtrlHw1,
    lo_cal_ctrl_hw2: LoCalCtrlHw2,
    lo_cal_ctrl_hw3: LoCalCtrlHw3,
    lo_cal_ctrl_hw4: LoCalCtrlHw4,
    lo_cal_ctrl_hw5: LoCalCtrlHw5,
    lo_cal_ctrl_hw6: LoCalCtrlHw6,
    lo_cal_ctrl_hw7: LoCalCtrlHw7,
    lo_cal_ctrl_hw8: LoCalCtrlHw8,
    lo_cal_ctrl_hw9: LoCalCtrlHw9,
    lo_cal_ctrl_hw10: LoCalCtrlHw10,
    lo_cal_ctrl_hw11: LoCalCtrlHw11,
    rosdac_ctrl_hw1: RosdacCtrlHw1,
    rosdac_ctrl_hw2: RosdacCtrlHw2,
    rxiq_ctrl_hw1: RxiqCtrlHw1,
    rxiq_ctrl_hw2: RxiqCtrlHw2,
    rxiq_ctrl_hw3: RxiqCtrlHw3,
    rxiq_ctrl_hw4: RxiqCtrlHw4,
    tosdac_ctrl_hw1: TosdacCtrlHw1,
    tosdac_ctrl_hw2: TosdacCtrlHw2,
    tosdac_ctrl_hw3: TosdacCtrlHw3,
    tosdac_ctrl_hw4: TosdacCtrlHw4,
    tx_iq_gain_hw0: TxIqGainHw0,
    tx_iq_gain_hw1: TxIqGainHw1,
    tx_iq_gain_hw2: TxIqGainHw2,
    tx_iq_gain_hw3: TxIqGainHw3,
    tx_iq_gain_hw4: TxIqGainHw4,
    tx_iq_gain_hw5: TxIqGainHw5,
    tx_iq_gain_hw6: TxIqGainHw6,
    tx_iq_gain_hw7: TxIqGainHw7,
    lo_sdm_ctrl_hw1: LoSdmCtrlHw1,
    lo_sdm_ctrl_hw2: LoSdmCtrlHw2,
    lo_sdm_ctrl_hw3: LoSdmCtrlHw3,
    lo_sdm_ctrl_hw4: LoSdmCtrlHw4,
    lo_sdm_ctrl_hw5: LoSdmCtrlHw5,
    lo_sdm_ctrl_hw6: LoSdmCtrlHw6,
    lo_sdm_ctrl_hw7: LoSdmCtrlHw7,
    lo_sdm_ctrl_hw8: LoSdmCtrlHw8,
    rbb_bw_ctrl_hw: RbbBwCtrlHw,
    _reserved106: [u8; 0x38],
    singen_ctrl0: SingenCtrl0,
    singen_ctrl1: SingenCtrl1,
    singen_ctrl2: SingenCtrl2,
    singen_ctrl3: SingenCtrl3,
    singen_ctrl4: SingenCtrl4,
    rfif_dfe_ctrl0: RfifDfeCtrl0,
    rfif_test_read: RfifTestRead,
    rfif_dig_ctrl: RfifDigCtrl,
    rf_data_temp_0: RfDataTemp0,
    rf_data_temp_1: RfDataTemp1,
    rf_data_temp_2: RfDataTemp2,
    rf_data_temp_3: RfDataTemp3,
    rf_sram_ctrl0: RfSramCtrl0,
    rf_sram_ctrl1: RfSramCtrl1,
    rf_sram_ctrl2: RfSramCtrl2,
    rf_sram_ctrl3: RfSramCtrl3,
    rf_sram_ctrl4: RfSramCtrl4,
    rf_sram_ctrl5: RfSramCtrl5,
    rf_sram_ctrl6: RfSramCtrl6,
    rf_ical_ctrl0: RfIcalCtrl0,
    rf_ical_ctrl1: RfIcalCtrl1,
    rf_ical_ctrl2: RfIcalCtrl2,
    rf_fsm_ctrl0: RfFsmCtrl0,
    rf_fsm_ctrl1: RfFsmCtrl1,
    rf_fsm_ctrl2: RfFsmCtrl2,
    rf_pkdet_ctrl0: RfPkdetCtrl0,
    _reserved132: [u8; 0x038c],
    dfe_ctrl_0: DfeCtrl0,
    dfe_ctrl_1: DfeCtrl1,
    dfe_ctrl_2: DfeCtrl2,
    dfe_ctrl_3: DfeCtrl3,
    dfe_ctrl_4: DfeCtrl4,
    dfe_ctrl_5: DfeCtrl5,
    dfe_ctrl_6: DfeCtrl6,
    dfe_ctrl_7: DfeCtrl7,
    dfe_ctrl_8: DfeCtrl8,
    dfe_ctrl_9: DfeCtrl9,
    dfe_ctrl_10: DfeCtrl10,
    dfe_ctrl_11: DfeCtrl11,
    dfe_ctrl_12: DfeCtrl12,
    dfe_ctrl_13: DfeCtrl13,
    dfe_ctrl_14: DfeCtrl14,
    dfe_ctrl_15: DfeCtrl15,
    dfe_ctrl_16: DfeCtrl16,
    dfe_ctrl_17: DfeCtrl17,
    dfe_ctrl_18: DfeCtrl18,
}
impl RegisterBlock {
    #[doc = "0x00 - Silicon revision"]
    #[inline(always)]
    pub const fn rf_rev(&self) -> &RfRev {
        &self.rf_rev
    }
    #[doc = "0x04 - Digital Control"]
    #[inline(always)]
    pub const fn rf_fsm_ctrl_hw(&self) -> &RfFsmCtrlHw {
        &self.rf_fsm_ctrl_hw
    }
    #[doc = "0x08 - rfsm status reg"]
    #[inline(always)]
    pub const fn rf_fsm_ctrl_sw(&self) -> &RfFsmCtrlSw {
        &self.rf_fsm_ctrl_sw
    }
    #[doc = "0x0c - Control logic switch"]
    #[inline(always)]
    pub const fn rfctrl_hw_en(&self) -> &RfctrlHwEn {
        &self.rfctrl_hw_en
    }
    #[doc = "0x10 - temp_comp."]
    #[inline(always)]
    pub const fn temp_comp(&self) -> &TempComp {
        &self.temp_comp
    }
    #[doc = "0x14 - rfcal_status."]
    #[inline(always)]
    pub const fn rfcal_status(&self) -> &RfcalStatus {
        &self.rfcal_status
    }
    #[doc = "0x18 - rfcal_status2."]
    #[inline(always)]
    pub const fn rfcal_status2(&self) -> &RfcalStatus2 {
        &self.rfcal_status2
    }
    #[doc = "0x1c - Calibration mode register"]
    #[inline(always)]
    pub const fn rfcal_ctrlen(&self) -> &RfcalCtrlen {
        &self.rfcal_ctrlen
    }
    #[doc = "0x20 - rf calibration state enabl in full cal list"]
    #[inline(always)]
    pub const fn rfcal_stateen(&self) -> &RfcalStateen {
        &self.rfcal_stateen
    }
    #[doc = "0x24 - SARADC Control Registers"]
    #[inline(always)]
    pub const fn saradc_resv(&self) -> &SaradcResv {
        &self.saradc_resv
    }
    #[doc = "0x28 - ZRF Control register 0"]
    #[inline(always)]
    pub const fn rf_base_ctrl1(&self) -> &RfBaseCtrl1 {
        &self.rf_base_ctrl1
    }
    #[doc = "0x2c - ZRF Control register 0"]
    #[inline(always)]
    pub const fn rf_base_ctrl2(&self) -> &RfBaseCtrl2 {
        &self.rf_base_ctrl2
    }
    #[doc = "0x30 - pucr1."]
    #[inline(always)]
    pub const fn pucr1(&self) -> &Pucr1 {
        &self.pucr1
    }
    #[doc = "0x34 - read only from hardware logic"]
    #[inline(always)]
    pub const fn pucr1_hw(&self) -> &Pucr1Hw {
        &self.pucr1_hw
    }
    #[doc = "0x38 - pucr2."]
    #[inline(always)]
    pub const fn pucr2(&self) -> &Pucr2 {
        &self.pucr2
    }
    #[doc = "0x3c - pucr2_hw."]
    #[inline(always)]
    pub const fn pucr2_hw(&self) -> &Pucr2Hw {
        &self.pucr2_hw
    }
    #[doc = "0x40 - ppu_ctrl_hw."]
    #[inline(always)]
    pub const fn ppu_ctrl_hw(&self) -> &PpuCtrlHw {
        &self.ppu_ctrl_hw
    }
    #[doc = "0x44 - pud_ctrl_hw."]
    #[inline(always)]
    pub const fn pud_ctrl_hw(&self) -> &PudCtrlHw {
        &self.pud_ctrl_hw
    }
    #[doc = "0x48 - gain control1"]
    #[inline(always)]
    pub const fn trx_gain1(&self) -> &TrxGain1 {
        &self.trx_gain1
    }
    #[doc = "0x4c - trx gain hardware readback"]
    #[inline(always)]
    pub const fn trx_gain_hw(&self) -> &TrxGainHw {
        &self.trx_gain_hw
    }
    #[doc = "0x50 - dc test register"]
    #[inline(always)]
    pub const fn ten_dc(&self) -> &TenDc {
        &self.ten_dc
    }
    #[doc = "0x54 - digital test register"]
    #[inline(always)]
    pub const fn ten_dig(&self) -> &TenDig {
        &self.ten_dig
    }
    #[doc = "0x58 - ac test register"]
    #[inline(always)]
    pub const fn ten_ac(&self) -> &TenAc {
        &self.ten_ac
    }
    #[doc = "0x5c - pmip_mv2aon."]
    #[inline(always)]
    pub const fn pmip_mv2aon(&self) -> &PmipMv2aon {
        &self.pmip_mv2aon
    }
    #[doc = "0x60 - RX normal bias mode registers"]
    #[inline(always)]
    pub const fn cip(&self) -> &Cip {
        &self.cip
    }
    #[doc = "0x64 - pa1."]
    #[inline(always)]
    pub const fn pa1(&self) -> &Pa1 {
        &self.pa1
    }
    #[doc = "0x68 - RX normal bias mode registers"]
    #[inline(always)]
    pub const fn pa2(&self) -> &Pa2 {
        &self.pa2
    }
    #[doc = "0x6c - tmx."]
    #[inline(always)]
    pub const fn tmx(&self) -> &Tmx {
        &self.tmx
    }
    #[doc = "0x70 - tbb."]
    #[inline(always)]
    pub const fn tbb(&self) -> &Tbb {
        &self.tbb
    }
    #[doc = "0x74 - lna."]
    #[inline(always)]
    pub const fn lna(&self) -> &Lna {
        &self.lna
    }
    #[doc = "0x78 - rmxgm."]
    #[inline(always)]
    pub const fn rmxgm(&self) -> &Rmxgm {
        &self.rmxgm
    }
    #[doc = "0x7c - rbb1."]
    #[inline(always)]
    pub const fn rbb1(&self) -> &Rbb1 {
        &self.rbb1
    }
    #[doc = "0x80 - rbb2."]
    #[inline(always)]
    pub const fn rbb2(&self) -> &Rbb2 {
        &self.rbb2
    }
    #[doc = "0x84 - rbb3."]
    #[inline(always)]
    pub const fn rbb3(&self) -> &Rbb3 {
        &self.rbb3
    }
    #[doc = "0x88 - rbb4."]
    #[inline(always)]
    pub const fn rbb4(&self) -> &Rbb4 {
        &self.rbb4
    }
    #[doc = "0x8c - adda1."]
    #[inline(always)]
    pub const fn adda1(&self) -> &Adda1 {
        &self.adda1
    }
    #[doc = "0x90 - adda2."]
    #[inline(always)]
    pub const fn adda2(&self) -> &Adda2 {
        &self.adda2
    }
    #[doc = "0xa0 - vco1."]
    #[inline(always)]
    pub const fn vco1(&self) -> &Vco1 {
        &self.vco1
    }
    #[doc = "0xa4 - vco2."]
    #[inline(always)]
    pub const fn vco2(&self) -> &Vco2 {
        &self.vco2
    }
    #[doc = "0xa8 - vco3."]
    #[inline(always)]
    pub const fn vco3(&self) -> &Vco3 {
        &self.vco3
    }
    #[doc = "0xac - vco4."]
    #[inline(always)]
    pub const fn vco4(&self) -> &Vco4 {
        &self.vco4
    }
    #[doc = "0xb0 - pfdcp."]
    #[inline(always)]
    pub const fn pfdcp(&self) -> &Pfdcp {
        &self.pfdcp
    }
    #[doc = "0xb4 - lo."]
    #[inline(always)]
    pub const fn lo(&self) -> &Lo {
        &self.lo
    }
    #[doc = "0xb8 - fbdv."]
    #[inline(always)]
    pub const fn fbdv(&self) -> &Fbdv {
        &self.fbdv
    }
    #[doc = "0xbc - lodist."]
    #[inline(always)]
    pub const fn lodist(&self) -> &Lodist {
        &self.lodist
    }
    #[doc = "0xc0 - sdm1."]
    #[inline(always)]
    pub const fn sdm1(&self) -> &Sdm1 {
        &self.sdm1
    }
    #[doc = "0xc4 - sdm2."]
    #[inline(always)]
    pub const fn sdm2(&self) -> &Sdm2 {
        &self.sdm2
    }
    #[doc = "0xc8 - sdm3."]
    #[inline(always)]
    pub const fn sdm3(&self) -> &Sdm3 {
        &self.sdm3
    }
    #[doc = "0xec - rf_resv_reg_0."]
    #[inline(always)]
    pub const fn rf_resv_reg_0(&self) -> &RfResvReg0 {
        &self.rf_resv_reg_0
    }
    #[doc = "0xf0 - rf_resv_reg_1."]
    #[inline(always)]
    pub const fn rf_resv_reg_1(&self) -> &RfResvReg1 {
        &self.rf_resv_reg_1
    }
    #[doc = "0xf4 - rf_resv_reg_2."]
    #[inline(always)]
    pub const fn rf_resv_reg_2(&self) -> &RfResvReg2 {
        &self.rf_resv_reg_2
    }
    #[doc = "0xf8 - rrf_gain_index1."]
    #[inline(always)]
    pub const fn rrf_gain_index1(&self) -> &RrfGainIndex1 {
        &self.rrf_gain_index1
    }
    #[doc = "0xfc - rrf_gain_index2."]
    #[inline(always)]
    pub const fn rrf_gain_index2(&self) -> &RrfGainIndex2 {
        &self.rrf_gain_index2
    }
    #[doc = "0x100 - lna_ctrl_hw_mux."]
    #[inline(always)]
    pub const fn lna_ctrl_hw_mux(&self) -> &LnaCtrlHwMux {
        &self.lna_ctrl_hw_mux
    }
    #[doc = "0x104 - rbb_gain_index1."]
    #[inline(always)]
    pub const fn rbb_gain_index1(&self) -> &RbbGainIndex1 {
        &self.rbb_gain_index1
    }
    #[doc = "0x108 - rbb_gain_index2."]
    #[inline(always)]
    pub const fn rbb_gain_index2(&self) -> &RbbGainIndex2 {
        &self.rbb_gain_index2
    }
    #[doc = "0x10c - rbb_gain_index3."]
    #[inline(always)]
    pub const fn rbb_gain_index3(&self) -> &RbbGainIndex3 {
        &self.rbb_gain_index3
    }
    #[doc = "0x110 - rbb_gain_index4."]
    #[inline(always)]
    pub const fn rbb_gain_index4(&self) -> &RbbGainIndex4 {
        &self.rbb_gain_index4
    }
    #[doc = "0x114 - rbb_gain_index5."]
    #[inline(always)]
    pub const fn rbb_gain_index5(&self) -> &RbbGainIndex5 {
        &self.rbb_gain_index5
    }
    #[doc = "0x118 - tbb_gain_index1."]
    #[inline(always)]
    pub const fn tbb_gain_index1(&self) -> &TbbGainIndex1 {
        &self.tbb_gain_index1
    }
    #[doc = "0x11c - tbb_gain_index2."]
    #[inline(always)]
    pub const fn tbb_gain_index2(&self) -> &TbbGainIndex2 {
        &self.tbb_gain_index2
    }
    #[doc = "0x120 - tbb_gain_index3."]
    #[inline(always)]
    pub const fn tbb_gain_index3(&self) -> &TbbGainIndex3 {
        &self.tbb_gain_index3
    }
    #[doc = "0x124 - tbb_gain_index4."]
    #[inline(always)]
    pub const fn tbb_gain_index4(&self) -> &TbbGainIndex4 {
        &self.tbb_gain_index4
    }
    #[doc = "0x128 - pa_reg_ctrl_hw1."]
    #[inline(always)]
    pub const fn pa_reg_ctrl_hw1(&self) -> &PaRegCtrlHw1 {
        &self.pa_reg_ctrl_hw1
    }
    #[doc = "0x12c - pa_reg_ctrl_hw2."]
    #[inline(always)]
    pub const fn pa_reg_ctrl_hw2(&self) -> &PaRegCtrlHw2 {
        &self.pa_reg_ctrl_hw2
    }
    #[doc = "0x130 - pa_reg_wifi_ctrl_hw."]
    #[inline(always)]
    pub const fn pa_reg_wifi_ctrl_hw(&self) -> &PaRegWifiCtrlHw {
        &self.pa_reg_wifi_ctrl_hw
    }
    #[doc = "0x134 - adda_reg_ctrl_hw."]
    #[inline(always)]
    pub const fn adda_reg_ctrl_hw(&self) -> &AddaRegCtrlHw {
        &self.adda_reg_ctrl_hw
    }
    #[doc = "0x138 - lo_reg_ctrl_hw1."]
    #[inline(always)]
    pub const fn lo_reg_ctrl_hw1(&self) -> &LoRegCtrlHw1 {
        &self.lo_reg_ctrl_hw1
    }
    #[doc = "0x13c - lo_cal_ctrl_hw1."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw1(&self) -> &LoCalCtrlHw1 {
        &self.lo_cal_ctrl_hw1
    }
    #[doc = "0x140 - lo_cal_ctrl_hw2."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw2(&self) -> &LoCalCtrlHw2 {
        &self.lo_cal_ctrl_hw2
    }
    #[doc = "0x144 - lo_cal_ctrl_hw3."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw3(&self) -> &LoCalCtrlHw3 {
        &self.lo_cal_ctrl_hw3
    }
    #[doc = "0x148 - lo_cal_ctrl_hw4."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw4(&self) -> &LoCalCtrlHw4 {
        &self.lo_cal_ctrl_hw4
    }
    #[doc = "0x14c - lo_cal_ctrl_hw5."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw5(&self) -> &LoCalCtrlHw5 {
        &self.lo_cal_ctrl_hw5
    }
    #[doc = "0x150 - lo_cal_ctrl_hw6."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw6(&self) -> &LoCalCtrlHw6 {
        &self.lo_cal_ctrl_hw6
    }
    #[doc = "0x154 - lo_cal_ctrl_hw7."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw7(&self) -> &LoCalCtrlHw7 {
        &self.lo_cal_ctrl_hw7
    }
    #[doc = "0x158 - lo_cal_ctrl_hw8."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw8(&self) -> &LoCalCtrlHw8 {
        &self.lo_cal_ctrl_hw8
    }
    #[doc = "0x15c - lo_cal_ctrl_hw9."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw9(&self) -> &LoCalCtrlHw9 {
        &self.lo_cal_ctrl_hw9
    }
    #[doc = "0x160 - lo_cal_ctrl_hw10."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw10(&self) -> &LoCalCtrlHw10 {
        &self.lo_cal_ctrl_hw10
    }
    #[doc = "0x164 - lo_cal_ctrl_hw11."]
    #[inline(always)]
    pub const fn lo_cal_ctrl_hw11(&self) -> &LoCalCtrlHw11 {
        &self.lo_cal_ctrl_hw11
    }
    #[doc = "0x168 - rosdac_ctrl_hw1."]
    #[inline(always)]
    pub const fn rosdac_ctrl_hw1(&self) -> &RosdacCtrlHw1 {
        &self.rosdac_ctrl_hw1
    }
    #[doc = "0x16c - rosdac_ctrl_hw2."]
    #[inline(always)]
    pub const fn rosdac_ctrl_hw2(&self) -> &RosdacCtrlHw2 {
        &self.rosdac_ctrl_hw2
    }
    #[doc = "0x170 - rxiq_ctrl_hw1."]
    #[inline(always)]
    pub const fn rxiq_ctrl_hw1(&self) -> &RxiqCtrlHw1 {
        &self.rxiq_ctrl_hw1
    }
    #[doc = "0x174 - rxiq_ctrl_hw2."]
    #[inline(always)]
    pub const fn rxiq_ctrl_hw2(&self) -> &RxiqCtrlHw2 {
        &self.rxiq_ctrl_hw2
    }
    #[doc = "0x178 - rxiq_ctrl_hw3."]
    #[inline(always)]
    pub const fn rxiq_ctrl_hw3(&self) -> &RxiqCtrlHw3 {
        &self.rxiq_ctrl_hw3
    }
    #[doc = "0x17c - rxiq_ctrl_hw4."]
    #[inline(always)]
    pub const fn rxiq_ctrl_hw4(&self) -> &RxiqCtrlHw4 {
        &self.rxiq_ctrl_hw4
    }
    #[doc = "0x180 - tosdac_ctrl_hw1."]
    #[inline(always)]
    pub const fn tosdac_ctrl_hw1(&self) -> &TosdacCtrlHw1 {
        &self.tosdac_ctrl_hw1
    }
    #[doc = "0x184 - tosdac_ctrl_hw2."]
    #[inline(always)]
    pub const fn tosdac_ctrl_hw2(&self) -> &TosdacCtrlHw2 {
        &self.tosdac_ctrl_hw2
    }
    #[doc = "0x188 - tosdac_ctrl_hw3."]
    #[inline(always)]
    pub const fn tosdac_ctrl_hw3(&self) -> &TosdacCtrlHw3 {
        &self.tosdac_ctrl_hw3
    }
    #[doc = "0x18c - tosdac_ctrl_hw4."]
    #[inline(always)]
    pub const fn tosdac_ctrl_hw4(&self) -> &TosdacCtrlHw4 {
        &self.tosdac_ctrl_hw4
    }
    #[doc = "0x190 - tx_iq_gain_hw0."]
    #[inline(always)]
    pub const fn tx_iq_gain_hw0(&self) -> &TxIqGainHw0 {
        &self.tx_iq_gain_hw0
    }
    #[doc = "0x194 - tx_iq_gain_hw1."]
    #[inline(always)]
    pub const fn tx_iq_gain_hw1(&self) -> &TxIqGainHw1 {
        &self.tx_iq_gain_hw1
    }
    #[doc = "0x198 - tx_iq_gain_hw2."]
    #[inline(always)]
    pub const fn tx_iq_gain_hw2(&self) -> &TxIqGainHw2 {
        &self.tx_iq_gain_hw2
    }
    #[doc = "0x19c - tx_iq_gain_hw3."]
    #[inline(always)]
    pub const fn tx_iq_gain_hw3(&self) -> &TxIqGainHw3 {
        &self.tx_iq_gain_hw3
    }
    #[doc = "0x1a0 - tx_iq_gain_hw4."]
    #[inline(always)]
    pub const fn tx_iq_gain_hw4(&self) -> &TxIqGainHw4 {
        &self.tx_iq_gain_hw4
    }
    #[doc = "0x1a4 - tx_iq_gain_hw5."]
    #[inline(always)]
    pub const fn tx_iq_gain_hw5(&self) -> &TxIqGainHw5 {
        &self.tx_iq_gain_hw5
    }
    #[doc = "0x1a8 - tx_iq_gain_hw6."]
    #[inline(always)]
    pub const fn tx_iq_gain_hw6(&self) -> &TxIqGainHw6 {
        &self.tx_iq_gain_hw6
    }
    #[doc = "0x1ac - tx_iq_gain_hw7."]
    #[inline(always)]
    pub const fn tx_iq_gain_hw7(&self) -> &TxIqGainHw7 {
        &self.tx_iq_gain_hw7
    }
    #[doc = "0x1b0 - lo_sdm_ctrl_hw1."]
    #[inline(always)]
    pub const fn lo_sdm_ctrl_hw1(&self) -> &LoSdmCtrlHw1 {
        &self.lo_sdm_ctrl_hw1
    }
    #[doc = "0x1b4 - lo_sdm_ctrl_hw2."]
    #[inline(always)]
    pub const fn lo_sdm_ctrl_hw2(&self) -> &LoSdmCtrlHw2 {
        &self.lo_sdm_ctrl_hw2
    }
    #[doc = "0x1b8 - lo_sdm_ctrl_hw3."]
    #[inline(always)]
    pub const fn lo_sdm_ctrl_hw3(&self) -> &LoSdmCtrlHw3 {
        &self.lo_sdm_ctrl_hw3
    }
    #[doc = "0x1bc - lo_sdm_ctrl_hw4."]
    #[inline(always)]
    pub const fn lo_sdm_ctrl_hw4(&self) -> &LoSdmCtrlHw4 {
        &self.lo_sdm_ctrl_hw4
    }
    #[doc = "0x1c0 - lo_sdm_ctrl_hw5."]
    #[inline(always)]
    pub const fn lo_sdm_ctrl_hw5(&self) -> &LoSdmCtrlHw5 {
        &self.lo_sdm_ctrl_hw5
    }
    #[doc = "0x1c4 - lo_sdm_ctrl_hw6."]
    #[inline(always)]
    pub const fn lo_sdm_ctrl_hw6(&self) -> &LoSdmCtrlHw6 {
        &self.lo_sdm_ctrl_hw6
    }
    #[doc = "0x1c8 - lo_sdm_ctrl_hw7."]
    #[inline(always)]
    pub const fn lo_sdm_ctrl_hw7(&self) -> &LoSdmCtrlHw7 {
        &self.lo_sdm_ctrl_hw7
    }
    #[doc = "0x1cc - lo_sdm_ctrl_hw8."]
    #[inline(always)]
    pub const fn lo_sdm_ctrl_hw8(&self) -> &LoSdmCtrlHw8 {
        &self.lo_sdm_ctrl_hw8
    }
    #[doc = "0x1d0 - rbb_bw_ctrl_hw."]
    #[inline(always)]
    pub const fn rbb_bw_ctrl_hw(&self) -> &RbbBwCtrlHw {
        &self.rbb_bw_ctrl_hw
    }
    #[doc = "0x20c - singen_ctrl0."]
    #[inline(always)]
    pub const fn singen_ctrl0(&self) -> &SingenCtrl0 {
        &self.singen_ctrl0
    }
    #[doc = "0x210 - singen_ctrl1."]
    #[inline(always)]
    pub const fn singen_ctrl1(&self) -> &SingenCtrl1 {
        &self.singen_ctrl1
    }
    #[doc = "0x214 - singen_ctrl2."]
    #[inline(always)]
    pub const fn singen_ctrl2(&self) -> &SingenCtrl2 {
        &self.singen_ctrl2
    }
    #[doc = "0x218 - singen_ctrl3."]
    #[inline(always)]
    pub const fn singen_ctrl3(&self) -> &SingenCtrl3 {
        &self.singen_ctrl3
    }
    #[doc = "0x21c - singen_ctrl4."]
    #[inline(always)]
    pub const fn singen_ctrl4(&self) -> &SingenCtrl4 {
        &self.singen_ctrl4
    }
    #[doc = "0x220 - rfif_dfe_ctrl0."]
    #[inline(always)]
    pub const fn rfif_dfe_ctrl0(&self) -> &RfifDfeCtrl0 {
        &self.rfif_dfe_ctrl0
    }
    #[doc = "0x224 - rfif_test_read."]
    #[inline(always)]
    pub const fn rfif_test_read(&self) -> &RfifTestRead {
        &self.rfif_test_read
    }
    #[doc = "0x228 - rfif_dig_ctrl."]
    #[inline(always)]
    pub const fn rfif_dig_ctrl(&self) -> &RfifDigCtrl {
        &self.rfif_dig_ctrl
    }
    #[doc = "0x22c - rf_data_temp_0."]
    #[inline(always)]
    pub const fn rf_data_temp_0(&self) -> &RfDataTemp0 {
        &self.rf_data_temp_0
    }
    #[doc = "0x230 - rf_data_temp_1."]
    #[inline(always)]
    pub const fn rf_data_temp_1(&self) -> &RfDataTemp1 {
        &self.rf_data_temp_1
    }
    #[doc = "0x234 - rf_data_temp_2."]
    #[inline(always)]
    pub const fn rf_data_temp_2(&self) -> &RfDataTemp2 {
        &self.rf_data_temp_2
    }
    #[doc = "0x238 - rf_data_temp_3."]
    #[inline(always)]
    pub const fn rf_data_temp_3(&self) -> &RfDataTemp3 {
        &self.rf_data_temp_3
    }
    #[doc = "0x23c - rf_sram_ctrl0."]
    #[inline(always)]
    pub const fn rf_sram_ctrl0(&self) -> &RfSramCtrl0 {
        &self.rf_sram_ctrl0
    }
    #[doc = "0x240 - rf_sram_ctrl1."]
    #[inline(always)]
    pub const fn rf_sram_ctrl1(&self) -> &RfSramCtrl1 {
        &self.rf_sram_ctrl1
    }
    #[doc = "0x244 - rf_sram_ctrl2."]
    #[inline(always)]
    pub const fn rf_sram_ctrl2(&self) -> &RfSramCtrl2 {
        &self.rf_sram_ctrl2
    }
    #[doc = "0x248 - rf_sram_ctrl3."]
    #[inline(always)]
    pub const fn rf_sram_ctrl3(&self) -> &RfSramCtrl3 {
        &self.rf_sram_ctrl3
    }
    #[doc = "0x24c - rf_sram_ctrl4."]
    #[inline(always)]
    pub const fn rf_sram_ctrl4(&self) -> &RfSramCtrl4 {
        &self.rf_sram_ctrl4
    }
    #[doc = "0x250 - rf_sram_ctrl5."]
    #[inline(always)]
    pub const fn rf_sram_ctrl5(&self) -> &RfSramCtrl5 {
        &self.rf_sram_ctrl5
    }
    #[doc = "0x254 - rf_sram_ctrl6."]
    #[inline(always)]
    pub const fn rf_sram_ctrl6(&self) -> &RfSramCtrl6 {
        &self.rf_sram_ctrl6
    }
    #[doc = "0x258 - rf_ical_ctrl0."]
    #[inline(always)]
    pub const fn rf_ical_ctrl0(&self) -> &RfIcalCtrl0 {
        &self.rf_ical_ctrl0
    }
    #[doc = "0x25c - rf_ical_ctrl1."]
    #[inline(always)]
    pub const fn rf_ical_ctrl1(&self) -> &RfIcalCtrl1 {
        &self.rf_ical_ctrl1
    }
    #[doc = "0x260 - rf_ical_ctrl2."]
    #[inline(always)]
    pub const fn rf_ical_ctrl2(&self) -> &RfIcalCtrl2 {
        &self.rf_ical_ctrl2
    }
    #[doc = "0x264 - rf_fsm_ctrl0."]
    #[inline(always)]
    pub const fn rf_fsm_ctrl0(&self) -> &RfFsmCtrl0 {
        &self.rf_fsm_ctrl0
    }
    #[doc = "0x268 - rf_fsm_ctrl1."]
    #[inline(always)]
    pub const fn rf_fsm_ctrl1(&self) -> &RfFsmCtrl1 {
        &self.rf_fsm_ctrl1
    }
    #[doc = "0x26c - rf_fsm_ctrl2."]
    #[inline(always)]
    pub const fn rf_fsm_ctrl2(&self) -> &RfFsmCtrl2 {
        &self.rf_fsm_ctrl2
    }
    #[doc = "0x270 - rf_pkdet_ctrl0."]
    #[inline(always)]
    pub const fn rf_pkdet_ctrl0(&self) -> &RfPkdetCtrl0 {
        &self.rf_pkdet_ctrl0
    }
    #[doc = "0x600 - dfe_ctrl_0."]
    #[inline(always)]
    pub const fn dfe_ctrl_0(&self) -> &DfeCtrl0 {
        &self.dfe_ctrl_0
    }
    #[doc = "0x604 - dfe_ctrl_1."]
    #[inline(always)]
    pub const fn dfe_ctrl_1(&self) -> &DfeCtrl1 {
        &self.dfe_ctrl_1
    }
    #[doc = "0x608 - dfe_ctrl_2."]
    #[inline(always)]
    pub const fn dfe_ctrl_2(&self) -> &DfeCtrl2 {
        &self.dfe_ctrl_2
    }
    #[doc = "0x60c - dfe_ctrl_3."]
    #[inline(always)]
    pub const fn dfe_ctrl_3(&self) -> &DfeCtrl3 {
        &self.dfe_ctrl_3
    }
    #[doc = "0x610 - dfe_ctrl_4."]
    #[inline(always)]
    pub const fn dfe_ctrl_4(&self) -> &DfeCtrl4 {
        &self.dfe_ctrl_4
    }
    #[doc = "0x614 - dfe_ctrl_5."]
    #[inline(always)]
    pub const fn dfe_ctrl_5(&self) -> &DfeCtrl5 {
        &self.dfe_ctrl_5
    }
    #[doc = "0x618 - dfe_ctrl_6."]
    #[inline(always)]
    pub const fn dfe_ctrl_6(&self) -> &DfeCtrl6 {
        &self.dfe_ctrl_6
    }
    #[doc = "0x61c - dfe_ctrl_7."]
    #[inline(always)]
    pub const fn dfe_ctrl_7(&self) -> &DfeCtrl7 {
        &self.dfe_ctrl_7
    }
    #[doc = "0x620 - dfe_ctrl_8."]
    #[inline(always)]
    pub const fn dfe_ctrl_8(&self) -> &DfeCtrl8 {
        &self.dfe_ctrl_8
    }
    #[doc = "0x624 - dfe_ctrl_9."]
    #[inline(always)]
    pub const fn dfe_ctrl_9(&self) -> &DfeCtrl9 {
        &self.dfe_ctrl_9
    }
    #[doc = "0x628 - dfe_ctrl_10."]
    #[inline(always)]
    pub const fn dfe_ctrl_10(&self) -> &DfeCtrl10 {
        &self.dfe_ctrl_10
    }
    #[doc = "0x62c - dfe_ctrl_11."]
    #[inline(always)]
    pub const fn dfe_ctrl_11(&self) -> &DfeCtrl11 {
        &self.dfe_ctrl_11
    }
    #[doc = "0x630 - dfe_ctrl_12."]
    #[inline(always)]
    pub const fn dfe_ctrl_12(&self) -> &DfeCtrl12 {
        &self.dfe_ctrl_12
    }
    #[doc = "0x634 - dfe_ctrl_13."]
    #[inline(always)]
    pub const fn dfe_ctrl_13(&self) -> &DfeCtrl13 {
        &self.dfe_ctrl_13
    }
    #[doc = "0x638 - dfe_ctrl_14."]
    #[inline(always)]
    pub const fn dfe_ctrl_14(&self) -> &DfeCtrl14 {
        &self.dfe_ctrl_14
    }
    #[doc = "0x63c - dfe_ctrl_15."]
    #[inline(always)]
    pub const fn dfe_ctrl_15(&self) -> &DfeCtrl15 {
        &self.dfe_ctrl_15
    }
    #[doc = "0x640 - dfe_ctrl_16."]
    #[inline(always)]
    pub const fn dfe_ctrl_16(&self) -> &DfeCtrl16 {
        &self.dfe_ctrl_16
    }
    #[doc = "0x644 - dfe_ctrl_17."]
    #[inline(always)]
    pub const fn dfe_ctrl_17(&self) -> &DfeCtrl17 {
        &self.dfe_ctrl_17
    }
    #[doc = "0x648 - dfe_ctrl_18."]
    #[inline(always)]
    pub const fn dfe_ctrl_18(&self) -> &DfeCtrl18 {
        &self.dfe_ctrl_18
    }
}
#[doc = "rf_rev (rw) register accessor: Silicon revision\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_rev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_rev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_rev`] module"]
#[doc(alias = "rf_rev")]
pub type RfRev = crate::Reg<rf_rev::RfRevSpec>;
#[doc = "Silicon revision"]
pub mod rf_rev;
#[doc = "rf_fsm_ctrl_hw (rw) register accessor: Digital Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm_ctrl_hw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm_ctrl_hw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm_ctrl_hw`] module"]
#[doc(alias = "rf_fsm_ctrl_hw")]
pub type RfFsmCtrlHw = crate::Reg<rf_fsm_ctrl_hw::RfFsmCtrlHwSpec>;
#[doc = "Digital Control"]
pub mod rf_fsm_ctrl_hw;
#[doc = "rf_fsm_ctrl_sw (rw) register accessor: rfsm status reg\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm_ctrl_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm_ctrl_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm_ctrl_sw`] module"]
#[doc(alias = "rf_fsm_ctrl_sw")]
pub type RfFsmCtrlSw = crate::Reg<rf_fsm_ctrl_sw::RfFsmCtrlSwSpec>;
#[doc = "rfsm status reg"]
pub mod rf_fsm_ctrl_sw;
#[doc = "rfctrl_hw_en (rw) register accessor: Control logic switch\n\nYou can [`read`](crate::Reg::read) this register and get [`rfctrl_hw_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfctrl_hw_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfctrl_hw_en`] module"]
#[doc(alias = "rfctrl_hw_en")]
pub type RfctrlHwEn = crate::Reg<rfctrl_hw_en::RfctrlHwEnSpec>;
#[doc = "Control logic switch"]
pub mod rfctrl_hw_en;
#[doc = "temp_comp (rw) register accessor: temp_comp.\n\nYou can [`read`](crate::Reg::read) this register and get [`temp_comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`temp_comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp_comp`] module"]
#[doc(alias = "temp_comp")]
pub type TempComp = crate::Reg<temp_comp::TempCompSpec>;
#[doc = "temp_comp."]
pub mod temp_comp;
#[doc = "rfcal_status (rw) register accessor: rfcal_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcal_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcal_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcal_status`] module"]
#[doc(alias = "rfcal_status")]
pub type RfcalStatus = crate::Reg<rfcal_status::RfcalStatusSpec>;
#[doc = "rfcal_status."]
pub mod rfcal_status;
#[doc = "rfcal_status2 (rw) register accessor: rfcal_status2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcal_status2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcal_status2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcal_status2`] module"]
#[doc(alias = "rfcal_status2")]
pub type RfcalStatus2 = crate::Reg<rfcal_status2::RfcalStatus2Spec>;
#[doc = "rfcal_status2."]
pub mod rfcal_status2;
#[doc = "rfcal_ctrlen (rw) register accessor: Calibration mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcal_ctrlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcal_ctrlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcal_ctrlen`] module"]
#[doc(alias = "rfcal_ctrlen")]
pub type RfcalCtrlen = crate::Reg<rfcal_ctrlen::RfcalCtrlenSpec>;
#[doc = "Calibration mode register"]
pub mod rfcal_ctrlen;
#[doc = "rfcal_stateen (rw) register accessor: rf calibration state enabl in full cal list\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcal_stateen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcal_stateen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcal_stateen`] module"]
#[doc(alias = "rfcal_stateen")]
pub type RfcalStateen = crate::Reg<rfcal_stateen::RfcalStateenSpec>;
#[doc = "rf calibration state enabl in full cal list"]
pub mod rfcal_stateen;
#[doc = "saradc_resv (rw) register accessor: SARADC Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`saradc_resv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saradc_resv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_resv`] module"]
#[doc(alias = "saradc_resv")]
pub type SaradcResv = crate::Reg<saradc_resv::SaradcResvSpec>;
#[doc = "SARADC Control Registers"]
pub mod saradc_resv;
#[doc = "rf_base_ctrl1 (rw) register accessor: ZRF Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_base_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_base_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_base_ctrl1`] module"]
#[doc(alias = "rf_base_ctrl1")]
pub type RfBaseCtrl1 = crate::Reg<rf_base_ctrl1::RfBaseCtrl1Spec>;
#[doc = "ZRF Control register 0"]
pub mod rf_base_ctrl1;
#[doc = "rf_base_ctrl2 (rw) register accessor: ZRF Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_base_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_base_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_base_ctrl2`] module"]
#[doc(alias = "rf_base_ctrl2")]
pub type RfBaseCtrl2 = crate::Reg<rf_base_ctrl2::RfBaseCtrl2Spec>;
#[doc = "ZRF Control register 0"]
pub mod rf_base_ctrl2;
#[doc = "pucr1 (rw) register accessor: pucr1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pucr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucr1`] module"]
#[doc(alias = "pucr1")]
pub type Pucr1 = crate::Reg<pucr1::Pucr1Spec>;
#[doc = "pucr1."]
pub mod pucr1;
#[doc = "pucr1_hw (rw) register accessor: read only from hardware logic\n\nYou can [`read`](crate::Reg::read) this register and get [`pucr1_hw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr1_hw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucr1_hw`] module"]
#[doc(alias = "pucr1_hw")]
pub type Pucr1Hw = crate::Reg<pucr1_hw::Pucr1HwSpec>;
#[doc = "read only from hardware logic"]
pub mod pucr1_hw;
#[doc = "pucr2 (rw) register accessor: pucr2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pucr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucr2`] module"]
#[doc(alias = "pucr2")]
pub type Pucr2 = crate::Reg<pucr2::Pucr2Spec>;
#[doc = "pucr2."]
pub mod pucr2;
#[doc = "pucr2_hw (rw) register accessor: pucr2_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`pucr2_hw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr2_hw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucr2_hw`] module"]
#[doc(alias = "pucr2_hw")]
pub type Pucr2Hw = crate::Reg<pucr2_hw::Pucr2HwSpec>;
#[doc = "pucr2_hw."]
pub mod pucr2_hw;
#[doc = "ppu_ctrl_hw (rw) register accessor: ppu_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppu_ctrl_hw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppu_ctrl_hw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppu_ctrl_hw`] module"]
#[doc(alias = "ppu_ctrl_hw")]
pub type PpuCtrlHw = crate::Reg<ppu_ctrl_hw::PpuCtrlHwSpec>;
#[doc = "ppu_ctrl_hw."]
pub mod ppu_ctrl_hw;
#[doc = "pud_ctrl_hw (rw) register accessor: pud_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`pud_ctrl_hw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pud_ctrl_hw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pud_ctrl_hw`] module"]
#[doc(alias = "pud_ctrl_hw")]
pub type PudCtrlHw = crate::Reg<pud_ctrl_hw::PudCtrlHwSpec>;
#[doc = "pud_ctrl_hw."]
pub mod pud_ctrl_hw;
#[doc = "trx_gain1 (rw) register accessor: gain control1\n\nYou can [`read`](crate::Reg::read) this register and get [`trx_gain1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trx_gain1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trx_gain1`] module"]
#[doc(alias = "trx_gain1")]
pub type TrxGain1 = crate::Reg<trx_gain1::TrxGain1Spec>;
#[doc = "gain control1"]
pub mod trx_gain1;
#[doc = "trx_gain_hw (rw) register accessor: trx gain hardware readback\n\nYou can [`read`](crate::Reg::read) this register and get [`trx_gain_hw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trx_gain_hw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trx_gain_hw`] module"]
#[doc(alias = "trx_gain_hw")]
pub type TrxGainHw = crate::Reg<trx_gain_hw::TrxGainHwSpec>;
#[doc = "trx gain hardware readback"]
pub mod trx_gain_hw;
#[doc = "ten_dc (rw) register accessor: dc test register\n\nYou can [`read`](crate::Reg::read) this register and get [`ten_dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ten_dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ten_dc`] module"]
#[doc(alias = "ten_dc")]
pub type TenDc = crate::Reg<ten_dc::TenDcSpec>;
#[doc = "dc test register"]
pub mod ten_dc;
#[doc = "ten_dig (rw) register accessor: digital test register\n\nYou can [`read`](crate::Reg::read) this register and get [`ten_dig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ten_dig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ten_dig`] module"]
#[doc(alias = "ten_dig")]
pub type TenDig = crate::Reg<ten_dig::TenDigSpec>;
#[doc = "digital test register"]
pub mod ten_dig;
#[doc = "ten_ac (rw) register accessor: ac test register\n\nYou can [`read`](crate::Reg::read) this register and get [`ten_ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ten_ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ten_ac`] module"]
#[doc(alias = "ten_ac")]
pub type TenAc = crate::Reg<ten_ac::TenAcSpec>;
#[doc = "ac test register"]
pub mod ten_ac;
#[doc = "pmip_mv2aon (rw) register accessor: pmip_mv2aon.\n\nYou can [`read`](crate::Reg::read) this register and get [`pmip_mv2aon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmip_mv2aon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmip_mv2aon`] module"]
#[doc(alias = "pmip_mv2aon")]
pub type PmipMv2aon = crate::Reg<pmip_mv2aon::PmipMv2aonSpec>;
#[doc = "pmip_mv2aon."]
pub mod pmip_mv2aon;
#[doc = "cip (rw) register accessor: RX normal bias mode registers\n\nYou can [`read`](crate::Reg::read) this register and get [`cip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cip`] module"]
#[doc(alias = "cip")]
pub type Cip = crate::Reg<cip::CipSpec>;
#[doc = "RX normal bias mode registers"]
pub mod cip;
#[doc = "pa1 (rw) register accessor: pa1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa1`] module"]
#[doc(alias = "pa1")]
pub type Pa1 = crate::Reg<pa1::Pa1Spec>;
#[doc = "pa1."]
pub mod pa1;
#[doc = "pa2 (rw) register accessor: RX normal bias mode registers\n\nYou can [`read`](crate::Reg::read) this register and get [`pa2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa2`] module"]
#[doc(alias = "pa2")]
pub type Pa2 = crate::Reg<pa2::Pa2Spec>;
#[doc = "RX normal bias mode registers"]
pub mod pa2;
#[doc = "tmx (rw) register accessor: tmx.\n\nYou can [`read`](crate::Reg::read) this register and get [`tmx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmx`] module"]
#[doc(alias = "tmx")]
pub type Tmx = crate::Reg<tmx::TmxSpec>;
#[doc = "tmx."]
pub mod tmx;
#[doc = "tbb (rw) register accessor: tbb.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbb`] module"]
#[doc(alias = "tbb")]
pub type Tbb = crate::Reg<tbb::TbbSpec>;
#[doc = "tbb."]
pub mod tbb;
#[doc = "lna (rw) register accessor: lna.\n\nYou can [`read`](crate::Reg::read) this register and get [`lna::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lna::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lna`] module"]
#[doc(alias = "lna")]
pub type Lna = crate::Reg<lna::LnaSpec>;
#[doc = "lna."]
pub mod lna;
#[doc = "rmxgm (rw) register accessor: rmxgm.\n\nYou can [`read`](crate::Reg::read) this register and get [`rmxgm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmxgm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmxgm`] module"]
#[doc(alias = "rmxgm")]
pub type Rmxgm = crate::Reg<rmxgm::RmxgmSpec>;
#[doc = "rmxgm."]
pub mod rmxgm;
#[doc = "rbb1 (rw) register accessor: rbb1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb1`] module"]
#[doc(alias = "rbb1")]
pub type Rbb1 = crate::Reg<rbb1::Rbb1Spec>;
#[doc = "rbb1."]
pub mod rbb1;
#[doc = "rbb2 (rw) register accessor: rbb2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb2`] module"]
#[doc(alias = "rbb2")]
pub type Rbb2 = crate::Reg<rbb2::Rbb2Spec>;
#[doc = "rbb2."]
pub mod rbb2;
#[doc = "rbb3 (rw) register accessor: rbb3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb3`] module"]
#[doc(alias = "rbb3")]
pub type Rbb3 = crate::Reg<rbb3::Rbb3Spec>;
#[doc = "rbb3."]
pub mod rbb3;
#[doc = "rbb4 (rw) register accessor: rbb4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb4`] module"]
#[doc(alias = "rbb4")]
pub type Rbb4 = crate::Reg<rbb4::Rbb4Spec>;
#[doc = "rbb4."]
pub mod rbb4;
#[doc = "adda1 (rw) register accessor: adda1.\n\nYou can [`read`](crate::Reg::read) this register and get [`adda1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adda1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adda1`] module"]
#[doc(alias = "adda1")]
pub type Adda1 = crate::Reg<adda1::Adda1Spec>;
#[doc = "adda1."]
pub mod adda1;
#[doc = "adda2 (rw) register accessor: adda2.\n\nYou can [`read`](crate::Reg::read) this register and get [`adda2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adda2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adda2`] module"]
#[doc(alias = "adda2")]
pub type Adda2 = crate::Reg<adda2::Adda2Spec>;
#[doc = "adda2."]
pub mod adda2;
#[doc = "vco1 (rw) register accessor: vco1.\n\nYou can [`read`](crate::Reg::read) this register and get [`vco1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vco1`] module"]
#[doc(alias = "vco1")]
pub type Vco1 = crate::Reg<vco1::Vco1Spec>;
#[doc = "vco1."]
pub mod vco1;
#[doc = "vco2 (rw) register accessor: vco2.\n\nYou can [`read`](crate::Reg::read) this register and get [`vco2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vco2`] module"]
#[doc(alias = "vco2")]
pub type Vco2 = crate::Reg<vco2::Vco2Spec>;
#[doc = "vco2."]
pub mod vco2;
#[doc = "vco3 (rw) register accessor: vco3.\n\nYou can [`read`](crate::Reg::read) this register and get [`vco3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vco3`] module"]
#[doc(alias = "vco3")]
pub type Vco3 = crate::Reg<vco3::Vco3Spec>;
#[doc = "vco3."]
pub mod vco3;
#[doc = "vco4 (rw) register accessor: vco4.\n\nYou can [`read`](crate::Reg::read) this register and get [`vco4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vco4`] module"]
#[doc(alias = "vco4")]
pub type Vco4 = crate::Reg<vco4::Vco4Spec>;
#[doc = "vco4."]
pub mod vco4;
#[doc = "pfdcp (rw) register accessor: pfdcp.\n\nYou can [`read`](crate::Reg::read) this register and get [`pfdcp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfdcp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfdcp`] module"]
#[doc(alias = "pfdcp")]
pub type Pfdcp = crate::Reg<pfdcp::PfdcpSpec>;
#[doc = "pfdcp."]
pub mod pfdcp;
#[doc = "lo (rw) register accessor: lo.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo`] module"]
#[doc(alias = "lo")]
pub type Lo = crate::Reg<lo::LoSpec>;
#[doc = "lo."]
pub mod lo;
#[doc = "fbdv (rw) register accessor: fbdv.\n\nYou can [`read`](crate::Reg::read) this register and get [`fbdv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbdv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbdv`] module"]
#[doc(alias = "fbdv")]
pub type Fbdv = crate::Reg<fbdv::FbdvSpec>;
#[doc = "fbdv."]
pub mod fbdv;
#[doc = "lodist (rw) register accessor: lodist.\n\nYou can [`read`](crate::Reg::read) this register and get [`lodist::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lodist::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lodist`] module"]
#[doc(alias = "lodist")]
pub type Lodist = crate::Reg<lodist::LodistSpec>;
#[doc = "lodist."]
pub mod lodist;
#[doc = "sdm1 (rw) register accessor: sdm1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdm1`] module"]
#[doc(alias = "sdm1")]
pub type Sdm1 = crate::Reg<sdm1::Sdm1Spec>;
#[doc = "sdm1."]
pub mod sdm1;
#[doc = "sdm2 (rw) register accessor: sdm2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdm2`] module"]
#[doc(alias = "sdm2")]
pub type Sdm2 = crate::Reg<sdm2::Sdm2Spec>;
#[doc = "sdm2."]
pub mod sdm2;
#[doc = "sdm3 (rw) register accessor: sdm3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdm3`] module"]
#[doc(alias = "sdm3")]
pub type Sdm3 = crate::Reg<sdm3::Sdm3Spec>;
#[doc = "sdm3."]
pub mod sdm3;
#[doc = "rf_resv_reg_0 (rw) register accessor: rf_resv_reg_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_resv_reg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_resv_reg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_resv_reg_0`] module"]
#[doc(alias = "rf_resv_reg_0")]
pub type RfResvReg0 = crate::Reg<rf_resv_reg_0::RfResvReg0Spec>;
#[doc = "rf_resv_reg_0."]
pub mod rf_resv_reg_0;
#[doc = "rf_resv_reg_1 (rw) register accessor: rf_resv_reg_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_resv_reg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_resv_reg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_resv_reg_1`] module"]
#[doc(alias = "rf_resv_reg_1")]
pub type RfResvReg1 = crate::Reg<rf_resv_reg_1::RfResvReg1Spec>;
#[doc = "rf_resv_reg_1."]
pub mod rf_resv_reg_1;
#[doc = "rf_resv_reg_2 (rw) register accessor: rf_resv_reg_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_resv_reg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_resv_reg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_resv_reg_2`] module"]
#[doc(alias = "rf_resv_reg_2")]
pub type RfResvReg2 = crate::Reg<rf_resv_reg_2::RfResvReg2Spec>;
#[doc = "rf_resv_reg_2."]
pub mod rf_resv_reg_2;
#[doc = "rrf_gain_index1 (rw) register accessor: rrf_gain_index1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rrf_gain_index1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrf_gain_index1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrf_gain_index1`] module"]
#[doc(alias = "rrf_gain_index1")]
pub type RrfGainIndex1 = crate::Reg<rrf_gain_index1::RrfGainIndex1Spec>;
#[doc = "rrf_gain_index1."]
pub mod rrf_gain_index1;
#[doc = "rrf_gain_index2 (rw) register accessor: rrf_gain_index2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rrf_gain_index2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrf_gain_index2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrf_gain_index2`] module"]
#[doc(alias = "rrf_gain_index2")]
pub type RrfGainIndex2 = crate::Reg<rrf_gain_index2::RrfGainIndex2Spec>;
#[doc = "rrf_gain_index2."]
pub mod rrf_gain_index2;
#[doc = "lna_ctrl_hw_mux (rw) register accessor: lna_ctrl_hw_mux.\n\nYou can [`read`](crate::Reg::read) this register and get [`lna_ctrl_hw_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lna_ctrl_hw_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lna_ctrl_hw_mux`] module"]
#[doc(alias = "lna_ctrl_hw_mux")]
pub type LnaCtrlHwMux = crate::Reg<lna_ctrl_hw_mux::LnaCtrlHwMuxSpec>;
#[doc = "lna_ctrl_hw_mux."]
pub mod lna_ctrl_hw_mux;
#[doc = "rbb_gain_index1 (rw) register accessor: rbb_gain_index1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb_gain_index1`] module"]
#[doc(alias = "rbb_gain_index1")]
pub type RbbGainIndex1 = crate::Reg<rbb_gain_index1::RbbGainIndex1Spec>;
#[doc = "rbb_gain_index1."]
pub mod rbb_gain_index1;
#[doc = "rbb_gain_index2 (rw) register accessor: rbb_gain_index2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb_gain_index2`] module"]
#[doc(alias = "rbb_gain_index2")]
pub type RbbGainIndex2 = crate::Reg<rbb_gain_index2::RbbGainIndex2Spec>;
#[doc = "rbb_gain_index2."]
pub mod rbb_gain_index2;
#[doc = "rbb_gain_index3 (rw) register accessor: rbb_gain_index3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb_gain_index3`] module"]
#[doc(alias = "rbb_gain_index3")]
pub type RbbGainIndex3 = crate::Reg<rbb_gain_index3::RbbGainIndex3Spec>;
#[doc = "rbb_gain_index3."]
pub mod rbb_gain_index3;
#[doc = "rbb_gain_index4 (rw) register accessor: rbb_gain_index4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb_gain_index4`] module"]
#[doc(alias = "rbb_gain_index4")]
pub type RbbGainIndex4 = crate::Reg<rbb_gain_index4::RbbGainIndex4Spec>;
#[doc = "rbb_gain_index4."]
pub mod rbb_gain_index4;
#[doc = "rbb_gain_index5 (rw) register accessor: rbb_gain_index5.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_gain_index5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_gain_index5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb_gain_index5`] module"]
#[doc(alias = "rbb_gain_index5")]
pub type RbbGainIndex5 = crate::Reg<rbb_gain_index5::RbbGainIndex5Spec>;
#[doc = "rbb_gain_index5."]
pub mod rbb_gain_index5;
#[doc = "tbb_gain_index1 (rw) register accessor: tbb_gain_index1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb_gain_index1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb_gain_index1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbb_gain_index1`] module"]
#[doc(alias = "tbb_gain_index1")]
pub type TbbGainIndex1 = crate::Reg<tbb_gain_index1::TbbGainIndex1Spec>;
#[doc = "tbb_gain_index1."]
pub mod tbb_gain_index1;
#[doc = "tbb_gain_index2 (rw) register accessor: tbb_gain_index2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb_gain_index2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb_gain_index2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbb_gain_index2`] module"]
#[doc(alias = "tbb_gain_index2")]
pub type TbbGainIndex2 = crate::Reg<tbb_gain_index2::TbbGainIndex2Spec>;
#[doc = "tbb_gain_index2."]
pub mod tbb_gain_index2;
#[doc = "tbb_gain_index3 (rw) register accessor: tbb_gain_index3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb_gain_index3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb_gain_index3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbb_gain_index3`] module"]
#[doc(alias = "tbb_gain_index3")]
pub type TbbGainIndex3 = crate::Reg<tbb_gain_index3::TbbGainIndex3Spec>;
#[doc = "tbb_gain_index3."]
pub mod tbb_gain_index3;
#[doc = "tbb_gain_index4 (rw) register accessor: tbb_gain_index4.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbb_gain_index4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbb_gain_index4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbb_gain_index4`] module"]
#[doc(alias = "tbb_gain_index4")]
pub type TbbGainIndex4 = crate::Reg<tbb_gain_index4::TbbGainIndex4Spec>;
#[doc = "tbb_gain_index4."]
pub mod tbb_gain_index4;
#[doc = "pa_reg_ctrl_hw1 (rw) register accessor: pa_reg_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_reg_ctrl_hw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg_ctrl_hw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_reg_ctrl_hw1`] module"]
#[doc(alias = "pa_reg_ctrl_hw1")]
pub type PaRegCtrlHw1 = crate::Reg<pa_reg_ctrl_hw1::PaRegCtrlHw1Spec>;
#[doc = "pa_reg_ctrl_hw1."]
pub mod pa_reg_ctrl_hw1;
#[doc = "pa_reg_ctrl_hw2 (rw) register accessor: pa_reg_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_reg_ctrl_hw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg_ctrl_hw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_reg_ctrl_hw2`] module"]
#[doc(alias = "pa_reg_ctrl_hw2")]
pub type PaRegCtrlHw2 = crate::Reg<pa_reg_ctrl_hw2::PaRegCtrlHw2Spec>;
#[doc = "pa_reg_ctrl_hw2."]
pub mod pa_reg_ctrl_hw2;
#[doc = "pa_reg_wifi_ctrl_hw (rw) register accessor: pa_reg_wifi_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_reg_wifi_ctrl_hw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg_wifi_ctrl_hw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_reg_wifi_ctrl_hw`] module"]
#[doc(alias = "pa_reg_wifi_ctrl_hw")]
pub type PaRegWifiCtrlHw = crate::Reg<pa_reg_wifi_ctrl_hw::PaRegWifiCtrlHwSpec>;
#[doc = "pa_reg_wifi_ctrl_hw."]
pub mod pa_reg_wifi_ctrl_hw;
#[doc = "adda_reg_ctrl_hw (rw) register accessor: adda_reg_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`adda_reg_ctrl_hw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adda_reg_ctrl_hw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adda_reg_ctrl_hw`] module"]
#[doc(alias = "adda_reg_ctrl_hw")]
pub type AddaRegCtrlHw = crate::Reg<adda_reg_ctrl_hw::AddaRegCtrlHwSpec>;
#[doc = "adda_reg_ctrl_hw."]
pub mod adda_reg_ctrl_hw;
#[doc = "lo_reg_ctrl_hw1 (rw) register accessor: lo_reg_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_reg_ctrl_hw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_reg_ctrl_hw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_reg_ctrl_hw1`] module"]
#[doc(alias = "lo_reg_ctrl_hw1")]
pub type LoRegCtrlHw1 = crate::Reg<lo_reg_ctrl_hw1::LoRegCtrlHw1Spec>;
#[doc = "lo_reg_ctrl_hw1."]
pub mod lo_reg_ctrl_hw1;
#[doc = "lo_cal_ctrl_hw1 (rw) register accessor: lo_cal_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw1`] module"]
#[doc(alias = "lo_cal_ctrl_hw1")]
pub type LoCalCtrlHw1 = crate::Reg<lo_cal_ctrl_hw1::LoCalCtrlHw1Spec>;
#[doc = "lo_cal_ctrl_hw1."]
pub mod lo_cal_ctrl_hw1;
#[doc = "lo_cal_ctrl_hw2 (rw) register accessor: lo_cal_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw2`] module"]
#[doc(alias = "lo_cal_ctrl_hw2")]
pub type LoCalCtrlHw2 = crate::Reg<lo_cal_ctrl_hw2::LoCalCtrlHw2Spec>;
#[doc = "lo_cal_ctrl_hw2."]
pub mod lo_cal_ctrl_hw2;
#[doc = "lo_cal_ctrl_hw3 (rw) register accessor: lo_cal_ctrl_hw3.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw3`] module"]
#[doc(alias = "lo_cal_ctrl_hw3")]
pub type LoCalCtrlHw3 = crate::Reg<lo_cal_ctrl_hw3::LoCalCtrlHw3Spec>;
#[doc = "lo_cal_ctrl_hw3."]
pub mod lo_cal_ctrl_hw3;
#[doc = "lo_cal_ctrl_hw4 (rw) register accessor: lo_cal_ctrl_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw4`] module"]
#[doc(alias = "lo_cal_ctrl_hw4")]
pub type LoCalCtrlHw4 = crate::Reg<lo_cal_ctrl_hw4::LoCalCtrlHw4Spec>;
#[doc = "lo_cal_ctrl_hw4."]
pub mod lo_cal_ctrl_hw4;
#[doc = "lo_cal_ctrl_hw5 (rw) register accessor: lo_cal_ctrl_hw5.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw5`] module"]
#[doc(alias = "lo_cal_ctrl_hw5")]
pub type LoCalCtrlHw5 = crate::Reg<lo_cal_ctrl_hw5::LoCalCtrlHw5Spec>;
#[doc = "lo_cal_ctrl_hw5."]
pub mod lo_cal_ctrl_hw5;
#[doc = "lo_cal_ctrl_hw6 (rw) register accessor: lo_cal_ctrl_hw6.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw6`] module"]
#[doc(alias = "lo_cal_ctrl_hw6")]
pub type LoCalCtrlHw6 = crate::Reg<lo_cal_ctrl_hw6::LoCalCtrlHw6Spec>;
#[doc = "lo_cal_ctrl_hw6."]
pub mod lo_cal_ctrl_hw6;
#[doc = "lo_cal_ctrl_hw7 (rw) register accessor: lo_cal_ctrl_hw7.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw7`] module"]
#[doc(alias = "lo_cal_ctrl_hw7")]
pub type LoCalCtrlHw7 = crate::Reg<lo_cal_ctrl_hw7::LoCalCtrlHw7Spec>;
#[doc = "lo_cal_ctrl_hw7."]
pub mod lo_cal_ctrl_hw7;
#[doc = "lo_cal_ctrl_hw8 (rw) register accessor: lo_cal_ctrl_hw8.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw8`] module"]
#[doc(alias = "lo_cal_ctrl_hw8")]
pub type LoCalCtrlHw8 = crate::Reg<lo_cal_ctrl_hw8::LoCalCtrlHw8Spec>;
#[doc = "lo_cal_ctrl_hw8."]
pub mod lo_cal_ctrl_hw8;
#[doc = "lo_cal_ctrl_hw9 (rw) register accessor: lo_cal_ctrl_hw9.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw9`] module"]
#[doc(alias = "lo_cal_ctrl_hw9")]
pub type LoCalCtrlHw9 = crate::Reg<lo_cal_ctrl_hw9::LoCalCtrlHw9Spec>;
#[doc = "lo_cal_ctrl_hw9."]
pub mod lo_cal_ctrl_hw9;
#[doc = "lo_cal_ctrl_hw10 (rw) register accessor: lo_cal_ctrl_hw10.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw10`] module"]
#[doc(alias = "lo_cal_ctrl_hw10")]
pub type LoCalCtrlHw10 = crate::Reg<lo_cal_ctrl_hw10::LoCalCtrlHw10Spec>;
#[doc = "lo_cal_ctrl_hw10."]
pub mod lo_cal_ctrl_hw10;
#[doc = "lo_cal_ctrl_hw11 (rw) register accessor: lo_cal_ctrl_hw11.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_cal_ctrl_hw11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_cal_ctrl_hw11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_cal_ctrl_hw11`] module"]
#[doc(alias = "lo_cal_ctrl_hw11")]
pub type LoCalCtrlHw11 = crate::Reg<lo_cal_ctrl_hw11::LoCalCtrlHw11Spec>;
#[doc = "lo_cal_ctrl_hw11."]
pub mod lo_cal_ctrl_hw11;
#[doc = "rosdac_ctrl_hw1 (rw) register accessor: rosdac_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rosdac_ctrl_hw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rosdac_ctrl_hw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rosdac_ctrl_hw1`] module"]
#[doc(alias = "rosdac_ctrl_hw1")]
pub type RosdacCtrlHw1 = crate::Reg<rosdac_ctrl_hw1::RosdacCtrlHw1Spec>;
#[doc = "rosdac_ctrl_hw1."]
pub mod rosdac_ctrl_hw1;
#[doc = "rosdac_ctrl_hw2 (rw) register accessor: rosdac_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rosdac_ctrl_hw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rosdac_ctrl_hw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rosdac_ctrl_hw2`] module"]
#[doc(alias = "rosdac_ctrl_hw2")]
pub type RosdacCtrlHw2 = crate::Reg<rosdac_ctrl_hw2::RosdacCtrlHw2Spec>;
#[doc = "rosdac_ctrl_hw2."]
pub mod rosdac_ctrl_hw2;
#[doc = "rxiq_ctrl_hw1 (rw) register accessor: rxiq_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxiq_ctrl_hw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxiq_ctrl_hw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxiq_ctrl_hw1`] module"]
#[doc(alias = "rxiq_ctrl_hw1")]
pub type RxiqCtrlHw1 = crate::Reg<rxiq_ctrl_hw1::RxiqCtrlHw1Spec>;
#[doc = "rxiq_ctrl_hw1."]
pub mod rxiq_ctrl_hw1;
#[doc = "rxiq_ctrl_hw2 (rw) register accessor: rxiq_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxiq_ctrl_hw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxiq_ctrl_hw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxiq_ctrl_hw2`] module"]
#[doc(alias = "rxiq_ctrl_hw2")]
pub type RxiqCtrlHw2 = crate::Reg<rxiq_ctrl_hw2::RxiqCtrlHw2Spec>;
#[doc = "rxiq_ctrl_hw2."]
pub mod rxiq_ctrl_hw2;
#[doc = "rxiq_ctrl_hw3 (rw) register accessor: rxiq_ctrl_hw3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxiq_ctrl_hw3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxiq_ctrl_hw3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxiq_ctrl_hw3`] module"]
#[doc(alias = "rxiq_ctrl_hw3")]
pub type RxiqCtrlHw3 = crate::Reg<rxiq_ctrl_hw3::RxiqCtrlHw3Spec>;
#[doc = "rxiq_ctrl_hw3."]
pub mod rxiq_ctrl_hw3;
#[doc = "rxiq_ctrl_hw4 (rw) register accessor: rxiq_ctrl_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxiq_ctrl_hw4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxiq_ctrl_hw4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxiq_ctrl_hw4`] module"]
#[doc(alias = "rxiq_ctrl_hw4")]
pub type RxiqCtrlHw4 = crate::Reg<rxiq_ctrl_hw4::RxiqCtrlHw4Spec>;
#[doc = "rxiq_ctrl_hw4."]
pub mod rxiq_ctrl_hw4;
#[doc = "tosdac_ctrl_hw1 (rw) register accessor: tosdac_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tosdac_ctrl_hw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tosdac_ctrl_hw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tosdac_ctrl_hw1`] module"]
#[doc(alias = "tosdac_ctrl_hw1")]
pub type TosdacCtrlHw1 = crate::Reg<tosdac_ctrl_hw1::TosdacCtrlHw1Spec>;
#[doc = "tosdac_ctrl_hw1."]
pub mod tosdac_ctrl_hw1;
#[doc = "tosdac_ctrl_hw2 (rw) register accessor: tosdac_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tosdac_ctrl_hw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tosdac_ctrl_hw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tosdac_ctrl_hw2`] module"]
#[doc(alias = "tosdac_ctrl_hw2")]
pub type TosdacCtrlHw2 = crate::Reg<tosdac_ctrl_hw2::TosdacCtrlHw2Spec>;
#[doc = "tosdac_ctrl_hw2."]
pub mod tosdac_ctrl_hw2;
#[doc = "tosdac_ctrl_hw3 (rw) register accessor: tosdac_ctrl_hw3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tosdac_ctrl_hw3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tosdac_ctrl_hw3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tosdac_ctrl_hw3`] module"]
#[doc(alias = "tosdac_ctrl_hw3")]
pub type TosdacCtrlHw3 = crate::Reg<tosdac_ctrl_hw3::TosdacCtrlHw3Spec>;
#[doc = "tosdac_ctrl_hw3."]
pub mod tosdac_ctrl_hw3;
#[doc = "tosdac_ctrl_hw4 (rw) register accessor: tosdac_ctrl_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`tosdac_ctrl_hw4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tosdac_ctrl_hw4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tosdac_ctrl_hw4`] module"]
#[doc(alias = "tosdac_ctrl_hw4")]
pub type TosdacCtrlHw4 = crate::Reg<tosdac_ctrl_hw4::TosdacCtrlHw4Spec>;
#[doc = "tosdac_ctrl_hw4."]
pub mod tosdac_ctrl_hw4;
#[doc = "tx_iq_gain_hw0 (rw) register accessor: tx_iq_gain_hw0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_iq_gain_hw0`] module"]
#[doc(alias = "tx_iq_gain_hw0")]
pub type TxIqGainHw0 = crate::Reg<tx_iq_gain_hw0::TxIqGainHw0Spec>;
#[doc = "tx_iq_gain_hw0."]
pub mod tx_iq_gain_hw0;
#[doc = "tx_iq_gain_hw1 (rw) register accessor: tx_iq_gain_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_iq_gain_hw1`] module"]
#[doc(alias = "tx_iq_gain_hw1")]
pub type TxIqGainHw1 = crate::Reg<tx_iq_gain_hw1::TxIqGainHw1Spec>;
#[doc = "tx_iq_gain_hw1."]
pub mod tx_iq_gain_hw1;
#[doc = "tx_iq_gain_hw2 (rw) register accessor: tx_iq_gain_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_iq_gain_hw2`] module"]
#[doc(alias = "tx_iq_gain_hw2")]
pub type TxIqGainHw2 = crate::Reg<tx_iq_gain_hw2::TxIqGainHw2Spec>;
#[doc = "tx_iq_gain_hw2."]
pub mod tx_iq_gain_hw2;
#[doc = "tx_iq_gain_hw3 (rw) register accessor: tx_iq_gain_hw3.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_iq_gain_hw3`] module"]
#[doc(alias = "tx_iq_gain_hw3")]
pub type TxIqGainHw3 = crate::Reg<tx_iq_gain_hw3::TxIqGainHw3Spec>;
#[doc = "tx_iq_gain_hw3."]
pub mod tx_iq_gain_hw3;
#[doc = "tx_iq_gain_hw4 (rw) register accessor: tx_iq_gain_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_iq_gain_hw4`] module"]
#[doc(alias = "tx_iq_gain_hw4")]
pub type TxIqGainHw4 = crate::Reg<tx_iq_gain_hw4::TxIqGainHw4Spec>;
#[doc = "tx_iq_gain_hw4."]
pub mod tx_iq_gain_hw4;
#[doc = "tx_iq_gain_hw5 (rw) register accessor: tx_iq_gain_hw5.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_iq_gain_hw5`] module"]
#[doc(alias = "tx_iq_gain_hw5")]
pub type TxIqGainHw5 = crate::Reg<tx_iq_gain_hw5::TxIqGainHw5Spec>;
#[doc = "tx_iq_gain_hw5."]
pub mod tx_iq_gain_hw5;
#[doc = "tx_iq_gain_hw6 (rw) register accessor: tx_iq_gain_hw6.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_iq_gain_hw6`] module"]
#[doc(alias = "tx_iq_gain_hw6")]
pub type TxIqGainHw6 = crate::Reg<tx_iq_gain_hw6::TxIqGainHw6Spec>;
#[doc = "tx_iq_gain_hw6."]
pub mod tx_iq_gain_hw6;
#[doc = "tx_iq_gain_hw7 (rw) register accessor: tx_iq_gain_hw7.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_iq_gain_hw7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_iq_gain_hw7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_iq_gain_hw7`] module"]
#[doc(alias = "tx_iq_gain_hw7")]
pub type TxIqGainHw7 = crate::Reg<tx_iq_gain_hw7::TxIqGainHw7Spec>;
#[doc = "tx_iq_gain_hw7."]
pub mod tx_iq_gain_hw7;
#[doc = "lo_sdm_ctrl_hw1 (rw) register accessor: lo_sdm_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_sdm_ctrl_hw1`] module"]
#[doc(alias = "lo_sdm_ctrl_hw1")]
pub type LoSdmCtrlHw1 = crate::Reg<lo_sdm_ctrl_hw1::LoSdmCtrlHw1Spec>;
#[doc = "lo_sdm_ctrl_hw1."]
pub mod lo_sdm_ctrl_hw1;
#[doc = "lo_sdm_ctrl_hw2 (rw) register accessor: lo_sdm_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_sdm_ctrl_hw2`] module"]
#[doc(alias = "lo_sdm_ctrl_hw2")]
pub type LoSdmCtrlHw2 = crate::Reg<lo_sdm_ctrl_hw2::LoSdmCtrlHw2Spec>;
#[doc = "lo_sdm_ctrl_hw2."]
pub mod lo_sdm_ctrl_hw2;
#[doc = "lo_sdm_ctrl_hw3 (rw) register accessor: lo_sdm_ctrl_hw3.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_sdm_ctrl_hw3`] module"]
#[doc(alias = "lo_sdm_ctrl_hw3")]
pub type LoSdmCtrlHw3 = crate::Reg<lo_sdm_ctrl_hw3::LoSdmCtrlHw3Spec>;
#[doc = "lo_sdm_ctrl_hw3."]
pub mod lo_sdm_ctrl_hw3;
#[doc = "lo_sdm_ctrl_hw4 (rw) register accessor: lo_sdm_ctrl_hw4.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_sdm_ctrl_hw4`] module"]
#[doc(alias = "lo_sdm_ctrl_hw4")]
pub type LoSdmCtrlHw4 = crate::Reg<lo_sdm_ctrl_hw4::LoSdmCtrlHw4Spec>;
#[doc = "lo_sdm_ctrl_hw4."]
pub mod lo_sdm_ctrl_hw4;
#[doc = "lo_sdm_ctrl_hw5 (rw) register accessor: lo_sdm_ctrl_hw5.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_sdm_ctrl_hw5`] module"]
#[doc(alias = "lo_sdm_ctrl_hw5")]
pub type LoSdmCtrlHw5 = crate::Reg<lo_sdm_ctrl_hw5::LoSdmCtrlHw5Spec>;
#[doc = "lo_sdm_ctrl_hw5."]
pub mod lo_sdm_ctrl_hw5;
#[doc = "lo_sdm_ctrl_hw6 (rw) register accessor: lo_sdm_ctrl_hw6.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_sdm_ctrl_hw6`] module"]
#[doc(alias = "lo_sdm_ctrl_hw6")]
pub type LoSdmCtrlHw6 = crate::Reg<lo_sdm_ctrl_hw6::LoSdmCtrlHw6Spec>;
#[doc = "lo_sdm_ctrl_hw6."]
pub mod lo_sdm_ctrl_hw6;
#[doc = "lo_sdm_ctrl_hw7 (rw) register accessor: lo_sdm_ctrl_hw7.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_sdm_ctrl_hw7`] module"]
#[doc(alias = "lo_sdm_ctrl_hw7")]
pub type LoSdmCtrlHw7 = crate::Reg<lo_sdm_ctrl_hw7::LoSdmCtrlHw7Spec>;
#[doc = "lo_sdm_ctrl_hw7."]
pub mod lo_sdm_ctrl_hw7;
#[doc = "lo_sdm_ctrl_hw8 (rw) register accessor: lo_sdm_ctrl_hw8.\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_sdm_ctrl_hw8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lo_sdm_ctrl_hw8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_sdm_ctrl_hw8`] module"]
#[doc(alias = "lo_sdm_ctrl_hw8")]
pub type LoSdmCtrlHw8 = crate::Reg<lo_sdm_ctrl_hw8::LoSdmCtrlHw8Spec>;
#[doc = "lo_sdm_ctrl_hw8."]
pub mod lo_sdm_ctrl_hw8;
#[doc = "rbb_bw_ctrl_hw (rw) register accessor: rbb_bw_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_bw_ctrl_hw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_bw_ctrl_hw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbb_bw_ctrl_hw`] module"]
#[doc(alias = "rbb_bw_ctrl_hw")]
pub type RbbBwCtrlHw = crate::Reg<rbb_bw_ctrl_hw::RbbBwCtrlHwSpec>;
#[doc = "rbb_bw_ctrl_hw."]
pub mod rbb_bw_ctrl_hw;
#[doc = "singen_ctrl0 (rw) register accessor: singen_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singen_ctrl0`] module"]
#[doc(alias = "singen_ctrl0")]
pub type SingenCtrl0 = crate::Reg<singen_ctrl0::SingenCtrl0Spec>;
#[doc = "singen_ctrl0."]
pub mod singen_ctrl0;
#[doc = "singen_ctrl1 (rw) register accessor: singen_ctrl1.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singen_ctrl1`] module"]
#[doc(alias = "singen_ctrl1")]
pub type SingenCtrl1 = crate::Reg<singen_ctrl1::SingenCtrl1Spec>;
#[doc = "singen_ctrl1."]
pub mod singen_ctrl1;
#[doc = "singen_ctrl2 (rw) register accessor: singen_ctrl2.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singen_ctrl2`] module"]
#[doc(alias = "singen_ctrl2")]
pub type SingenCtrl2 = crate::Reg<singen_ctrl2::SingenCtrl2Spec>;
#[doc = "singen_ctrl2."]
pub mod singen_ctrl2;
#[doc = "singen_ctrl3 (rw) register accessor: singen_ctrl3.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singen_ctrl3`] module"]
#[doc(alias = "singen_ctrl3")]
pub type SingenCtrl3 = crate::Reg<singen_ctrl3::SingenCtrl3Spec>;
#[doc = "singen_ctrl3."]
pub mod singen_ctrl3;
#[doc = "singen_ctrl4 (rw) register accessor: singen_ctrl4.\n\nYou can [`read`](crate::Reg::read) this register and get [`singen_ctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singen_ctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singen_ctrl4`] module"]
#[doc(alias = "singen_ctrl4")]
pub type SingenCtrl4 = crate::Reg<singen_ctrl4::SingenCtrl4Spec>;
#[doc = "singen_ctrl4."]
pub mod singen_ctrl4;
#[doc = "rfif_dfe_ctrl0 (rw) register accessor: rfif_dfe_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfif_dfe_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfif_dfe_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfif_dfe_ctrl0`] module"]
#[doc(alias = "rfif_dfe_ctrl0")]
pub type RfifDfeCtrl0 = crate::Reg<rfif_dfe_ctrl0::RfifDfeCtrl0Spec>;
#[doc = "rfif_dfe_ctrl0."]
pub mod rfif_dfe_ctrl0;
#[doc = "rfif_test_read (rw) register accessor: rfif_test_read.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfif_test_read::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfif_test_read::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfif_test_read`] module"]
#[doc(alias = "rfif_test_read")]
pub type RfifTestRead = crate::Reg<rfif_test_read::RfifTestReadSpec>;
#[doc = "rfif_test_read."]
pub mod rfif_test_read;
#[doc = "rfif_dig_ctrl (rw) register accessor: rfif_dig_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfif_dig_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfif_dig_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfif_dig_ctrl`] module"]
#[doc(alias = "rfif_dig_ctrl")]
pub type RfifDigCtrl = crate::Reg<rfif_dig_ctrl::RfifDigCtrlSpec>;
#[doc = "rfif_dig_ctrl."]
pub mod rfif_dig_ctrl;
#[doc = "rf_data_temp_0 (rw) register accessor: rf_data_temp_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_data_temp_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_data_temp_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_data_temp_0`] module"]
#[doc(alias = "rf_data_temp_0")]
pub type RfDataTemp0 = crate::Reg<rf_data_temp_0::RfDataTemp0Spec>;
#[doc = "rf_data_temp_0."]
pub mod rf_data_temp_0;
#[doc = "rf_data_temp_1 (rw) register accessor: rf_data_temp_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_data_temp_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_data_temp_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_data_temp_1`] module"]
#[doc(alias = "rf_data_temp_1")]
pub type RfDataTemp1 = crate::Reg<rf_data_temp_1::RfDataTemp1Spec>;
#[doc = "rf_data_temp_1."]
pub mod rf_data_temp_1;
#[doc = "rf_data_temp_2 (rw) register accessor: rf_data_temp_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_data_temp_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_data_temp_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_data_temp_2`] module"]
#[doc(alias = "rf_data_temp_2")]
pub type RfDataTemp2 = crate::Reg<rf_data_temp_2::RfDataTemp2Spec>;
#[doc = "rf_data_temp_2."]
pub mod rf_data_temp_2;
#[doc = "rf_data_temp_3 (rw) register accessor: rf_data_temp_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_data_temp_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_data_temp_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_data_temp_3`] module"]
#[doc(alias = "rf_data_temp_3")]
pub type RfDataTemp3 = crate::Reg<rf_data_temp_3::RfDataTemp3Spec>;
#[doc = "rf_data_temp_3."]
pub mod rf_data_temp_3;
#[doc = "rf_sram_ctrl0 (rw) register accessor: rf_sram_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_sram_ctrl0`] module"]
#[doc(alias = "rf_sram_ctrl0")]
pub type RfSramCtrl0 = crate::Reg<rf_sram_ctrl0::RfSramCtrl0Spec>;
#[doc = "rf_sram_ctrl0."]
pub mod rf_sram_ctrl0;
#[doc = "rf_sram_ctrl1 (rw) register accessor: rf_sram_ctrl1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_sram_ctrl1`] module"]
#[doc(alias = "rf_sram_ctrl1")]
pub type RfSramCtrl1 = crate::Reg<rf_sram_ctrl1::RfSramCtrl1Spec>;
#[doc = "rf_sram_ctrl1."]
pub mod rf_sram_ctrl1;
#[doc = "rf_sram_ctrl2 (rw) register accessor: rf_sram_ctrl2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_sram_ctrl2`] module"]
#[doc(alias = "rf_sram_ctrl2")]
pub type RfSramCtrl2 = crate::Reg<rf_sram_ctrl2::RfSramCtrl2Spec>;
#[doc = "rf_sram_ctrl2."]
pub mod rf_sram_ctrl2;
#[doc = "rf_sram_ctrl3 (rw) register accessor: rf_sram_ctrl3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_sram_ctrl3`] module"]
#[doc(alias = "rf_sram_ctrl3")]
pub type RfSramCtrl3 = crate::Reg<rf_sram_ctrl3::RfSramCtrl3Spec>;
#[doc = "rf_sram_ctrl3."]
pub mod rf_sram_ctrl3;
#[doc = "rf_sram_ctrl4 (rw) register accessor: rf_sram_ctrl4.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_sram_ctrl4`] module"]
#[doc(alias = "rf_sram_ctrl4")]
pub type RfSramCtrl4 = crate::Reg<rf_sram_ctrl4::RfSramCtrl4Spec>;
#[doc = "rf_sram_ctrl4."]
pub mod rf_sram_ctrl4;
#[doc = "rf_sram_ctrl5 (rw) register accessor: rf_sram_ctrl5.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_sram_ctrl5`] module"]
#[doc(alias = "rf_sram_ctrl5")]
pub type RfSramCtrl5 = crate::Reg<rf_sram_ctrl5::RfSramCtrl5Spec>;
#[doc = "rf_sram_ctrl5."]
pub mod rf_sram_ctrl5;
#[doc = "rf_sram_ctrl6 (rw) register accessor: rf_sram_ctrl6.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_sram_ctrl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_sram_ctrl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_sram_ctrl6`] module"]
#[doc(alias = "rf_sram_ctrl6")]
pub type RfSramCtrl6 = crate::Reg<rf_sram_ctrl6::RfSramCtrl6Spec>;
#[doc = "rf_sram_ctrl6."]
pub mod rf_sram_ctrl6;
#[doc = "rf_ical_ctrl0 (rw) register accessor: rf_ical_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_ical_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_ical_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_ical_ctrl0`] module"]
#[doc(alias = "rf_ical_ctrl0")]
pub type RfIcalCtrl0 = crate::Reg<rf_ical_ctrl0::RfIcalCtrl0Spec>;
#[doc = "rf_ical_ctrl0."]
pub mod rf_ical_ctrl0;
#[doc = "rf_ical_ctrl1 (rw) register accessor: rf_ical_ctrl1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_ical_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_ical_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_ical_ctrl1`] module"]
#[doc(alias = "rf_ical_ctrl1")]
pub type RfIcalCtrl1 = crate::Reg<rf_ical_ctrl1::RfIcalCtrl1Spec>;
#[doc = "rf_ical_ctrl1."]
pub mod rf_ical_ctrl1;
#[doc = "rf_ical_ctrl2 (rw) register accessor: rf_ical_ctrl2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_ical_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_ical_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_ical_ctrl2`] module"]
#[doc(alias = "rf_ical_ctrl2")]
pub type RfIcalCtrl2 = crate::Reg<rf_ical_ctrl2::RfIcalCtrl2Spec>;
#[doc = "rf_ical_ctrl2."]
pub mod rf_ical_ctrl2;
#[doc = "rf_fsm_ctrl0 (rw) register accessor: rf_fsm_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm_ctrl0`] module"]
#[doc(alias = "rf_fsm_ctrl0")]
pub type RfFsmCtrl0 = crate::Reg<rf_fsm_ctrl0::RfFsmCtrl0Spec>;
#[doc = "rf_fsm_ctrl0."]
pub mod rf_fsm_ctrl0;
#[doc = "rf_fsm_ctrl1 (rw) register accessor: rf_fsm_ctrl1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm_ctrl1`] module"]
#[doc(alias = "rf_fsm_ctrl1")]
pub type RfFsmCtrl1 = crate::Reg<rf_fsm_ctrl1::RfFsmCtrl1Spec>;
#[doc = "rf_fsm_ctrl1."]
pub mod rf_fsm_ctrl1;
#[doc = "rf_fsm_ctrl2 (rw) register accessor: rf_fsm_ctrl2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_fsm_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_fsm_ctrl2`] module"]
#[doc(alias = "rf_fsm_ctrl2")]
pub type RfFsmCtrl2 = crate::Reg<rf_fsm_ctrl2::RfFsmCtrl2Spec>;
#[doc = "rf_fsm_ctrl2."]
pub mod rf_fsm_ctrl2;
#[doc = "rf_pkdet_ctrl0 (rw) register accessor: rf_pkdet_ctrl0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_pkdet_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_pkdet_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_pkdet_ctrl0`] module"]
#[doc(alias = "rf_pkdet_ctrl0")]
pub type RfPkdetCtrl0 = crate::Reg<rf_pkdet_ctrl0::RfPkdetCtrl0Spec>;
#[doc = "rf_pkdet_ctrl0."]
pub mod rf_pkdet_ctrl0;
#[doc = "dfe_ctrl_0 (rw) register accessor: dfe_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_0`] module"]
#[doc(alias = "dfe_ctrl_0")]
pub type DfeCtrl0 = crate::Reg<dfe_ctrl_0::DfeCtrl0Spec>;
#[doc = "dfe_ctrl_0."]
pub mod dfe_ctrl_0;
#[doc = "dfe_ctrl_1 (rw) register accessor: dfe_ctrl_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_1`] module"]
#[doc(alias = "dfe_ctrl_1")]
pub type DfeCtrl1 = crate::Reg<dfe_ctrl_1::DfeCtrl1Spec>;
#[doc = "dfe_ctrl_1."]
pub mod dfe_ctrl_1;
#[doc = "dfe_ctrl_2 (rw) register accessor: dfe_ctrl_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_2`] module"]
#[doc(alias = "dfe_ctrl_2")]
pub type DfeCtrl2 = crate::Reg<dfe_ctrl_2::DfeCtrl2Spec>;
#[doc = "dfe_ctrl_2."]
pub mod dfe_ctrl_2;
#[doc = "dfe_ctrl_3 (rw) register accessor: dfe_ctrl_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_3`] module"]
#[doc(alias = "dfe_ctrl_3")]
pub type DfeCtrl3 = crate::Reg<dfe_ctrl_3::DfeCtrl3Spec>;
#[doc = "dfe_ctrl_3."]
pub mod dfe_ctrl_3;
#[doc = "dfe_ctrl_4 (rw) register accessor: dfe_ctrl_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_4`] module"]
#[doc(alias = "dfe_ctrl_4")]
pub type DfeCtrl4 = crate::Reg<dfe_ctrl_4::DfeCtrl4Spec>;
#[doc = "dfe_ctrl_4."]
pub mod dfe_ctrl_4;
#[doc = "dfe_ctrl_5 (rw) register accessor: dfe_ctrl_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_5`] module"]
#[doc(alias = "dfe_ctrl_5")]
pub type DfeCtrl5 = crate::Reg<dfe_ctrl_5::DfeCtrl5Spec>;
#[doc = "dfe_ctrl_5."]
pub mod dfe_ctrl_5;
#[doc = "dfe_ctrl_6 (rw) register accessor: dfe_ctrl_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_6`] module"]
#[doc(alias = "dfe_ctrl_6")]
pub type DfeCtrl6 = crate::Reg<dfe_ctrl_6::DfeCtrl6Spec>;
#[doc = "dfe_ctrl_6."]
pub mod dfe_ctrl_6;
#[doc = "dfe_ctrl_7 (rw) register accessor: dfe_ctrl_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_7`] module"]
#[doc(alias = "dfe_ctrl_7")]
pub type DfeCtrl7 = crate::Reg<dfe_ctrl_7::DfeCtrl7Spec>;
#[doc = "dfe_ctrl_7."]
pub mod dfe_ctrl_7;
#[doc = "dfe_ctrl_8 (rw) register accessor: dfe_ctrl_8.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_8`] module"]
#[doc(alias = "dfe_ctrl_8")]
pub type DfeCtrl8 = crate::Reg<dfe_ctrl_8::DfeCtrl8Spec>;
#[doc = "dfe_ctrl_8."]
pub mod dfe_ctrl_8;
#[doc = "dfe_ctrl_9 (rw) register accessor: dfe_ctrl_9.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_9`] module"]
#[doc(alias = "dfe_ctrl_9")]
pub type DfeCtrl9 = crate::Reg<dfe_ctrl_9::DfeCtrl9Spec>;
#[doc = "dfe_ctrl_9."]
pub mod dfe_ctrl_9;
#[doc = "dfe_ctrl_10 (rw) register accessor: dfe_ctrl_10.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_10`] module"]
#[doc(alias = "dfe_ctrl_10")]
pub type DfeCtrl10 = crate::Reg<dfe_ctrl_10::DfeCtrl10Spec>;
#[doc = "dfe_ctrl_10."]
pub mod dfe_ctrl_10;
#[doc = "dfe_ctrl_11 (rw) register accessor: dfe_ctrl_11.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_11`] module"]
#[doc(alias = "dfe_ctrl_11")]
pub type DfeCtrl11 = crate::Reg<dfe_ctrl_11::DfeCtrl11Spec>;
#[doc = "dfe_ctrl_11."]
pub mod dfe_ctrl_11;
#[doc = "dfe_ctrl_12 (rw) register accessor: dfe_ctrl_12.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_12`] module"]
#[doc(alias = "dfe_ctrl_12")]
pub type DfeCtrl12 = crate::Reg<dfe_ctrl_12::DfeCtrl12Spec>;
#[doc = "dfe_ctrl_12."]
pub mod dfe_ctrl_12;
#[doc = "dfe_ctrl_13 (rw) register accessor: dfe_ctrl_13.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_13`] module"]
#[doc(alias = "dfe_ctrl_13")]
pub type DfeCtrl13 = crate::Reg<dfe_ctrl_13::DfeCtrl13Spec>;
#[doc = "dfe_ctrl_13."]
pub mod dfe_ctrl_13;
#[doc = "dfe_ctrl_14 (rw) register accessor: dfe_ctrl_14.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_14`] module"]
#[doc(alias = "dfe_ctrl_14")]
pub type DfeCtrl14 = crate::Reg<dfe_ctrl_14::DfeCtrl14Spec>;
#[doc = "dfe_ctrl_14."]
pub mod dfe_ctrl_14;
#[doc = "dfe_ctrl_15 (rw) register accessor: dfe_ctrl_15.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_15`] module"]
#[doc(alias = "dfe_ctrl_15")]
pub type DfeCtrl15 = crate::Reg<dfe_ctrl_15::DfeCtrl15Spec>;
#[doc = "dfe_ctrl_15."]
pub mod dfe_ctrl_15;
#[doc = "dfe_ctrl_16 (rw) register accessor: dfe_ctrl_16.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_16`] module"]
#[doc(alias = "dfe_ctrl_16")]
pub type DfeCtrl16 = crate::Reg<dfe_ctrl_16::DfeCtrl16Spec>;
#[doc = "dfe_ctrl_16."]
pub mod dfe_ctrl_16;
#[doc = "dfe_ctrl_17 (rw) register accessor: dfe_ctrl_17.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_17`] module"]
#[doc(alias = "dfe_ctrl_17")]
pub type DfeCtrl17 = crate::Reg<dfe_ctrl_17::DfeCtrl17Spec>;
#[doc = "dfe_ctrl_17."]
pub mod dfe_ctrl_17;
#[doc = "dfe_ctrl_18 (rw) register accessor: dfe_ctrl_18.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfe_ctrl_18`] module"]
#[doc(alias = "dfe_ctrl_18")]
pub type DfeCtrl18 = crate::Reg<dfe_ctrl_18::DfeCtrl18Spec>;
#[doc = "dfe_ctrl_18."]
pub mod dfe_ctrl_18;
