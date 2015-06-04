extern crate nl80211;
fn main() {
    println!("{}\n", nl80211::Commands::RegisterFrame as i32);
    println!("{}\n", nl80211::Commands::Frame as i32);
    println!("{}\n", nl80211::Commands::FrameTxStatus as i32);
}
