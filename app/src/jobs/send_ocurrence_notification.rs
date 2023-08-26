use super::Job;
use crate::{
  models::plantation_pathogenic_occurrences::PlantationPathogenicOccurrences,
  utils::database::DataBase,
};
use async_trait::async_trait;

pub(crate) struct SendOcurrenceNotification<'a> {
  pub ocurrence: PlantationPathogenicOccurrences,
  pub db: &'a DataBase,
}

#[async_trait]
impl Job for SendOcurrenceNotification<'_> {
  async fn run(&self) {}
}
