use crate::{
  jobs::{send_ocurrence_notification::SendOcurrenceNotification, Job},
  middleware::ensure_json,
  models::{
    culture::Culture,
    pathogenic::Pathogenic,
    plantation::Plantation,
    plantation_pathogenic_occurrences::PlantationPathogenicOccurrences,
    stations::Station,
    user::User,
  },
  utils::{
    database::{self, DataBase},
    response::{self, JsonError},
  },
};
use chrono::NaiveDate;
use garde::Validate;
use poem::{
  get,
  handler,
  http::StatusCode,
  post,
  web::{Data, Json, Multipart, Path},
  EndpointExt,
  Response,
  Route,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, sync::Arc};
use uuid::Uuid;

#[derive(Deserialize, Validate, Debug)]
struct PlantationCreate {
  #[garde(required, ascii, length(min = 3, max = 25))]
  alias: Option<String>,
  #[garde(required)]
  culture_id: Option<i64>,
  #[garde(required)]
  latitude: Option<f64>,
  #[garde(required)]
  longitude: Option<f64>,
  #[garde(required)]
  area: Option<f64>,
  #[garde(
    required,
    pattern(r"([12]\d{3}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]))")
  )]
  planting_date: Option<String>,
}

#[derive(Serialize)]

struct PlantationResponse {
  id: String,
  alias: Option<String>,
  latitude: Option<f64>,
  longitude: Option<f64>,
  area: f64,
  planting_date: chrono::NaiveDateTime,
  create_date: chrono::NaiveDateTime,
  update_date: Option<chrono::NaiveDateTime>,
  culture: Culture,
  station: Option<Station>,
  has_ocurrences: bool,
  plantation_ocurrences: Option<Vec<PlantationPathogenicOccurrencesResponse>>,
  region_ocurrences: Option<Vec<PlantationPathogenicOccurrencesResponse>>,
}

#[derive(Serialize)]

struct PlantationPathogenicOccurrencesResponse {
  id: String,
  pathogenic: Pathogenic,
  image: Option<String>,
  occurrence_date: chrono::NaiveDateTime,
  temperature: Option<f64>,
  humidity: Option<f64>,
  create_date: chrono::NaiveDateTime,
  update_date: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize)]
struct PlantationAllResponse {
  id: String,
  alias: Option<String>,
  latitude: Option<f64>,
  longitude: Option<f64>,
  area: f64,
  planting_date: chrono::NaiveDateTime,
  create_date: chrono::NaiveDateTime,
  update_date: Option<chrono::NaiveDateTime>,
  culture: Culture,
  station: Option<Station>,
  has_ocurrences: bool,
}

#[derive(Deserialize, Validate)]
struct PlantationPathogenicOccurrencesCreate {
  #[garde(required)]
  pathogenic_id: Option<i64>,
  #[garde(
    required,
    pattern(r"([12]\d{3}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]))")
  )]
  occurrence_date: Option<String>,
}

#[handler]
async fn all(db: Data<&database::DataBase>, user: Data<&User>) -> Response {
  let plantations = Plantation::all_by_user_id(&db, user.id).await;

  let mut response: Vec<PlantationAllResponse> = Vec::new();
  for plantation in plantations {
    let culture = Culture::find_by_id(&db, plantation.culture_id)
      .await
      .unwrap();

    let mut stations: Option<Station> = None;
    if plantation.station_id.is_some() {
      stations = Some(Station::find_by_id(&db, plantation.station_id.unwrap()).await);
    }

    response.push(PlantationAllResponse {
      id: plantation.id.to_string(),
      alias: plantation.alias,
      latitude: plantation.latitude,
      longitude: plantation.longitude,
      area: plantation.area,
      planting_date: plantation.planting_date,
      create_date: plantation.create_date,
      update_date: plantation.update_date,
      culture,
      station: stations,
      has_ocurrences: Plantation::has_ocurrence_last_24h(&db, plantation.id)
        .await
        .unwrap_or(false),
    });
  }

  response::json_ok(serde_json::json!({ "plantations": response }))
}

