# rustypipe
Write raw queries to your steampipe DB with a HTTP API interface
## USAGE:

export the required env vars
```
export DB_HOST='localhost'
export DB_USER='steampipe'
export DB_PORT='9193'
export DB_PASSWORD='000hSt34my'

cargo run
```

make plaintext requests in raw SQL
```
curl -v -d "select instance_id,monitoring_state,tags from aws_ec2_instance" http://localhost:3000/raw
```

Features:

Implemented:
- Run a raw SQL query through curl and get the output back as JSON including.
- Deserialize JSONB (postgres) to native JSON output (web-api)

```
Serialization:
/// | Rust type                         | Postgres type(s)                              |
/// |-----------------------------------|-----------------------------------------------|
/// | `bool`                            | BOOL                                          | -> Bool
/// | `i8`                              | "char"                                        | -> Str
/// | `i16`                             | SMALLINT, SMALLSERIAL                         | -> Int
/// | `i32`                             | INT, SERIAL                                   | -> Int
/// | `u32`                             | OID                                           | -> Int
/// | `i64`                             | BIGINT, BIGSERIAL                             | -> Int
/// | `f32`                             | REAL                                          | -> Dec
/// | `f64`                             | DOUBLE PRECISION                              | -> Dec
/// | `&str`/`String`                   | VARCHAR, CHAR(n), TEXT, CITEXT, NAME, UNKNOWN | -> Str
/// |                                   | LTREE, LQUERY, LTXTQUERY                      | -> Str
/// | `&[u8]`/`Vec<u8>`                 | BYTEA                                         | -> Str
/// | `HashMap<String, Option<String>>` | HSTORE                                        | -> JSON
/// | `SystemTime`                      | TIMESTAMP, TIMESTAMP WITH TIME ZONE           | -> Str
/// | `IpAddr`                          | INET                                          | -> str


/// | Rust type                       | Postgres type(s)                    |
/// |---------------------------------|-------------------------------------|
/// | `chrono::NaiveDateTime`         | TIMESTAMP                           | -> Str?
/// | `chrono::DateTime<Utc>`         | TIMESTAMP WITH TIME ZONE            | -> Str?
/// | `chrono::DateTime<Local>`       | TIMESTAMP WITH TIME ZONE            | -> Str?
/// | `chrono::DateTime<FixedOffset>` | TIMESTAMP WITH TIME ZONE            | -> Str?
/// | `chrono::NaiveDate`             | DATE                                | -> Str?
/// | `chrono::NaiveTime`             | TIME                                | -> Str?
/// | `time::PrimitiveDateTime`       | TIMESTAMP                           | -> Str?
/// | `time::OffsetDateTime`          | TIMESTAMP WITH TIME ZONE            | -> Str?
/// | `time::Date`                    | DATE                                | -> Str?
/// | `time::Time`                    | TIME                                | -> Str?
/// | `eui48::MacAddress`             | MACADDR                             | -> Str
/// | `geo_types::Point<f64>`         | POINT                               | -> Supported?
/// | `geo_types::Rect<f64>`          | BOX                                 | -> Supported?
/// | `geo_types::LineString<f64>`    | PATH                                | -> Supported?
/// | `serde_json::Value`             | JSON, JSONB                         | -> Deserialized json
/// | `uuid::Uuid`                    | UUID                                | -> Str
/// | `bit_vec::BitVec`               | BIT, VARBIT                         | -> Supported?
/// | `eui48::MacAddress`             | MACADDR                             | -> Str
/// | `cidr::InetCidr`                | CIDR                                | -> Str
/// | `cidr::InetAddr`                | INET                                | -> Str
/// | `smol_str::SmolStr`             | VARCHAR, CHAR(n), TEXT, CITEXT,     | -> Str
/// |                                 | NAME, UNKNOWN, LTREE, LQUERY,       | -> Str
/// |                                 | LTXTQUERY                           |
```

Todo:
- Front end query writer and table display
    - using leptos? a girl can dream
- on disk cache for fast retrieval
    - add params ?cache=['save','use']
    - hash the query and save the results to disk
    - if called with "use" , then the steampipe call will be skipped and it will be read from on-disk instead.

Use case:

Your devops infrastructure will never be perfectly procured and managed through an IAC tool, though that is always the goal - and even if it was you wouldn't know until you audited it.
My main use case for this tool is allowing other pieces of my automation to grab data from my infrastructure live without needing a node, which means sandboxed environments such as active choices will work.
This is how I am able to get dynamically updating menu choices in jenkins that reflect the live infrastructure without needing to rebuild the job.