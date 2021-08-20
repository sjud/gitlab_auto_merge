use clap::Clap;
use gitlab::Gitlab;
use lazy_static::lazy_static;
use gitlab::api::projects::merge_requests::{CreateMergeRequest, MergeRequests, MergeRequestState, ApproveMergeRequest};
use gitlab::api::Query;
use serde::Deserialize;

lazy_static! {
    static ref TOKEN: String =
        std::env::var("GITLAB_PRIVATE_TOKEN").expect("Expecting a GITLAB_PRIVATE_TOKEN env var");
}

#[derive(Debug,Deserialize)]
pub struct Iid{
    iid: u64,
}

fn main() {
    let source_branch = clap::Arg::new("source-branch")
        .long("source-branch")
        .required(true)
        .forbid_empty_values(true)
        .takes_value(true)
        .about("The source branch to merge into.  [required]");

    let project_id = clap::Arg::new("project-id")
        .long("project-id")
        .required(true)
        .forbid_empty_values(true)
        .takes_value(true)
        .about("The project ID on GitLab to create the MR for.[required]");

    let gitlab_url = clap::Arg::new("gitlab-url")
        .long("gitlab-url")
        .required(true)
        .forbid_empty_values(true)
        .takes_value(true)
        .about("The GitLab URL i.e. gitlab.com.  [required]");

    let user_id = clap::Arg::new("user-id")
        .long("user-id")
        .required(true)
        .forbid_empty_values(true)
        .takes_value(true)
        .about("The GitLab user ID(s) to assign the created MR to.  [required]");

    let insecure = clap::Arg::new("insecure")
        .long("insecure")
        .takes_value(false)
        .short('k')
        .about("Do not verify server SSL certificate.");

    let target_branch = clap::Arg::new("target-branch")
        .long("target-branch")
        .takes_value(true)
        .forbid_empty_values(true)
        .short('t')
        .about("The target branch to merge onto.");

    let commit_prefix = clap::Arg::new("commit-prefix")
        .long("commit-prefix")
        .takes_value(true)
        .forbid_empty_values(true)
        .short('c')
        .about("Prefix for the MR title i.e. WIP.");

    let remove_branch = clap::Arg::new("remove-branch")
        .long("remove-branch")
        .takes_value(false)
        .short('r')
        .about("If set will remove the source branch after MR.");

    let squash_commits = clap::Arg::new("squash-commits")
        .long("squash-commits")
        .takes_value(false)
        .short('s')
        .about("If set will squash commits on merge.");

    let description = clap::Arg::new("description")
        .long("description")
        .takes_value(true)
        .forbid_empty_values(true)
        .short('d')
        .about("Path to file to use as the description for the MR.");

    let title = clap::Arg::new("title")
        .long("title")
        .takes_value(true)
        .forbid_empty_values(true)
        .about("Custom tile for the MR.");

    let use_issue_name = clap::Arg::new("use-issue-name")
        .long("use-issue-name")
        .takes_value(false)
        .short('i')
        .about(
            "If set will use information from issue in branch \
        name, must be in the form #issue-number, i.efeature/#6.",
        );

    let allow_collaboration = clap::Arg::new("allow-collaboration")
        .long("allow-collaboration")
        .takes_value(false)
        .about("If set allow, commits from members who can merge to the target branch.");

    let auto_merge = clap::Arg::new("auto-merge")
        .long("auto-merge")
        .takes_value(false)
        .short('a')
        .about("Automatically approves the issued merge request if true.");

    let matches = clap::App::new("gitlab_auto_merge")
        .args(&[
            source_branch,
            project_id,
            gitlab_url,
            user_id,
            insecure,
            target_branch,
            commit_prefix,
            remove_branch,
            squash_commits,
            description,
            title,
            use_issue_name,
            allow_collaboration,
            auto_merge
        ])
        .get_matches();
    // Deconstruct the required inputs.
    let (source_branch, project_id, gitlab_url, user_id) = (
        matches
            .value_of("source-branch")
            .expect("Source branch required."),
        matches
            .value_of("project-id")
            .expect("Project ID required."),
        // Parse/Validate URL for future use.
        url::Url::parse(
            matches
                .value_of("gitlab-url")
                .expect("Gitlab URL required."),
        )
        .expect("Expecting valid URL"),
        matches.value_of("user-id").expect("User ID required"),
    );
    let insecure = matches.is_present("insecure");
    // A merge request is issued to the source branch unless given a target
    let target_branch = matches
        .value_of("target-branch")
        .unwrap_or(source_branch.clone());
    let commit_prefix = matches.value_of("commit-prefix");
    let remove_branch = matches.is_present("remove-branch");
    let squash_commits = matches.is_present("squash-commits");
    let description = matches.value_of("description")
        .unwrap_or(&format!("Merge with {}",source_branch.clone())).to_owned();
    let title = matches.value_of("title").unwrap_or(
        &format!("{}{}", commit_prefix.unwrap_or(""), source_branch.clone())
    ).to_owned();
    let use_issue_name = matches.is_present("use-issue-name");
    let allow_collaboration = matches.is_present("allow-collaboration");
    let auto_merge = matches.is_present("auto-merge");

    let project_path = gitlab_url.path().clone();

    // Get host from parsed URL and TOKEN from Env Var
    let client = Gitlab::new(
        gitlab_url.host_str().expect("Expecting valid host"),
        TOKEN.clone(),
    )
    .expect("Requires token.");

    // Uses cmd args to build a merge request.
    let endpoint : CreateMergeRequest = CreateMergeRequest::builder()
        .project(project_id)
        .source_branch(source_branch)
        .target_branch(target_branch)
        .title(title)
        .description(description)
        .allow_collaboration(allow_collaboration)
        .squash(squash_commits)
        .remove_source_branch(remove_branch)
        .build()
        .expect("Error creating merge request");

    // Post our merge request.
    let iid: Iid = endpoint.query(&client).unwrap();

    if auto_merge {
        let endpoint: ApproveMergeRequest = ApproveMergeRequest::builder()
            .project(project_id)
            .merge_request(iid.iid)
            .build()
            .expect("Error building ApproveMergeRequest");
        let _ = gitlab::api::ignore(endpoint).query(&client).unwrap();
    }
}
