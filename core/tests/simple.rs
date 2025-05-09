#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

use roctogen::api::{self, repos};
#[cfg(any(feature = "reqwest", feature = "ureq", target_arch = "wasm32"))]
use roctokit::adapters::client;
use roctokit::auth::Auth;

use roctogen::models;

use log::debug;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
fn init_log() {
    console_log::init_with_level(log::Level::Info).unwrap_or(());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn get_wasm_fail() {
    init_log();

    let auth = Auth::None;
    let client = client(&auth).expect("Cannot create client");

    let per_page = api::PerPage::new(1);

    let req = repos::new(&client)
        .list_commits_async("this-user-does-not-exist", "bollard", Some(&per_page))
        .await;
    match &req {
        Ok(_) => {}
        Err(roctokit::adapters::AdapterError::Endpoint {
            description,
            status_code: 404,
            ..
        }) => {
            debug!("{}", description);
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn get_sync_fail() {
    let auth = Auth::None;
    let client = client(&auth).expect("Cannot create client");

    let per_page = api::PerPage::new(1);

    let req =
        repos::new(&client).list_commits("this-user-does-not-exist", "bollard", Some(&per_page));
    match &req {
        Ok(_) => {}
        Err(roctokit::adapters::AdapterError::Endpoint {
            description,
            status_code: 404,
            ..
        }) => {
            debug!("{}", description);
        }
        Err(x) => {
            debug!("{:?}", x);
            panic!();
        }
    };

    assert!(req.is_err());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn get_sync_ok() {
    let auth = Auth::None;
    let client = client(&auth).expect("Cannot create client");

    let per_page = api::PerPage::new(1);

    let req = repos::new(&client).list_commits("fussybeaver", "bollard", Some(&per_page));
    match &req {
        Ok(ref commits) => {
            assert!(!&commits.is_empty());
        }
        Err(ref e) => debug!("err: {}", e),
    };

    assert!(req.is_ok());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn get_wasm_ok() {
    init_log();

    let auth = Auth::None;
    let client = client(&auth).expect("Cannot create client");
    let per_page = api::PerPage::new(1);

    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params = params.author("fussybeaver").page(2);

    let req = repos::new(&client)
        .list_commits_async("fussybeaver", "bollard", Some(params))
        .await;
    match req {
        Ok(ref repos) => {
            assert!(!&repos.is_empty());
        }
        Err(ref e) => debug!("err: {}", e),
    };

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
#[tokio::test]
async fn get_async_ok() {
    let auth = Auth::None;
    let client = client(&auth).expect("Cannot create client");
    let per_page = api::PerPage::new(1);

    let mut params: repos::ReposListCommitsParams = per_page.as_ref().into();
    params = params.author("fussybeaver").page(2);
    let req = repos::new(&client)
        .list_commits_async("fussybeaver", "bollard", Some(params))
        .await;

    match req {
        Ok(ref repos) => {
            assert!(!&repos.is_empty());
        }
        Err(ref e) => debug!("err: {}", e),
    };

    assert!(req.is_ok());
}

#[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
#[tokio::test]
async fn get_async_fail() {
    let auth = Auth::None;
    let client = client(&auth).expect("Cannot create client");

    let per_page = api::PerPage::new(1);

    let req = repos::new(&client)
        .list_commits_async("this-user-does-not-exist", "bollard", Some(&per_page))
        .await;
    match &req {
        Ok(_) => {}
        Err(roctokit::adapters::AdapterError::Endpoint {
            description,
            status_code: 404,
            ..
        }) => {
            debug!("{}", description);
        }
        Err(x) => {
            debug!("{:?}", x);
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn post_wasm_fail() {
    init_log();

    let auth = Auth::None;
    let client = client(&auth).expect("Cannot create client");

    let body = models::PostReposAddUserAccessRestrictions {
        users: vec!["fussybeaver".to_string()].into(),
    };

    let req = repos::new(&client)
        .add_user_access_restrictions_async("fussybeaver", "bollard", "master", body)
        .await;
    match &req {
        Ok(_) => {}
        Err(roctokit::adapters::AdapterError::Endpoint {
            description,
            status_code: 404,
            ..
        }) => {
            debug!("{}", description);
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "reqwest")))]
#[test]
fn post_sync_fail() {
    let auth = Auth::None;
    let client = client(&auth).expect("Cannot create client");

    let body = models::PostReposAddUserAccessRestrictions {
        users: vec!["fussybeaver".to_string()].into(),
    };

    let req =
        repos::new(&client).add_user_access_restrictions("fussybeaver", "bollard", "master", body);
    match &req {
        Ok(_) => {}
        Err(roctokit::adapters::AdapterError::Endpoint {
            description,
            status_code: 404,
            ..
        }) => {
            debug!("{}", description);
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}

#[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
#[tokio::test]
async fn post_async_fail() {
    env_logger::try_init();
    let auth = Auth::None;
    let client = client(&auth).expect("Cannot create client");

    let body = models::PostReposAddUserAccessRestrictions {
        users: vec!["fussybeaver".to_string()].into(),
    };

    let req = repos::new(&client)
        .add_user_access_restrictions_async("fussybeaver", "bollard", "master", body)
        .await;
    match &req {
        Ok(_) => {}
        Err(
            e @ roctokit::adapters::AdapterError::Endpoint {
                description,
                status_code: 404,
                ..
            },
        ) => {
            debug!("{:?}", e);
        }
        Err(_) => {
            assert!(false);
        }
    };

    assert!(req.is_err());
}
