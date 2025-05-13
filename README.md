> What is amqp?

AMQP is short for Advanced Message Queueing Protocol. It is an open standard application layer protocol for message-oriented middleware. RabbitMQ is one of the most popular implementation of AMQP.

> What does it mean? guest:guest@localhost:5672, what is the first guest, and what is the second guest, and what is localhost:5672 is for?

- The first `guest` is the username.
- The second `guest` is the password.
- `localhost:5672` specifies the hostname and port number where the RabbitMQ server is running.
  - `localhost` refers to my local machine
  - `5672` is the default port used by RabbitMQ connections

![image](https://github.com/user-attachments/assets/268aac37-77a6-4b55-9f06-fa3a8416b977)
![image](https://github.com/user-attachments/assets/d073e242-7677-45d9-9152-9437a2c1dc98)
![image](https://github.com/user-attachments/assets/53adb56d-995e-4655-8f22-8b50bc9420c6)

The publisher is publishing the messages and the subscriber is listening to them. The RabbitMQ server is doing its job of receiving the messages from the publisher and delivering them to the subscriber.
