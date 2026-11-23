# Changelog

## [0.15.0](https://github.com/tembo/tembo-rs/compare/v0.14.0...v0.15.0) (2026-11-22)


### ⚠ BREAKING CHANGES

* **api:** 187 breaking changes to the SDK surface.
    - types resources::shares::CreateRequestBuilder — struct only in before (breaking: the new snapshot no longer has it)
    - types resources::shares::DeleteRequestBuilder — struct only in before (breaking: the new snapshot no longer has it)
    - types resources::shares::ListRequestBuilder — struct only in before (breaking: the new snapshot no longer has it)
    - types resources::shares::SharesResource — struct only in before (breaking: the new snapshot no longer has it)
    - params models::BillingListUsageResponseItemVariant1.new/#8 — argument total_credits retyped (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant1.new/#9 — new required argument r#type in after; existing calls lack it (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant1.new — positional arguments moved (compute_credits, total_credits, r#type); existing calls pass each value in another slot (breaking: changed in a way existing callers can observe)
    - fields models::BillingListUsageResponseItemVariant2.cpu — field only in before (breaking: removed; model, platform, os_version (and 3 more) appeared in its place, likely a rename)
    - fields models::BillingListUsageResponseItemVariant2.memory_mb — field only in before (breaking: removed; os_version, platform, session_id (and 3 more) appeared in its place, likely a rename)
    - fields models::BillingListUsageResponseItemVariant2.runtime_ms — field only in before (breaking: removed; os_version, session_id, billed_seconds (and 3 more) appeared in its place, likely a rename)
    - fields models::BillingListUsageResponseItemVariant2.virtual_machine_id — field only in before (breaking: removed; session_id, billed_seconds, session_title (and 3 more) appeared in its place, likely a rename)
    - params models::BillingListUsageResponseItemVariant2.new/#2 — argument session_title retyped (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant2.new/#3 — argument platform retyped (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant2.new/#4 — argument model retyped (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant2.new/#5 — argument os_version retyped (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant2.new/#6 — argument started_at retyped (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant2.new/#7 — argument ended_at retyped (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant2.new/#8 — argument billed_seconds retyped (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant2.new/#9 — new required argument compute_credits in after; existing calls lack it (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant2.new/#10 — new required argument r#type in after; existing calls lack it (breaking: changed in a way existing callers can observe)
    - params models::BillingListUsageResponseItemVariant2.new — positional arguments moved (started_at, ended_at, compute_credits, r#type); existing calls pass each value in another slot (breaking: changed in a way existing callers can observe)
    - params models::ModelsListResponseItem.new/#3 — argument provider retyped (breaking: changed in a way existing call…

### Features

* **api:** update SDK surface (432 changes) ([f80ec97](https://github.com/tembo/tembo-rs/commit/f80ec97b7b5920f7a39902ef642c194ce7811dc9))

## [0.14.0](https://github.com/tembo/tembo-rs/compare/v0.13.0...v0.14.0) (2026-11-20)


### ⚠ BREAKING CHANGES

* **api:** 6 breaking changes to the SDK surface.
    - variants models::TriggerFilters.Variant41 — variant only in before (breaking: removed; Variant46, Variant47, Variant48 (and 3 more) appeared in its place, likely a rename)
    - variants models::TriggerFilters.Variant42 — variant only in before (breaking: removed; Variant46, Variant47, Variant48 (and 3 more) appeared in its place, likely a rename)
    - variants models::TriggerFilters.Variant43 — variant only in before (breaking: removed; Variant46, Variant47, Variant48 (and 3 more) appeared in its place, likely a rename)
    - variants models::TriggerFiltersOutput.Variant41 — variant only in before (breaking: removed; Variant46, Variant47, Variant48 (and 3 more) appeared in its place, likely a rename)
    - variants models::TriggerFiltersOutput.Variant42 — variant only in before (breaking: removed; Variant46, Variant47, Variant48 (and 3 more) appeared in its place, likely a rename)
    - variants models::TriggerFiltersOutput.Variant43 — variant only in before (breaking: removed; Variant46, Variant47, Variant48 (and 3 more) appeared in its place, likely a rename)

### Features

* **api:** update SDK surface (34 changes) ([d057195](https://github.com/tembo/tembo-rs/commit/d057195ef87b11784fad43cc26e119cb042e899e))

## [0.13.0](https://github.com/tembo/tembo-rs/compare/v0.12.0...v0.13.0) (2026-11-18)


### ⚠ BREAKING CHANGES

* **api:** 46 breaking changes to the SDK surface.
    - Serialization or defaults of query param `enabled` on `repositories.list` changed.
    - Property `tip_tap_ordered_list_node.attrs` validation changed (`start.minimum` none → `-9007199254740991`, `start.maximum` none → `9007199254740991`).
    - Property `session_event_data.index` validation changed (`minimum` none → `-9007199254740991`, `maximum` none → `9007199254740991`).
    - Property `session_event_data.sequenceNumber` validation changed (`minimum` none → `-9007199254740991`, `maximum` none → `9007199254740991`).
    - Schema `atlassian_issue_added_to_board_filters` shape changed.
    - Schema `atlassian_issue_assigned_to_user_filters` shape changed.
    - Schema `atlassian_issue_comment_created_filters` shape changed.
    - Schema `atlassian_issue_created_filters` shape changed.
    - Schema `atlassian_issue_label_added_filters` shape changed.
    - Schema `atlassian_issue_status_changed_filters` shape changed.
    - Schema `atlassian_issue_updated_filters` shape changed.
    - Schema `bitbucket_pull_request_opened_filters` shape changed.
    - Schema `github_create_filters` shape changed.
    - Schema `github_issue_closed_filters` shape changed.
    - Schema `github_issue_comment_created_filters` shape changed.
    - Schema `github_issue_opened_filters` shape changed.
    - Schema `github_pull_request_closed_filters` shape changed.
    - Schema `github_pull_request_merged_filters` shape changed.
    - Schema `github_pull_request_opened_filters` shape changed.
    - Schema `github_pull_request_ready_for_review_filters` shape changed.
    - Schema `github_release_published_filters` shape changed.
    - Schema `github_pull_request_synchronize_filters` shape changed.
    - Schema `github_tembo_pull_request_opened_filters` shape changed.
    - Schema `github_workflow_run_failed_filters` shape changed.
    - Schema `github_workflow_run_completed_filters` shape changed.
    - Schema `gitlab_bot_merge_request_comment_filters` shape changed.
    - Schema `gitlab_issue_closed_filters` shape changed.
    - Schema `gitlab_issue_opened_filters` shape changed.
    - Schema `gitlab_issue_label_added_filters` shape changed.
    - Schema `gitlab_issue_updated_filters` shape changed.
    - Schema `gitlab_merge_request_closed_filters` shape changed.
    - Schema `gitlab_merge_request_merged_filters` shape changed.
    - Schema `gitlab_merge_request_opened_filters` shape changed.
    - Schema `gitlab_merge_request_updated_filters` shape changed.
    - Schema `gitlab_merge_request_ready_for_review_filters` shape changed.
    - Schema `gitlab_pipeline_failed_filters` shape changed.
    - Schema `gitlab_ref_created_filters` shape changed.
    - Schema `linear_issue_create_filters` shape changed.
    - Schema `linear_issue_label_added_filters` shape changed.
    - Schema `linear_issue_status_changed_filters` shape changed.
    - Schema `linear_issue_updated_filters` shape changed.
    - Schema `sentry_issue_created_filters` shape changed.
    - Schema `slack_message_reaction_added_filters` shape changed.
    - Schema `slack_message_sent_in_channel_filters` shape changed.
    - Schema `empty_trigger_filters` shape changed.
    - Schema `trigger_filters` shape changed.

### Features

* **api:** update SDK surface (68 changes) ([bef374d](https://github.com/tembo/tembo-rs/commit/bef374d0b75b0a0331e552d19da3ac184cbbc19f))


### Chores

* **api:** update generated SDK content ([a0643c3](https://github.com/tembo/tembo-rs/commit/a0643c36cfb267a22341df97cc4d4b38e1023f7d))

## [0.12.0](https://github.com/tembo/tembo-rs/compare/v0.11.0...v0.12.0) (2026-11-16)


### ⚠ BREAKING CHANGES

* **api:** Schema `trigger_filters` shape changed.

### Features

* **api:** update schema trigger_filters (+8 more changes) ([74bacc0](https://github.com/tembo/tembo-rs/commit/74bacc09dfa2de997274eaaa669ce995c543898d))


### Chores

* **api:** update generated SDK content ([5a9ef34](https://github.com/tembo/tembo-rs/commit/5a9ef346822b132c2db6bd4c9cbeb5ac51957a91))

## [0.11.0](https://github.com/tembo/tembo-rs/compare/v0.10.1...v0.11.0) (2026-11-11)


### ⚠ BREAKING CHANGES

* **api:** 3 breaking changes to the SDK surface.
    - Added required body field `roleId` to `organizations.members.create`.
    - Removed body field `role` from `organizations.members.create`.
    - Removed body field `role` from `organizations.members.update`.

### Features

* **api:** add operation organizations.roles.list (+10 more changes) ([f294966](https://github.com/tembo/tembo-rs/commit/f294966a43c1126d3c1baec437aeef62ecdca8b0))
* **api:** update SDK surface (6 changes) ([67e72fa](https://github.com/tembo/tembo-rs/commit/67e72fa92858d9d24f0e9704a1c6b01c813c91bd))

## [0.10.1](https://github.com/tembo/tembo-rs/compare/v0.10.0...v0.10.1) (2026-11-08)


### Chores

* **api:** update generated SDK content ([791d052](https://github.com/tembo/tembo-rs/commit/791d052c1fbf1138f2eb0c6c041752ca7f55110d))

## [0.10.0](https://github.com/tembo/tembo-rs/compare/v0.9.0...v0.10.0) (2026-11-06)


### ⚠ BREAKING CHANGES

* **api:** Removed optional property `session_event_data.isProgressUpdate`.

### Features

* **api:** update SDK surface (2 changes) ([de7ae1e](https://github.com/tembo/tembo-rs/commit/de7ae1eb76983fad6eb00a03b1e18ddf02e8d936))

## [0.9.0](https://github.com/tembo/tembo-rs/compare/v0.8.0...v0.9.0) (2026-11-06)


### ⚠ BREAKING CHANGES

* **api:** Schema `trigger_filters` shape changed.

### Features

* **api:** add operation sessions.import ([75aef76](https://github.com/tembo/tembo-rs/commit/75aef7622c0728b152a71bd1d5cc351deacc2d7f))
* **api:** update schema trigger_filters (+6 more changes) ([d401579](https://github.com/tembo/tembo-rs/commit/d401579bdd941efeaadfa5bc5cc9df6b7a6f6fa2))

## [0.8.0](https://github.com/tembo/tembo-rs/compare/v0.7.0...v0.8.0) (2026-11-05)


### Features

* **api:** update SDK surface (2 changes) ([b2ef088](https://github.com/tembo/tembo-rs/commit/b2ef088af77d564648ac16114910d7c78e479cdc))


### Bug Fixes

* resolve Scalar conflict for generator 0.38.0 and smoke test stack overflow ([06005ba](https://github.com/tembo/tembo-rs/commit/06005ba4941eac4abe28da0e5afc42e9783d447b))
* **tests:** raise test thread stack for generated smoke test ([17c9d98](https://github.com/tembo/tembo-rs/commit/17c9d98031ed47ae7621a2266e8f0868c1e6665b))

## [0.7.0](https://github.com/tembo/tembo-rs/compare/v0.6.0...v0.7.0) (2026-11-03)


### Features

* **api:** add 403 error response on agents.runs.create (+1 more change) ([7a9e059](https://github.com/tembo/tembo-rs/commit/7a9e059605a197a37eb35e50468d5f87234b2a4b))

## [0.6.0](https://github.com/tembo/tembo-rs/compare/v0.5.1...v0.6.0) (2026-10-29)


### Features

* **api:** add query param parentSessionId on sessions.list ([0a81dc8](https://github.com/tembo/tembo-rs/commit/0a81dc83615cd4362dd65ec2302f4c1606c3970b))

## [0.5.1](https://github.com/tembo/tembo-rs/compare/v0.5.0...v0.5.1) (2026-10-29)


### Chores

* **api:** update generated SDK content ([489a947](https://github.com/tembo/tembo-rs/commit/489a94757c2a00bd531f5d8bf696bb1f0af6c2bd))

## [0.5.0](https://github.com/tembo/tembo-rs/compare/v0.4.0...v0.5.0) (2026-10-28)


### ⚠ BREAKING CHANGES

* **api:** 3 breaking changes to the SDK surface.
    - Removed operation `organizations.subscriptions.reset` (`POST /v1/organizations/{organizationId}/subscriptions/reset`).
    - Removed operation `users.delete` (`DELETE /v1/users/{userId}`).
    - Removed operation `users.subscriptions.reset` (`POST /v1/users/{userId}/subscriptions/reset`).

### Features

* **api:** update SDK surface (10 changes) ([2dfaad6](https://github.com/tembo/tembo-rs/commit/2dfaad6c6bb8bd666fcf456b89ce890dd1f70f1f))
* **api:** update SDK surface (3 changes) ([cf4faea](https://github.com/tembo/tembo-rs/commit/cf4faeab1c3f3f3dc0d493f565e549825dde03a5))


### Bug Fixes

* **models:** allow clearing warmPoolSchedule, select resourceSettings variants ([d88c321](https://github.com/tembo/tembo-rs/commit/d88c321a1a32d697a56a227e88f0d2aad7b95de3))
* **models:** allow clearing warmPoolSchedule, select resourceSettings variants ([02d56d8](https://github.com/tembo/tembo-rs/commit/02d56d8c1fe3678e9724298d84b406f7c2383d28))

## [0.4.0](https://github.com/tembo/tembo-rs/compare/v0.3.0...v0.4.0) (2026-10-23)


### ⚠ BREAKING CHANGES

* **api:** Schema `agent_options_input` shape changed.

### Features

* **api:** update schema agent_options_input (+7 more changes) ([ded6917](https://github.com/tembo/tembo-rs/commit/ded69178d849bcbbd107d3a32803c5564ed58dd6))

## [0.3.0](https://github.com/tembo/tembo-rs/compare/v0.2.1...v0.3.0) (2026-10-21)


### Features

* **api:** add operation sessions.inputs.list (+3 more changes) ([aacd725](https://github.com/tembo/tembo-rs/commit/aacd7252b5d3edfded22f1f6f728c7e20416e7f5))
* **api:** update SDK surface (1 change) ([1a602e1](https://github.com/tembo/tembo-rs/commit/1a602e13e58672b05dc8af2659ed187670f045fa))

## [0.2.1](https://github.com/tembo/tembo-rs/compare/v0.2.0...v0.2.1) (2026-10-20)


### Chores

* **api:** regenerate SDK ([7d4120a](https://github.com/tembo/tembo-rs/commit/7d4120a2b8b74f7aaa68eb5d0da76ed7917a9f67))

## [0.2.0](https://github.com/tembo/tembo-rs/compare/v0.1.0...v0.2.0) (2026-10-19)


### Features

* **api:** add body field expiresAt on apiKeys.create (+7 more changes) ([40e9940](https://github.com/tembo/tembo-rs/commit/40e994010ca20fe0bb1b04b378c12c16bffab2db))
* **api:** add operation projects.duplicate (+4 more changes) ([df47091](https://github.com/tembo/tembo-rs/commit/df470918f14b1e279252d30485bbc9cd67bdd5ef))
* **api:** add query param range on insights.retrieve (+2 more changes) ([c2946e2](https://github.com/tembo/tembo-rs/commit/c2946e23b804f346ce25b63e1bdd0e027c7aa1c6))
* **api:** initial SDK generation ([41373fd](https://github.com/tembo/tembo-rs/commit/41373fd436c1ad6f070514ed245190107a07f1d5))
* **api:** update SDK surface (3 changes) ([64341c0](https://github.com/tembo/tembo-rs/commit/64341c0885049cfbc93f810a0ede9fb2d67ebffb))


### Bug Fixes

* **models:** allow module_inception for the models resource ([7effd3c](https://github.com/tembo/tembo-rs/commit/7effd3c3c5d483cf08c5b30ac23d8b85a56de3b6))
* **models:** allow module_inception for the models resource ([3866856](https://github.com/tembo/tembo-rs/commit/3866856a0008af0eed3c3f02377842d864f8a281))

## Changelog

All notable changes to `tembo-rs` are documented here. Release
tooling appends a section per released version below.

## Unreleased

- Initial generation of the `tembo-rs` SDK.
- Response-only models are marked `#[non_exhaustive]`, so new response
  fields can be added in future versions without a breaking release;
  request models stay literally constructible.
