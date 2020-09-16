extern crate byteorder;

use std::net::UdpSocket;
use std::io::Cursor;
use std::io;
use std::io::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;

// struct DirtData {
//     time: f32,
//     lap_time: f32,
//     lap_distance: f32,
//     total_distance: f32,
//     pos_x: f32,
//     pos_y: f32,
//     pos_z: f32,
//     speed: f32,
//     velocity_x: f32,
//     velocity_y: f32,
//     velocity_z: f32,
//     roll_x: f32,
//     roll_y: f32,
//     roll_z: f32,
//     pitch_x: f32,
//     pitch_y: f32,
//     pitch_z: f32,
//     suspension_pos_bl: f32,
//     suspension_pos_br: f32,
//     suspension_pos_fl: f32,
//     suspension_pos_fr: f32,
//     suspension_velocity_bl: f32,
//     suspension_velocity_br: f32,
//     suspension_velocity_fl: f32,
//     suspension_velocity_fr: f32,
//     wheel_velocity_bl: f32,
//     wheel_velocity_br: f32,
//     wheel_velocity_fl: f32,
//     wheel_velocity_fr: f32,
//     throttle: f32,
//     steering: f32,
//     brake: f32,
//     clutch: f32,
//     gear: f32,
//     gforce_lat: f32,
//     geforce_lon: f32,
//     lap: f32,
//     engine_speed: f32,
//     sli_pro_native_support: f32, //Unused
//     car_position: f32,
//     kers_level: f32, //Unused
//     kers_max_level: f32, //Unused
//     drs: f32, //Unused
//     traction_control: f32, //Unused
//     abs: f32, //Unused
//     fuel_in_tank: f32, //Unused
//     fuel_capacity: f32, //Unused
//     in_pits: f32, //Unused
//     sector: f32, //Unused
//     sector1_time: f32,
//     sector2_time: f32,
//     brake_temp_bl: f32,
//     brake_temp_br: f32,
//     brake_temp_fl: f32,
//     brake_temp_fr: f32,
//     track_size: f32, //Unused
//     last_lap_time_f1: f32, //Unused
//     max_rpm_f1: f32, //Unused
//     idle_rpm: f32, //Unused
//     current_lap_rx: f32,
//     total_laps: f32, //rx only, rally = 1
//     track_length: f32,
//     last_lap_time: f32, //stage time in rally
//     max_rpm: f32
// }

#[derive(Debug, Serialize, Copy, Clone)]
struct DirtData {
    time: f32,
    lap_time: f32,
    speed: f32,
    throttle: f32,
    steering: f32,
    brake: f32,
    clutch: f32,
    gear: u8,
    lap: u8,
    rpm: f32,
    max_rpm: f32,
}

impl Default for DirtData {
    fn default() -> DirtData {
        DirtData{
            time: 0.0,
            lap_time: 0.0,
            speed: 0.0,
            throttle: 0.0,
            steering: 0.0,
            brake: 0.0,
            clutch: 0.0,
            gear: 0,
            lap: 1,
            rpm: 0.0,
            max_rpm: 0.0
        }
    }
}

fn main() -> std::io::Result<()> {
    {
        //sending from main to udp thread
        let (main_tx, main_rx) = mpsc::channel();

        //sending from udp thread to main
        let (udp_tx, udp_rx) = mpsc::channel();

        let stats: Vec<DirtData> = Vec::new();
        std::thread::spawn(move || {
            let mut stats: Vec<DirtData> = Vec::new();
            let mut socket = UdpSocket::bind("127.0.0.1:20777").expect("socket failed to open");
            socket.set_read_timeout(Some(Duration::new(1,0))); //set timeout to 1 second

            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = vec![0u8; 68 * std::mem::size_of::<f32>()];
            
            loop {
                println!("_____________________________________");
                if(socket.recv_from(&mut buf).is_ok()){
                    let mut reader = Cursor::new(&buf);
    
                
        
    
                    let mut data: DirtData = DirtData::default();
                    
                    for i in 0..68{
                        let value = reader.read_f32::<LittleEndian>().unwrap();
        
        
                       //put data into struct
                       match i {
                            0 => {data = DirtData{time: value, ..data}},
                            1 => {data = DirtData{lap_time: value, ..data}},
                            7 => {data = DirtData{speed: value, ..data}},
                            29 => {data = DirtData{throttle: value, ..data}},
                            30 => {data = DirtData{steering: value, ..data}},
                            31 => {data = DirtData{brake: value, ..data}},
                            32 => {data = DirtData{clutch: value, ..data}},
                            33 => {data = DirtData{gear: value as u8, ..data}},
                            36 => {data = DirtData{lap: value as u8, ..data}},
                            37 => {data = DirtData{rpm: value, ..data}},
                            63 => {data = DirtData{max_rpm: value, ..data}},
                            _ => {}
                       }    
                    }
        
                    
                    println!("Speed: {}, Throttle: {}, Steering: {}, Brake: {}, Clutch: {}, Gear: {}, RPM: {}",
                                data.speed, data.throttle, data.steering, data.brake, data.clutch, data.gear, data.rpm);
                    stats.push(data);
                    // for i in 0..68{
                    //     print!("{:x?} ", buf[i]);
                    // }
                    

                }
                //break loop if a keypress is done on another thread
                match main_rx.try_recv() {
                    Ok(_) => break,
                    Err(msg) => {
                        if msg == TryRecvError::Disconnected {
                            break;
                        }
                    },
                };
            }
            //send back stats
            udp_tx.send(stats);
        });
        
        //wait till user input
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();
        // let lines : Vec<String> = stdin_lock.lines().filter_map(|line| line.ok()).collect();
        for line in stdin_lock.lines(){
            println!("{}", line.expect("Broken"));
            break;
        }

        //tell udp thread to stop
        main_tx.send(());

        let stats = udp_rx.recv().expect("Couldn't get game data from udp thread");
        // write to csv
        let mut writer = csv::Writer::from_writer(std::fs::File::create("a.csv").expect("can't write"));
        for stat in stats{
            writer.serialize(stat).expect("couldn't serialize");
        }
    } // the socket is closed here
    Ok(())
}