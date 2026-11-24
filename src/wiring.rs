use crate::swd::Swd;
use embassy_stm32::{
    Peripherals, bind_interrupts,
    can::Can,
    gpio::{Flex, Level, Output, Speed},
    peripherals,
    usart::{BufferedInterruptHandler, BufferedUart, Config},
};
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    UART7 => BufferedInterruptHandler<peripherals::UART7>;
});

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

static TX_BUF: StaticCell<[u8; 256]> = StaticCell::new();
static RX_BUF: StaticCell<[u8; 256]> = StaticCell::new();

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    let swclk = Output::new(p.PE9, Level::Low, Speed::Low);
    let swdio = Flex::new(p.PE13);
    let swd = Some(Swd::new(swclk, swdio));

    let mut config = Config::default();
    config.baudrate = 115_200;
    let uart = Some(
        BufferedUart::new(
            p.UART7,
            p.PE7,
            p.PE8,
            TX_BUF.init([0; 256]),
            RX_BUF.init([0; 256]),
            Irqs,
            config,
        )
        .unwrap(),
    );

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);
    (usb, Ports { can, uart }, swd)
}
