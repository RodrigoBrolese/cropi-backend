use async_trait::async_trait;

pub mod send_ocurrence_notification;

#[async_trait]
pub(crate) trait Job {
  async fn run(&self);
}
