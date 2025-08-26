# CRUD Endpoints

## Health
- `GET /health` — check service status (baseline endpoint)

---

## Workspaces (Tenancy)
- `POST /v1/workspaces` — create a new workspace  
- `GET /v1/workspaces` — list all workspaces  
- `GET /v1/workspaces/{id}` — retrieve a specific workspace  
- `PATCH /v1/workspaces/{id}` — update a workspace  
- `DELETE /v1/workspaces/{id}` — delete a workspace  

---

## Policies (Redaction Rules + Prompts)
- `POST /v1/policies` — create a policy  
- `GET /v1/policies` — list all policies  
- `GET /v1/policies/{id}` — retrieve a specific policy  
- `PATCH /v1/policies/{id}` — update a policy  
- `DELETE /v1/policies/{id}` — delete a policy  
- `POST /v1/policies/{id}:dry-run` — simulate policy on sample media *(later)*  

---

## Jobs (Process Media with Policy)
- `POST /v1/jobs` — create a job (start processing)  
- `GET /v1/jobs` — list all jobs  
- `GET /v1/jobs/{id}` — retrieve a specific job  
- `PATCH /v1/jobs/{id}` — update job (cancel / retry)  
- `DELETE /v1/jobs/{id}` — delete a job *(optional, if not immutable)*  
- `GET /v1/jobs/{id}/logs` — stream job logs *(later)*  

---

## Webhooks (Event Notifications)
- `POST /v1/webhooks` — register a webhook  
- `GET /v1/webhooks` — list all webhooks  
- `GET /v1/webhooks/{id}` — retrieve a specific webhook  
- `PATCH /v1/webhooks/{id}` — update a webhook  
- `DELETE /v1/webhooks/{id}` — delete a webhook  

---

## Media (Uploads / References / Presigned URLs)
- `POST /v1/media` — register or create upload intent  
- `GET /v1/media` — list all media entries  
- `GET /v1/media/{id}` — retrieve a specific media entry  
- `DELETE /v1/media/{id}` — delete a media entry  

---

## Audit Events
- `GET /v1/audit` — list audit events  
- `GET /v1/audit/{id}` — retrieve a specific audit event  

---

## API Keys
- `POST /v1/apikeys` — create an API key  
- `GET /v1/apikeys` — list all API keys  
- `DELETE /v1/apikeys/{id}` — revoke an API key  
