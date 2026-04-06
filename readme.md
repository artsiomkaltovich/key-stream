# Key Stream

I needed a small async library to send and receive messages within a single process by key, but I couldn't find one that met all of the following criteria:

- Asynchronous
- Allows subscription and publishing by key
- Avoids copying every message to every receiver and then filter
- Lightweight

So, I decided to create my own.

## What This Library Is Not

- Not for networking: messages are sent only within a single process.
- No persistence: if there are no subscribers, messages are not saved. It is expected that you obtain a snapshot elsewhere and subscribe only for updates.
If no subscribers exist, it means no one is interested at the moment and a full snapshot can be retrieved later.
It also means that the key and associated sender are deleted, so memory will not grow indefinitely.
