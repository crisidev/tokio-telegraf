Lightweight wrapper library for general metrics writing using Telegraf. Telegraf is a micro-service provided
by InfluxData for making metrics reporting easy for distributed services - see their [docs](https://docs.influxdata.com/telegraf/v1.13/introduction/installation/) for more information.

This library does not provide querying or other InfluxDB client-library features. This is meant to be lightweight and simple for services to report metrics.

# Install

Add it to your Cargo.toml:

```
[dependencies]
telegraf = "*"
```

# How to use

All usage will start by creating a socket connection via a `Client`. This supports multiple connection protocols - which one you use will be determined by how your Telegraf `input.socket_listener` configuration is setup.

Once a client is setup there are multiple different ways to write points:

## Define structs that represent metrics using the derive macro

```rust
use telegraf::*;

let mut client = Client::new("tcp://localhost:8094".to_owned()).unwrap();

#[derive(Metric)]
struct MyMetric {
    field1: i32,
    #[telegraf(tag)]
    tag1: String,
}

let point = MyMetric { field1: 1, tag1: "tag".to_owned() };
client.write(&point);
```

By default the measurement name will be the same as the struct. You can override this via derive attributes:

```rust
use telegraf::*;

#[derive(Metric)]
#[measurement = "custom_name"]
struct MyMetric {
    field1: i32,
}
```

As with any Telegraf point, tags are optional but at least one field is required.

## Use the `point` macro to do ad-hoc metrics

```rust
use telegraf::*;

let mut client = Client::new("tcp://localhost:8094".to_owned()).unwrap();

let p = point!("measurement", ("tag1", "tag1Val"), ("field1", "field1Val"));
client.write_point(&p);
```

The macro syntax is the following format:

```
(<measurement>, [(<tagName>, <tagVal>)], [(<fieldName>, <fieldVal>)])
```

Measurement name, tag set, and field set are space separated. Tag and field sets are space separated. The tag set is optional.

## Manual `Point` initialization

```rust
use telegraf::{Client, Point};

let c = Client::new("tcp://localhost:8094").unwrap();

let p = Point::new(
    String::from("measurement"),
    vec![
        (String::from("tag1"), String::from("tag1value"))
    ],
    vec![
        (String::from("field1"), Box::new(10)),
        (String::from("field2"), Box::new(20.5)),
        (String::from("field3"), Box::new("anything!"))
    ]
);

c.write_point(p)
```

### Field Data

Any attribute that will the value of a field must implement the `IntoFieldData` trait provided by this library.

```rust
pub trait IntoFieldData {
    fn into_field_data(&self) -> FieldData;
}
```

Out of the box implementations are provided for many common data types, but manual implementation is possible for other data types.
