use diesel::{
    result::Error as DieselError, BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl,
    RunQueryDsl,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    database::Db,
    errors::{ApiError, ApiErrorType},
    models::{
        auth::Auth, Board, BoardColumn, BoardInfo, BoardUsersRelation, CardInfo, ColumnCard,
        NewBoard, NewCard, NewColumn, PubAttachment, PubBoard, PubCard, PubColumn, ReturnedCard,
        ReturnedColumn, UploadAttachment, SELECT_CARD,
    },
    models::file::UploadedFile,
    schema::{board_column, board_users_relation, boards, card_attachments, column_card, files},
};

pub struct BoardQueries;

impl BoardQueries {
    pub async fn create_board_and_relation(
        db: &Db,
        auth: Auth,
        board: NewBoard,
    ) -> Result<Uuid, ApiError> {
        db.run(move |conn| {
            conn.transaction(|conn| {
                let board_id = diesel::insert_into(boards::table)
                    .values(Board {
                        id: None,
                        name: board.name,
                        creator_id: auth.id,
                    })
                    .returning(boards::id)
                    .get_result::<Uuid>(conn)?;

                diesel::insert_into(board_users_relation::table)
                    .values(BoardUsersRelation {
                        user_id: auth.id,
                        board_id,
                    })
                    .execute(conn)?;

                Ok::<Uuid, DieselError>(board_id)
            })
        })
        .await
        .map_err(ApiError::from)
    }

    pub async fn get_boards(db: &Db, auth: Auth) -> Result<Vec<PubBoard>, ApiError> {
        db.run(move |conn| {
            let ids = board_users_relation::table
                .filter(board_users_relation::user_id.eq(auth.id))
                .select(board_users_relation::board_id)
                .load::<Uuid>(conn)?;

            let boards_data = boards::table
                .filter(boards::id.eq_any(ids))
                .select((boards::id, boards::name))
                .load::<(Uuid, String)>(conn)?
                .into_iter()
                .map(|(id, name)| PubBoard { id, name })
                .collect();

            Ok::<Vec<PubBoard>, DieselError>(boards_data)
        })
        .await
        .map_err(ApiError::from)
    }

    pub async fn get_board(db: &Db, auth: Auth, board_id: String) -> Result<BoardInfo, ApiError> {
        let board_id = parse_uuid(&board_id)?;

        db.run(move |conn| {
            ensure_board_access(conn, board_id, auth.id)?;

            let board_name = boards::table
                .filter(boards::id.eq(board_id))
                .select(boards::name)
                .first::<String>(conn)?;

            let columns = board_column::table
                .filter(board_column::board_id.eq(board_id))
                .select((board_column::id, board_column::name, board_column::position))
                .load::<ReturnedColumn>(conn)?
                .into_iter()
                .map(|column| PubColumn {
                    id: column.0,
                    name: column.1,
                    position: column.2,
                })
                .collect::<Vec<PubColumn>>();

            let cards = column_card::table
                .filter(column_card::column_id.eq_any(columns.iter().map(|column| column.id)))
                .select((
                    column_card::id,
                    column_card::name,
                    column_card::cover_attachment,
                    column_card::position,
                    column_card::description,
                    column_card::column_id,
                ))
                .load::<ReturnedCard>(conn)?
                .into_iter()
                .map(|card| PubCard {
                    id: card.0,
                    name: card.1,
                    cover_attachment: card.2,
                    position: card.3,
                    description: card.4,
                    column_id: card.5,
                })
                .collect::<Vec<PubCard>>();

            Ok::<BoardInfo, DieselError>(BoardInfo {
                name: board_name,
                id: board_id,
                columns,
                cards,
            })
        })
        .await
        .map_err(ApiError::from)
    }

