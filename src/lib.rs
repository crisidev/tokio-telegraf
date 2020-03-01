pub mod protocol;

use std::io::Write;
use url::Url;
use std::net::TcpStream;

use protocol::*;

#[derive(Debug, Clone)]
pub struct Point {
    pub measurement: String,
    pub tags: Vec<Tag>,
    pub fields: Vec<Field>
}

impl Point {
    pub fn new(
        measurement: String,
        tags: Vec<(String, String)>,
        fields: Vec<(String, Box<dyn IntoFieldData>)>,
    ) -> Self {
        let t = tags.into_iter()
            .map(|(n,v)| Tag { name: n, value: v })
            .collect();
        let f = fields.into_iter()
            .map(|(n,v)| Field { name: n, value: v.into_field_data() })
            .collect();
        Point {
            measurement: measurement,
            tags: t,
            fields: f,
        }
    }

    pub fn to_lp(&self) -> LineProtocol {
        let tag_attrs: Vec<Attr> = self.tags.clone()
            .into_iter()
            .map(|t| Attr::Tag(t))
            .collect();
        let field_attrs: Vec<Attr> = self.fields.clone()
            .into_iter()
            .map(|f| Attr::Field(f))
            .collect();
        let tag_str = format_attr(tag_attrs);
        let field_str = format_attr(field_attrs);
        LineProtocol::new(self.measurement.clone(), tag_str, field_str)
    }
}

pub struct Client {
    conn: Connector
}

impl Client {
    pub fn new(conn_url: String) -> Result<Self, String> {
        let conn = create_connection(&conn_url);
        match conn {
            Ok(c) => Ok(Self { conn: c }),
            Err(e) => Err(format!("error creating connection {}", e))
        }
    }

    // Writes the protocol representation of a point
    // to the established connection. 
    pub fn write_point(&self, point: Point) -> Result<(), String> {
        let lp = point.clone().to_lp();
        let bytes = lp.to_str().as_bytes();
        match &self.conn {
            Connector::TCP(c) => {
                let mut tc = &c.conn;

                let r = tc.write(bytes);
                match r {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("failed to write to tcp socket {}", e))
                }
            }
        }
    }
}

fn create_connection(conn_url: &str) -> Result<Connector, String> {
    let url = Url::parse(&conn_url);
        match url {
            Ok(u) => {
                let host = u.host_str().unwrap();
                let port = u.port().unwrap();
                let scheme = u.scheme();
                match scheme {
                    "tcp" => {
                        let conn = TcpStream::connect(format!("{}:{}", host, port));
                        match conn {
                            Ok(c) => Ok(Connector::TCP(TcPConnection { conn: c })),
                            Err(_) => Err("error connecting to tcp socket".to_owned())
                        }
                    },
                    "udp" => Err("udp not supported yet".to_owned()),
                    "unix" => Err("unix not supported yet".to_owned()),
                    _ => Err(format!("unknown connection protocol {}", scheme))
                }
            },
            Err(_) => Err(format!("invalid connection URL {}", conn_url))
        }
}

enum Connector {
    TCP(TcPConnection),
    // UDP(UdPConnection),
}

struct TcPConnection {
    conn: TcpStream
}

// struct UdPConnection {
//     conn: String
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_point_lp() {
        let p = Point::new(
            String::from("Foo"),
            vec![
                ("t1".to_owned(), "v".to_owned())
            ],
            vec![
                ("f1".to_owned(), Box::new(10)),
                ("f2".to_owned(), Box::new(10.3)),
                ("f3".to_owned(), Box::new("b"))
            ]
        );

        let lp = p.to_lp();
        assert_eq!(lp.to_str(), "Foo,t1=v f1=10i,f2=10.3,f3=\"b\"");
    }
}