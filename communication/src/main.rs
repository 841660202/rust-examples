mod communication;

fn main() {
    communication::voice::speak();
    communication::voice::calls::make_call();
    communication::text::write();
    communication::text::messages::send_message();
}
