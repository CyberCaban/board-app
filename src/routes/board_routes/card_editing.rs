use diesel::{BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    form::Form, http::CookieJar, serde::json::Json, tokio::io::AsyncReadExt,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::{
        BoardUsersRelation, PubAttachment, ReturnedCard, UploadAttachment, UploadedFile,
        SELECT_CARD,
    },
    schema::*,
    validate_user_token,
};

/// # GET /boards/<board_id>/cards/<card_id>
/// Returns the card with the given id
/// Used only to get the card modal
/// # Arguments
/// * `board_id` - The id of the board
/// * `card_id` - The id of the card
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `card` - The card
/// ```json
/// {
///     "id": <card_id>,
///     "column_id": <column_id>,
///     "description": <card_description>,
///     "position": <card_position>
/// }
/// ```
#[get("/<board_id>/cards/<card_id>")]
pub async fn boards_get_card_by_id(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    card_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let card_id = Uuid::try_parse(&card_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;

            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            let card = column_card::table
                .filter(column_card::id.eq(card_id))
                .select(SELECT_CARD)
                .first::<ReturnedCard>(conn)?;
            let attachments = card_attachments::table
                .filter(card_attachments::card_id.eq(card_id))
                .inner_join(files::table)
                .select((files::id, files::name))
                .load::<(Uuid, String)>(conn)?
                .iter()
                .map(|(id, name)| PubAttachment {
                    id: *id,
                    url: name.clone(),
                })
                .collect::<Vec<PubAttachment>>();

            Ok::<Json<Value>, diesel::result::Error>(Json(json!({
                "id": card.0,
                "name": card.1,
                "cover_attachment": card.2,
                "position": card.3,
                "description": card.4,
                "column_id": card.5,
                "attachments": attachments
            })))
        })
    })
    .await
    .map_err(|e| ApiError::from_error(e).to_json())
}

/// # POST /boards/<board_id>/cards/<card_id>/attachments
/// Adds an attachment to the card
/// # Arguments
/// * `board_id` - The id of the board
/// * `card_id` - The id of the card
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `card` - The card
/// ```json
/// {
///     "id": <card_id>,
///     "column_id": <column_id>,
///     "description": <card_description>,
///     "position": <card_position>
/// }
/// ```
#[post("/<board_id>/cards/<card_id>/attachments", data = "<card>")]
pub async fn boards_add_attachment_to_card(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    card_id: String,
    card: Form<UploadAttachment<'_>>,
) -> Result<Json<Value>, Json<Value>> {
    let uploader_id = validate_user_token!(cookies);

    let filename = card.filename.clone();
    let file_name = format!("{}-{}", Uuid::new_v4(), filename);
    let file_name_clone = file_name.clone();
    let transaction = db
        .run(move |conn| {
            conn.transaction(|conn| {
                let board_id = Uuid::try_parse(&board_id)
                    .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
                let card_id = Uuid::try_parse(&card_id)
                    .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
                let _ = board_users_relation::table
                    .filter(
                        board_users_relation::board_id
                            .eq(board_id)
                            .and(board_users_relation::user_id.eq(uploader_id)),
                    )
                    .first::<BoardUsersRelation>(conn)?;

                let (card_id, cover) = column_card::table
                    .filter(column_card::id.eq(card_id))
                    .select((column_card::id, column_card::cover_attachment))
                    .first::<(Uuid, Option<String>)>(conn)?;
                let new_attachment = UploadedFile {
                    id: Uuid::new_v4(),
                    name: file_name.clone(),
                    user_id: uploader_id,
                    private: false,
                };
                diesel::insert_into(files::table)
                    .values(&new_attachment)
                    .execute(conn)?;
                diesel::insert_into(card_attachments::table)
                    .values((
                        card_attachments::file_id.eq(new_attachment.id),
                        card_attachments::card_id.eq(card_id),
                    ))
                    .execute(conn)?;
                if cover.is_none() {
                    diesel::update(column_card::table)
                        .filter(column_card::id.eq(card_id))
                        .set(column_card::cover_attachment.eq(file_name.clone()))
                        .execute(conn)?;
                }

                Ok::<(), diesel::result::Error>(())
            })
        })
        .await;
    match transaction {
        Err(e) => return Err(ApiError::from_error(e).to_json()),
        Ok(_) => {
            let mut file = card.file.open().await.unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).await.unwrap();
            let file_path = format!("tmp/{}", file_name_clone);
            std::fs::write(&file_path, buf).unwrap();
            Ok(Json(json!("Attachment added")))
        }
    }
}

