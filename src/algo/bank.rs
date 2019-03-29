//!某银行营业厅共有 6 个营业窗口，设有排队系统广播叫号，该银行的业务分为公积金、 银行卡、理财卡等三种。
//! 公积金业务指定 1 号窗口，银行卡业务指定 2、3、4 号窗口， 理财卡业务指定 5、6 号窗口。
//! 但如果 5、6 号窗口全忙，而 2、3、4 号窗口有空闲时， 理财卡业务也可以在空闲的 2、3、4 号窗口之一办理。
//! 客户领号、业务完成可以作为输入信息，要求可以随时显示 6 个营业窗口的状态。

use std::ops::Range;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Service {
    AccumulationFund(usize),
    CreditCard(usize),
    WealthManagement(usize),
}

impl Service {
    pub fn get_client_number(&self) -> usize {
        match self {
            Service::AccumulationFund(num) => *num,
            Service::CreditCard(num) => *num,
            Service::WealthManagement(num) => *num
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Window(Option<Service>);

impl Window {
    fn is_empty(&self) -> bool {
        self.0.is_none()
    }
    fn serve(&mut self, value: Service) {
        assert_eq!(false, self.is_empty());
        self.0.replace(value);
    }
    fn clear(&mut self) -> Option<Service> {
        self.0.take()
    }
}

#[derive(Debug)]
pub struct Bank {
    windows: [Window; 6],
    next_number: usize,
    to_serve: Vec<Service>,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            windows: [Window(None); 6],
            next_number: 1,
            to_serve: Vec::new(),
        }
    }
    pub fn new_client_number(&mut self) -> usize {
        self.next_number += 1;
        self.next_number - 1
    }
    pub fn serve(&mut self, service: Service) -> Option<u8> {
        let mut empty_then_serve = |service: Service, range: Range<usize>| -> Option<u8> {
            let mut window_index = None;
            for i in range {
                if self.windows[i].is_empty() {
                    self.windows[i].serve(service);
                    window_index = Some(i as u8);
                    break;
                }
            }
            window_index
        };

        match service {
            Service::AccumulationFund(_) => {
                if self.windows[0].is_empty() {
                    self.windows[0].serve(service);
                    Some(0)
                } else {
                    self.to_serve.push(service);
                    None
                }
            }
            Service::CreditCard(_) => {
                let window_index = empty_then_serve(service, 1..4);
                if window_index.is_none() {
                    self.to_serve.push(service);
                }
                window_index
            }
            Service::WealthManagement(_) => {
                let mut window_index = empty_then_serve(service, 4..6);
                if window_index.is_none() {
                    window_index = empty_then_serve(service, 1..4);
                }
                if window_index.is_none() {
                    self.to_serve.push(service);
                }
                window_index
            }
        }
    }
    pub fn served(&mut self, client_number: usize) -> Option<Service> {
        let mut service = None;
        if let Some(window) = self.windows.iter_mut()
            .filter(|window|
                window.0.map_or(false, |service| service.get_client_number() == client_number))
            .next() {
            service = window.clear();
            if let Some(to_serve) = self.to_serve.pop() {
                self.serve(to_serve);
            }
        }
        service
    }
}