# Event Hub Platform

Event Hub Platform is a lightweight event discovery platform focused on local activities for children and families.  
It aggregates and presents events (e.g. libraries, community centers) in a structured and accessible way.

## 🚀 Purpose

The goal of this project is to build a scalable, cloud-native application that demonstrates:

- Event aggregation from external sources
- Unified event data model
- Modern frontend for browsing and filtering events
- Production-like deployment using cloud and container technologies

## 🧩 Architecture

- **Backend:** Rust (API, data processing, scraping/aggregation)
- **Frontend:** Angular (UI for browsing events)
- **Infrastructure:** AWS (ECS/EKS, S3, IAM, etc.)
- **Containerization:** Docker
- **Orchestration:** Kubernetes
- **CI/CD:** GitHub Actions

## ⚙️ Features

- Aggregate events from multiple sources
- Normalize and store event data
- Filter events by category, date, location
- Simple and responsive UI

## 🛠️ Tech Stack

| Layer             | Technology        |
|-------------------|-------------------|
| Backend           | Rust              |
| Frontend          | Angular           |
| Infrastructure    | AWS               |
| Containers        | Docker            |
| Orchestration     | Kubernetes        |
| CI/CD             | GitHub Actions    |

## 📌 Status
Work in progress — actively evolving architecture and features.

Milestone 1
    Design schema and sample JSON for 5 events

Milestone 2
    Rust scraper for list page

Milestone 3
    Rust scraper for detail page

Milestone 4
    Store events in Postgres

Milestone 5
    Expose /events API

Milestone 6
    React UI with filters

Milestone 7
    Docker Compose

Milestone 8
    Kubernetes later