#[handler]
async fn create(
  db: Data<&database::DataBase>,
  user: Data<&User>,
  req: Json<PlantationCreate>,
) -> Response {
  if let Err(e) = req.0.validate(&()) {
    return response::json(response::garde_error_to_json(e), StatusCode::BAD_REQUEST);
  }

  let culture = Culture::find_by_id(&db, req.0.culture_id.unwrap()).await;

  if culture.is_err() {
    return response::json(
      serde_json::json!({ "errors": vec![JsonError::new("culture".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let latitude = req.0.latitude.unwrap();
  let longitude = req.0.longitude.unwrap();

  let station = Station::find_closest_by_latitude_longitude(&db, latitude, longitude).await;

  if station.is_err() {
    return response::json(
      serde_json::json!({ "errors": vec![JsonError::new("station".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let plantation_uuid = Plantation::insert(
    &db,
    user.id,
    req.0.culture_id.unwrap(),
    Some(station.ok().unwrap().id),
    req.0.alias.unwrap(),
    latitude,
    longitude,
    req.0.area.unwrap(),
    NaiveDate::parse_from_str(&req.0.planting_date.unwrap(), "%Y-%m-%d")
      .unwrap()
      .and_hms_opt(0, 0, 0)
      .unwrap(),
  )
  .await
  .unwrap();

  response::json(
    serde_json::json!({ "plantation": plantation_uuid}),
    StatusCode::CREATED,
  )
}

#[handler]
async fn show(db: Data<&database::DataBase>, user: Data<&User>, id: Path<String>) -> Response {
  let plantation_result = find_plantation_by_id(&db, id.0.to_string(), user.id).await;
  if plantation_result.is_none() {
    return response::json(
      serde_json::json!({ "errors": vec![JsonError::new("plantation".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let plantation = plantation_result.unwrap();

  let culture = Culture::find_by_id(&db, plantation.culture_id)
    .await
    .unwrap();

  let mut stations: Option<Station> = None;
  if plantation.station_id.is_some() {
    stations = Some(Station::find_by_id(&db, plantation.station_id.unwrap()).await);
  }

  let mut ocurrences_db: Result<Vec<PlantationPathogenicOccurrences>, sqlx::Error> =
    PlantationPathogenicOccurrences::get_by_plantation_id(&db, plantation.id).await;
  let mut ocurrences: Vec<PlantationPathogenicOccurrencesResponse> = Vec::new();
  if ocurrences_db.is_ok() {
    for ocurrence in ocurrences_db.as_mut().unwrap() {
      let pathogenic = Pathogenic::find_by_id(&db, &ocurrence.pathogenic_id)
        .await
        .unwrap();

      ocurrences.push(PlantationPathogenicOccurrencesResponse {
        id: ocurrence.id.to_string(),
        pathogenic,
        image: ocurrence.image.clone(),
        occurrence_date: ocurrence.occurrence_date,
        temperature: ocurrence.temperature,
        humidity: ocurrence.humidity,
        create_date: ocurrence.create_date,
        update_date: ocurrence.update_date,
      });
    }
  }

  let mut region_ocurrences_db: Result<Vec<PlantationPathogenicOccurrences>, sqlx::Error> =
    PlantationPathogenicOccurrences::get_closest_by_plantation_id(&db, plantation.id).await;
  let mut region_ocurrences: Vec<PlantationPathogenicOccurrencesResponse> = Vec::new();

  if region_ocurrences_db.is_ok() {
    for ocurrence in region_ocurrences_db.as_mut().unwrap() {
      let pathogenic = Pathogenic::find_by_id(&db, &ocurrence.pathogenic_id)
        .await
        .unwrap();

      region_ocurrences.push(PlantationPathogenicOccurrencesResponse {
        id: ocurrence.id.to_string(),
        pathogenic,
        image: None,
        occurrence_date: ocurrence.occurrence_date,
        temperature: ocurrence.temperature,
        humidity: ocurrence.humidity,
        create_date: ocurrence.create_date,
        update_date: ocurrence.update_date,
      });
    }
  }

  response::json_ok(serde_json::json!(PlantationResponse {
    id: plantation.id.to_string(),
    alias: plantation.alias,
    latitude: plantation.latitude,
    longitude: plantation.longitude,
    area: plantation.area,
    planting_date: plantation.planting_date,
    create_date: plantation.create_date,
    update_date: plantation.update_date,
    culture,
    station: stations,
    has_ocurrences: Plantation::has_ocurrence_last_24h(&db, plantation.id)
      .await
      .unwrap_or(false),
    plantation_ocurrences: Some(ocurrences),
    region_ocurrences: Some(region_ocurrences)
  }))
}

#[handler]
async fn update(
  db: Data<&database::DataBase>,
  user: Data<&User>,
  id: Path<String>,
  req: Json<PlantationCreate>,
) -> Response {
  if let Err(e) = req.0.validate(&()) {
    return response::json(response::garde_error_to_json(e), StatusCode::BAD_REQUEST);
  }

  let plantation_result = find_plantation_by_id(&db, id.0.to_string(), user.id).await;
  if plantation_result.is_none() {
    return response::json(
      serde_json::json!({ "errors": vec![JsonError::new("plantation".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let plantation = plantation_result.unwrap();

  let culture = Culture::find_by_id(&db, req.0.culture_id.unwrap()).await;

  if culture.is_err() {
    return response::json(
      serde_json::json!({ "errors": vec![JsonError::new("culture".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let latitude = req.0.latitude.unwrap();
  let longitude = req.0.longitude.unwrap();

  let station = Station::find_closest_by_latitude_longitude(&db, latitude, longitude).await;

  if station.is_err() {
    return response::json(
      serde_json::json!({ "errors": vec![JsonError::new("station".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let _ = Plantation::update(
    &db,
    plantation.id,
    req.0.culture_id.unwrap(),
    Some(station.ok().unwrap().id),
    req.0.alias.unwrap(),
    latitude,
    longitude,
    req.0.area.unwrap(),
    NaiveDate::parse_from_str(&req.0.planting_date.unwrap(), "%Y-%m-%d")
      .unwrap()
      .and_hms_opt(0, 0, 0)
      .unwrap(),
  )
  .await;

  response::json_ok(
    serde_json::json!({ "plantation": Plantation::find_by_uuid(&db, plantation.id).await.unwrap() }),
  )
}

#[handler]
async fn delete(db: Data<&database::DataBase>, user: Data<&User>, id: Path<String>) -> Response {
  let plantation_result = find_plantation_by_id(&db, id.0.to_string(), user.id).await;
  if plantation_result.is_none() {
    return response::json(
      serde_json::json!({ "errors": vec![JsonError::new("plantation".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let plantation = plantation_result.unwrap();

  let _ = Plantation::delete(&db, plantation.id).await;

  response::json_ok(serde_json::json!({ "plantation": "ok" }))
}

#[handler]
async fn all_ocurrences(
  db: Data<&database::DataBase>,
  user: Data<&User>,
  plantation_id: Path<String>,
) -> Response {
  let plantation_result = find_plantation_by_id(&db, plantation_id.0.to_string(), user.id).await;
  if plantation_result.is_none() {
    return response::json(
      serde_json::json!({ "error": vec![JsonError::new("plantation".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let plantation = plantation_result.unwrap();

  let ocurrences = PlantationPathogenicOccurrences::get_by_plantation_id(&db, plantation.id).await;

  return response::json_ok(serde_json::json!({ "ocurrences": ocurrences.unwrap() }));
}

#[handler]
async fn create_ocurrence(
  db: Data<&database::DataBase>,
  user: Data<&User>,
  plantation_id: Path<String>,
  req: Json<PlantationPathogenicOccurrencesCreate>,
) -> Response {
  if let Err(e) = req.0.validate(&()) {
    return response::json(response::garde_error_to_json(e), StatusCode::BAD_REQUEST);
  }

  let plantation_result = find_plantation_by_id(&db, plantation_id.0.to_string(), user.id).await;
  if plantation_result.is_none() {
    return response::json(
      serde_json::json!({ "error": vec![JsonError::new("plantation".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let plantation = plantation_result.unwrap();

  let ocurrence = PlantationPathogenicOccurrences::insert(
    &db,
    user.id,
    plantation.id,
    req.0.pathogenic_id.unwrap(),
    None,
    NaiveDate::parse_from_str(&req.0.occurrence_date.unwrap(), "%Y-%m-%d")
      .unwrap()
      .and_hms_opt(23, 59, 59)
      .unwrap(),
    None,
    None,
  )
  .await
  .unwrap();

  // Send the notification to the all the closes plantations that have the same culture
  let ocurrence_arc = Arc::new(ocurrence.clone());
  let db_arc = Arc::new(db.clone());

  tokio::spawn(async move {
    println!("Sending notification for ocurrence: {:?}", ocurrence_arc);
    let job = SendOcurrenceNotification {
      ocurrence: ocurrence_arc.as_ref().clone(),
      db: db_arc.as_ref(),
    };

    job.run().await;
  });

  return response::json_ok(serde_json::json!({ "ocurrence": ocurrence }));
}

#[handler]
async fn ocurrence_add_image(
  db: Data<&database::DataBase>,
  user: Data<&User>,
  Path((plantation_id, ocurrence_id)): Path<(String, String)>,
  mut multipart: Multipart,
) -> Response {
  let plantation_result = find_plantation_by_id(&db, plantation_id.to_string(), user.id).await;
  if plantation_result.is_none() {
    return response::json(
      serde_json::json!({ "error": vec![JsonError::new("plantation".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let ocurrence_result = PlantationPathogenicOccurrences::find_by_id(
    &db,
    uuid::Uuid::parse_str(&ocurrence_id.to_string()).unwrap(),
  )
  .await;

  if ocurrence_result.is_err() {
    return response::json(
      serde_json::json!({ "error": vec![JsonError::new("ocurrence".to_string(), "not found".to_string())] }),
      StatusCode::NOT_FOUND,
    );
  }

  let mut file_path = String::new();

  while let Ok(Some(field)) = multipart.next_field().await {
    let mime = field.content_type().unwrap();

    if mime != "image/png" && mime != "image/jpeg" && mime != "image/jpg" {
      return response::json(
        serde_json::json!({ "error": vec![JsonError::new("image".to_string(), "invalid".to_string())] }),
        StatusCode::BAD_REQUEST,
      );
    }

    file_path = format!(
      "/images/ocurrences/{}.{}",
      ocurrence_result.as_ref().unwrap().id.to_string(),
      "png".to_string()
    );

    if let Ok(bytes) = field.bytes().await {
      let mut file = File::create(&format!("app/{}", file_path)).unwrap();
      file.write_all(&bytes).unwrap();

      let _ = PlantationPathogenicOccurrences::add_image_by_id(
        &db,
        ocurrence_result.as_ref().unwrap().id,
        file_path.as_mut().to_string(),
      )
      .await
      .unwrap();
    }
  }

  response::json_ok(serde_json::json!({
    "image": file_path
  }))
}

async fn find_plantation_by_id(
  db: &DataBase,
  plantation_id: String,
  user_id: Uuid,
) -> Option<Plantation> {
  let plantation_id = uuid::Uuid::parse_str(&plantation_id);

  if plantation_id.is_err() {
    return None;
  }

  let plantation_result = Plantation::find_by_uuid(&db, plantation_id.unwrap()).await;
  if plantation_result.is_err() {
    return None;
  }

  let plantation_result = plantation_result.unwrap();

  if plantation_result.user_id != user_id {
    return None;
  }

  Some(plantation_result)
}

pub fn routes() -> Route {
  Route::new()
    .just_at(get(all).post(create).around(ensure_json::handle))
    .at(
      "/:id",
      get(show)
        .patch(update)
        .delete(delete)
        .around(ensure_json::handle),
    )
    .at(
      "/:plantation_id/ocurrences",
      get(all_ocurrences)
        .post(create_ocurrence)
        .around(ensure_json::handle),
    )
    .at(
      "/:plantation_id/ocurrences/:ocurrence_id/image",
      post(ocurrence_add_image),
    )
}
