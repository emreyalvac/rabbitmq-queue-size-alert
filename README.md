# Davsvn Alert

*alerts.toml syntax*

First, define slack web hook url

```toml
slack_webhook_url = "${URL}"
```

and define cronjob time,

```toml
cron = "0 0/5 * * * *" # Every 5 minute
```

then, u can define as many alert as you want.

```toml
[[rabbit]] # Root object. Contains RabbitMQ Connection
name = "${NAME}"
url = "${URL}"
[[rabbit.alert]] # Alert object
queue_name = "${QUEUE_NAME}"
threshold = ""${ THRESHOLD }" # Max message size *int
[[rabbit.alert]]
# ...
[[rabbit.alert]]
# ...
[[rabbit.alert]]
# ...
[[rabbit.alert]]
# ...
[[rabbit.alert]]
# ...
```

*example config*

```toml
[[rabbit]]
name = "CLUSTER_1"
url = "amqp://admin:123456@localhost:5672"
[[rabbit.alerts]]
queue_name = "image_queue"
threshold = 15000

[[rabbit]]
name = "CLUSTER_2"
url = "amqp://admin:123456@localhost:5672"
[[rabbit.alerts]]
queue_name = "my_queue"
threshold = 200
[[rabbit.alerts]]
queue_name = "test_queue"
threshold = 1100

slack_webhook_url = "webhook_url"

cron = "0 0/5 * * * *" # Every 5 minute
```