# 🚀 EventHub Platform — MVP Requirements

## 🎯 MVP Definition

**EventHub MVP = a simple event management backend + fully deployed in Kubernetes**

The goal is not just to build an API, but to create a **production-style, end-to-end system** that demonstrates backend engineering and DevOps skills.

---

## 🧩 1. Backend (Required)

### ✔️ Core API Endpoints

The Rust backend must provide:

- `GET /health`  
  → Health check endpoint to verify service availability

- `POST /events`  
  → Create a new event

- `GET /events`  
  → Retrieve all events

- `GET /events/{id}`  
  → Retrieve a specific event by ID

---

### ✔️ Data Model (Minimal)
Event:
	•	id (UUID)
	•	title
	•	description
	•	date
	•	created_at


### ✔️ Architecture (Required Structure)

Project must follow a clean, modular structure:
handlers/   → HTTP layer
models/     → data structures
services/   → business logic

This demonstrates production-level organization and separation of concerns.

---

## 🗄 2. Database (Required)

### ✔️ PostgreSQL Integration

- Connect backend to PostgreSQL
- Persist events in the database

### ✔️ Required Operations

- Insert event
- Select all events
- Select event by ID

No need for advanced migrations initially — simple SQL is sufficient.

---

## 🐳 3. Docker (Required)

### ✔️ Containerization

- Create a `Dockerfile`
- Build and run the backend inside a container

### ✔️ Verification

- Application must start successfully in Docker
- `/health` endpoint must respond correctly

---

## ☸️ 4. Kubernetes (Core Requirement)

This is the key part that makes the project DevOps-relevant.

### ✔️ Deployment

- Kubernetes Deployment with 1–2 replicas

### ✔️ Service

- ClusterIP service for internal access

### ✔️ Health Checks

- Readiness probe → `/health`
- Liveness probe → `/health`

---

### ✔️ Required Kubernetes Files
deployment.yaml
service.yaml


Helm is not required for MVP — plain YAML is preferred for learning.

---

## 🌐 5. Service Access

At least one method to access the service:

- `kubectl port-forward` (sufficient for MVP)
- or NodePort

---

## 🧪 6. Testing (Minimal)

At least one way to test the API:

- curl
- Postman
- or a simple integration test

---

## 📦 7. README (Required)

The project must include a clear README with:

- Project description
- Technology stack
- Instructions to run locally
- Instructions to deploy to Kubernetes

---

## ✅ Definition of Done

The MVP is considered complete when:

### 🔹 Backend
- You can create an event via API
- You can retrieve it from the database

### 🔹 Infrastructure
- Application is deployed to Kubernetes
- Pods are running successfully
- Logs are accessible

### 🔹 Demo Capability

You can demonstrate:
kubectl get pods
kubectl logs
curl /events

---

## 🚫 Out of Scope (Not Included in MVP)

The following features are intentionally excluded:

- ❌ Authentication / Authorization
- ❌ Frontend (Angular)
- ❌ WebSockets / Realtime features
- ❌ Event-driven architecture
- ❌ Terraform
- ❌ CI/CD pipelines

These will be part of future phases.

---

## 💡 Summary

**MVP = CRUD events + PostgreSQL + Docker + Kubernetes deployment**
