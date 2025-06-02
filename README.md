# Spot Feed

A geo-location–based social connector app for coordinating activities without exchanging direct contact details.

## Concept

**Purpose:** Create "Joints" (groups) to coordinate activities in specific areas with privacy-focused social interaction.

## Core Features

- **Location-based Discovery:** Find groups near you
- **Temporary Groups:** Self-expiring Joints with 6-hour lifespan
- **Visibility Options:** Public, private, and hidden configurations
- **Rich Messaging:** Text, images, audio, and video
- **Moderation Tools:** Creator roles and moderator delegation
- **Social Integration:** Link Facebook, Instagram, WhatsApp with mask/unmask options
- **Privacy Controls:** Save profiles for later contact
- **Smart Notifications:** Join requests, proximity alerts, creator controls
- **Guest Access:** Temporary account support

## Technical Stack

- **Backend:** Rust
- **Mobile App:** Flutter
- **Database:** PostgreSQL

## API Endpoints

### Auth
- `POST /api/auth/login`
- `POST /api/auth/verify-otp`
- `POST /api/auth/guest`

### User
- `GET /api/user/profile`
- `PUT /api/user/profile`
- `POST /api/user/social-links`

### Joints
- `GET /api/joints/active`
- `GET /api/joints/nearby?lat={lat}&lng={lng}`
- `POST /api/joints` (create new joint)
- `PUT /api/joints/{jointID}` (update settings)
- `POST /api/joints/{jointID}/join-request`
- `DELETE /api/joints/{jointID}`
- `POST /api/joints/{jointID}/messages`

### Moderation & Reporting
- `POST /api/joints/{jointID}/report`
- `PUT /api/joints/{jointID}/moderation`

## High-Level Modular Design
We'll use modular boundaries to separate concerns:
```
┌────────────────────────────┐
│        API Gateway         │  (optional, for routing/ratelimiting)
└────────────────────────────┘
           │
           ▼
┌────────────────────────────┐
│        Web Layer           │
│  (REST controllers, WS)    │
└────────────────────────────┘
           │
           ▼
┌────────────────────────────┐
│      Application Layer     │
│   (business logic, DTOs)   │
└────────────────────────────┘
           │
           ▼
┌────────────────────────────┐
│        Domain Layer        │
│  (entities, traits, types) │
└────────────────────────────┘
           │
           ▼
┌────────────────────────────┐
│      Infrastructure Layer  │
│  (DB, external APIs, etc)  │
└────────────────────────────┘
```

## Project Status

- Current version: 0.1.0 (Early development)

## Roadmap

- [ ] MVP Release (0.1.0)
- [ ] User Authentication & Profile Management
- [ ] Joint Creation & Management
- [ ] In-Group Messaging
- [ ] Moderation & Reporting
- [ ] Social Linking
- [ ] Privacy Controls
- [ ] Notifications
- [ ] Guest Login
- [ ] Mobile App Development
- [ ] Testing & Quality Assurance
- [x] Documentation
