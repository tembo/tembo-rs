# TemboSdk Rust SDK Reference

## Operations

### client.api_keys().list()

List API keys

- HTTP: `GET /v1/api-keys`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.api_keys().create(…)

Create an API key

- HTTP: `POST /v1/api-keys`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.api_keys().retrieve(…)

Retrieve an API key

- HTTP: `GET /v1/api-keys/{apiKeyId}`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.api_keys().update(…)

Update an API key

- HTTP: `PATCH /v1/api-keys/{apiKeyId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.api_keys().delete(…)

Delete an API key

- HTTP: `DELETE /v1/api-keys/{apiKeyId}`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.skills().list()

List skills

- HTTP: `GET /v1/skills`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.skills().create(…)

Create a skill

- HTTP: `POST /v1/skills`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.skills().retrieve(…)

Retrieve a skill

- HTTP: `GET /v1/skills/{skillId}`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.skills().update(…)

Update a skill

- HTTP: `PATCH /v1/skills/{skillId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.skills().delete(…)

Delete a skill

- HTTP: `DELETE /v1/skills/{skillId}`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.skills().marketplace().list()

List marketplace skills

- HTTP: `GET /v1/skills/marketplace`
- Response body: `application/json`
- Errors: `400, 404, 500, 502`

### client.skills().marketplace().retrieve(…)

Retrieve a marketplace skill

- HTTP: `GET /v1/skills/marketplace/{skillId}`
- Response body: `application/json`
- Errors: `400, 404, 500, 502`

### client.insights().retrieve()

Retrieve insights

- HTTP: `GET /v1/insights`
- Response body: `application/json`
- Errors: `400, 409, 500`

### client.insights().update(…)

Update insights

- HTTP: `PATCH /v1/insights`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 409, 500`

### client.insights().list_members()

List insight members

- HTTP: `GET /v1/insights/members`
- Response body: `application/json`
- Errors: `400, 409, 500`

### client.insights().list_repositories()

List insight repositories

- HTTP: `GET /v1/insights/repositories`
- Response body: `application/json`
- Errors: `400, 409, 500`

### client.organizations().create(…)

Create an organization

- HTTP: `POST /v1/organizations`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().retrieve(…)

Retrieve an organization

- HTTP: `GET /v1/organizations/{organizationId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().update(…)

Update an organization

- HTTP: `PATCH /v1/organizations/{organizationId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().delete(…)

Delete an organization

- HTTP: `DELETE /v1/organizations/{organizationId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().members().list(…)

List organization members

- HTTP: `GET /v1/organizations/{organizationId}/members`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().members().create(…)

Create an organization member

- HTTP: `POST /v1/organizations/{organizationId}/members`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().members().retrieve(…)

Retrieve an organization member

- HTTP: `GET /v1/organizations/{organizationId}/members/{memberId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().members().update(…)

Update an organization member

- HTTP: `PATCH /v1/organizations/{organizationId}/members/{memberId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().members().delete(…)

Remove an organization member

- HTTP: `DELETE /v1/organizations/{organizationId}/members/{memberId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().secrets().list(…)

List organization secrets

- HTTP: `GET /v1/organizations/{organizationId}/secrets`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().secrets().create(…)

Create an organization secret

- HTTP: `POST /v1/organizations/{organizationId}/secrets`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().secrets().retrieve(…)

Retrieve an organization secret

- HTTP: `GET /v1/organizations/{organizationId}/secrets/{secretId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().secrets().update(…)

Update an organization secret

- HTTP: `PATCH /v1/organizations/{organizationId}/secrets/{secretId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().secrets().delete(…)

Delete an organization secret

- HTTP: `DELETE /v1/organizations/{organizationId}/secrets/{secretId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().settings().retrieve(…)

Retrieve organization settings

- HTTP: `GET /v1/organizations/{organizationId}/settings`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().settings().update(…)

Update organization settings

- HTTP: `PATCH /v1/organizations/{organizationId}/settings`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().settings().validate_credentials(…)

Validate organization provider credentials

- HTTP: `POST /v1/organizations/{organizationId}/settings/validate-credentials`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 503`

### client.organizations().subscriptions().retrieve(…)

Retrieve organization subscriptions

- HTTP: `GET /v1/organizations/{organizationId}/subscriptions`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502, 503`

### client.organizations().subscriptions().reset(…)

Reset organization subscription usage

- HTTP: `POST /v1/organizations/{organizationId}/subscriptions/reset`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502, 503`

### client.mcp_connections().list()

List MCP connections

- HTTP: `GET /v1/mcp-connections`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.mcp_connections().create(…)

Create an MCP connection

- HTTP: `POST /v1/mcp-connections`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.mcp_connections().retrieve(…)

Retrieve an MCP connection

- HTTP: `GET /v1/mcp-connections/{mcpConnectionId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.mcp_connections().update(…)

Update an MCP connection

- HTTP: `PATCH /v1/mcp-connections/{mcpConnectionId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.mcp_connections().delete(…)

Delete an MCP connection

- HTTP: `DELETE /v1/mcp-connections/{mcpConnectionId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.mcp_connections().authorize(…)

Authorize an MCP connection

- HTTP: `POST /v1/mcp-connections/{mcpConnectionId}/authorize`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.mcp_connections().test(…)

Test an MCP connection

- HTTP: `POST /v1/mcp-connections/{mcpConnectionId}/test`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.artifacts().list()

List artifacts

- HTTP: `GET /v1/artifacts`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.artifacts().create(…)

Create an artifact

- HTTP: `POST /v1/artifacts`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.artifacts().retrieve(…)

Retrieve an artifact

- HTTP: `GET /v1/artifacts/{artifactId}`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.artifacts().delete(…)

Delete an artifact

- HTTP: `DELETE /v1/artifacts/{artifactId}`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.messages().list()

List messages

- HTTP: `GET /v1/messages`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.messages().create(…)

Create a message

- HTTP: `POST /v1/messages`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.messages().retrieve(…)

Retrieve a message

- HTTP: `GET /v1/messages/{messageId}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.messages().update(…)

Update a message

- HTTP: `PATCH /v1/messages/{messageId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.messages().delete(…)

Delete a message

- HTTP: `DELETE /v1/messages/{messageId}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.models().list()

List models

- HTTP: `GET /v1/models`
- Response body: `application/json`
- Errors: `400, 403, 404, 500`

### client.models().update(…)

Update a model

- HTTP: `PATCH /v1/models/{modelName}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 500`

### client.runtimes().list()

List runtimes

- HTTP: `GET /v1/runtimes`
- Response body: `application/json`
- Errors: `400`

### client.users().me()

Retrieve the current user

- HTTP: `GET /v1/users/me`
- Response body: `application/json`
- Errors: `400, 403, 404, 500`

### client.users().retrieve(…)

Retrieve a user

- HTTP: `GET /v1/users/{userId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 500`

### client.users().delete(…)

Delete a user

- HTTP: `DELETE /v1/users/{userId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 500`

### client.users().settings().retrieve(…)

Retrieve user settings

- HTTP: `GET /v1/users/{userId}/settings`
- Response body: `application/json`
- Errors: `400, 403, 404, 500`

### client.users().settings().update(…)

Update user settings

- HTTP: `PATCH /v1/users/{userId}/settings`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 500`

### client.users().connected_accounts().list(…)

List user connected accounts

- HTTP: `GET /v1/users/{userId}/connected-accounts`
- Response body: `application/json`
- Errors: `400, 403, 404, 500`

### client.users().connected_accounts().retrieve(…)

Retrieve a user connected account

- HTTP: `GET /v1/users/{userId}/connected-accounts/{connectedAccountId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 500`

### client.users().subscriptions().retrieve(…)

Retrieve user subscriptions

- HTTP: `GET /v1/users/{userId}/subscriptions`
- Response body: `application/json`
- Errors: `400, 403, 404, 500, 502`

### client.users().subscriptions().reset(…)

Reset user subscription usage

- HTTP: `POST /v1/users/{userId}/subscriptions/reset`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.users().profile_picture().retrieve(…)

Retrieve a user profile picture

- HTTP: `GET /v1/users/{userId}/profile-picture`
- Errors: `400, 401, 403, 404, 500, 503`

### client.users().profile_picture().update(…)

Update your profile picture

- HTTP: `PUT /v1/users/{userId}/profile-picture`
- Request body: `multipart/form-data`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 413, 500, 503`

### client.users().profile_picture().delete(…)

Delete your profile picture

- HTTP: `DELETE /v1/users/{userId}/profile-picture`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 500, 503`

### client.sessions().list()

List sessions

- HTTP: `GET /v1/sessions`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().create(…)

Create a session

- HTTP: `POST /v1/sessions`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().retrieve(…)

Retrieve a session

- HTTP: `GET /v1/sessions/{sessionId}`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().update(…)

Update a session

- HTTP: `PATCH /v1/sessions/{sessionId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().delete(…)

Delete a session

- HTTP: `DELETE /v1/sessions/{sessionId}`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().fork(…)

Fork a session

- HTTP: `POST /v1/sessions/{sessionId}/fork`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().stop(…)

Stop a session

- HTTP: `POST /v1/sessions/{sessionId}/stop`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().list_events(…)

List session events

- HTTP: `GET /v1/sessions/{sessionId}/events`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().sources().list()

List session sources

- HTTP: `GET /v1/sessions/sources`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().sources().retrieve(…)

Retrieve a session source

- HTTP: `GET /v1/sessions/sources/{sourceId}`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().diffs().list(…)

List session diffs

- HTTP: `GET /v1/sessions/{sessionId}/diffs`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.sessions().diffs().retrieve(…)

Retrieve a session diff

- HTTP: `GET /v1/sessions/{sessionId}/diffs/{diffId}`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502`

### client.projects().update_defaults(…)

Update project defaults

- HTTP: `PUT /v1/projects/defaults`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().list()

List projects

- HTTP: `GET /v1/projects`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().create(…)

Create a project

- HTTP: `POST /v1/projects`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().retrieve(…)

Retrieve a project

- HTTP: `GET /v1/projects/{projectId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().update(…)

Update a project

- HTTP: `PATCH /v1/projects/{projectId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().delete(…)

Delete a project

- HTTP: `DELETE /v1/projects/{projectId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().duplicate(…)

Duplicate a project

- HTTP: `POST /v1/projects/{projectId}/duplicate`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().environment_variables().list(…)

List project environment variables

- HTTP: `GET /v1/projects/{projectId}/environment-variables`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().environment_variables().create(…)

Create project environment variables

- HTTP: `POST /v1/projects/{projectId}/environment-variables`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().environment_variables().delete(…)

Delete a project environment variable

- HTTP: `DELETE /v1/projects/{projectId}/environment-variables/{key}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().snapshots().list(…)

List project snapshots

- HTTP: `GET /v1/projects/{projectId}/snapshots`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().snapshots().retrieve(…)

Retrieve a project snapshot

- HTTP: `GET /v1/projects/{projectId}/snapshots/{snapshotId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().snapshots().update(…)

Update a project snapshot

- HTTP: `PATCH /v1/projects/{projectId}/snapshots/{snapshotId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().snapshots().delete(…)

Delete a project snapshot

- HTTP: `DELETE /v1/projects/{projectId}/snapshots/{snapshotId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().builds().list(…)

List project builds

- HTTP: `GET /v1/projects/{projectId}/builds`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().builds().create(…)

Create a project build

- HTTP: `POST /v1/projects/{projectId}/builds`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().builds().retrieve(…)

Retrieve a project build

- HTTP: `GET /v1/projects/{projectId}/builds/{buildId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().builds().cancel(…)

Cancel a project build

- HTTP: `POST /v1/projects/{projectId}/builds/{buildId}/cancel`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().schedules().list(…)

List project schedules

- HTTP: `GET /v1/projects/{projectId}/schedules`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().schedules().create(…)

Create a project schedule

- HTTP: `POST /v1/projects/{projectId}/schedules`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().schedules().retrieve(…)

Retrieve a project schedule

- HTTP: `GET /v1/projects/{projectId}/schedules/{scheduleId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().schedules().update(…)

Update a project schedule

- HTTP: `PATCH /v1/projects/{projectId}/schedules/{scheduleId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.projects().schedules().delete(…)

Delete a project schedule

- HTTP: `DELETE /v1/projects/{projectId}/schedules/{scheduleId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500`

### client.pins().list()

List pins

- HTTP: `GET /v1/pins`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.pins().create(…)

Create a pin

- HTTP: `POST /v1/pins`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.pins().update(…)

Update a pin

- HTTP: `PATCH /v1/pins/{resourceType}/{resourceId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.pins().delete(…)

Delete a pin

- HTTP: `DELETE /v1/pins/{resourceType}/{resourceId}`
- Response body: `application/json`
- Errors: `400, 404, 500`

### client.pull_requests().list()

List pull requests

- HTTP: `GET /v1/pull-requests`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.pull_requests().retrieve(…)

Retrieve a pull request

- HTTP: `GET /v1/pull-requests/{pullRequestId}`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 500, 502`

### client.agents().list()

List agents

- HTTP: `GET /v1/agents`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().create(…)

Create an agent

- HTTP: `POST /v1/agents`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().retrieve(…)

Retrieve an agent

- HTTP: `GET /v1/agents/{agentId}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().update(…)

Update an agent

- HTTP: `PATCH /v1/agents/{agentId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().delete(…)

Delete an agent

- HTTP: `DELETE /v1/agents/{agentId}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().templates().list()

List agent templates

- HTTP: `GET /v1/agents/templates`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().templates().retrieve(…)

Retrieve an agent template

- HTTP: `GET /v1/agents/templates/{templateName}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().schedules().list(…)

List agent schedules

- HTTP: `GET /v1/agents/{agentId}/schedules`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().schedules().create(…)

Create an agent schedule

- HTTP: `POST /v1/agents/{agentId}/schedules`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().schedules().retrieve(…)

Retrieve an agent schedule

- HTTP: `GET /v1/agents/{agentId}/schedules/{scheduleId}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().schedules().update(…)

Update an agent schedule

- HTTP: `PATCH /v1/agents/{agentId}/schedules/{scheduleId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().schedules().delete(…)

Delete an agent schedule

- HTTP: `DELETE /v1/agents/{agentId}/schedules/{scheduleId}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().triggers().list(…)

List agent triggers

- HTTP: `GET /v1/agents/{agentId}/triggers`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().triggers().create(…)

Create an agent trigger

- HTTP: `POST /v1/agents/{agentId}/triggers`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().triggers().retrieve(…)

Retrieve an agent trigger

- HTTP: `GET /v1/agents/{agentId}/triggers/{triggerId}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().triggers().update(…)

Update an agent trigger

- HTTP: `PATCH /v1/agents/{agentId}/triggers/{triggerId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().triggers().delete(…)

Delete an agent trigger

- HTTP: `DELETE /v1/agents/{agentId}/triggers/{triggerId}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().runs().list(…)

List agent runs

- HTTP: `GET /v1/agents/{agentId}/runs`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().runs().create(…)

Create an agent run

- HTTP: `POST /v1/agents/{agentId}/runs`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().runs().retrieve(…)

Retrieve an agent run

- HTTP: `GET /v1/agents/{agentId}/runs/{runId}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().content_revisions().list(…)

List agent instruction revisions

- HTTP: `GET /v1/agents/{agentId}/content-revisions`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().content_revisions().retrieve(…)

Retrieve an agent instruction revision

- HTTP: `GET /v1/agents/{agentId}/content-revisions/{version}`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.agents().content_revisions().restore(…)

Restore agent instructions

- HTTP: `POST /v1/agents/{agentId}/content-revisions/{version}/restore`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 404, 409, 500`

### client.repositories().list()

List repositories

- HTTP: `GET /v1/repositories`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 504`

### client.repositories().retrieve(…)

Retrieve a repository

- HTTP: `GET /v1/repositories/{repositoryId}`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 504`

### client.repositories().update(…)

Update a repository

- HTTP: `PATCH /v1/repositories/{repositoryId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 504`

### client.integrations().list()

List integrations

- HTTP: `GET /v1/integrations`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().create(…)

Create an integration

- HTTP: `POST /v1/integrations`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().retrieve(…)

Retrieve an integration

- HTTP: `GET /v1/integrations/{integrationId}`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().update(…)

Update an integration

- HTTP: `PATCH /v1/integrations/{integrationId}`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().delete(…)

Delete an integration

- HTTP: `DELETE /v1/integrations/{integrationId}`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().authorize(…)

Authorize an integration

- HTTP: `POST /v1/integrations/{integrationId}/authorize`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().test(…)

Test an integration

- HTTP: `POST /v1/integrations/{integrationId}/test`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().sync(…)

Sync an integration

- HTTP: `POST /v1/integrations/{integrationId}/sync`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().sync_all(…)

Sync eligible integrations

- HTTP: `POST /v1/integrations/sync`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().retrieve_sync_status()

Retrieve integration sync status

- HTTP: `GET /v1/integrations/sync/status`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().retrieve_rate_limit(…)

Retrieve an integration rate limit

- HTTP: `GET /v1/integrations/{integrationId}/rate-limit`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().list_sentry_environments(…)

List Sentry environments

- HTTP: `GET /v1/integrations/{integrationId}/sentry-environments`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().list_slack_channels(…)

List Slack channels

- HTTP: `GET /v1/integrations/{integrationId}/slack-channels`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().providers().list()

List integration providers

- HTTP: `GET /v1/integrations/providers`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().providers().discover_snyk_organizations(…)

Discover Snyk organizations

- HTTP: `POST /v1/integrations/providers/snyk/organizations`
- Request body: `application/json`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().triggers().list()

List integration triggers

- HTTP: `GET /v1/integrations/triggers`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().triggers().retrieve(…)

Retrieve an integration trigger

- HTTP: `GET /v1/integrations/{integrationId}/triggers/{triggerName}`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.integrations().triggers().list_filter_options(…)

List integration trigger filter options

- HTTP: `GET /v1/integrations/{integrationId}/triggers/{triggerName}/filter-options/{field}`
- Response body: `application/json`
- Errors: `400, 401, 403, 404, 409, 500, 502, 503, 504`

### client.billing().retrieve()

Retrieve billing

- HTTP: `GET /v1/billing`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 502`

### client.billing().list_usage()

List billing usage

- HTTP: `GET /v1/billing/usage`
- Response body: `application/json`
- Errors: `400, 403, 404, 409, 502`

### client.live().authorize()

Authorize a live connection

- HTTP: `POST /v1/live`
- Response body: `application/json`
- Errors: `401`

### client.live().connect(…)

Live resource subscriptions

- HTTP: `GET /v1/live`
- Errors: `401`

## Models

- `TipTapDocument`
- `TipTapNode`
- `TipTapLinkMark`
- `TipTapMark`
- `TipTapExtensionAttributes`
- `TipTapImageAttributes`
- `TipTapMentionAttributes`
- `TipTapContentNode`
- `CustomDocument`
- `AgentOptionsInput`
- `AgentOptions`
- `RuntimeAgentOptions`
- `SessionTipTapDocument`
- `SessionDocumentNode`
- `SessionEventData`
- `AgentState`
- `TriggerFilters`
- `AtlassianIssueAddedToBoardFilters`
- `AtlassianIssueAssignedToUserFilters`
- `AtlassianIssueCommentCreatedFilters`
- `AtlassianIssueCreatedFilters`
- `AtlassianIssueLabelAddedFilters`
- `AtlassianIssueStatusChangedFilters`
- `AtlassianIssueUpdatedFilters`
- `BitbucketPullRequestOpenedFilters`
- `GithubCreateFilters`
- `GithubIssueClosedFilters`
- `GithubIssueCommentCreatedFilters`
- `GithubIssueOpenedFilters`
- `GithubPullRequestClosedFilters`
- `GithubPullRequestMergedFilters`
- `GithubPullRequestOpenedFilters`
- `GithubPullRequestReadyForReviewFilters`
- `GithubReleasePublishedFilters`
- `GithubPullRequestSynchronizeFilters`
- `GithubTemboPullRequestOpenedFilters`
- `GithubWorkflowRunFailedFilters`
- `GithubWorkflowRunCompletedFilters`
- `GitlabBotMergeRequestCommentFilters`
- `GitlabIssueClosedFilters`
- `GitlabIssueOpenedFilters`
- `GitlabMergeRequestClosedFilters`
- `GitlabMergeRequestMergedFilters`
- `GitlabMergeRequestOpenedFilters`
- `GitlabMergeRequestUpdatedFilters`
- `GitlabPipelineFailedFilters`
- `GitlabRefCreatedFilters`
- `LinearIssueCreateFilters`
- `LinearIssueLabelAddedFilters`
- `LinearIssueStatusChangedFilters`
- `LinearIssueUpdatedFilters`
- `SentryIssueCreatedFilters`
- `SlackMessageReactionAddedFilters`
- `SlackMessageSentInChannelFilters`
- `EmptyTriggerFilters`
- `AgentRunEventPayload`
- `TipTapImageNode`
- `TipTapMentionNode`
- `TipTapHeadingNode`
- `TipTapOrderedListNode`
- `TipTapTaskItemNode`
- `TipTapCodeBlockNode`
