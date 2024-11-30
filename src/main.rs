#![windows_subsystem = "windows"]

use i_slint_backend_winit::winit::window::WindowButtons;
use i_slint_backend_winit::WinitWindowAccessor;
use serial2::SerialPort;
use shadow_rs::shadow;
use slint::SharedString;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
//use std::rc::Weak;
//use std::sync::Arc;

slint::include_modules!();

struct Port {
    name: SharedString,
}
struct Ports(Vec<Port>);

impl slint::Model for Ports {
    type Data = SharedString;

    fn row_count(&self) -> usize {
        self.0.len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.0.get(row).map(|x| x.name.clone())
    }

    fn model_tracker(&self) -> &dyn slint::ModelTracker {
        &()
    }
}
//#[derive(Clone, Copy)]
struct PortManage {
    conn_status: Option<SerialPort>,
}

fn main() {
    shadow!(build);
    let version = env!("CARGO_PKG_VERSION");

    let myapp = myapp::new().unwrap();

    //disable maximize button
    myapp
        .window()
        .with_winit_window(|winit_win: &i_slint_backend_winit::winit::window::Window| {
            winit_win.set_enabled_buttons(WindowButtons::MINIMIZE | WindowButtons::CLOSE)
        });

    let myapp_weak = myapp.as_weak();
    myapp_weak
        .unwrap()
        .set_appname(format!("P3x打印机测试工具 v{}_{}", version, build::BUILD_TIME).into());

    let myapp_weak = myapp.as_weak();

    // share var in closures;
    let port_on = Rc::new(RefCell::new(PortManage { conn_status: None }));

    let portlist = ls_ports();
    if portlist.len() > 0 {
        let list: Vec<Port> = portlist.iter().map(|x| Port { name: x.into() }).collect();
        let llist = Ports(list);
        let rc_list = Rc::new(llist);
        myapp_weak
            .unwrap()
            .set_comlist(slint::ModelRc::from(rc_list.clone()));
    }
    let myapp_weak = myapp.as_weak();

    let port_on_b = port_on.clone();
    myapp.on_openport(move || {
        let ss = myapp_weak.unwrap().get_current_com();
        match SerialPort::open(ss.as_str(), 38400) {
            Ok(c) => {
                let mut port_on_borrow = port_on_b.borrow_mut();
                port_on_borrow.conn_status = Some(c);
                myapp_weak.unwrap().set_openstatus(true);
                myapp_weak
                    .unwrap()
                    .set_portstatus_text(format!("已打开端口{}", ss).into());
                myapp_weak
                    .unwrap()
                    .set_portstatus_color(slint::Brush::SolidColor(slint::Color::from_rgb_u8(
                        0, 180, 0,
                    )));
            }
            Err(_) => {
                myapp_weak
                    .unwrap()
                    .set_portstatus_text("端口打开失败，请重试".into());
                myapp_weak
                    .unwrap()
                    .set_portstatus_color(slint::Brush::SolidColor(slint::Color::from_rgb_u8(
                        255, 0, 0,
                    )));
            }
        }
    });

    let myapp_weak = myapp.as_weak();
    let port_on_b = port_on.clone();
    myapp.on_closeport(move || {
        let mut port_on_borrow = port_on_b.borrow_mut();
        port_on_borrow.conn_status = None;
        myapp_weak.unwrap().set_openstatus(false);
        myapp_weak.unwrap().set_portstatus_text("端口已关闭".into());
        myapp_weak
            .unwrap()
            .set_portstatus_color(slint::Brush::SolidColor(slint::Color::from_rgb_u8(
                255, 0, 0,
            )));
    });

    let myapp_weak = myapp.as_weak();
    myapp.on_refreshport(move || {
        let portlist = ls_ports();
        if portlist.len() > 0 {
            let list: Vec<Port> = portlist.iter().map(|x| Port { name: x.into() }).collect();
            let llist = Ports(list);
            let rc_list = Rc::new(llist);
            myapp_weak
                .unwrap()
                .set_comlist(slint::ModelRc::from(rc_list.clone()));
        }
    });

    let myapp_weak = myapp.as_weak();
    let port_on_b = port_on.clone();
    myapp.on_printline(move || {
        if myapp_weak.unwrap().get_openstatus() {
            let port_on_borrow = port_on_b.borrow_mut();
            port_on_borrow
                .conn_status
                .as_ref()
                .expect("error")
                .write(&[0xa])
                .unwrap();
        }
    });

    let myapp_weak = myapp.as_weak();
    let port_on_b = port_on.clone();
    myapp.on_printlabel(move || {
        if myapp_weak.unwrap().get_openstatus() {
            let port_on_borrow = port_on_b.borrow_mut();
            port_on_borrow
                .conn_status
                .as_ref()
                .expect("error")
                .write(&[0xc])
                .unwrap();
        }
    });

    let myapp_weak = myapp.as_weak();
    let port_on_b = port_on.clone();
    myapp.on_selftest(move || {
        if myapp_weak.unwrap().get_openstatus() {
            let port_on_borrow = port_on_b.borrow_mut();
            port_on_borrow
                .conn_status
                .as_ref()
                .expect("error")
                .write(&[0xa, 0x1D, 0x28, 0x41, 0x02, 0x00, 0x02, 0x02])
                .unwrap();
        }
    });

    let myapp_weak = myapp.as_weak();
    let port_on_b = port_on.clone();
    myapp.on_headtest(move || {
        if myapp_weak.unwrap().get_openstatus() {
            let port_on_borrow = port_on_b.borrow_mut();
            let head_test = include_bytes!("../assets/HeadTest_CPCL.dat");
            port_on_borrow
                .conn_status
                .as_ref()
                .expect("error")
                .write(head_test)
                .unwrap();
        }
    });

    let myapp_weak = myapp.as_weak();
    let port_on_b = port_on.clone();
    myapp.on_sftest(move || {
        if myapp_weak.unwrap().get_openstatus() {
            let port_on_borrow = port_on_b.borrow_mut();
            let sf_test = include_bytes!("../assets/顺丰样张.dat");
            port_on_borrow
                .conn_status
                .as_ref()
                .expect("error")
                .write(sf_test)
                .unwrap();
        }
    });

    let myapp_weak = myapp.as_weak();
    let port_on_b = port_on.clone();
    myapp.on_writename(move || {
        if myapp_weak.unwrap().get_openstatus() {
            let port_on_borrow = port_on_b.borrow_mut();
            let prt_name_untrim = myapp_weak.unwrap().get_printer_name();
            let prt_name = prt_name_untrim.trim();
            let mut ptr_name_hex = vec![0x1B, 0x73, 0x42, 0x45, 0x92, 0x99, 0x1D, 0x49, 0x43];

            ptr_name_hex.extend_from_slice(&prt_name.as_bytes());
            ptr_name_hex.extend_from_slice(&[0x00, 0x0a]);
            //println!("{:x?}", ptr_name_hex);

            if prt_name.len() > 0 {
                //println!("{:x?}", ptr_name_hex);
                port_on_borrow
                    .conn_status
                    .as_ref()
                    .expect("error")
                    .write(&ptr_name_hex)
                    .unwrap();
            }
        }
    });

    let myapp_weak = myapp.as_weak();
    let port_on_b = port_on.clone();
    myapp.on_writesn(move || {
        if myapp_weak.unwrap().get_openstatus() {
            let port_on_borrow = port_on_b.borrow_mut();
            let sn_untrim = myapp_weak.unwrap().get_printer_sn();
            let sn = sn_untrim.trim();
            let mut sn_hex = vec![0x1B, 0x73, 0x42, 0x45, 0x92, 0x99, 0x1D, 0x49, 0x44];

            sn_hex.extend_from_slice(&sn.as_bytes());
            sn_hex.extend_from_slice(&[0x00, 0x0a]);
            // println!("{:x?}", sn_hex);

            if sn.len() > 0 {
                //println!("{:x?}", ptr_name_hex);
                port_on_borrow
                    .conn_status
                    .as_ref()
                    .expect("error")
                    .write(&sn_hex)
                    .unwrap();
            }
        }
    });

    let myapp_weak = myapp.as_weak();
    let port_on_b = port_on.clone();
    myapp.on_paperpwm(move || {
        if myapp_weak.unwrap().get_openstatus() {
            let port_on_borrow = port_on_b.borrow_mut();
            port_on_borrow
                .conn_status
                .as_ref()
                .expect("error")
                .write(&[0x1D, 0x99, 0x42, 0x45, 0x92, 0x9A, 0x51, 0x01, 0x09, 0x9A])
                .unwrap();
        }
    });

    /*
        let myapp_weak = myapp.as_weak();
        let thread = std::thread::spawn(move || {
            loop {
                //println!("lslslslsl");
                //let open_status = myapp.as_weak().unwrap().get_open_action();
                println!("{}", open_status);
                if comstat.open_action {
                    //myapp.as_weak().unwrap().set_open_action(false);
                    //let ss = myapp.as_weak().unwrap().get_current_com();
                    //port_on.open_com(ss.to_string());
                    println!("open open ");
                }
            }

            myapp_weak.upgrade_in_event_loop(move |myapp| {
                println!("run alll ");
            })
        });
    */
    myapp.run().unwrap();
}
// list  ports
fn ls_ports() -> Vec<String> {
    let ls_ports = SerialPort::available_ports();
    //println!("{:?}", ls_ports);
    match ls_ports {
        Ok(p) => {
            let mut ports = vec!["".to_string()];
            for i in 0..p.len() {
                let s = &p[i];
                ports.push(
                    <PathBuf as Clone>::clone(&s)
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                );
            }
            ports
        }
        Err(_) => {
            vec!["".to_string()]
        }
    }
}