/// # GET /boards/<board_id>/cards/<card_id>/attachments
/// Returns the attachments of the card
/// # Arguments
/// * `board_id` - The id of the board
/// * `card_id` - The id of the card
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `attachments` - The attachments
/// ```json
/// [
///     {
///         "id": <attachment_id>,
///         "name": <attachment_name>,
///         "url": <attachment_url>
///     },
///     ...
/// ]
/// ```
#[get("/<board_id>/cards/<card_id>/attachments")]
pub async fn boards_get_attachments_of_card(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    card_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    db.run(move |conn| {
        conn.transaction(|conn| {
            let board_id = Uuid::try_parse(&board_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let card_id = Uuid::try_parse(&card_id)
                .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
            let _ = board_users_relation::table
                .filter(
                    board_users_relation::board_id
                        .eq(board_id)
                        .and(board_users_relation::user_id.eq(token)),
                )
                .first::<BoardUsersRelation>(conn)?;

            let attachments = card_attachments::table
                .filter(card_attachments::card_id.eq(card_id))
                .inner_join(files::table)
                .select((files::id, files::name))
                .load::<(Uuid, String)>(conn)?
                .iter()
                .map(|(id, name)| PubAttachment {
                    id: *id,
                    url: name.to_string(),
                })
                .collect::<Vec<PubAttachment>>();

            Ok::<Vec<PubAttachment>, diesel::result::Error>(attachments)
        })
    })
    .await
    .map(|attachments| Json(json!(attachments)))
    .map_err(|e| ApiError::from_error(e).to_json())
}

/// # DELETE /boards/<board_id>/cards/<card_id>/attachments/<attachment_id>
/// Deletes the attachment with the given id
/// # Arguments
/// * `board_id` - The id of the board
/// * `card_id` - The id of the card
/// * `attachment_id` - The id of the attachment
/// * `cookies` - Takes the token of the user
/// # Returns
/// * `card` - The card
/// ```json
/// {
///     "id": <card_id>,
///     "column_id": <column_id>,
///     "description": <card_description>,
///     "position": <card_position>
/// }
/// ```
#[delete("/<board_id>/cards/<card_id>/attachments/<attachment_id>")]
pub async fn boards_delete_attachment_of_card(
    db: Db,
    cookies: &CookieJar<'_>,
    board_id: String,
    card_id: String,
    attachment_id: String,
) -> Result<Json<Value>, Json<Value>> {
    let token = validate_user_token!(cookies);

    let transaction = db
        .run(move |conn| {
            conn.transaction(|conn| {
                let board_id = Uuid::try_parse(&board_id)
                    .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
                let card_id = Uuid::try_parse(&card_id)
                    .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
                let attachment_id = Uuid::try_parse(&attachment_id)
                    .map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))?;
                let _ = board_users_relation::table
                    .filter(
                        board_users_relation::board_id
                            .eq(board_id)
                            .and(board_users_relation::user_id.eq(token)),
                    )
                    .first::<BoardUsersRelation>(conn)?;

                let (card_id, cover) = column_card::table
                    .filter(column_card::id.eq(card_id))
                    .select((column_card::id, column_card::cover_attachment))
                    .first::<(Uuid, Option<String>)>(conn)?;
                diesel::delete(card_attachments::table)
                    .filter(card_attachments::card_id.eq(card_id))
                    .filter(card_attachments::file_id.eq(attachment_id))
                    .execute(conn)?;
                let file_name = diesel::delete(files::table)
                    .filter(files::id.eq(attachment_id))
                    .returning(files::name)
                    .get_result(conn)?;
                if cover.is_some() {
                    diesel::update(column_card::table)
                        .filter(column_card::id.eq(card_id))
                        .set(column_card::cover_attachment.eq(None::<String>))
                        .execute(conn)?;
                }
                Ok::<String, diesel::result::Error>(file_name)
            })
        })
        .await;
    match transaction {
        Err(e) => return Err(ApiError::from_error(e).to_json()),
        Ok(file_name) => {
            std::fs::remove_file(format!("tmp/{}", file_name))
                .map_err(|e| ApiError::from_error(e).to_json())?;
            Ok(Json(json!("Attachment deleted")))
        }
    }
}