    pub async fn update_board(
        db: &Db,
        auth: Auth,
        board_id: String,
        board_name: String,
    ) -> Result<Uuid, ApiError> {
        let board_id = parse_uuid(&board_id)?;

        db.run(move |conn| {
            diesel::update(boards::table.filter(
                boards::id.eq(board_id).and(boards::creator_id.eq(auth.id)),
            ))
            .set(boards::name.eq(board_name))
            .returning(boards::id)
            .get_result::<Uuid>(conn)
        })
        .await
        .map_err(ApiError::from)
    }

    pub async fn delete_board(db: &Db, auth: Auth, board_id: String) -> Result<Uuid, ApiError> {
        let board_id = parse_uuid(&board_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                let column_ids = board_column::table
                    .filter(board_column::board_id.eq(board_id))
                    .select(board_column::id)
                    .load::<Uuid>(conn)?;

                for column_id in column_ids {
                    let cards = column_card::table
                        .filter(column_card::column_id.eq(column_id))
                        .select(column_card::id)
                        .load::<Uuid>(conn)?;

                    for card_id in cards {
                        let attachments = card_attachments::table
                            .filter(card_attachments::card_id.eq(card_id))
                            .inner_join(files::table)
                            .select((card_attachments::file_id, files::name))
                            .load::<(Uuid, String)>(conn)?;

                        for (attachment_id, file_name) in attachments {
                            diesel::delete(card_attachments::table)
                                .filter(card_attachments::card_id.eq(card_id))
                                .filter(card_attachments::file_id.eq(attachment_id))
                                .execute(conn)?;
                            diesel::delete(files::table)
                                .filter(files::id.eq(attachment_id))
                                .execute(conn)?;
                            std::fs::remove_file(format!("tmp/{}", file_name))
                                .map_err(map_io_error)?;
                        }
                    }

                    diesel::delete(column_card::table)
                        .filter(column_card::column_id.eq(column_id))
                        .execute(conn)?;
                }

                diesel::delete(board_column::table.filter(board_column::board_id.eq(board_id)))
                    .execute(conn)?;
                diesel::delete(
                    board_users_relation::table.filter(board_users_relation::board_id.eq(board_id)),
                )
                .execute(conn)?;

                let deleted = diesel::delete(
                    boards::table.filter(boards::id.eq(board_id).and(boards::creator_id.eq(auth.id))),
                )
                .returning(boards::id)
                .get_result::<Uuid>(conn)?;

                Ok::<Uuid, ApiError>(deleted)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn create_column(
        db: &Db,
        auth: Auth,
        board_id: String,
        column: NewColumn,
    ) -> Result<Uuid, ApiError> {
        let board_id = parse_uuid(&board_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                diesel::insert_into(board_column::table)
                    .values(BoardColumn {
                        id: None,
                        name: column.name,
                        board_id,
                        position: column.position,
                    })
                    .returning(board_column::id)
                    .get_result::<Uuid>(conn)
                    .map_err(ApiError::from)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_columns(
        db: &Db,
        auth: Auth,
        board_id: String,
    ) -> Result<Vec<PubColumn>, ApiError> {
        let board_id = parse_uuid(&board_id)?;

        db.run(move |conn| {
            ensure_board_access(conn, board_id, auth.id)?;

            let columns = board_column::table
                .filter(board_column::board_id.eq(board_id))
                .select((board_column::id, board_column::name, board_column::position))
                .load::<ReturnedColumn>(conn)
                .map_err(ApiError::from)?
                .into_iter()
                .map(|col| PubColumn {
                    id: col.0,
                    name: col.1,
                    position: col.2,
                })
                .collect::<Vec<PubColumn>>();

            Ok::<Vec<PubColumn>, ApiError>(columns)
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_column(
        db: &Db,
        auth: Auth,
        board_id: String,
        column_id: String,
    ) -> Result<PubColumn, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let column_id = parse_uuid(&column_id)?;

        db.run(move |conn| {
            ensure_board_access(conn, board_id, auth.id)?;

            let column = board_column::table
                .filter(board_column::id.eq(column_id))
                .select((board_column::id, board_column::name, board_column::position))
                .first::<ReturnedColumn>(conn)
                .map_err(ApiError::from)?;

            Ok::<PubColumn, ApiError>(PubColumn {
                id: column.0,
                name: column.1,
                position: column.2,
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn update_column(
        db: &Db,
        auth: Auth,
        board_id: String,
        column_id: String,
        column: NewColumn,
    ) -> Result<PubColumn, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let column_id = parse_uuid(&column_id)?;

        db.run(move |conn| {
            ensure_board_access(conn, board_id, auth.id)?;

            let column = diesel::update(board_column::table)
                .filter(board_column::id.eq(column_id))
                .set((
                    board_column::name.eq(column.name),
                    board_column::position.eq(column.position),
                ))
                .returning((board_column::id, board_column::name, board_column::position))
                .get_result::<ReturnedColumn>(conn)
                .map_err(ApiError::from)?;

            Ok::<PubColumn, ApiError>(PubColumn {
                id: column.0,
                name: column.1,
                position: column.2,
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn delete_column(
        db: &Db,
        auth: Auth,
        board_id: String,
        column_id: String,
    ) -> Result<PubColumn, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let column_id = parse_uuid(&column_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let cards = column_card::table
                    .filter(column_card::column_id.eq(column_id))
                    .select((column_card::id, column_card::name))
                    .load::<(Uuid, String)>(conn)
                    .map_err(ApiError::from)?;

                for (card_id, _) in cards {
                    let attachments = card_attachments::table
                        .filter(card_attachments::card_id.eq(card_id))
                        .inner_join(files::table)
                        .select((card_attachments::file_id, files::name))
                        .load::<(Uuid, String)>(conn)
                        .map_err(ApiError::from)?;

                    for (attachment_id, file_name) in attachments {
                        diesel::delete(card_attachments::table)
                            .filter(card_attachments::card_id.eq(card_id))
                            .filter(card_attachments::file_id.eq(attachment_id))
                            .execute(conn)
                            .map_err(ApiError::from)?;
                        diesel::delete(files::table)
                            .filter(files::id.eq(attachment_id))
                            .execute(conn)
                            .map_err(ApiError::from)?;
                        std::fs::remove_file(format!("tmp/{}", file_name))
                            .map_err(map_io_error)?;
                    }
                }

                diesel::delete(column_card::table)
                    .filter(column_card::column_id.eq(column_id))
                    .execute(conn)
                    .map_err(ApiError::from)?;

                let column = diesel::delete(board_column::table)
                    .filter(board_column::id.eq(column_id))
                    .returning((board_column::id, board_column::name, board_column::position))
                    .get_result::<ReturnedColumn>(conn)
                    .map_err(ApiError::from)?;

                Ok::<PubColumn, ApiError>(PubColumn {
                    id: column.0,
                    name: column.1,
                    position: column.2,
                })
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn create_card(
        db: &Db,
        auth: Auth,
        board_id: String,
        column_id: String,
        card: NewCard,
    ) -> Result<PubCard, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let column_id = parse_uuid(&column_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let column = board_column::table
                    .filter(board_column::id.eq(column_id))
                    .select(board_column::id)
                    .first::<Uuid>(conn)
                    .map_err(ApiError::from)?;

                let card = diesel::insert_into(column_card::table)
                    .values(ColumnCard {
                        id: None,
                        name: card.name,
                        column_id: column,
                        position: card.position,
                        description: card.description,
                    })
                    .returning((
                        column_card::id,
                        column_card::name,
                        column_card::cover_attachment,
                        column_card::position,
                        column_card::description,
                        column_card::column_id,
                    ))
                    .get_result::<ReturnedCard>(conn)
                    .map_err(ApiError::from)?;

                Ok::<PubCard, ApiError>(to_pub_card(card))
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_cards(
        db: &Db,
        auth: Auth,
        board_id: String,
        column_id: String,
    ) -> Result<Vec<PubCard>, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let column_id = parse_uuid(&column_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let column = board_column::table
                    .filter(board_column::id.eq(column_id))
                    .select(board_column::id)
                    .first::<Uuid>(conn)
                    .map_err(ApiError::from)?;

                let cards = column_card::table
                    .filter(column_card::column_id.eq(column))
                    .select((
                        column_card::id,
                        column_card::name,
                        column_card::cover_attachment,
                        column_card::position,
                        column_card::description,
                        column_card::column_id,
                    ))
                    .get_results::<ReturnedCard>(conn)
                    .map_err(ApiError::from)?
                    .into_iter()
                    .map(to_pub_card)
                    .collect::<Vec<PubCard>>();

                Ok::<Vec<PubCard>, ApiError>(cards)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_card(
        db: &Db,
        auth: Auth,
        board_id: String,
        column_id: String,
        card_id: String,
    ) -> Result<Value, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let column_id = parse_uuid(&column_id)?;
        let card_id = parse_uuid(&card_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let column = board_column::table
                    .filter(board_column::id.eq(column_id))
                    .select(board_column::id)
                    .first::<Uuid>(conn)
                    .map_err(ApiError::from)?;

                let card = column_card::table
                    .filter(column_card::id.eq(card_id))
                    .filter(column_card::column_id.eq(column))
                    .select((
                        column_card::id,
                        column_card::name,
                        column_card::cover_attachment,
                        column_card::position,
                        column_card::description,
                        column_card::column_id,
                    ))
                    .first::<ReturnedCard>(conn)
                    .map_err(ApiError::from)?;

                let attachments = card_attachments::table
                    .filter(card_attachments::card_id.eq(card_id))
                    .inner_join(files::table)
                    .select((files::id, files::name))
                    .load::<(Uuid, String)>(conn)
                    .map_err(ApiError::from)?
                    .into_iter()
                    .map(|(id, name)| PubAttachment { id, url: name })
                    .collect::<Vec<PubAttachment>>();

                Ok::<Value, ApiError>(json!({
                    "id": card.0,
                    "name": card.1,
                    "cover_attachment": card.2,
                    "position": card.3,
                    "description": card.4,
                    "column_id": card.5,
                    "attachments": attachments
                }))
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn update_card(
        db: &Db,
        auth: Auth,
        board_id: String,
        column_id: String,
        card_id: String,
        card: CardInfo,
    ) -> Result<PubCard, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let column_id = parse_uuid(&column_id)?;
        let card_id = parse_uuid(&card_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let column = board_column::table
                    .filter(board_column::id.eq(column_id))
                    .select(board_column::id)
                    .first::<Uuid>(conn)
                    .map_err(ApiError::from)?;

                let card = diesel::update(column_card::table)
                    .filter(column_card::id.eq(card_id))
                    .filter(column_card::column_id.eq(column))
                    .set((
                        column_card::name.eq(card.name),
                        column_card::description.eq(card.description),
                    ))
                    .returning((
                        column_card::id,
                        column_card::name,
                        column_card::cover_attachment,
                        column_card::position,
                        column_card::description,
                        column_card::column_id,
                    ))
                    .get_result::<ReturnedCard>(conn)
                    .map_err(ApiError::from)?;

                Ok::<PubCard, ApiError>(to_pub_card(card))
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn reorder_cards(
        db: &Db,
        auth: Auth,
        board_id: String,
        from_column_id: String,
        card_id: String,
        to_column_id: String,
        to_pos: i32,
    ) -> Result<Vec<PubCard>, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let from_column_id = parse_uuid(&from_column_id)?;
        let card_id = parse_uuid(&card_id)?;
        let to_column_id = parse_uuid(&to_column_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let (card_id, pos) = column_card::table
                    .filter(column_card::id.eq(card_id))
                    .filter(column_card::column_id.eq(from_column_id))
                    .select((column_card::id, column_card::position))
                    .first::<(Uuid, i32)>(conn)
                    .map_err(ApiError::from)?;

                diesel::update(column_card::table)
                    .filter(column_card::column_id.eq(from_column_id))
                    .filter(column_card::position.gt(pos))
                    .set(column_card::position.eq(column_card::position - 1))
                    .execute(conn)
                    .map_err(ApiError::from)?;

                diesel::update(column_card::table)
                    .filter(column_card::column_id.eq(to_column_id))
                    .filter(column_card::position.ge(to_pos))
                    .set(column_card::position.eq(column_card::position + 1))
                    .execute(conn)
                    .map_err(ApiError::from)?;

                let card = diesel::update(column_card::table)
                    .filter(column_card::id.eq(card_id))
                    .set((
                        column_card::column_id.eq(to_column_id),
                        column_card::position.eq(to_pos),
                    ))
                    .returning((
                        column_card::id,
                        column_card::name,
                        column_card::cover_attachment,
                        column_card::position,
                        column_card::description,
                        column_card::column_id,
                    ))
                    .get_result::<ReturnedCard>(conn)
                    .map_err(ApiError::from)?;

                Ok::<Vec<PubCard>, ApiError>(vec![to_pub_card(card)])
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn delete_card(
        db: &Db,
        auth: Auth,
        board_id: String,
        column_id: String,
        card_id: String,
    ) -> Result<Uuid, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let column_id = parse_uuid(&column_id)?;
        let card_id = parse_uuid(&card_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let column = board_column::table
                    .filter(board_column::id.eq(column_id))
                    .select(board_column::id)
                    .first::<Uuid>(conn)
                    .map_err(ApiError::from)?;

                let (deleted_card_id, pos) = column_card::table
                    .filter(column_card::id.eq(card_id))
                    .select((column_card::id, column_card::position))
                    .first::<(Uuid, i32)>(conn)
                    .map_err(ApiError::from)?;

                diesel::update(column_card::table)
                    .filter(
                        column_card::column_id
                            .eq(column)
                            .and(column_card::position.gt(pos)),
                    )
                    .set(column_card::position.eq(column_card::position - 1))
                    .execute(conn)
                    .map_err(ApiError::from)?;

                let attachments = card_attachments::table
                    .filter(card_attachments::card_id.eq(deleted_card_id))
                    .inner_join(files::table)
                    .select((card_attachments::file_id, files::name))
                    .load::<(Uuid, String)>(conn)
                    .map_err(ApiError::from)?;

                for (attachment_id, file_name) in attachments {
                    diesel::delete(card_attachments::table)
                        .filter(card_attachments::card_id.eq(deleted_card_id))
                        .filter(card_attachments::file_id.eq(attachment_id))
                        .execute(conn)
                        .map_err(ApiError::from)?;
                    diesel::delete(files::table)
                        .filter(files::id.eq(attachment_id))
                        .execute(conn)
                        .map_err(ApiError::from)?;
                    std::fs::remove_file(format!("tmp/{}", file_name))
                        .map_err(map_io_error)?;
                }

                let deleted = diesel::delete(column_card::table)
                    .filter(
                        column_card::id
                            .eq(deleted_card_id)
                            .and(column_card::column_id.eq(column)),
                    )
                    .returning(column_card::id)
                    .get_result::<Uuid>(conn)
                    .map_err(ApiError::from)?;

                Ok::<Uuid, ApiError>(deleted)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_card_by_id(
        db: &Db,
        auth: Auth,
        board_id: String,
        card_id: String,
    ) -> Result<Value, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let card_id = parse_uuid(&card_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let card = column_card::table
                    .filter(column_card::id.eq(card_id))
                    .select(SELECT_CARD)
                    .first::<ReturnedCard>(conn)
                    .map_err(ApiError::from)?;

                let attachments = card_attachments::table
                    .filter(card_attachments::card_id.eq(card_id))
                    .inner_join(files::table)
                    .select((files::id, files::name))
                    .load::<(Uuid, String)>(conn)
                    .map_err(ApiError::from)?
                    .into_iter()
                    .map(|(id, name)| PubAttachment { id, url: name })
                    .collect::<Vec<PubAttachment>>();

                Ok::<Value, ApiError>(json!({
                    "id": card.0,
                    "name": card.1,
                    "cover_attachment": card.2,
                    "position": card.3,
                    "description": card.4,
                    "column_id": card.5,
                    "attachments": attachments
                }))
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn add_attachment_to_card(
        db: &Db,
        auth: Auth,
        board_id: String,
        card_id: String,
        card: UploadAttachment<'_>,
    ) -> Result<(String, Vec<u8>), ApiError> {
        let filename = card.filename.clone();
        let generated_file_name = format!("{}-{}", Uuid::new_v4(), filename);
        let generated_file_name_copy = generated_file_name.clone();
        let uploader_id = auth.id;

        let mut opened_file = card.file.open().await.map_err(ApiError::from_error)?;
        let mut bytes = Vec::new();
        use rocket::tokio::io::AsyncReadExt;
        opened_file
            .read_to_end(&mut bytes)
            .await
            .map_err(ApiError::from_error)?;

        let board_id = parse_uuid(&board_id)?;
        let card_id = parse_uuid(&card_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, uploader_id)?;

                let (card_id, cover) = column_card::table
                    .filter(column_card::id.eq(card_id))
                    .select((column_card::id, column_card::cover_attachment))
                    .first::<(Uuid, Option<String>)>(conn)
                    .map_err(ApiError::from)?;

                let new_attachment = UploadedFile {
                    id: Uuid::new_v4(),
                    name: generated_file_name,
                    user_id: uploader_id,
                    private: false,
                };

                diesel::insert_into(files::table)
                    .values(&new_attachment)
                    .execute(conn)
                    .map_err(ApiError::from)?;
                diesel::insert_into(card_attachments::table)
                    .values((
                        card_attachments::file_id.eq(new_attachment.id),
                        card_attachments::card_id.eq(card_id),
                    ))
                    .execute(conn)
                    .map_err(ApiError::from)?;

                if cover.is_none() {
                    diesel::update(column_card::table)
                        .filter(column_card::id.eq(card_id))
                        .set(column_card::cover_attachment.eq(new_attachment.name.clone()))
                        .execute(conn)
                        .map_err(ApiError::from)?;
                }

                Ok::<(String, Vec<u8>), ApiError>((generated_file_name_copy, bytes))
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_attachments_of_card(
        db: &Db,
        auth: Auth,
        board_id: String,
        card_id: String,
    ) -> Result<Vec<PubAttachment>, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let card_id = parse_uuid(&card_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let attachments = card_attachments::table
                    .filter(card_attachments::card_id.eq(card_id))
                    .inner_join(files::table)
                    .select((files::id, files::name))
                    .load::<(Uuid, String)>(conn)
                    .map_err(ApiError::from)?
                    .into_iter()
                    .map(|(id, name)| PubAttachment { id, url: name })
                    .collect::<Vec<PubAttachment>>();

                Ok::<Vec<PubAttachment>, ApiError>(attachments)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn delete_attachment_of_card(
        db: &Db,
        auth: Auth,
        board_id: String,
        card_id: String,
        attachment_id: String,
    ) -> Result<String, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let card_id = parse_uuid(&card_id)?;
        let attachment_id = parse_uuid(&attachment_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                let (card_id, cover) = column_card::table
                    .filter(column_card::id.eq(card_id))
                    .select((column_card::id, column_card::cover_attachment))
                    .first::<(Uuid, Option<String>)>(conn)
                    .map_err(ApiError::from)?;

                diesel::delete(card_attachments::table)
                    .filter(card_attachments::card_id.eq(card_id))
                    .filter(card_attachments::file_id.eq(attachment_id))
                    .execute(conn)
                    .map_err(ApiError::from)?;

                let file_name = diesel::delete(files::table)
                    .filter(files::id.eq(attachment_id))
                    .returning(files::name)
                    .get_result::<String>(conn)
                    .map_err(ApiError::from)?;

                if cover.is_some() {
                    diesel::update(column_card::table)
                        .filter(column_card::id.eq(card_id))
                        .set(column_card::cover_attachment.eq(None::<String>))
                        .execute(conn)
                        .map_err(ApiError::from)?;
                }

                Ok::<String, ApiError>(file_name)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn add_collaborator(
        db: &Db,
        auth: Auth,
        board_id: String,
        collaborator_id: Uuid,
    ) -> Result<Uuid, ApiError> {
        let board_id = parse_uuid(&board_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;
                ensure_board_creator(conn, board_id, auth.id)?;

                diesel::insert_into(board_users_relation::table)
                    .values(BoardUsersRelation {
                        board_id,
                        user_id: collaborator_id,
                    })
                    .returning(board_users_relation::user_id)
                    .get_result::<Uuid>(conn)
                    .map_err(ApiError::from)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_collaborators(
        db: &Db,
        auth: Auth,
        board_id: String,
    ) -> Result<Vec<Uuid>, ApiError> {
        let board_id = parse_uuid(&board_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                board_users_relation::table
                    .filter(board_users_relation::board_id.eq(board_id))
                    .select(board_users_relation::user_id)
                    .load::<Uuid>(conn)
                    .map_err(ApiError::from)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn get_collaborator(
        db: &Db,
        auth: Auth,
        board_id: String,
        collaborator_id: String,
    ) -> Result<Uuid, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let collaborator_id = parse_uuid(&collaborator_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_access(conn, board_id, auth.id)?;

                board_users_relation::table
                    .filter(
                        board_users_relation::board_id
                            .eq(board_id)
                            .and(board_users_relation::user_id.eq(collaborator_id)),
                    )
                    .select(board_users_relation::user_id)
                    .first::<Uuid>(conn)
                    .map_err(ApiError::from)
            })
        })
        .await
        .map_err(|e| e)
    }

    pub async fn remove_collaborator(
        db: &Db,
        auth: Auth,
        board_id: String,
        collaborator_id: String,
    ) -> Result<Uuid, ApiError> {
        let board_id = parse_uuid(&board_id)?;
        let collaborator_id = parse_uuid(&collaborator_id)?;

        db.run(move |conn| {
            conn.transaction(|conn| {
                ensure_board_creator(conn, board_id, auth.id)?;

                diesel::delete(
                    board_users_relation::table
                        .filter(board_users_relation::board_id.eq(board_id))
                        .filter(board_users_relation::user_id.eq(collaborator_id)),
                )
                .execute(conn)
                .map_err(ApiError::from)?;

                Ok::<Uuid, ApiError>(collaborator_id)
            })
        })
        .await
        .map_err(|e| e)
    }
}

fn ensure_board_access(
    conn: &mut diesel::PgConnection,
    board_id: Uuid,
    user_id: Uuid,
) -> Result<(), ApiError> {
    board_users_relation::table
        .filter(
            board_users_relation::board_id
                .eq(board_id)
                .and(board_users_relation::user_id.eq(user_id)),
        )
        .first::<BoardUsersRelation>(conn)
        .map(|_| ())
        .map_err(ApiError::from)
}

fn ensure_board_creator(
    conn: &mut diesel::PgConnection,
    board_id: Uuid,
    user_id: Uuid,
) -> Result<(), ApiError> {
    boards::table
        .filter(boards::id.eq(board_id).and(boards::creator_id.eq(user_id)))
        .select(boards::id)
        .first::<Uuid>(conn)
        .map(|_| ())
        .map_err(ApiError::from)
}

fn parse_uuid(value: &str) -> Result<Uuid, ApiError> {
    Uuid::try_parse(value).map_err(|_| ApiError::from_type(ApiErrorType::FailedToParseUUID))
}

fn to_pub_card(card: ReturnedCard) -> PubCard {
    PubCard {
        id: card.0,
        name: card.1,
        cover_attachment: card.2,
        position: card.3,
        description: card.4,
        column_id: card.5,
    }
}

fn map_io_error(error: std::io::Error) -> ApiError {
    ApiError::from_error(error)
}
