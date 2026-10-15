//! `land`: rebase onto the ref head and compare-and-swap, no merge queue.

use leep_core::api::{LandResponse, RebaseResponse};
use leep_core::{ChangesetId, Error, Result};

use crate::Repo;

const MAX_LAND_ATTEMPTS: u32 = 16;

impl Repo {
    /// Land `cs` on `target_ref`, rebasing onto the current head if the ref
    /// moved. Retries when the CAS loses; conflicts are returned.
    pub async fn land(&self, cs: ChangesetId, target_ref: &str) -> Result<LandResponse> {
        let changeset = self.get_changeset(cs).await?;
        for attempt in 1..=MAX_LAND_ATTEMPTS {
            let head = self.meta().get_ref(target_ref).await?;
            let new = match head {
                None => cs,
                Some(h) if h == cs || self.is_ancestor(cs, h).await? => {
                    return Ok(LandResponse::Landed { landed: cs, rebased: false, attempts: attempt });
                }
                Some(h) if changeset.parents.first() == Some(&h) => cs,
                Some(h) => match self.rebase(cs, h).await? {
                    RebaseResponse::Rebased { id } => id,
                    RebaseResponse::Conflicts { conflicts } => {
                        return Ok(LandResponse::Conflicts { onto: h, conflicts });
                    }
                },
            };
            if self.meta().cas_ref(target_ref, head, Some(new)).await? {
                return Ok(LandResponse::Landed { landed: new, rebased: new != cs, attempts: attempt });
            }
            tokio::task::yield_now().await;
        }
        Err(Error::Other(format!(
            "land onto {target_ref:?} lost the race {MAX_LAND_ATTEMPTS} times"
        )))
    }
}
