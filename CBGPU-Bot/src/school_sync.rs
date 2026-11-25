use crate::models::School;
use crate::storage::Store;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

const API_URL: &str = "https://cbgpu.com/api/public/schools";
const SYNC_INTERVAL: Duration = Duration::from_secs(6 * 3600);

#[derive(Deserialize)]
struct Response {
    items: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    display_name: String,
    status: String,
}

/// Names of schools cbgpu.com marks as operating (status APPROVED).
/// The public list is sorted with approved schools first.
async fn fetch_operating_schools() -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(20))
        .build()?;
    let resp: Response = client
        .get(API_URL)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(resp
        .items
        .into_iter()
        .filter(|i| i.status == "APPROVED")
        .map(|i| i.display_name)
        .collect())
}

/// Marks the given schools as participating, creating them if needed. Returns how many changed.
pub fn apply_operating_schools(store: &mut Store, names: &[String]) -> usize {
    let mut changed = 0;
    for name in names {
        match store.find_school_by_name(name) {
            Some(i) => {
                let s = &mut store.storage_mut().schools[i];
                if !s.cbgu_participating {
                    s.cbgu_participating = true;
                    changed += 1;
                }
            }
            None => {
                let mut s = School::new(name.clone());
                s.cbgu_participating = true;
                store.storage_mut().schools.push(s);
                changed += 1;
            }
        }
    }
    changed
}

/// Syncs once at startup, then every 6 hours. Failures are logged and ignored.
pub async fn sync_loop(storage: Arc<Mutex<Store>>) {
    loop {
        match fetch_operating_schools().await {
            Ok(names) => {
                let mut store = storage.lock().await;
                let changed = apply_operating_schools(&mut store, &names);
                if changed > 0 {
                    match store.save() {
                        Ok(()) => println!("School sync: {changed} school(s) updated from cbgpu.com."),
                        Err(e) => eprintln!("School sync: could not save: {e}"),
                    }
                }
            }
            Err(e) => eprintln!("School sync failed (will retry): {e}"),
        }
        tokio::time::sleep(SYNC_INTERVAL).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_api_response() {
        let json = r#"{"items":[{"id":"cbsh","display_name":"충북과학고등학교","status":"APPROVED","region":"x"},
            {"id":"n1","display_name":"가경중학교","status":"LISTED"}],"total":2}"#;
        let r: Response = serde_json::from_str(json).unwrap();
        let ok: Vec<_> = r.items.iter().filter(|i| i.status == "APPROVED").collect();
        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0].display_name, "충북과학고등학교");
    }
}
