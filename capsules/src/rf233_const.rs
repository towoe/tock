#![allow(non_camel_case_types)]

#[derive(PartialEq, Copy, Clone)]
pub enum RF233Register {
    MIN           = 0x00,
    TRX_STATUS    = 0x01,
    TRX_STATE     = 0x02,
    TRX_CTRL_0    = 0x03,
    TRX_CTRL_1    = 0x04,
    PHY_TX_PWR    = 0x05,
    PHY_RSSI      = 0x06,
    PHY_ED_LEVEL  = 0x07,
    PHY_CC_CCA    = 0x08,
    CCA_THRES     = 0x09,
    RX_CTRL       = 0x0A,
    SFD_VALUE     = 0x0B,
    TRX_CTRL_2    = 0x0C,
    ANT_DIV       = 0x0D,
    IRQ_MASK      = 0x0E,
    IRQ_STATUS    = 0x0F,
    VCTRL         = 0x10,
    BATMON        = 0x11,
    XOSC_CTRL     = 0x12,
    CC_CTRL_0     = 0x13,
    CC_CTRL_1     = 0x14,
    RX_SYN        = 0x15,
    TRX_RPC       = 0x16,
    XAH_CTRL_1    = 0x17,
    FTN_CTRL      = 0x18,
    XAH_CTRL_2    = 0x19,
    PLL_CF        = 0x1A,
    PLL_DCU       = 0x1B,
    PART_NUM      = 0x1C,
    VERSION_NUM   = 0x1D,
    MAN_ID_0      = 0x1E,
    MAN_ID_1      = 0x1F,
    SHORT_ADDR_0  = 0x20,
    SHORT_ADDR_1  = 0x21,
    PAN_ID_0      = 0x22,
    PAN_ID_1      = 0x23,
    IEEE_ADDR_0   = 0x24,
    IEEE_ADDR_1   = 0x25,
    IEEE_ADDR_2   = 0x26,
    IEEE_ADDR_3   = 0x27,
    IEEE_ADDR_4   = 0x28,
    IEEE_ADDR_5   = 0x29,
    IEEE_ADDR_6   = 0x2A,
    IEEE_ADDR_7   = 0x2B,
    XAH_CTRL_0    = 0x2C,
    CSMA_SEED_0   = 0x2D,
    CSMA_SEED_1   = 0x2E,
    CSMA_BE       = 0x2F,
    TST_CTRL_DIGI = 0x36,
    PHY_TX_TIME   = 0x3B,
    TST_AGC       = 0x3C,
    TST_SDM       = 0x3D,
    MAX           = 0x3E,
}

pub const TRX_CTRL_1_DIG34_RXTX_INDICATOR: u8 = 1 << 7;
pub const TRX_CTRL_1_SPI_CMD_TRX_STATUS: u8   = 1 << 2;
pub const TRX_CTRL_1_AUTO_CRC: u8             = 1 << 5;
pub const PHY_TX_PWR_4: u8                    = 0;
pub const PHY_CC_CCA_MODE_CS_OR_ED: u8        = 0 << 5;
pub const PHY_CC_CCA_MODE_ED: u8              = 1 << 5;
pub const PHY_CC_CCA_MODE_CS: u8              = 2 << 5;
pub const PHY_CC_CCA_MODE_CS_AND_ED: u8       = 3 << 5;
pub const TRX_CTRL_2_RX_SAFE_MODE: u8         = 1 << 7;
pub const TRX_CTRL_2_DATA_RATE_250: u8        = 0;
pub const IRQ_TRXBUF_ACCESS_VIOLATION: u8     = 1 << 6;
pub const IRQ_TRX_DONE: u8                    = 1 << 3;
pub const IRQ_RX_START: u8                    = 1 << 2;
pub const IRQ_PLL_LOCK: u8                    = 1 << 0;
pub const XAH_CTRL_1_AACK_PROM_MODE: u8       = 1 << 1;


pub const TRX_CTRL_1:u8   = (TRX_CTRL_1_DIG34_RXTX_INDICATOR |
                         TRX_CTRL_1_SPI_CMD_TRX_STATUS |
                         TRX_CTRL_1_AUTO_CRC);
pub const TRX_CTRL_2:u8   = (TRX_CTRL_2_RX_SAFE_MODE |
                      TRX_CTRL_2_DATA_RATE_250);
pub const PHY_CC_CCA:u8   = 26 | PHY_CC_CCA_MODE_CS_OR_ED;
pub const PHY_TX_PWR:u8   = PHY_TX_PWR_4;
pub const IRQ_MASK:u8     = (IRQ_TRXBUF_ACCESS_VIOLATION |
                             IRQ_TRX_DONE |
                             IRQ_PLL_LOCK |
                             IRQ_RX_START);
pub const XAH_CTRL_1:u8   = XAH_CTRL_1_AACK_PROM_MODE;
pub const XAH_CTRL_0:u8   = 0;
pub const PAN_ID_0:u8     = 0x22;
pub const PAN_ID_1:u8     = 0x22;
pub const IEEE_ADDR_0:u8  = 0x11;
pub const IEEE_ADDR_1:u8  = 0x22;
pub const IEEE_ADDR_2:u8  = 0x33;
pub const IEEE_ADDR_3:u8  = 0x44;
pub const IEEE_ADDR_4:u8  = 0x55;
pub const IEEE_ADDR_5:u8  = 0x66;
pub const IEEE_ADDR_6:u8  = 0x77;
pub const IEEE_ADDR_7:u8  = 0x88;
pub const SHORT_ADDR_0:u8 = 0x11;
pub const SHORT_ADDR_1:u8 = 0x22;
pub const TRX_RPC:u8      = 0xFF;

pub const IRQ_7_BAT_LOW:u8     = 0x80;
pub const IRQ_6_TRX_UR:u8      = 0x40;
pub const IRQ_5_AMI:u8         = 0x20;
pub const IRQ_4_CCA_ED_DONE:u8 = 0x10;
pub const IRQ_3_TRX_END:u8     = 0x08;
pub const IRQ_2_RX_START:u8    = 0x04;
pub const IRQ_1_PLL_UNLOCK:u8  = 0x02;
pub const IRQ_0_PLL_LOCK:u8    = 0x01;

#[derive(PartialEq, Copy, Clone)]
pub enum RF233BusCommand {
    REGISTER_READ     = 0x80,
    REGISTER_WRITE    = 0xC0,
    FRAME_READ   = 0x20,
    FRAME_WRITE  = 0x60,
    SRAM_READ    = 0x00,
    SRAM_WRITE   = 0x40,
}
#[derive(PartialEq, Copy, Clone)]
pub enum ExternalState {
    ON              = 0x00,
    BUSY_RX         = 0x01,
    BUSY_TX         = 0x02,
    RX_ON           = 0x06,
    TRX_OFF         = 0x08,
    PLL_ON          = 0x09,
    SLEEP           = 0x0F,
    PREP_DEEP_SLEEP = 0x10,
    BUSY_RX_AACK    = 0x11,
    BUSY_TX_ARET    = 0x12,
    RX_AACK_ON      = 0x16,
    TX_ARET_ON      = 0x19,
    STATE_TRANSITION_IN_PROGRESS   = 0x1F,

}

pub enum RF233TrxCmd {
    TX_START   = 0x02,
    RX_ON      = 0x06,
    OFF        = 0x08,
    PLL_ON     = 0x09,
    TX_ARET_ON = 0x19,
}
