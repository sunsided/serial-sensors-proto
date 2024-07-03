# serial-sensors-proto

> A simple wire format for transmitting MEMS sensor data and friends.

The approach is threefold:

- The protocol is a little bit extensible in sensor and data types and supports 1-, 3- and 4-dimensional readings.
- Data packets are serialized using [bincode](https://crates.io/crates/bincode) first, then byte-stuffed
  using [corncobs](https://crates.io/crates/corncobs) (i.e. using Consistent Overhead Byte Stuffing, COBS).

On the receiving end, the entire process runs in reverse.

---

See [stm32f3disco-rust](https://github.com/sunsided/stm32f3disco-rust)
and [serial-sensors](https://github.com/sunsided/serial-sensors)
for an example. YMMV, but this is how it could work:

```rust
fn example() {
    let value = AccelerometerI16::new(Vector3Data { x: 1, y: -2, z: 3 });
    let frame = Version1DataFrame::new(u32::MAX, 12, 0, value);

    // Serialize into a transmit buffer.
    let mut buffer = [0_u8; 48];
    let buffer = serialize(frame, &mut buffer).unwrap();
    assert_eq!(buffer.len(), 21);

    // ... send the buffer over the wire ...

    // Deserialization the received buffer.
    let data = deserialize(buffer).unwrap();
    assert_eq!(data.version, Version1);
    assert_eq!(data.data.global_sequence, u32::MAX);
    assert_eq!(data.data.sensor_sequence, 12);
    assert_eq!(data.data.sensor_tag, 0);

    let data: AccelerometerI16 = data.try_into().unwrap();
    assert_eq!(data.x, 1);
    assert_eq!(data.y, -2);
    assert_eq!(data.z, 3);
}
```
