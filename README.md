# Moria

A reverse proxy jwt authenticator written in Rust, that leverages the Actix web framework, named after the treacherous 
mines of Moria that were traversed by the fellowship of the ring.

## Purpose

Moria is an attempt at implementing a system similar to the Unix system of `users` and `groups` to `urls`. In a system
where an agent is attempting to access `url` paths without proper access, Moria can be implemented as a gatekeeper to
prevent unauthorized access so long as the token issuer adheres to Moria's rules.

## Design

The functionality of Moria can be understood without knowledge of Rust or programming for that matter. In order to run
a Moria instance, two files are required: `config.json` and `endpoints.json`. The composition of `config.json` is
expanded below:

```json
{
  "JwtKeyName": "some key name for the token issuer",
  "JwtSecret": "some secret shared with the token issuer"
}
```

The composition of `endpoints.json` follows:

```json
[
  {
    "origin": "base_url.com",
    "endpoints": [
      {
        "path": "/guarded-path",
        "method": "GET",
        "groups": ["A","B"]
      }
    ]
  }
]
```

In this design, the agent trying to access `path` cannot have knowledge of or direct to access to `origin`. In fact,
the strongest design is one where the service that runs on `origin` and owns `path` should only accept HTTP requests
from Moria. When the agent sends a request to the Moria service, that agent must belong to at least one of the groups
defined adjacent to that path. So if the agent, Alice, were to send a `GET` request for `/guarded-path`, she would have
to belong to group `A` or `B`; otherwise, she will not have access.