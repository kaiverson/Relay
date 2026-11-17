curl -X POST http://localhost:1234/event \
    -H "Content-Type: application/json" \
    -d '{
        "event_type": "USER_CREATED",
        "payload": {"user_id":123,"name":"John"}
    }'