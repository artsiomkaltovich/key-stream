# Key Stream

I needed a small async library to send and receive messages within a single process by key, but I couldn't find one that met all of the following criteria:

- Asynchronous
- Allows subscription and publishing by key
- Avoids copying every message to every receiver and then filter
- Lightweight

So, I decided to create my own.

## What This Library Is Not

- *Not for networking*: messages are sent only within a single process.
- *No persistence*: if there are no subscribers, messages are not saved. It is expected that you obtain a snapshot elsewhere and subscribe only for updates.
If no subscribers exist, it means no one is interested at the moment and a full snapshot can be retrieved later.
It also means that the key and associated sender are deleted, so memory will not grow indefinitely.

## Usage

### Simple Usage

```rust
let key_stream = KeyStream::<String, String>::new(10);
let sender = key_stream.sender();
let mut receiver = sender.subscribe("1".to_string()).await;
sender
    .send(&"1".to_string(), "value".to_string())
    .await
    .except("send failed");
assert_eq!(receiver.recv().await.except("recv failed"), "value".to_string());
```

### Receiver Dropping

If all receivers are dropped, the related key is also removed.
Messages sent to it will be ignored.
The deletion of keys is implemented as a background task in `KeyStream`.

```rust
let key_stream = KeyStream::<String, String>::new(10);
let sender = key_stream.sender();
let receiver = sender.subscribe("1".to_string()).await;
assert_eq!(key_stream.n_keys().await, 1);
drop(receiver);
// give the key drop task a chance to run
tokio::task::yield_now().await;
let result = sender
    .send(&"1".to_string(), "value".to_string())
    .await
    .unwrap();
assert_eq!(result, 0);
assert_eq!(key_stream.n_keys().await, 0);
```
