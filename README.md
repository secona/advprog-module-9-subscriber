#### What is amqp?

AMQP is short for Advanced Message Queueing Protocol. It is an open standard application layer protocol for message-oriented middleware. RabbitMQ is one of the most popular implementation of AMQP.

---

#### What does it mean? guest:guest@localhost:5672, what is the first guest, and what is the second guest, and what is localhost:5672 is for?

- The first `guest` is the username.
- The second `guest` is the password.
- `localhost:5672` specifies the hostname and port number where the RabbitMQ server is running.
  - `localhost` refers to my local machine
  - `5672` is the default port used by RabbitMQ connections

---

#### Slow Subscriber Simulation

![image](https://github.com/user-attachments/assets/e10fbce2-6e13-4137-baf7-d9e0253ec07a)

I ran the publisher program 10 times. This means there should be 50 messages. However, since it takes time for RabbitMQ to receive every messages and display them to the dashboard, some messages has been processed by the time the message count is displayed. That's why it showed message count of a little under 40.

---

#### Multiple Subscribers Simulation

![image](https://github.com/user-attachments/assets/6572e365-2808-487c-ace3-4fa876983da2)
![image](https://github.com/user-attachments/assets/23988da6-7c0e-48ea-8114-7df41b1665fc)

I ran four subscriber and executed the publisher program 10 times. Compared to having only one subscriber, the message count decreased much faster. This is because the load was disributed across four subscribers instead of just one, reducing the workload of each individual subscriber. While one subscriber is busy processing a message, others can handle incoming messages simultaneously, reducing bottlenecks and improving overall throughput.
