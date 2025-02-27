use crate::helpers::test_context::TestContext;
use cargo_utils::LocalManifest;

#[tokio::test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
async fn release_plz_opens_pr_with_default_config() {
    let context = TestContext::new().await;

    context.run_release_pr().success();

    let opened_prs = context.opened_release_prs().await;
    let today = today();
    assert_eq!(opened_prs.len(), 1);
    assert_eq!(opened_prs[0].title, "chore: release v0.1.0");
    let username = context.gitea.user.username();
    let package = &context.gitea.repo;
    assert_eq!(
        opened_prs[0].body.as_ref().unwrap().trim(),
        format!(
            r#"
## 🤖 New release

* `{package}`: 0.1.0

<details><summary><i><b>Changelog</b></i></summary><p>

<blockquote>

## [0.1.0](https://localhost/{username}/{package}/releases/tag/v0.1.0) - {today}

### Other

- cargo init
- Initial commit
</blockquote>


</p></details>

---
This PR was generated with [release-plz](https://github.com/release-plz/release-plz/)."#,
        )
        .trim()
    );
}

#[tokio::test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
async fn release_plz_opens_pr_with_two_packages_and_default_config() {
    let one = "one";
    let two = "two";
    let context =
        TestContext::new_workspace(&[&format!("crates/{one}"), &format!("crates/{two}")]).await;

    context.run_release_pr().success();

    let opened_prs = context.opened_release_prs().await;
    let today = today();
    assert_eq!(opened_prs.len(), 1);
    assert_eq!(opened_prs[0].title, "chore: release v0.1.0");
    let username = context.gitea.user.username();
    let repo = &context.gitea.repo;
    assert_eq!(
        opened_prs[0].body.as_ref().unwrap().trim(),
        format!(
            r#"
## 🤖 New release

* `{one}`: 0.1.0
* `{two}`: 0.1.0

<details><summary><i><b>Changelog</b></i></summary><p>

## `{one}`

<blockquote>

## [0.1.0](https://localhost/{username}/{repo}/releases/tag/{one}-v0.1.0) - {today}

### Other

- cargo init
</blockquote>

## `{two}`

<blockquote>

## [0.1.0](https://localhost/{username}/{repo}/releases/tag/{two}-v0.1.0) - {today}

### Other

- cargo init
</blockquote>


</p></details>

---
This PR was generated with [release-plz](https://github.com/release-plz/release-plz/)."#,
        )
        .trim()
    );
}

#[tokio::test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
async fn release_plz_should_set_custom_pr_details() {
    let context = TestContext::new().await;

    let config = r#"
[workspace]
pr_name = "release: {{ package }} {{ version }}"
pr_body = """
{% for release in releases %}
{% if release.title %}
### {{release.title}}
{% endif %}
Package: {{release.package}} {{release.previous_version}} -> {{release.next_version}}
{% if release.changelog %}
Changes:
{{release.changelog}}
{% endif %}
{% endfor -%}
"""
    "#;

    context.write_release_plz_toml(config);
    context.run_release_pr().success();

    let expected_title = format!("release: {} 0.1.0", context.gitea.repo);
    let opened_prs = context.opened_release_prs().await;
    assert_eq!(opened_prs.len(), 1);
    assert_eq!(opened_prs[0].title, expected_title);
    let today = today();
    let package = &context.gitea.repo;
    let username = context.gitea.user.username();
    assert_eq!(
        opened_prs[0].body.as_ref().unwrap().trim(),
        format!(
            r#"
### [0.1.0](https://localhost/{username}/{package}/releases/tag/v0.1.0) - {today}

Package: {package} 0.1.0 -> 0.1.0

Changes:
### Other

- add config file
- cargo init
- Initial commit"#,
        )
        .trim()
    );

    context.merge_release_pr().await;
    // The commit contains the PR id number
    let expected_commit = format!("{expected_title} (#1)");
    assert_eq!(
        context.repo.current_commit_message().unwrap(),
        expected_commit
    );
}

#[tokio::test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
async fn release_plz_should_fail_for_multi_package_pr() {
    let context = TestContext::new_workspace(&["crates/one", "crates/two"]).await;

    let config = r#"
    [workspace]
    pr_name = "release: {{ package }} {{ version }}"
    "#;

    context.write_release_plz_toml(config);
    // This should fail because the workspace contains multiple packages
    // so the `package` variable is not available
    let outcome = context.run_release_pr().failure();
    let stderr = String::from_utf8_lossy(&outcome.get_output().stderr);
    assert!(stderr.contains("failed to render pr_name"));
}

#[tokio::test]
#[ignore = "This test fails in CI, but works locally on MacOS. TODO: fix this."]
// #[cfg_attr(not(feature = "docker-tests"), ignore)]
async fn release_plz_detects_edited_readme_cargo_toml_field() {
    let context = TestContext::new().await;

    context.run_release_pr().success();
    context.merge_release_pr().await;

    let expected_tag = "v0.1.0";

    context.run_release().success();

    let gitea_release = context.gitea.get_gitea_release(expected_tag).await;
    assert_eq!(gitea_release.name, expected_tag);

    move_readme(&context, "move readme");

    context.run_release_pr().success();
    context.merge_release_pr().await;

    let expected_tag = "v0.1.1";

    context.run_release().success();

    let gitea_release = context.gitea.get_gitea_release(expected_tag).await;
    assert_eq!(gitea_release.name, expected_tag);
    expect_test::expect![[r#"
        ### Other

        - move readme"#]]
    .assert_eq(&gitea_release.body);
}

#[tokio::test]
#[ignore = "This test fails in CI, but works locally on MacOS. TODO: fix this."]
// #[cfg_attr(not(feature = "docker-tests"), ignore)]
async fn release_plz_honors_features_always_increment_minor_flag() {
    let context = TestContext::new().await;

    let config = r#"
    [workspace]
    features_always_increment_minor = true
    "#;
    context.write_release_plz_toml(config);

    context.run_release_pr().success();
    context.merge_release_pr().await;

    let expected_tag = "v0.1.0";

    context.run_release().success();

    let gitea_release = context.gitea.get_gitea_release(expected_tag).await;
    assert_eq!(gitea_release.name, expected_tag);

    move_readme(&context, "feat: move readme");

    let outcome = context.run_release_pr().success();

    let opened_prs = context.opened_release_prs().await;
    let open_pr = &opened_prs[0];
    let expected_stdout = serde_json::json!({
        "prs": [{
            "base_branch": "main",
            "head_branch": open_pr.branch(),
            "html_url": open_pr.html_url,
            "number": open_pr.number,
            "releases": [{
                "package_name": context.gitea.repo,
                "version": "0.2.0"
            }]
        }]
    });
    outcome.stdout(format!("{expected_stdout}\n"));
    context.merge_release_pr().await;

    let expected_tag = "v0.2.0";

    context.run_release().success();

    let gitea_release = context.gitea.get_gitea_release(expected_tag).await;
    assert_eq!(gitea_release.name, expected_tag);
    expect_test::expect![[r#"
        ### Added

        - move readme"#]]
    .assert_eq(&gitea_release.body);
}

#[tokio::test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
async fn changelog_is_not_updated_if_version_already_exists_in_changelog() {
    let context = TestContext::new().await;
    context.run_release_pr().success();
    // Merge release PR to update changelog of v0.1.0 of crate
    context.merge_release_pr().await;

    // do a random commit
    move_readme(&context, "feat: move readme");

    // Run again release-plz to create a new release PR.
    // Since we haven't published the crate, release-plz doesn't change the version.
    // Release-plz releazes that the version already exists in the changelog and doesn't update it.
    context.run_release_pr().success();

    // Since the changelog is not updated, the PR is not created because there are no changes to do.
    let opened_prs = context.opened_release_prs().await;
    assert_eq!(opened_prs.len(), 0);
}

#[tokio::test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
async fn release_plz_adds_labels_to_release_pr() {
    let test_context = TestContext::new().await;

    // Initial PR setup with two labels
    let initial_config = r#"
    [workspace]
    pr_labels = ["bug", "enhancement"]
    "#;
    let initial_labels = ["bug", "enhancement"];

    test_context.write_release_plz_toml(initial_config);
    test_context.run_release_pr().success();

    let initial_prs = test_context.opened_release_prs().await;
    assert_eq!(initial_prs.len(), 1, "Expected one PR to be created");

    let initial_pr = &initial_prs[0];
    assert_eq!(initial_pr.labels.len(), 2, "Expected 2 labels");

    assert_eq!(
        initial_pr.label_names(),
        initial_labels,
        "Labels don't match expected values"
    );

    // Update PR with additional label
    let updated_config = r#"
    [workspace]
    pr_name = "add labels to release label update"
    pr_labels = ["needs-testing"]
    "#;
    let expected_labels = ["bug", "enhancement", "needs-testing"];

    test_context.write_release_plz_toml(updated_config);
    test_context.run_release_pr().success();

    let updated_prs = test_context.opened_release_prs().await;
    assert_eq!(updated_prs.len(), 1, "Expected one PR after update");

    let updated_pr = &updated_prs[0];
    assert_eq!(updated_pr.title, "add labels to release label update");
    assert_eq!(updated_pr.labels.len(), 3, "Expected 3 labels after update");

    assert_eq!(
        updated_pr.label_names(),
        expected_labels,
        "Updated labels don't match expected values"
    );
}

#[tokio::test]
#[cfg_attr(not(feature = "docker-tests"), ignore)]
async fn release_plz_doesnt_add_invalid_labels_to_release_pr() {
    let test_context = TestContext::new().await;
    let test_cases: &[(&str, &str)] = &[
        // (label config, expected error message)
        (
            r#"
            [workspace]
            pr_labels = [" "]
        "#,
            "leading or trailing whitespace is not allowed",
        ), // space label
        (
            r#"
            [workspace]
            pr_labels = ["this-is-a-very-long-label-that-exceeds-the-maximum-length-allowed-by-git-providers"]
            "#,
            "it exceeds maximum length of 50 characters",
        ), // Too long
        (
            r#"
            [workspace]
            pr_labels = [""]
            "#,
            "empty labels are not allowed",
        ),
        (
            r#"
            [workspace]
            pr_labels = ["abc", "abc"]
            "#,
            "duplicate labels are not allowed",
        ),
    ];

    for test_case in test_cases {
        let initial_config = test_case.0;
        test_context.write_release_plz_toml(initial_config);
        let error = test_context.run_release_pr().failure().to_string();
        assert!(
            error.contains("Failed to add label") && error.contains(test_case.1),
            "Expected label creation failure got: {error}"
        );
    }
}

fn move_readme(context: &TestContext, message: &str) {
    let readme = "README.md";
    let new_readme = format!("NEW_{readme}");
    let old_readme_path = context.repo_dir().join(readme);
    let new_readme_path = context.repo_dir().join(&new_readme);
    fs_err::rename(old_readme_path, new_readme_path).unwrap();

    let cargo_toml_path = context.repo_dir().join("Cargo.toml");
    let mut cargo_toml = LocalManifest::try_new(&cargo_toml_path).unwrap();
    cargo_toml.data["package"]["readme"] = toml_edit::value(new_readme);
    cargo_toml.write().unwrap();

    context.repo.add_all_and_commit(message).unwrap();
    context.repo.git(&["push"]).unwrap();
}

fn today() -> String {
    // The changelogs specify the release date in UTC.
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}
