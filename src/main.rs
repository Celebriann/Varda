use chrono::Datelike;
use chrono::Local;
use chrono::{TimeZone, Utc};
use std::io;
use std::io::prelude::*;
use std::ptr::null;
use std::str;
//default coordinates: 43.483293 -1.353646
//default time: 23h 15min 00sec
//default date: 10 mai 2026
struct Object {
    RA: f64,  //in degs
    DEC: f64, //in degs
}
struct User {
    time: f64, //in hrs
    lat: f64,  //in degs
    long: f64, //in degs
}
impl Object {
    fn new(RA: String, DEC: String) -> Self {
        //main "new" get all user data and transfer it to the necessary struct
        let mut sRA: Vec<&str> = RA.split(' ').collect();
        let mut sDEC: Vec<&str> = DEC.split(' ').collect();
        Self {
            RA: (sRA[0].parse::<f64>().unwrap()
                + (sRA[1].parse::<f64>().unwrap() / 60.0)
                + (sRA[2].trim().parse::<f64>().unwrap() / 3600.0))
                * 15.0,
            DEC: sDEC[0].parse::<f64>().unwrap()
                + (sDEC[1].parse::<f64>().unwrap() / 60.0)
                + (sDEC[2].trim().parse::<f64>().unwrap() / 3600.0),
        }
    }
}
//register user data like position etc until the program is shutdown
impl User {
    fn new(TIME: String, LAT: String, LONG: String) -> Self {
        let mut time: Vec<&str> = TIME.split(':').collect();
        let mut lat: Vec<&str> = LAT.split(' ').collect();
        let mut long: Vec<&str> = LONG.split(' ').collect();

        let offset = Local::now().offset().local_minus_utc() as f64 / 3600.0; // add an offset if user is not in utc

        Self {
            time: (time[0].parse::<f64>().unwrap() - offset as f64)
                + (time[1].trim().parse::<f64>().unwrap() / 60.0),
            lat: lat[0].parse::<f64>().unwrap() + (lat[1].trim().parse::<f64>().unwrap() / 60.0),
            long: -(long[0].parse::<f64>().unwrap()
                + (long[1].trim().parse::<f64>().unwrap() / 60.0)),
        }
    }
}
fn J2000(obj: &User) -> f64 {
    let j2000_epoch = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let now = Utc::now();

    let diff = now.signed_duration_since(j2000_epoch);
    let J2000 = (diff.num_seconds() as f64 / 86400.0);

    return J2000;
    //println!("j2000: {}", J2000);
}
//Calculating Local Sideral Time
fn LST(j2000: f64, obj: &User) -> f64 {
    let lst: f64 = (100.46 + 0.985647 * j2000 + obj.long + 15.0 * obj.time) % 360.0;
    return lst;
    //println!("LST {}", lst)
}
fn HourAngle(LST: f64, obj: &Object) -> f64 {
    let ha = LST - obj.RA;
    if ha > 180.0 {
        ha - 360.0;
    } else if ha < -180.0 {
        ha + 360.0;
    } else {
        ha;
    }
    return ha; // return in degs
}
fn altitude(user: &User, obj: &Object, ha: f64) -> f64 {
    let alt = (obj.DEC.to_radians().sin() * user.lat.to_radians().sin()
        + obj.DEC.to_radians().cos() * user.lat.to_radians().cos() * ha.to_radians().cos())
    .asin();
    return alt.to_degrees();
}
fn azimute(user: &User, obj: &Object, alt: f64, ha: f64) -> f64 {
    let mut az: f64 = 0.0;
    let mut a = ((obj.DEC.to_radians().sin()
        - alt.to_radians().sin() * user.lat.to_radians().sin())
        / (alt.to_radians().cos() * user.lat.to_radians().cos()))
    .acos()
    .to_degrees();
    if ha.to_radians().sin() < 0.0 {
        az = a;
    } else {
        az = 360.0 - a
    }
    return az;
}
fn calculation(object: &Object, user: &User) {
    let j2000 = J2000(user);
    let lst = LST(j2000, user);
    let HA = HourAngle(lst, object);
    let alt = altitude(user, object, HA);
    let az = azimute(user, object, alt, HA);

    println!("HA {}", HA);
    println!("DEC: {}", object.DEC);
    println!("lat: {}", user.lat);
    println!("J2000: {}", j2000);
    println!("lst: {}", lst);
    println!("alt: {}", alt);
    println!("az: {}", az);
    println!("{}", toDegree(alt, az))
}
fn toDegree(alt: f64, az: f64) -> String {
    let alt_d = alt.round();
    let alt_m = (alt - alt_d) * 60.0;

    let az_d = az.round();
    let az_m = (az - az_d) * 60.0;

    format!("{}°{}' / {}°{}'", alt_d, alt_m, az_d, az_m)
}
fn main() {
    println!(
        "###########################\nEnter RA (h/m/s) with space between each value\nSame goes for DEC\nTime must be write with a : between hours and minutes, do not enter the seconds\nThe longitude and Latitude only take two arguments\nThe longitude is set to East remember to check your settings in Stellarium, etc...\n###########################"
    );
    print!("RA: ");
    io::stdout().flush().unwrap();
    let mut RA = String::new();
    io::stdin().read_line(&mut RA).expect("Failed to read line");
    print!("DEC: ");
    io::stdout().flush().unwrap();
    let mut DEC = String::new();
    io::stdin()
        .read_line(&mut DEC)
        .expect("Failed to read line");
    print!("Time: ");
    io::stdout().flush().unwrap();
    let mut TIME = String::new();
    io::stdin()
        .read_line(&mut TIME)
        .expect("Failed to read line");
    print!("Lat: ");
    io::stdout().flush().unwrap();
    let mut LAT = String::new();
    io::stdin()
        .read_line(&mut LAT)
        .expect("Failed to read line");
    print!("Long: ");
    io::stdout().flush().unwrap();
    let mut LONG = String::new();
    io::stdin()
        .read_line(&mut LONG)
        .expect("Failed to read line");
    let user = User::new(TIME, LAT, LONG);
    let object = Object::new(RA, DEC);
    calculation(&object, &user);
    //Object::new(RA, DEC, TIME, LAT, LONG);
}
