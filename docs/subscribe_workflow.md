# Subscribe Workflow sequence diagram

This chart shows the sequence for subscribing to indexer, collecting fees and unsubscribe

```mermaid
sequenceDiagram
    autonumber
    participant B as Bob
    participant E as Eve
    participant C as Charlie 
    participant S as Subscription
    participant SP as SubscriptionProxy
    participant R as Registry
    participant RP as RegistryProxy
    C -) RP : get()
    C -) SP : get()
    RP->>C : registry{AccountID{Hash{"0x..."}}}
    SP-->>C : subscription{AccountID{Hash{"0x..."}}}
    C -) R : get_hash("myname")
    R-->>C : Hash{"0x..."}
    C ->>+ S : not_expired(Hash{"0x..."})
    alt Some
        S-->>C : Some
    else None
        S-->>C : None
    end
    alt Some
        alt Eve subscribes in name of Charlie
            E ->>+ S : subscribe(Hash{"0x..."}, AccountId{"Charlie"})
            note left of S: Eve Pays Balance{10000}.
        else Charlie himself subscribe
            C ->>+ S : subscribe(Hash{"0x..."}, AccountId{"Charlie"})
            note left of S: Charlie Pays Balance{10000}.            
        end
        C ->>+ S : get_subscription(Hash{"0x..."}, AccountId{"Charlie"})
        S-->>C : Some~SubscriberData~
    end
    note right of B: Bob Wait Min threshold epochs period.
    B ->>+ S : claim_fees(Hash{"0x..."})
    note right of C: Charlie Wait Min threshold epochs period.
    C ->>+ S : unsubscribe(Hash{"0x..."})
```