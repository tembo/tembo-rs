//! Regression tests for hand-written model customizations (see `src/custom.rs`).

use serde_json::json;
use tembo_rs::models::{
    ProjectsCreateBodyResourceSettings, ProjectsRetrieveResponseResourceSettings, ProjectsUpdateBody,
};

#[test]
fn warm_pool_schedule_distinguishes_omitted_null_and_set() {
    let omitted = ProjectsUpdateBody::default();
    assert_eq!(serde_json::to_value(&omitted).unwrap(), json!({}));

    let cleared = ProjectsUpdateBody {
        warm_pool_schedule: Some(None),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(&cleared).unwrap(),
        json!({ "warmPoolSchedule": null })
    );

    let schedule = json!({ "timezone": "UTC", "days": [1, 2], "startTime": "09:00", "endTime": "17:00" });
    let set: ProjectsUpdateBody = serde_json::from_value(json!({ "warmPoolSchedule": schedule })).unwrap();
    assert!(matches!(set.warm_pool_schedule, Some(Some(_))));
    assert_eq!(
        serde_json::to_value(&set).unwrap(),
        json!({ "warmPoolSchedule": schedule })
    );

    let null: ProjectsUpdateBody = serde_json::from_value(json!({ "warmPoolSchedule": null })).unwrap();
    assert_eq!(null.warm_pool_schedule, Some(None));
    assert_eq!(
        serde_json::to_value(&null).unwrap(),
        json!({ "warmPoolSchedule": null })
    );

    let absent: ProjectsUpdateBody = serde_json::from_value(json!({})).unwrap();
    assert_eq!(absent.warm_pool_schedule, None);
}

#[test]
fn resource_settings_select_the_variant_matching_automatic_sizing() {
    let allocation = json!({ "memoryMinMb": 2048, "memoryMaxMb": 8192, "diskSizeGb": 50 });

    let automatic = json!({ "automaticSizing": true, "allocation": allocation });
    let fixed = json!({ "automaticSizing": false, "allocation": allocation });

    let response: ProjectsRetrieveResponseResourceSettings = serde_json::from_value(automatic.clone()).unwrap();
    assert!(matches!(
        response,
        ProjectsRetrieveResponseResourceSettings::ProjectsRetrieveResponseResourceSettingsVariant1(_)
    ));
    assert_eq!(serde_json::to_value(&response).unwrap(), automatic);

    let response: ProjectsRetrieveResponseResourceSettings = serde_json::from_value(fixed.clone()).unwrap();
    assert!(matches!(
        response,
        ProjectsRetrieveResponseResourceSettings::ProjectsRetrieveResponseResourceSettingsVariant2(_)
    ));
    assert_eq!(serde_json::to_value(&response).unwrap(), fixed);

    let body: ProjectsCreateBodyResourceSettings = serde_json::from_value(fixed.clone()).unwrap();
    assert!(matches!(
        body,
        ProjectsCreateBodyResourceSettings::ProjectsCreateBodyResourceSettingsVariant2(_)
    ));
    assert_eq!(serde_json::to_value(&body).unwrap(), fixed);
}
