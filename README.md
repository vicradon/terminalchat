# CLI-Based Chat Service

This is a command-line interface chat service that allows users to login, register, send messages, and listen to chat rooms.

## Overview

Users can log in using their username and password or sign up for the service. All actions are performed by inputting commands with their respective arguments.

## State Management

State for the service is maintained in a JSON document with the following structure:

```json
{
    "users": [
        {
            "id": "UUIDv4()",
            "username": "daboi",
            "password": "hashedpassword"
        },
        {
            "id": "UUIDv4()",
            "username": "goodboi",
            "password": "hashedpassword"
        }
    ],
    "rooms": [
        {
            "id": "UUIDv4()",
            "name": "GM-Room",
            "users": [
                "UUIDv4()",
                "UUIDv4()",
                "UUIDv4()"
            ]
        }
    ],
    "messages": [
        {
            "id": "UUIDv4()",
            "room_id": "UUIDv4()",
            "user_id": "UUIDv4()",
            "text": "Wagwan"
        }
    ],
    "sessions": [
        {
            "id": "sid-nanoid()",
            "user_id": "UUIDv4()",
            "expires_at": "Timestamp"
        }
    ]
}
```

## Commands

### 1. Login

Authenticates a user and returns a session ID.

```
chatservice login --username "bro" --password "tenacious"
```

**Parameters:**
- `--username`: string
- `--password`: string

**Returns:**
- `session_id`

**Example output:**
```
logged in with session-id: sid-38493
```

### 2. Register

Creates a new user account and returns a session ID.

```
chatservice register --username "newuser" --password "securepass"
```

**Parameters:**
- `--username`: string
- `--password`: string

**Returns:**
- `session_id`

### 3. Send Message

Sends a message to a specified room or the default room.

```
chatservice send_msg "Hello everyone!" --room_id "room-123"
```

**Parameters:**
- Unnamed argument: message text
- `--room_id`: string (optional, if not provided, sends to default room)

### 4. Listen

Listens for incoming messages in a specified room or the default room.

```
chatservice listen --room_id "room-123"
```

**Parameters:**
- `--room_id`: string (optional, if not provided, listens on the default room)
- Logs data as messages enter the room
