# Tembo SDK Rust API

Complete reference of every operation, grouped by resource. See [the README](./README.md) for usage and configuration.

## Contents

- [`ApiKeys`](#apikeys)
  - [List API keys](#list-api-keys)
  - [Create an API key](#create-an-api-key)
  - [Retrieve an API key](#retrieve-an-api-key)
  - [Update an API key](#update-an-api-key)
  - [Delete an API key](#delete-an-api-key)
- [`Skills`](#skills)
  - [List skills](#list-skills)
  - [Create a skill](#create-a-skill)
  - [Retrieve a skill](#retrieve-a-skill)
  - [Update a skill](#update-a-skill)
  - [Delete a skill](#delete-a-skill)
  - [`Skills Marketplace`](#skills-marketplace)
    - [List marketplace skills](#list-marketplace-skills)
    - [Retrieve a marketplace skill](#retrieve-a-marketplace-skill)
- [`Insights`](#insights)
  - [Retrieve insights](#retrieve-insights)
  - [Update insights](#update-insights)
  - [List insight members](#list-insight-members)
  - [List insight repositories](#list-insight-repositories)
- [`Organizations`](#organizations)
  - [Create an organization](#create-an-organization)
  - [Retrieve an organization](#retrieve-an-organization)
  - [Update an organization](#update-an-organization)
  - [Delete an organization](#delete-an-organization)
  - [`Organizations Members`](#organizations-members)
    - [List organization members](#list-organization-members)
    - [Create an organization member](#create-an-organization-member)
    - [Retrieve an organization member](#retrieve-an-organization-member)
    - [Update an organization member](#update-an-organization-member)
    - [Remove an organization member](#remove-an-organization-member)
  - [`Organizations Secrets`](#organizations-secrets)
    - [List organization secrets](#list-organization-secrets)
    - [Create an organization secret](#create-an-organization-secret)
    - [Retrieve an organization secret](#retrieve-an-organization-secret)
    - [Update an organization secret](#update-an-organization-secret)
    - [Delete an organization secret](#delete-an-organization-secret)
  - [`Organizations Settings`](#organizations-settings)
    - [Retrieve organization settings](#retrieve-organization-settings)
    - [Update organization settings](#update-organization-settings)
    - [Validate organization provider credentials](#validate-organization-provider-credentials)
  - [`Organizations Subscriptions`](#organizations-subscriptions)
    - [Retrieve organization subscriptions](#retrieve-organization-subscriptions)
- [`McpConnections`](#mcpconnections)
  - [List MCP connections](#list-mcp-connections)
  - [Create an MCP connection](#create-an-mcp-connection)
  - [Retrieve an MCP connection](#retrieve-an-mcp-connection)
  - [Update an MCP connection](#update-an-mcp-connection)
  - [Delete an MCP connection](#delete-an-mcp-connection)
  - [Authorize an MCP connection](#authorize-an-mcp-connection)
  - [Test an MCP connection](#test-an-mcp-connection)
- [`Artifacts`](#artifacts)
  - [List artifacts](#list-artifacts)
  - [Create an artifact](#create-an-artifact)
  - [Retrieve an artifact](#retrieve-an-artifact)
  - [Delete an artifact](#delete-an-artifact)
- [`Messages`](#messages)
  - [List messages](#list-messages)
  - [Create a message](#create-a-message)
  - [Retrieve a message](#retrieve-a-message)
  - [Update a message](#update-a-message)
  - [Delete a message](#delete-a-message)
- [`Models`](#models)
  - [List models](#list-models)
  - [Update a model](#update-a-model)
- [`Runtimes`](#runtimes)
  - [List runtimes](#list-runtimes)
- [`Users`](#users)
  - [Retrieve the current user](#retrieve-the-current-user)
  - [Retrieve a user](#retrieve-a-user)
  - [`Users Settings`](#users-settings)
    - [Retrieve user settings](#retrieve-user-settings)
    - [Update user settings](#update-user-settings)
  - [`Users ConnectedAccounts`](#users-connectedaccounts)
    - [List user connected accounts](#list-user-connected-accounts)
    - [Retrieve a user connected account](#retrieve-a-user-connected-account)
  - [`Users Subscriptions`](#users-subscriptions)
    - [Retrieve user subscriptions](#retrieve-user-subscriptions)
  - [`Users EnvironmentVariables`](#users-environmentvariables)
    - [List personal environment variables](#list-personal-environment-variables)
    - [Create a personal environment variable](#create-a-personal-environment-variable)
    - [Delete a personal environment variable](#delete-a-personal-environment-variable)
  - [`Users ProfilePicture`](#users-profilepicture)
    - [Retrieve a user profile picture](#retrieve-a-user-profile-picture)
    - [Update your profile picture](#update-your-profile-picture)
    - [Delete your profile picture](#delete-your-profile-picture)
- [`Sessions`](#sessions)
  - [List sessions](#list-sessions)
  - [Create a session](#create-a-session)
  - [Retrieve a session](#retrieve-a-session)
  - [Update a session](#update-a-session)
  - [Delete a session](#delete-a-session)
  - [Fork a session](#fork-a-session)
  - [Stop a session](#stop-a-session)
  - [List session events](#list-session-events)
  - [`Sessions Sources`](#sessions-sources)
    - [List session sources](#list-session-sources)
    - [Retrieve a session source](#retrieve-a-session-source)
  - [`Sessions Diffs`](#sessions-diffs)
    - [List session diffs](#list-session-diffs)
    - [Retrieve a session diff](#retrieve-a-session-diff)
  - [`Sessions Inputs`](#sessions-inputs)
    - [List session inputs](#list-session-inputs)
    - [Update a session input](#update-a-session-input)
  - [`Sessions Queue`](#sessions-queue)
    - [Update a session queue](#update-a-session-queue)
  - [`Sessions Commands`](#sessions-commands)
    - [List session commands](#list-session-commands)
- [`Projects`](#projects)
  - [Update project defaults](#update-project-defaults)
  - [List projects](#list-projects)
  - [Create a project](#create-a-project)
  - [Retrieve a project](#retrieve-a-project)
  - [Update a project](#update-a-project)
  - [Delete a project](#delete-a-project)
  - [Duplicate a project](#duplicate-a-project)
  - [`Projects EnvironmentVariables`](#projects-environmentvariables)
    - [List project environment variables](#list-project-environment-variables)
    - [Create project environment variables](#create-project-environment-variables)
    - [Delete a project environment variable](#delete-a-project-environment-variable)
  - [`Projects Snapshots`](#projects-snapshots)
    - [List project snapshots](#list-project-snapshots)
    - [Retrieve a project snapshot](#retrieve-a-project-snapshot)
    - [Update a project snapshot](#update-a-project-snapshot)
    - [Delete a project snapshot](#delete-a-project-snapshot)
  - [`Projects Builds`](#projects-builds)
    - [List project builds](#list-project-builds)
    - [Create a project build](#create-a-project-build)
    - [Retrieve a project build](#retrieve-a-project-build)
    - [Cancel a project build](#cancel-a-project-build)
  - [`Projects Schedules`](#projects-schedules)
    - [List project schedules](#list-project-schedules)
    - [Create a project schedule](#create-a-project-schedule)
    - [Retrieve a project schedule](#retrieve-a-project-schedule)
    - [Update a project schedule](#update-a-project-schedule)
    - [Delete a project schedule](#delete-a-project-schedule)
- [`Pins`](#pins)
  - [List pins](#list-pins)
  - [Create a pin](#create-a-pin)
  - [Update a pin](#update-a-pin)
  - [Delete a pin](#delete-a-pin)
- [`PullRequests`](#pullrequests)
  - [List pull requests](#list-pull-requests)
  - [Retrieve a pull request](#retrieve-a-pull-request)
- [`Agents`](#agents)
  - [List agents](#list-agents)
  - [Create an agent](#create-an-agent)
  - [Retrieve an agent](#retrieve-an-agent)
  - [Update an agent](#update-an-agent)
  - [Delete an agent](#delete-an-agent)
  - [`Agents Templates`](#agents-templates)
    - [List agent templates](#list-agent-templates)
    - [Retrieve an agent template](#retrieve-an-agent-template)
  - [`Agents Schedules`](#agents-schedules)
    - [List agent schedules](#list-agent-schedules)
    - [Create an agent schedule](#create-an-agent-schedule)
    - [Retrieve an agent schedule](#retrieve-an-agent-schedule)
    - [Update an agent schedule](#update-an-agent-schedule)
    - [Delete an agent schedule](#delete-an-agent-schedule)
  - [`Agents Triggers`](#agents-triggers)
    - [List agent triggers](#list-agent-triggers)
    - [Create an agent trigger](#create-an-agent-trigger)
    - [Retrieve an agent trigger](#retrieve-an-agent-trigger)
    - [Update an agent trigger](#update-an-agent-trigger)
    - [Delete an agent trigger](#delete-an-agent-trigger)
  - [`Agents Runs`](#agents-runs)
    - [List agent runs](#list-agent-runs)
    - [Create an agent run](#create-an-agent-run)
    - [Retrieve an agent run](#retrieve-an-agent-run)
  - [`Agents ContentRevisions`](#agents-contentrevisions)
    - [List agent instruction revisions](#list-agent-instruction-revisions)
    - [Retrieve an agent instruction revision](#retrieve-an-agent-instruction-revision)
    - [Restore agent instructions](#restore-agent-instructions)
- [`Repositories`](#repositories)
  - [List repositories](#list-repositories)
  - [Retrieve a repository](#retrieve-a-repository)
  - [Update a repository](#update-a-repository)
- [`Integrations`](#integrations)
  - [List integrations](#list-integrations)
  - [Create an integration](#create-an-integration)
  - [Retrieve an integration](#retrieve-an-integration)
  - [Update an integration](#update-an-integration)
  - [Delete an integration](#delete-an-integration)
  - [Authorize an integration](#authorize-an-integration)
  - [Test an integration](#test-an-integration)
  - [Sync an integration](#sync-an-integration)
  - [Sync eligible integrations](#sync-eligible-integrations)
  - [Retrieve integration sync status](#retrieve-integration-sync-status)
  - [Retrieve an integration rate limit](#retrieve-an-integration-rate-limit)
  - [List Sentry environments](#list-sentry-environments)
  - [List Slack channels](#list-slack-channels)
  - [`Integrations Providers`](#integrations-providers)
    - [List integration providers](#list-integration-providers)
    - [Discover Snyk organizations](#discover-snyk-organizations)
  - [`Integrations Triggers`](#integrations-triggers)
    - [List integration triggers](#list-integration-triggers)
    - [Retrieve an integration trigger](#retrieve-an-integration-trigger)
    - [List integration trigger filter options](#list-integration-trigger-filter-options)
- [`Billing`](#billing)
  - [Retrieve billing](#retrieve-billing)
  - [List billing usage](#list-billing-usage)
- [`Live`](#live)
  - [Authorize a live connection](#authorize-a-live-connection)
  - [Live resource subscriptions](#live-resource-subscriptions)

## Setup

```rust
use tembo_rs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tembo::from_env()?;

    // ... the samples below go here

    Ok(())
}
```

## `ApiKeys`

### List API keys

List active API keys for the current organization.

| Direction | Type |
| --- | --- |
| Response | [`ApiKeysListResponse`](./src/models/api_keys.rs) |

```rust
let response = client.api_keys().list().send().await?;
```

### Create an API key

Create an API key for the current organization. Optionally set `expiresAt` to a future timestamp after which the key stops authenticating requests. The complete token is returned only by this operation.

| Direction | Type |
| --- | --- |
| Request | [`ApiKeysCreateBody`](./src/models/api_keys.rs) |
| Response | [`ApiKeysCreateResponse`](./src/models/api_keys.rs) |

```rust
let response = client
    .api_keys()
    .create(ApiKeysCreateBody {
        expires_at: None,
        name: None,
    })
    .send()
    .await?;
```

### Retrieve an API key

Retrieve an active API key without exposing its complete token.

| Direction | Type |
| --- | --- |
| Response | [`ApiKeysRetrieveResponse`](./src/models/api_keys.rs) |

```rust
let response = client
    .api_keys()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Update an API key

Rename an active API key.

| Direction | Type |
| --- | --- |
| Request | [`ApiKeysUpdateBody`](./src/models/api_keys.rs) |
| Response | [`ApiKeysUpdateResponse`](./src/models/api_keys.rs) |

```rust
let response = client
    .api_keys()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        ApiKeysUpdateBody { name: "x".to_string() },
    )
    .send()
    .await?;
```

### Delete an API key

Revoke an active API key so it can no longer authenticate requests.

| Direction | Type |
| --- | --- |
| Response | [`ApiKeysDeleteResponse`](./src/models/api_keys.rs) |

```rust
let response = client
    .api_keys()
    .delete("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

## `Skills`

### List skills

List organization skills or skills checked into a repository.

| Direction | Type |
| --- | --- |
| Response | [`SkillsListResponse`](./src/models/skills.rs) |

```rust
let response = client.skills().list().send().await?;
```

### Create a skill

Create an organization skill with its files.

| Direction | Type |
| --- | --- |
| Request | [`SkillsCreateBody`](./src/models/skills.rs) |
| Response | [`SkillsCreateResponse`](./src/models/skills.rs) |

```rust
let response = client
    .skills()
    .create(SkillsCreateBody {
        description: "".to_string(),
        name: "x".to_string(),
        files: vec![SkillsCreateBodyFile {
            content: "".to_string(),
            filename: "x".to_string(),
        }],
    })
    .send()
    .await?;
```

### Retrieve a skill

Retrieve an organization skill and all of its files.

| Direction | Type |
| --- | --- |
| Response | [`SkillsRetrieveResponse`](./src/models/skills.rs) |

```rust
let response = client
    .skills()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Update a skill

Update an organization skill. When files are provided, they replace the complete file set.

| Direction | Type |
| --- | --- |
| Request | [`SkillsUpdateBody`](./src/models/skills.rs) |
| Response | [`SkillsUpdateResponse`](./src/models/skills.rs) |

```rust
let response = client
    .skills()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        SkillsUpdateBody {
            description: None,
            name: None,
            files: None,
        },
    )
    .send()
    .await?;
```

### Delete a skill

Delete an organization skill.

| Direction | Type |
| --- | --- |
| Response | [`SkillsDeleteResponse`](./src/models/skills.rs) |

```rust
let response = client
    .skills()
    .delete("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### `Skills Marketplace`

#### List marketplace skills

List popular marketplace skills or search the catalog. Results have no continuation cursor.

| Direction | Type |
| --- | --- |
| Response | [`MarketplaceListResponse`](./src/models/marketplace.rs) |

```rust
let response = client.skills().marketplace().list().send().await?;
```

#### Retrieve a marketplace skill

Retrieve the files and content hash for a marketplace skill. This does not install the skill.

| Direction | Type |
| --- | --- |
| Response | [`MarketplaceRetrieveResponse`](./src/models/marketplace.rs) |

```rust
let response = client
    .skills()
    .marketplace()
    .retrieve("example", serde_json::json!(null))
    .send()
    .await?;
```

## `Insights`

### Retrieve insights

Retrieve pull request contribution insights and their collection status for the current organization.

| Direction | Type |
| --- | --- |
| Response | [`InsightsRetrieveResponse`](./src/models/insights.rs) |

```rust
let response = client.insights().retrieve().send().await?;
```

### Update insights

Enable or disable pull request insight collection. Enabling starts the initial refresh when needed.

| Direction | Type |
| --- | --- |
| Request | [`InsightsUpdateBody`](./src/models/insights.rs) |
| Response | [`InsightsUpdateResponse`](./src/models/insights.rs) |

```rust
let response = client
    .insights()
    .update(InsightsUpdateBody { enabled: false })
    .send()
    .await?;
```

### List insight members

List organization members included in pull request insights.

| Direction | Type |
| --- | --- |
| Response | [`InsightsListMembersResponse`](./src/models/insights.rs) |

```rust
let response = client.insights().list_members().send().await?;
```

### List insight repositories

List repositories included in pull request insights for the current organization.

| Direction | Type |
| --- | --- |
| Response | [`InsightsListRepositoriesResponse`](./src/models/insights.rs) |

```rust
let response = client.insights().list_repositories().send().await?;
```

## `Organizations`

### Create an organization

Create an organization and make the authenticated user its administrator.

| Direction | Type |
| --- | --- |
| Request | [`OrganizationsCreateBody`](./src/models/organizations.rs) |
| Response | [`OrganizationsCreateResponse`](./src/models/organizations.rs) |

```rust
let response = client
    .organizations()
    .create(OrganizationsCreateBody {
        name: "x".to_string(),
        slug: None,
    })
    .send()
    .await?;
```

### Retrieve an organization

Retrieve an organization profile.

| Direction | Type |
| --- | --- |
| Response | [`OrganizationsRetrieveResponse`](./src/models/organizations.rs) |

```rust
let response = client.organizations().retrieve("example").send().await?;
```

### Update an organization

Update organization profile fields or onboarding state.

| Direction | Type |
| --- | --- |
| Request | [`OrganizationsUpdateBody`](./src/models/organizations.rs) |
| Response | [`OrganizationsUpdateResponse`](./src/models/organizations.rs) |

```rust
let response = client
    .organizations()
    .update(
        "example",
        OrganizationsUpdateBody {
            name: None,
            slug: None,
            onboarding_completed: None,
        },
    )
    .send()
    .await?;
```

### Delete an organization

Delete the organization from the identity provider and Tembo.

| Direction | Type |
| --- | --- |
| Response | [`OrganizationsDeleteResponse`](./src/models/organizations.rs) |

```rust
let response = client.organizations().delete("example").send().await?;
```

### `Organizations Members`

#### List organization members

List active members or pending invitations with cursor pagination.

| Direction | Type |
| --- | --- |
| Response | [`MembersListResponse`](./src/models/members.rs) |

```rust
let response = client.organizations().members().list("example").send().await?;
```

#### Create an organization member

Invite or add a user to an organization with a role.

| Direction | Type |
| --- | --- |
| Request | [`MembersCreateBody`](./src/models/members.rs) |
| Response | [`MembersCreateResponse`](./src/models/members.rs) |

```rust
let response = client
    .organizations()
    .members()
    .create(
        "example",
        MembersCreateBody {
            email_address: "user@example.com".to_string(),
            role: None,
        },
    )
    .send()
    .await?;
```

#### Retrieve an organization member

Retrieve an active member or pending member invitation.

| Direction | Type |
| --- | --- |
| Response | [`MembersRetrieveResponse`](./src/models/members.rs) |

```rust
let response = client
    .organizations()
    .members()
    .retrieve("example", "example")
    .send()
    .await?;
```

#### Update an organization member

Update an organization member role.

| Direction | Type |
| --- | --- |
| Request | [`MembersUpdateBody`](./src/models/members.rs) |
| Response | [`MembersUpdateResponse`](./src/models/members.rs) |

```rust
let response = client
    .organizations()
    .members()
    .update(
        "example",
        "example",
        MembersUpdateBody {
            role: MembersUpdateBodyRole::OrgAdmin,
        },
    )
    .send()
    .await?;
```

#### Remove an organization member

Remove an active member or revoke a pending invitation.

| Direction | Type |
| --- | --- |
| Response | [`MembersDeleteResponse`](./src/models/members.rs) |

```rust
let response = client
    .organizations()
    .members()
    .delete("example", "example")
    .send()
    .await?;
```

### `Organizations Secrets`

#### List organization secrets

List redacted organization secret summaries with cursor pagination.

| Direction | Type |
| --- | --- |
| Response | [`SecretsListResponse`](./src/models/secrets.rs) |

```rust
let response = client.organizations().secrets().list("example").send().await?;
```

#### Create an organization secret

Create an encrypted organization secret.

| Direction | Type |
| --- | --- |
| Request | [`SecretsCreateBody`](./src/models/secrets.rs) |
| Response | [`SecretsCreateResponse`](./src/models/secrets.rs) |

```rust
let response = client
    .organizations()
    .secrets()
    .create(
        "example",
        SecretsCreateBody {
            key: "x".to_string(),
            value: "x".to_string(),
        },
    )
    .send()
    .await?;
```

#### Retrieve an organization secret

Retrieve a redacted organization secret summary.

| Direction | Type |
| --- | --- |
| Response | [`SecretsRetrieveResponse`](./src/models/secrets.rs) |

```rust
let response = client
    .organizations()
    .secrets()
    .retrieve("example", "7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Update an organization secret

Update a secret name, encrypted value, or both.

| Direction | Type |
| --- | --- |
| Request | [`SecretsUpdateBody`](./src/models/secrets.rs) |
| Response | [`SecretsUpdateResponse`](./src/models/secrets.rs) |

```rust
let response = client
    .organizations()
    .secrets()
    .update(
        "example",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        SecretsUpdateBody { key: None, value: None },
    )
    .send()
    .await?;
```

#### Delete an organization secret

Delete an organization secret.

| Direction | Type |
| --- | --- |
| Response | [`SecretsDeleteResponse`](./src/models/secrets.rs) |

```rust
let response = client
    .organizations()
    .secrets()
    .delete("example", "7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### `Organizations Settings`

#### Retrieve organization settings

Retrieve effective organization settings, including defaults for settings that have not been configured.

| Direction | Type |
| --- | --- |
| Response | [`SettingsRetrieveResponse`](./src/models/organizations_settings.rs) |

```rust
let response = client.organizations().settings().retrieve("example").send().await?;
```

#### Update organization settings

Configure organization defaults for agent/model, VM size and keep-alive; inference credentials; agent prompts and commit/PR instructions; bot feedback, signing, reports, and insights. Only supplied settings are changed.

| Direction | Type |
| --- | --- |
| Request | [`SettingsUpdateBody`](./src/models/organizations_settings.rs) |
| Response | [`SettingsUpdateResponse`](./src/models/organizations_settings.rs) |

```rust
let response = client
    .organizations()
    .settings()
    .update(
        "example",
        SettingsUpdateBody {
            amp_api_key: None,
            anthropic_api_key: None,
            aws_bedrock_access_key_id: None,
            aws_bedrock_region: None,
            aws_bedrock_secret_access_key: None,
            clone_depth: None,
            commit_instructions: None,
            cursor_api_key: None,
            custom_system_prompt: None,
            default_agent: None,
            default_keep_alive_minutes: None,
            default_sandbox_size: None,
            delete_branch_when_pr_closed: None,
            draft_prs: None,
            enable_coderabbit_bot: None,
            enable_cursor_bot: None,
            enable_gemini_code_assist_bot: None,
            enable_github_actions_bot: None,
            enable_graphite_bot: None,
            enable_greptile_bot: None,
            experimental_mcp: None,
            feature_mention_author: None,
            gcp_vertex_ai_location: None,
            gcp_vertex_ai_project: None,
            gcp_vertex_ai_service_account_json: None,
            kernel_api_key: None,
            max_bot_reviews: None,
            max_solutions: None,
            openai_api_key: None,
            xai_api_key: None,
            openrouter_api_key: None,
            pg_stat_statements_reset_frequency: None,
            pm_system_prompt: None,
            pr_title_instructions: None,
            pull_request_insights_enabled: None,
            should_sign_commits: None,
            slack_review_on_tembo: None,
            slow_query_threshold_ms: None,
            ssh_public_keys: None,
            weekly_report_enabled: None,
        },
    )
    .send()
    .await?;
```

#### Validate organization provider credentials

Validate supplied inference provider credentials for an organization without saving them. The provider must be available on the organization’s plan.

| Direction | Type |
| --- | --- |
| Request | [`SettingsValidateCredentialsBody`](./src/models/organizations_settings.rs) |
| Response | [`SettingsValidateCredentialsResponse`](./src/models/organizations_settings.rs) |

### `Organizations Subscriptions`

#### Retrieve organization subscriptions

Retrieve connection statuses and available ChatGPT and Claude usage. Provider usage errors are reported without failing the response.

| Direction | Type |
| --- | --- |
| Response | [`SubscriptionsRetrieveResponse`](./src/models/organizations_subscriptions.rs) |

```rust
let response = client
    .organizations()
    .subscriptions()
    .retrieve("example")
    .send()
    .await?;
```

## `McpConnections`

### List MCP connections

List MCP connections for the current organization with cursor pagination and optional search by name or URL. Command arguments, headers, and environment configuration are omitted.

| Direction | Type |
| --- | --- |
| Response | [`McpConnectionsListResponse`](./src/models/mcp_connections.rs) |

```rust
let response = client.mcp_connections().list().send().await?;
```

### Create an MCP connection

Create a new stdio or Streamable HTTP MCP connection. Remote connections support OAuth, Client Credentials (M2M), or static headers and begin authorization when required; stdio commands are stored but never executed by this API.

| Direction | Type |
| --- | --- |
| Request | [`McpConnectionsCreateBody`](./src/models/mcp_connections.rs) |
| Response | [`McpConnectionsCreateResponse`](./src/models/mcp_connections.rs) |

### Retrieve an MCP connection

Retrieve MCP connection configuration and authentication details. Stored header values, environment values, OAuth tokens, and client secrets are never returned.

| Direction | Type |
| --- | --- |
| Response | [`McpConnectionsRetrieveResponse`](./src/models/mcp_connections.rs) |

```rust
let response = client
    .mcp_connections()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Update an MCP connection

Update MCP configuration, including OAuth, Client Credentials (M2M), or static-header authentication. Supplied headers merge by key only for unchanged-server static_headers authentication; otherwise they are replaced. Environment values merge by key, null values remove individual keys, and a null environment clears all variables. Hidden credentials are preserved unless explicitly changed or authentication is replaced; changing a remote server URL clears credentials for the old server. Call authorize explicitly after changing the server or authentication to reconnect.

| Direction | Type |
| --- | --- |
| Request | [`McpConnectionsUpdateBody`](./src/models/mcp_connections.rs) |
| Response | [`McpConnectionsUpdateResponse`](./src/models/mcp_connections.rs) |

### Delete an MCP connection

Delete an MCP connection in the current organization.

| Direction | Type |
| --- | --- |
| Response | [`McpConnectionsDeleteResponse`](./src/models/mcp_connections.rs) |

```rust
let response = client
    .mcp_connections()
    .delete("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Authorize an MCP connection

Connect or authorize a remote Streamable HTTP MCP connection, returning an OAuth URL when user authorization is required. Stdio connections are rejected because this action cannot authorize them.

| Direction | Type |
| --- | --- |
| Response | [`McpConnectionsAuthorizeResponse`](./src/models/mcp_connections.rs) |

```rust
let response = client
    .mcp_connections()
    .authorize("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Test an MCP connection

Test a remote MCP connection by connecting to its server. Stdio connections are rejected because the API does not execute arbitrary local commands.

| Direction | Type |
| --- | --- |
| Response | [`McpConnectionsTestResponse`](./src/models/mcp_connections.rs) |

```rust
let response = client
    .mcp_connections()
    .test("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

## `Artifacts`

### List artifacts

List organization artifacts with optional session and type filters.

| Direction | Type |
| --- | --- |
| Response | [`ArtifactsListResponse`](./src/models/artifacts.rs) |

```rust
let response = client.artifacts().list().send().await?;
```

### Create an artifact

Create a file artifact for a session from inline content or an uploaded asset.

| Direction | Type |
| --- | --- |
| Request | [`ArtifactsCreateBody`](./src/models/artifacts.rs) |
| Response | [`ArtifactsCreateResponse`](./src/models/artifacts.rs) |

### Retrieve an artifact

Retrieve an organization artifact by ID.

| Direction | Type |
| --- | --- |
| Response | [`ArtifactsRetrieveResponse`](./src/models/artifacts.rs) |

```rust
let response = client
    .artifacts()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Delete an artifact

Delete an organization artifact and its backing asset when present.

| Direction | Type |
| --- | --- |
| Response | [`ArtifactsDeleteResponse`](./src/models/artifacts.rs) |

```rust
let response = client
    .artifacts()
    .delete("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

## `Messages`

### List messages

List organization messages with optional session and thread filters.

| Direction | Type |
| --- | --- |
| Response | [`MessagesListResponse`](./src/models/messages.rs) |

```rust
let response = client.messages().list().send().await?;
```

### Create a message

Send a task or follow-up message to an existing session, resume work, or steer an active agent turn using deliveryMode.

| Direction | Type |
| --- | --- |
| Request | [`MessagesCreateBody`](./src/models/messages.rs) |
| Response | [`MessagesCreateResponse`](./src/models/messages.rs) |

```rust
let response = client
    .messages()
    .create(MessagesCreateBody {
        artifact_id: None,
        content: "x".to_string(),
        mcp_servers: None,
        pull_request_id: None,
        session_id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        agent_options: None,
        delivery_mode: None,
        rich_content: None,
        slash_command: None,
    })
    .send()
    .await?;
```

### Retrieve a message

Retrieve an organization message by ID.

| Direction | Type |
| --- | --- |
| Response | [`MessagesRetrieveResponse`](./src/models/messages.rs) |

```rust
let response = client
    .messages()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Update a message

Update message content before its queued processing has started.

| Direction | Type |
| --- | --- |
| Request | [`MessagesUpdateBody`](./src/models/messages.rs) |
| Response | [`MessagesUpdateResponse`](./src/models/messages.rs) |

```rust
let response = client
    .messages()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        MessagesUpdateBody {
            content: None,
            rich_content: None,
        },
    )
    .send()
    .await?;
```

### Delete a message

Delete a message and cancel its queued jobs, runtime turns, and active sandbox sessions.

| Direction | Type |
| --- | --- |
| Response | [`MessagesDeleteResponse`](./src/models/messages.rs) |

```rust
let response = client
    .messages()
    .delete("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

## `Models`

### List models

List the models available to the current organization, including enablement and inference-provider access.

| Direction | Type |
| --- | --- |
| Response | [`ModelsListResponse`](./src/models/models.rs) |

```rust
let response = client.models().list().send().await?;
```

### Update a model

Enable or disable an organization model, or select its inference provider.

| Direction | Type |
| --- | --- |
| Request | [`ModelsUpdateBody`](./src/models/models.rs) |
| Response | [`ModelsUpdateResponse`](./src/models/models.rs) |

```rust
let response = client
    .models()
    .update(
        "claude-opus-5-5".to_string(),
        ModelsUpdateBody {
            enabled: None,
            inference_provider: None,
        },
    )
    .send()
    .await?;
```

## `Runtimes`

### List runtimes

List execution runtimes and their compatible model references for the current organization.

| Direction | Type |
| --- | --- |
| Response | [`RuntimesListResponse`](./src/models/runtimes.rs) |

```rust
let response = client.runtimes().list().send().await?;
```

## `Users`

### Retrieve the current user

Retrieve the authenticated user.

| Direction | Type |
| --- | --- |
| Response | [`UsersMeResponse`](./src/models/users.rs) |

```rust
let response = client.users().me().send().await?;
```

### Retrieve a user

Retrieve a user profile visible to the caller.

| Direction | Type |
| --- | --- |
| Response | [`UsersRetrieveResponse`](./src/models/users.rs) |

```rust
let response = client.users().retrieve("example").send().await?;
```

### `Users Settings`

#### Retrieve user settings

Retrieve user preferences without exposing stored credentials.

| Direction | Type |
| --- | --- |
| Response | [`SettingsRetrieveResponse2`](./src/models/users_settings.rs) |

```rust
let response = client.users().settings().retrieve("example").send().await?;
```

#### Update user settings

Configure personal agent/model and project defaults, system instructions, follow-up queue/steer behavior, VM restart, commit signing/attribution, notifications, themes, and review layout. Updates preserve unrelated settings.

| Direction | Type |
| --- | --- |
| Request | [`SettingsUpdateBody2`](./src/models/users_settings.rs) |
| Response | [`SettingsUpdateResponse2`](./src/models/users_settings.rs) |

```rust
let response = client
    .users()
    .settings()
    .update(
        "example",
        SettingsUpdateBody2 {
            auto_restart_vm: None,
            default_project_id: None,
            gpg_key: None,
            include_tembo_commit_attribution: None,
            desktop_notifications_enabled: None,
            desktop_notification_preferences: None,
            has_desktop_app: None,
            open_links_in_desktop_app: None,
            appearance_preferences: None,
            app_sidebar_preferences: None,
            diff_preferences: None,
            dashboard_preferences: None,
            composer_preferences: None,
            review_preferences: None,
            diff_light_theme: None,
            diff_dark_theme: None,
            default_agent: None,
            follow_up_behavior: None,
            custom_system_prompt: None,
        },
    )
    .send()
    .await?;
```

### `Users ConnectedAccounts`

#### List user connected accounts

List the user identity-provider accounts without returning credentials.

| Direction | Type |
| --- | --- |
| Response | [`ConnectedAccountsListResponse`](./src/models/connected_accounts.rs) |

```rust
let response = client.users().connected_accounts().list("example").send().await?;
```

#### Retrieve a user connected account

Retrieve one identity-provider account without returning credentials.

| Direction | Type |
| --- | --- |
| Response | [`ConnectedAccountsRetrieveResponse`](./src/models/connected_accounts.rs) |

```rust
let response = client
    .users()
    .connected_accounts()
    .retrieve("example", "example")
    .send()
    .await?;
```

### `Users Subscriptions`

#### Retrieve user subscriptions

Retrieve connection statuses and available ChatGPT and Claude usage. Provider usage errors are reported without failing the response.

| Direction | Type |
| --- | --- |
| Response | [`SubscriptionsRetrieveResponse2`](./src/models/users_subscriptions.rs) |

```rust
let response = client.users().subscriptions().retrieve("example").send().await?;
```

### `Users EnvironmentVariables`

#### List personal environment variables

List your personal environment variables in the current organization. Values are never returned.

| Direction | Type |
| --- | --- |
| Response | [`EnvironmentVariablesListResponse`](./src/models/users_environment_variables.rs) |

```rust
let response = client.users().environment_variables().list("example").send().await?;
```

#### Create a personal environment variable

Store an encrypted environment variable that is injected into every session you start in the current organization.

| Direction | Type |
| --- | --- |
| Request | [`EnvironmentVariablesCreateBody`](./src/models/users_environment_variables.rs) |
| Response | [`EnvironmentVariablesCreateResponse`](./src/models/users_environment_variables.rs) |

```rust
let response = client
    .users()
    .environment_variables()
    .create(
        "example",
        EnvironmentVariablesCreateBody {
            key: "x".to_string(),
            value: "x".to_string(),
        },
    )
    .send()
    .await?;
```

#### Delete a personal environment variable

Permanently delete a personal environment variable. Sessions started afterwards no longer receive it.

| Direction | Type |
| --- | --- |
| Response | [`EnvironmentVariablesDeleteResponse`](./src/models/users_environment_variables.rs) |

```rust
let response = client
    .users()
    .environment_variables()
    .delete("example", "7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### `Users ProfilePicture`

#### Retrieve a user profile picture

Redirect to a user's profile picture. Signed URLs are valid for one hour.

| Direction | Type |
| --- | --- |
| Response | `()` |

```rust
client.users().profile_picture().retrieve("example").send().await?;
```

#### Update your profile picture

Upload a JPEG, PNG, GIF, or WebP profile picture up to 5 MB.

| Direction | Type |
| --- | --- |
| Request | `tembo_rs::transport::multipart::Upload` |
| Response | [`ProfilePictureUpdateResponse`](./src/models/profile_picture.rs) |

```rust
let response = client
    .users()
    .profile_picture()
    .update(
        "example",
        tembo_rs::transport::multipart::Upload::from_bytes("example").filename("example.bin"),
    )
    .send()
    .await?;
```

#### Delete your profile picture

Delete your profile picture. This succeeds if no picture exists.

| Direction | Type |
| --- | --- |
| Response | [`ProfilePictureDeleteResponse`](./src/models/profile_picture.rs) |

```rust
let response = client.users().profile_picture().delete("example").send().await?;
```

## `Sessions`

### List sessions

List and search sessions, including active or archived sessions, with repository, project, creator, and source filters and cursor pagination.

| Direction | Type |
| --- | --- |
| Response | [`SessionsListResponse`](./src/models/sessions.rs) |

```rust
let response = client.sessions().list().send().await?;
```

### Create a session

Create a session for a task. Provide the initial prompt in description and optionally configure its repositories, agent, Build (normal) or Plan (planning) mode, and private or workspace visibility.

| Direction | Type |
| --- | --- |
| Request | [`SessionsCreateBody`](./src/models/sessions.rs) |
| Response | [`SessionsCreateResponse`](./src/models/sessions.rs) |

```rust
let response = client
    .sessions()
    .create(SessionsCreateBody {
        agent: Some("claudeCode:claude-fable-5-1".to_string()),
        base_branch: Some("main".to_string()),
        mcp_servers: Some(vec![]),
        project_id: Some("your-project-id".to_string()),
        sandbox_size: None,
        target_branch: Some("main".to_string()),
        visibility: Some(SessionsCreateBodyVisibility::Private),
        agent_options: None,
        branch_name: None,
        code_repository_ids: Some(vec!["123e4567-e89b-12d3-a456-426614174000".to_string()]),
        description: "Fix the authentication bug in the login component".to_string(),
        id: None,
        rich_content: None,
        pull_request_id: None,
        queue_right_away: None,
        automatic_sizing: None,
        session_mode: None,
        startup_agent: None,
    })
    .send()
    .await?;
```

### Retrieve a session

Retrieve a session you have permission to access.

| Direction | Type |
| --- | --- |
| Response | [`SessionsRetrieveResponse`](./src/models/sessions.rs) |

```rust
let response = client
    .sessions()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Update a session

Update a session: archive or restore it, set priority, rename it, change agent/model, switch between Build (normal) and Plan (planning) modes, configure its workspace and branches, or change visibility between private and visible to workspace (public). Archiving uses level=0; restoring archived sessions uses level=1 and does not restart work. Use pins to pin or unpin sessions. Submit memoryMb alone to request asynchronous memory growth for a running adaptive sandbox within project limits; sandbox metrics report progress.

| Direction | Type |
| --- | --- |
| Request | [`SessionsUpdateBody`](./src/models/sessions.rs) |
| Response | [`SessionsUpdateResponse`](./src/models/sessions.rs) |

```rust
let response = client
    .sessions()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        SessionsUpdateBody {
            memory_mb: None,
            agent: None,
            agent_options: None,
            base_branch: None,
            branch_name: None,
            description: None,
            rich_content: None,
            level: None,
            mcp_servers: None,
            mode: None,
            project_id: None,
            sandbox_size: None,
            target_branch: None,
            title: None,
            visibility: None,
        },
    )
    .send()
    .await?;
```

### Delete a session

Archive a session and stop its VM without permanently deleting it. Equivalent to updateSession with level=0; restore with updateSession and level=1.

| Direction | Type |
| --- | --- |
| Response | [`SessionsDeleteResponse`](./src/models/sessions.rs) |

```rust
let response = client
    .sessions()
    .delete("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Fork a session

Create an independent session from a fresh workspace snapshot, or the latest saved snapshot when stopped, without replaying messages or adding context.

| Direction | Type |
| --- | --- |
| Request | [`SessionsForkBody`](./src/models/sessions.rs) |
| Response | [`SessionsForkResponse`](./src/models/sessions.rs) |

```rust
let response = client
    .sessions()
    .fork(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        SessionsForkBody {
            prompt: None,
            title: None,
            queue_right_away: None,
        },
    )
    .send()
    .await?;
```

### Stop a session

Interrupt active session work without archiving it. Continue with the next queued message, or set cancelQueued=true to cancel pending messages too.

| Direction | Type |
| --- | --- |
| Response | [`SessionsStopResponse`](./src/models/sessions.rs) |

```rust
let response = client
    .sessions()
    .stop("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### List session events

List recorded session stream events with cursor pagination.

| Direction | Type |
| --- | --- |
| Response | [`SessionsListEventsResponse`](./src/models/sessions.rs) |

```rust
let response = client
    .sessions()
    .list_events("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### `Sessions Sources`

#### List session sources

List session sources in the authenticated organization.

| Direction | Type |
| --- | --- |
| Response | [`SourcesListResponse`](./src/models/sources.rs) |

```rust
let response = client.sessions().sources().list().send().await?;
```

#### Retrieve a session source

Retrieve one session source.

| Direction | Type |
| --- | --- |
| Response | [`SourcesRetrieveResponse`](./src/models/sources.rs) |

```rust
let response = client
    .sessions()
    .sources()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### `Sessions Diffs`

#### List session diffs

List recorded session diff metadata with cursor pagination.

| Direction | Type |
| --- | --- |
| Response | [`DiffsListResponse`](./src/models/diffs.rs) |

```rust
let response = client
    .sessions()
    .diffs()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Retrieve a session diff

Retrieve one recorded session diff and its content.

| Direction | Type |
| --- | --- |
| Response | [`DiffsRetrieveResponse`](./src/models/diffs.rs) |

```rust
let response = client
    .sessions()
    .diffs()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### `Sessions Inputs`

#### List session inputs

List pending questions for the active session agent.

| Direction | Type |
| --- | --- |
| Response | [`InputsListResponse`](./src/models/inputs.rs) |

```rust
let response = client
    .sessions()
    .inputs()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Update a session input

Answer a pending session question.

| Direction | Type |
| --- | --- |
| Request | [`InputsUpdateBody`](./src/models/inputs.rs) |
| Response | [`InputsUpdateResponse`](./src/models/inputs.rs) |

```rust
let response = client
    .sessions()
    .inputs()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "example",
        InputsUpdateBody {
            answers: std::collections::HashMap::new(),
        },
    )
    .send()
    .await?;
```

### `Sessions Queue`

#### Update a session queue

Replace the order of messages waiting in a session queue.

| Direction | Type |
| --- | --- |
| Request | [`QueueUpdateBody`](./src/models/queue.rs) |
| Response | [`QueueUpdateResponse`](./src/models/queue.rs) |

```rust
let response = client
    .sessions()
    .queue()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        QueueUpdateBody {
            message_ids: vec!["7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string()],
        },
    )
    .send()
    .await?;
```

### `Sessions Commands`

#### List session commands

List slash commands available to the session runtime.

| Direction | Type |
| --- | --- |
| Response | [`CommandsListResponse`](./src/models/commands.rs) |

```rust
let response = client
    .sessions()
    .commands()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

## `Projects`

### Update project defaults

Choose the default project and sandbox size for the organization.

| Direction | Type |
| --- | --- |
| Request | [`ProjectsUpdateDefaultsBody`](./src/models/projects.rs) |
| Response | [`ProjectsUpdateDefaultsResponse`](./src/models/projects.rs) |

```rust
let response = client
    .projects()
    .update_defaults(ProjectsUpdateDefaultsBody {
        project_id: None,
        sandbox_size: ProjectsUpdateDefaultsBodySandboxSize::Nano,
    })
    .send()
    .await?;
```

### List projects

List projects in the authenticated organization.

| Direction | Type |
| --- | --- |
| Response | [`ProjectsListResponse`](./src/models/projects.rs) |

```rust
let response = client.projects().list().send().await?;
```

### Create a project

Create a reusable project environment.

| Direction | Type |
| --- | --- |
| Request | [`ProjectsCreateBody`](./src/models/projects.rs) |
| Response | [`ProjectsCreateResponse`](./src/models/projects.rs) |

```rust
let response = client
    .projects()
    .create(ProjectsCreateBody {
        description: None,
        name: "x".to_string(),
        setup_script: None,
        code_repository_ids: None,
        agent_instructions: None,
        color: None,
        icon: None,
        include_deps: None,
        include_skills: None,
        onboarding_default: None,
        run_now: None,
        schedule: None,
        sizes: None,
        resource_settings: None,
    })
    .send()
    .await?;
```

### Retrieve a project

Retrieve project configuration. Use the project subresources to list snapshots, builds, schedules, and environment variables.

| Direction | Type |
| --- | --- |
| Response | [`ProjectsRetrieveResponse`](./src/models/projects.rs) |

```rust
let response = client.projects().retrieve("example").send().await?;
```

### Update a project

Update project repositories, setup script, agent instructions, environment variables, VM sizes and defaults; enable or pause scheduled builds or the warm pool; rename or customize its appearance.

| Direction | Type |
| --- | --- |
| Request | [`ProjectsUpdateBody`](./src/models/projects.rs) |
| Response | [`ProjectsUpdateResponse`](./src/models/projects.rs) |

```rust
let response = client
    .projects()
    .update(
        "example",
        ProjectsUpdateBody {
            description: None,
            name: None,
            setup_script: None,
            code_repository_ids: None,
            agent_instructions: None,
            color: None,
            icon: None,
            include_deps: None,
            include_skills: None,
            resource_settings: None,
            default_for_sizes: None,
            environment_variables: None,
            schedule: None,
            sizes: None,
            warm_pool_schedule: None,
            warm_pool_enabled: None,
        },
    )
    .send()
    .await?;
```

### Delete a project

Archive a project, remove its default VM-size assignments, and disable its build schedule. This does not permanently delete the project record.

| Direction | Type |
| --- | --- |
| Response | [`ProjectsDeleteResponse`](./src/models/projects.rs) |

```rust
let response = client.projects().delete("example").send().await?;
```

### Duplicate a project

Create a new project with the same configuration, environment variables, and schedule as an existing project. Snapshots are not copied.

| Direction | Type |
| --- | --- |
| Response | [`ProjectsDuplicateResponse`](./src/models/projects.rs) |

```rust
let response = client.projects().duplicate("example").send().await?;
```

### `Projects EnvironmentVariables`

#### List project environment variables

List environment-variable names without exposing values.

| Direction | Type |
| --- | --- |
| Response | [`EnvironmentVariablesListResponse2`](./src/models/projects_environment_variables.rs) |

```rust
let response = client.projects().environment_variables().list("example").send().await?;
```

#### Create project environment variables

Create one or more encrypted project environment variables.

| Direction | Type |
| --- | --- |
| Request | [`EnvironmentVariablesCreateBody2`](./src/models/projects_environment_variables.rs) |
| Response | [`EnvironmentVariablesCreateResponse2`](./src/models/projects_environment_variables.rs) |

```rust
let response = client
    .projects()
    .environment_variables()
    .create(
        "example",
        EnvironmentVariablesCreateBody2 {
            secrets: vec![EnvironmentVariablesCreateBody2Secret {
                key: "".to_string(),
                value: "x".to_string(),
            }],
        },
    )
    .send()
    .await?;
```

#### Delete a project environment variable

Delete one encrypted project environment variable.

| Direction | Type |
| --- | --- |
| Response | [`EnvironmentVariablesDeleteResponse2`](./src/models/projects_environment_variables.rs) |

```rust
let response = client
    .projects()
    .environment_variables()
    .delete("example", "example")
    .send()
    .await?;
```

### `Projects Snapshots`

#### List project snapshots

List recent VM snapshots built for a project.

| Direction | Type |
| --- | --- |
| Response | [`SnapshotsListResponse`](./src/models/snapshots.rs) |

```rust
let response = client.projects().snapshots().list("example").send().await?;
```

#### Retrieve a project snapshot

Retrieve one project snapshot.

| Direction | Type |
| --- | --- |
| Response | [`SnapshotsRetrieveResponse`](./src/models/snapshots.rs) |

```rust
let response = client
    .projects()
    .snapshots()
    .retrieve("example", "example")
    .send()
    .await?;
```

#### Update a project snapshot

Rename a snapshot or choose whether it is pinned.

| Direction | Type |
| --- | --- |
| Request | [`SnapshotsUpdateBody`](./src/models/snapshots.rs) |
| Response | [`SnapshotsUpdateResponse`](./src/models/snapshots.rs) |

```rust
let response = client
    .projects()
    .snapshots()
    .update(
        "example",
        "example",
        SnapshotsUpdateBody {
            name: None,
            pinned: None,
        },
    )
    .send()
    .await?;
```

#### Delete a project snapshot

Delete a project snapshot.

| Direction | Type |
| --- | --- |
| Response | [`SnapshotsDeleteResponse`](./src/models/snapshots.rs) |

```rust
let response = client
    .projects()
    .snapshots()
    .delete("example", "example")
    .send()
    .await?;
```

### `Projects Builds`

#### List project builds

List recent queued, running, and failed project builds.

| Direction | Type |
| --- | --- |
| Response | [`BuildsListResponse`](./src/models/builds.rs) |

```rust
let response = client.projects().builds().list("example").send().await?;
```

#### Create a project build

Build selected project sizes or rebuild one snapshot.

| Direction | Type |
| --- | --- |
| Request | [`BuildsCreateBody`](./src/models/builds.rs) |
| Response | [`BuildsCreateResponse`](./src/models/builds.rs) |

```rust
let response = client
    .projects()
    .builds()
    .create(
        "example",
        BuildsCreateBody {
            sizes: None,
            snapshot_id: None,
        },
    )
    .send()
    .await?;
```

#### Retrieve a project build

Retrieve project build status.

| Direction | Type |
| --- | --- |
| Response | [`BuildsRetrieveResponse`](./src/models/builds.rs) |

```rust
let response = client.projects().builds().retrieve("example", "example").send().await?;
```

#### Cancel a project build

Cancel an active queued or running project build.

| Direction | Type |
| --- | --- |
| Response | [`BuildsCancelResponse`](./src/models/builds.rs) |

```rust
let response = client.projects().builds().cancel("example", "example").send().await?;
```

### `Projects Schedules`

#### List project schedules

List schedules linked to a project.

| Direction | Type |
| --- | --- |
| Response | [`SchedulesListResponse`](./src/models/projects_schedules.rs) |

```rust
let response = client.projects().schedules().list("example").send().await?;
```

#### Create a project schedule

Create a recurring project build schedule.

| Direction | Type |
| --- | --- |
| Request | [`SchedulesCreateBody`](./src/models/projects_schedules.rs) |
| Response | [`SchedulesCreateResponse`](./src/models/projects_schedules.rs) |

```rust
let response = client
    .projects()
    .schedules()
    .create(
        "example",
        SchedulesCreateBody {
            cron: "x".to_string(),
            timezone: None,
        },
    )
    .send()
    .await?;
```

#### Retrieve a project schedule

Retrieve one project build schedule.

| Direction | Type |
| --- | --- |
| Response | [`SchedulesRetrieveResponse`](./src/models/projects_schedules.rs) |

```rust
let response = client
    .projects()
    .schedules()
    .retrieve("example", "example")
    .send()
    .await?;
```

#### Update a project schedule

Update timing or enabled state for a project schedule.

| Direction | Type |
| --- | --- |
| Request | [`SchedulesUpdateBody`](./src/models/projects_schedules.rs) |
| Response | [`SchedulesUpdateResponse`](./src/models/projects_schedules.rs) |

```rust
let response = client
    .projects()
    .schedules()
    .update(
        "example",
        "example",
        SchedulesUpdateBody {
            cron: None,
            enabled: None,
            timezone: None,
        },
    )
    .send()
    .await?;
```

#### Delete a project schedule

Disable a project build schedule.

| Direction | Type |
| --- | --- |
| Response | [`SchedulesDeleteResponse`](./src/models/projects_schedules.rs) |

```rust
let response = client
    .projects()
    .schedules()
    .delete("example", "example")
    .send()
    .await?;
```

## `Pins`

### List pins

List your pinned resources in pin order.

| Direction | Type |
| --- | --- |
| Response | [`PinsListResponse`](./src/models/pins.rs) |

```rust
let response = client.pins().list().send().await?;
```

### Create a pin

Pin a session, pull request, or project you have permission to access.

| Direction | Type |
| --- | --- |
| Request | [`PinsCreateBody`](./src/models/pins.rs) |
| Response | [`PinsCreateResponse`](./src/models/pins.rs) |

```rust
let response = client
    .pins()
    .create(PinsCreateBody {
        r#type: PinsCreateBodyType::Session,
        id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
        placement: None,
    })
    .send()
    .await?;
```

### Update a pin

Reorder a pinned session, pull request, or project before or after another pin for the authenticated user. Use createPin to pin a resource and deletePin to unpin it.

| Direction | Type |
| --- | --- |
| Request | [`PinsUpdateBody`](./src/models/pins.rs) |
| Response | [`PinsUpdateResponse`](./src/models/pins.rs) |

```rust
let response = client
    .pins()
    .update(
        "session".to_string(),
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        PinsUpdateBody {
            placement: PinsUpdateBodyPlacement {
                target: PinsUpdateBodyPlacementTarget {
                    r#type: PinsUpdateBodyPlacementTargetType::Session,
                    id: "7c9e6679-7425-40de-944b-e07fc1f90ae7".to_string(),
                },
                position: PinsUpdateBodyPlacementPosition::Before,
            },
        },
    )
    .send()
    .await?;
```

### Delete a pin

Unpin a session, pull request, or project for the authenticated user without deleting or archiving the resource.

| Direction | Type |
| --- | --- |
| Response | [`PinsDeleteResponse`](./src/models/pins.rs) |

```rust
let response = client
    .pins()
    .delete("session".to_string(), "7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

## `PullRequests`

### List pull requests

List pull requests for the authenticated organization with cursor pagination.

| Direction | Type |
| --- | --- |
| Response | [`PullRequestsListResponse`](./src/models/pull_requests.rs) |

```rust
let response = client.pull_requests().list().send().await?;
```

### Retrieve a pull request

Retrieve a pull request for the authenticated organization.

| Direction | Type |
| --- | --- |
| Response | [`PullRequestsRetrieveResponse`](./src/models/pull_requests.rs) |

```rust
let response = client
    .pull_requests()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

## `Agents`

### List agents

List agents available in your organization, with optional search, status, integration, and author filters.

| Direction | Type |
| --- | --- |
| Response | [`AgentsListResponse`](./src/models/agents.rs) |

```rust
let response = client.agents().list().send().await?;
```

### Create an agent

Create an agent from instructions, a template, or another agent. sourceAgentId cannot be combined with other fields. Template creation accepts only templateName, name, and triggers; creation from scratch requires name.

| Direction | Type |
| --- | --- |
| Request | [`AgentsCreateBody`](./src/models/agents.rs) |
| Response | [`AgentsCreateResponse`](./src/models/agents.rs) |

```rust
let response = client
    .agents()
    .create(AgentsCreateBody {
        name: None,
        instructions: None,
        mcp_servers: None,
        agent: None,
        agent_options: None,
        sandbox_size: None,
        project_id: None,
        artifact_type: None,
        repository_ids: None,
        triggers: None,
        schedules: None,
        template_name: None,
        source_agent_id: None,
    })
    .send()
    .await?;
```

### Retrieve an agent

Retrieve an agent and its configuration, repository associations, and run summary.

| Direction | Type |
| --- | --- |
| Response | [`AgentsRetrieveResponse`](./src/models/agents.rs) |

```rust
let response = client
    .agents()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Update an agent

Update agent instructions, model, workspace, repositories, or persistent state; enable or pause the agent; archive or restore it. enabled and archived also control its schedules and triggers. Supply expectedUpdatedAt to reject concurrent changes.

| Direction | Type |
| --- | --- |
| Request | [`AgentsUpdateBody`](./src/models/agents.rs) |
| Response | [`AgentsUpdateResponse`](./src/models/agents.rs) |

```rust
let response = client
    .agents()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        AgentsUpdateBody {
            expected_updated_at: None,
            name: None,
            key: None,
            instructions: None,
            mcp_servers: None,
            agent: None,
            agent_options: None,
            user_state: None,
            sandbox_size: None,
            project_id: None,
            template_id: None,
            script: None,
            artifact_type: None,
            repository_ids: None,
            enabled: None,
            archived: None,
        },
    )
    .send()
    .await?;
```

### Delete an agent

Archive an agent and disable its scheduled and triggered jobs without permanently deleting it. Restore with updateAgent and archived=false.

| Direction | Type |
| --- | --- |
| Response | [`AgentsDeleteResponse`](./src/models/agents.rs) |

```rust
let response = client
    .agents()
    .delete("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### `Agents Templates`

#### List agent templates

List reusable agent templates and their configuration requirements.

| Direction | Type |
| --- | --- |
| Response | [`TemplatesListResponse`](./src/models/templates.rs) |

```rust
let response = client.agents().templates().list().send().await?;
```

#### Retrieve an agent template

Retrieve a reusable agent template by name, including its instructions and required parameters.

| Direction | Type |
| --- | --- |
| Response | [`TemplatesRetrieveResponse`](./src/models/templates.rs) |

```rust
let response = client.agents().templates().retrieve("example").send().await?;
```

### `Agents Schedules`

#### List agent schedules

List the schedules configured for an agent.

| Direction | Type |
| --- | --- |
| Response | [`SchedulesListResponse2`](./src/models/agents_schedules.rs) |

```rust
let response = client
    .agents()
    .schedules()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Create an agent schedule

Create a recurring schedule that starts agent runs using a cron expression.

| Direction | Type |
| --- | --- |
| Request | [`SchedulesCreateBody2`](./src/models/agents_schedules.rs) |
| Response | [`SchedulesCreateResponse2`](./src/models/agents_schedules.rs) |

```rust
let response = client
    .agents()
    .schedules()
    .create(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        SchedulesCreateBody2 { cron: "x".to_string() },
    )
    .send()
    .await?;
```

#### Retrieve an agent schedule

Retrieve an agent schedule, including its enabled state and next execution time.

| Direction | Type |
| --- | --- |
| Response | [`SchedulesRetrieveResponse2`](./src/models/agents_schedules.rs) |

```rust
let response = client
    .agents()
    .schedules()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

#### Update an agent schedule

Change an agent schedule cron expression. Use updateAgent with enabled to enable or pause the agent and its schedules.

| Direction | Type |
| --- | --- |
| Request | [`SchedulesUpdateBody2`](./src/models/agents_schedules.rs) |
| Response | [`SchedulesUpdateResponse2`](./src/models/agents_schedules.rs) |

```rust
let response = client
    .agents()
    .schedules()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        SchedulesUpdateBody2 { cron: "x".to_string() },
    )
    .send()
    .await?;
```

#### Delete an agent schedule

Delete a recurring agent schedule.

| Direction | Type |
| --- | --- |
| Response | [`SchedulesDeleteResponse2`](./src/models/agents_schedules.rs) |

```rust
let response = client
    .agents()
    .schedules()
    .delete(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### `Agents Triggers`

#### List agent triggers

List the integration events configured to start runs of an agent.

| Direction | Type |
| --- | --- |
| Response | [`TriggersListResponse`](./src/models/agents_triggers.rs) |

```rust
let response = client
    .agents()
    .triggers()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Create an agent trigger

Configure an integration event to start agent runs. Available filters depend on the integration and event name.

| Direction | Type |
| --- | --- |
| Request | [`TriggersCreateBody`](./src/models/agents_triggers.rs) |
| Response | [`TriggersCreateResponse`](./src/models/agents_triggers.rs) |

```rust
let response = client
    .agents()
    .triggers()
    .create(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        TriggersCreateBody {
            name: "x".to_string(),
            integration_type: None,
            integration_id: None,
            filters: None,
        },
    )
    .send()
    .await?;
```

#### Retrieve an agent trigger

Retrieve an agent trigger and its integration event filters.

| Direction | Type |
| --- | --- |
| Response | [`TriggersRetrieveResponse`](./src/models/agents_triggers.rs) |

```rust
let response = client
    .agents()
    .triggers()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

#### Update an agent trigger

Change an agent trigger event, integration, or filters. Use updateAgent with enabled to enable or pause the agent and its triggers.

| Direction | Type |
| --- | --- |
| Request | [`TriggersUpdateBody`](./src/models/agents_triggers.rs) |
| Response | [`TriggersUpdateResponse`](./src/models/agents_triggers.rs) |

```rust
let response = client
    .agents()
    .triggers()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        TriggersUpdateBody {
            name: "x".to_string(),
            integration_id: None,
            filters: None,
        },
    )
    .send()
    .await?;
```

#### Delete an agent trigger

Delete an integration event trigger for an agent.

| Direction | Type |
| --- | --- |
| Response | [`TriggersDeleteResponse`](./src/models/agents_triggers.rs) |

```rust
let response = client
    .agents()
    .triggers()
    .delete(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### `Agents Runs`

#### List agent runs

List executions of an agent, including status, linked session, measured duration, and failure summary.

| Direction | Type |
| --- | --- |
| Response | [`RunsListResponse`](./src/models/runs.rs) |

```rust
let response = client
    .agents()
    .runs()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Create an agent run

Queue an agent run. The entire JSON object body is passed to the agent as event input; no eventPayload wrapper is needed. Omit the body to run without event input. The response confirms queuing, not completed execution.

| Direction | Type |
| --- | --- |
| Request | `std::collections::HashMap<String, serde_json::Value>` |
| Response | [`RunsCreateResponse`](./src/models/runs.rs) |

```rust
let response = client
    .agents()
    .runs()
    .create("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Retrieve an agent run

Retrieve the status, linked session, measured duration, and failure summary of an agent run.

| Direction | Type |
| --- | --- |
| Response | [`RunsRetrieveResponse`](./src/models/runs.rs) |

```rust
let response = client
    .agents()
    .runs()
    .retrieve(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
    )
    .send()
    .await?;
```

### `Agents ContentRevisions`

#### List agent instruction revisions

List saved revisions of an agent’s instruction JSON document, newest first. Revisions do not include other agent configuration. Author details are redacted from public API responses.

| Direction | Type |
| --- | --- |
| Response | [`ContentRevisionsListResponse`](./src/models/content_revisions.rs) |

```rust
let response = client
    .agents()
    .content_revisions()
    .list("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

#### Retrieve an agent instruction revision

Retrieve the instruction JSON document saved in one revision. Other agent configuration is not included. Author details are redacted from public API responses.

| Direction | Type |
| --- | --- |
| Response | [`ContentRevisionsRetrieveResponse`](./src/models/content_revisions.rs) |

```rust
let response = client
    .agents()
    .content_revisions()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7", 1)
    .send()
    .await?;
```

#### Restore agent instructions

Restore a saved instruction JSON document and its mentioned integrations, without reverting other agent configuration. A changed document creates a new revision attributed to the caller. Supply the agent’s current contentVersion as expectedContentVersion; a concurrent instruction change returns 409.

| Direction | Type |
| --- | --- |
| Request | [`ContentRevisionsRestoreBody`](./src/models/content_revisions.rs) |
| Response | [`ContentRevisionsRestoreResponse`](./src/models/content_revisions.rs) |

```rust
let response = client
    .agents()
    .content_revisions()
    .restore(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        1,
        ContentRevisionsRestoreBody {
            expected_content_version: 1,
        },
    )
    .send()
    .await?;
```

## `Repositories`

### List repositories

List repositories in the authenticated organization. Defaults to enabled repositories.

| Direction | Type |
| --- | --- |
| Response | [`RepositoriesListResponse`](./src/models/repositories.rs) |

```rust
let response = client.repositories().list().send().await?;
```

### Retrieve a repository

Retrieve a repository in the authenticated organization.

| Direction | Type |
| --- | --- |
| Response | [`RepositoriesRetrieveResponse`](./src/models/repositories.rs) |

```rust
let response = client
    .repositories()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Update a repository

Update repository configuration or enable or disable it.

| Direction | Type |
| --- | --- |
| Request | [`RepositoriesUpdateBody`](./src/models/repositories.rs) |
| Response | [`RepositoriesUpdateResponse`](./src/models/repositories.rs) |

```rust
let response = client
    .repositories()
    .update(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        RepositoriesUpdateBody {
            enabled: None,
            base_branch: None,
            target_branch: None,
        },
    )
    .send()
    .await?;
```

## `Integrations`

### List integrations

List integrations visible to the current organization or authenticated admin.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsListResponse`](./src/models/integrations.rs) |

```rust
let response = client.integrations().list().send().await?;
```

### Create an integration

Connect AWS, BigQuery, GitLab, PostgreSQL, Snyk, OpenConnect VPN, or Tailscale using provider credentials. For browser-authorized providers, use listIntegrationProviders and authorizeIntegration instead.

| Direction | Type |
| --- | --- |
| Request | [`IntegrationsCreateBody`](./src/models/integrations.rs) |
| Response | [`IntegrationsCreateResponse`](./src/models/integrations.rs) |

### Retrieve an integration

Retrieve an integration and its configuration.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsRetrieveResponse`](./src/models/integrations.rs) |

```rust
let response = client
    .integrations()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Update an integration

Update integration settings such as the default repository, Slack channel restrictions, automatic solving, or Sentry environments; replace AWS or PostgreSQL configuration; set or remove Atlassian service-account credentials.

| Direction | Type |
| --- | --- |
| Request | [`IntegrationsUpdateBody`](./src/models/integrations.rs) |
| Response | [`IntegrationsUpdateResponse`](./src/models/integrations.rs) |

### Delete an integration

Uninstall an integration from its provider when supported, then delete it from Tembo.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsDeleteResponse`](./src/models/integrations.rs) |

```rust
let response = client
    .integrations()
    .delete("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Authorize an integration

Return the provider authorization URL for an integration.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsAuthorizeResponse`](./src/models/integrations.rs) |

```rust
let response = client
    .integrations()
    .authorize("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Test an integration

Test the saved provider connection.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsTestResponse`](./src/models/integrations.rs) |

```rust
let response = client
    .integrations()
    .test("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### Sync an integration

Queue or immediately perform an integration sync.

| Direction | Type |
| --- | --- |
| Request | [`IntegrationsSyncBody`](./src/models/integrations.rs) |
| Response | [`IntegrationsSyncResponse`](./src/models/integrations.rs) |

```rust
let response = client
    .integrations()
    .sync(
        "7c9e6679-7425-40de-944b-e07fc1f90ae7",
        IntegrationsSyncBody { mode: None },
    )
    .send()
    .await?;
```

### Sync eligible integrations

Queue integration and pull-request sync jobs for integrations with enabled repositories.

| Direction | Type |
| --- | --- |
| Request | [`IntegrationsSyncAllBody`](./src/models/integrations.rs) |
| Response | [`IntegrationsSyncAllResponse`](./src/models/integrations.rs) |

```rust
let response = client
    .integrations()
    .sync_all(IntegrationsSyncAllBody {})
    .send()
    .await?;
```

### Retrieve integration sync status

Count active sync jobs for an integration or eligible integrations.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsRetrieveSyncStatusResponse`](./src/models/integrations.rs) |

```rust
let response = client.integrations().retrieve_sync_status().send().await?;
```

### Retrieve an integration rate limit

Retrieve the GitHub installation rate-limit state.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsRetrieveRateLimitResponse`](./src/models/integrations.rs) |

```rust
let response = client
    .integrations()
    .retrieve_rate_limit("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### List Sentry environments

List environments visible to a Sentry integration.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsListSentryEnvironmentsResponse`](./src/models/integrations.rs) |

```rust
let response = client
    .integrations()
    .list_sentry_environments("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### List Slack channels

List channels visible to a Slack integration.

| Direction | Type |
| --- | --- |
| Response | [`IntegrationsListSlackChannelsResponse`](./src/models/integrations.rs) |

```rust
let response = client
    .integrations()
    .list_slack_channels("7c9e6679-7425-40de-944b-e07fc1f90ae7")
    .send()
    .await?;
```

### `Integrations Providers`

#### List integration providers

List providers available to the current organization.

| Direction | Type |
| --- | --- |
| Response | [`ProvidersListResponse`](./src/models/providers.rs) |

```rust
let response = client.integrations().providers().list().send().await?;
```

#### Discover Snyk organizations

Validate Snyk service-account credentials and list accessible organizations.

| Direction | Type |
| --- | --- |
| Request | [`ProvidersDiscoverSnykOrganizationsBody`](./src/models/providers.rs) |
| Response | [`ProvidersDiscoverSnykOrganizationsResponse`](./src/models/providers.rs) |

```rust
let response = client
    .integrations()
    .providers()
    .discover_snyk_organizations(ProvidersDiscoverSnykOrganizationsBody {
        client_id: "x".to_string(),
        client_secret: "x".to_string(),
        cursor: None,
        limit: None,
    })
    .send()
    .await?;
```

### `Integrations Triggers`

#### List integration triggers

List visible automation triggers grouped by provider.

| Direction | Type |
| --- | --- |
| Response | [`TriggersListResponse2`](./src/models/integrations_triggers.rs) |

```rust
let response = client.integrations().triggers().list().send().await?;
```

#### Retrieve an integration trigger

Retrieve a trigger definition and its default filter values for an integration in the current organization.

| Direction | Type |
| --- | --- |
| Response | [`TriggersRetrieveResponse2`](./src/models/integrations_triggers.rs) |

```rust
let response = client
    .integrations()
    .triggers()
    .retrieve("7c9e6679-7425-40de-944b-e07fc1f90ae7", "example")
    .send()
    .await?;
```

#### List integration trigger filter options

List one cursor-page of selectable values for an integration trigger filter field, with optional search.

| Direction | Type |
| --- | --- |
| Response | [`TriggersListFilterOptionsResponse`](./src/models/integrations_triggers.rs) |

```rust
let response = client
    .integrations()
    .triggers()
    .list_filter_options("7c9e6679-7425-40de-944b-e07fc1f90ae7", "example", "example")
    .send()
    .await?;
```

## `Billing`

### Retrieve billing

Retrieve billing and credit information for the organization.

| Direction | Type |
| --- | --- |
| Response | [`BillingRetrieveResponse`](./src/models/billing.rs) |

```rust
let response = client.billing().retrieve().send().await?;
```

### List billing usage

List session usage by default, or include warm pool compute with the type filter. Uses cursor pagination.

| Direction | Type |
| --- | --- |
| Response | [`BillingListUsageResponse`](./src/models/billing.rs) |

```rust
let response = client.billing().list_usage().send().await?;
```

## `Live`

### Authorize a live connection

Issue a 30-second WebSocket ticket for one live connection that multiplexes resource subscriptions.

| Direction | Type |
| --- | --- |
| Response | [`LiveAuthorizeResponse`](./src/models/live.rs) |

```rust
let response = client.live().authorize().send().await?;
```

### Live resource subscriptions

Native WebSocket. Send `{ op: "subscribe", id, resource, input }`, `{ op: "unsubscribe", id }`, or `{ op: "ping" }`; receive `{ id, type: "ready" | "event" | "error" }` or `{ type: "pong" }`.

| Direction | Type |
| --- | --- |
| Response | [`WebSocketConnection<serde_json::Value, serde_json::Value>`](./src/websocket.rs) |

```rust
let _connection = client.live().connect("example").open().await?;
```
