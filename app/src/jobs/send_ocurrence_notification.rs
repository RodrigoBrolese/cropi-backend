use super::Job;
use crate::{
  models::{
    pathogenic::Pathogenic,
    plantation::Plantation,
    plantation_pathogenic_occurrences::PlantationPathogenicOccurrences,
    user::User,
  },
  utils::database::DataBase,
};
use async_trait::async_trait;
use reqwest::Client;

pub(crate) struct SendOcurrenceNotification<'a> {
  pub ocurrence: PlantationPathogenicOccurrences,
  pub db: &'a DataBase,
}

#[async_trait]
impl Job for SendOcurrenceNotification<'_> {
  async fn run(&self) {
    println!("Sending notification for ocurrence: {:?}", self.ocurrence);

    let plantation_ocurrence = Plantation::find_by_uuid(&self.db, self.ocurrence.plantation_id)
      .await
      .unwrap();

    let pathogenic = Pathogenic::find_by_id(&self.db, &self.ocurrence.pathogenic_id)
      .await
      .unwrap();

    let plantation_closest_users =
      Plantation::all_closest_to_plantation(&self.db, &plantation_ocurrence).await;

    let google_jwt_token = crate::utils::google_jwt::get_firebase_jwt().await;

    let mut already_notified_users: Vec<uuid::Uuid> = Vec::new();

    for plantation in plantation_closest_users {
      if already_notified_users.contains(&plantation.user_id) {
        println!("User already notified: {:?}", plantation.user_id);
        continue;
      }

      let user_db = User::find_by_uuid(&self.db, plantation.user_id).await;

      if user_db.is_err() {
        continue;
      }

      let user = user_db.unwrap();

      if plantation_ocurrence.user_id == user.id || user.notification_token.is_none() {
        continue;
      }

      let message = format!(
        "Uma ocorrência de {} foi registrada em uma plantação próxima.",
        pathogenic.name
      );

      let body = serde_json::json!({
        "message": {
          "token": user.notification_token.unwrap(),
          "notification": {
            "title": "Ocorrência de doença",
            "body": message,
          },
        }
      });

      sqlx::query!(
        "INSERT INTO user_notifications (user_id, message) VALUES ($1, $2)",
        user.id,
        message
      )
      .execute(&self.db.pool)
      .await
      .unwrap();

      let _ = Client::new()
        .post("https://fcm.googleapis.com/v1/projects/cropi-399723/messages:send")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", google_jwt_token))
        .body(body.to_string())
        .send()
        .await;

      already_notified_users.push(user.id);
    }
  }
}
