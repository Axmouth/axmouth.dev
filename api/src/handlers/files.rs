use crate::{
    app::{DynStaticFileAddress, DynStaticFileDir},
    auth_tokens::Claims,
    errors::{AppError, FileUploadError},
    extractors::AdminClaimsContext,
    util::{upload_bad_request_response, upload_error_response},
};
use axum::{
    extract::{ContentLengthLimit, Extension, Multipart},
    response::IntoResponse,
    Json,
};
use backend_repo_pg::{
    insertables::NewUploadedImage,
    models::responses::{FileUploadedDetails, FileUploadedResponse},
    pg_util::{pg_transaction, DynRepo},
    uploaded_images::UploadedImageRepo,
};
use chrono::Utc;
use hyper::StatusCode;
use std::{ffi::OsStr, path::Path};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn editor_js_upload(
    multipart: ContentLengthLimit<
        Multipart,
        {
            5 * 1024 * 1024 /* 5mb */
        },
    >,
    claims: AdminClaimsContext,
    repo: Extension<DynRepo>,
    static_file_dir: Extension<DynStaticFileDir>,
    static_file_address: Extension<DynStaticFileAddress>,
) -> Result<impl IntoResponse, AppError> {
    image_upload(
        multipart,
        claims,
        repo,
        static_file_dir,
        static_file_address,
    )
    .await
}

pub async fn image_upload(
    ContentLengthLimit(multipart): ContentLengthLimit<
        Multipart,
        {
            5 * 1024 * 1024 /* 5mb */
        },
    >,
    AdminClaimsContext { claims }: AdminClaimsContext,
    Extension(repo): Extension<DynRepo>,
    Extension(static_file_dir): Extension<DynStaticFileDir>,
    Extension(static_file_address): Extension<DynStaticFileAddress>,
) -> Result<impl IntoResponse, AppError> {
    let form_data = process_multipart(multipart).await?;
    let (field_name, filename_opt, file_data) = match form_data.get(0) {
        None => {
            return Ok(upload_bad_request_response("No file included"));
        }
        Some(value) => value,
    };

    if field_name != "image" {
        return Ok(upload_bad_request_response(
            "Expected an image file under name 'image', and only that",
        ));
    }
    let filename = match filename_opt {
        Some(value) => value,
        None => {
            return Ok(upload_bad_request_response(
                "No filename found, come on dude",
            ));
        }
    };

    let upload_folder = format!(
        "{}/media/images/{}",
        static_file_dir.static_file_dir(),
        Utc::now().timestamp()
    );

    let upload_details = upload_file(
        filename,
        file_data,
        &upload_folder,
        static_file_dir.static_file_dir(),
        static_file_address.static_file_address(),
        claims,
    )
    .await?;

    let extension = match get_extension_from_filename(filename) {
        Some(value) => value,
        None => {
            return Err(AppError::FileUploadError(FileUploadError::new(
                "Could not get file extension".to_string(),
            )));
        }
    };
    Ok(pg_transaction(repo, |conn| {
        let new_uploaded_image = NewUploadedImage {
            extension,
            height: None,
            width: None,
            user_id: 1,
            used_where: None,
            url: upload_details.url.clone(),
            path: upload_details.path.clone(),
        };
        let uploaded_images_repository = UploadedImageRepo::new(conn);
        match uploaded_images_repository.insert_one(new_uploaded_image) {
            Ok(_) => {}
            Err(err) => {
                return Ok(upload_error_response(err));
            }
        };

        let resp_body = Json(FileUploadedResponse {
            success: 1,
            file: Some(upload_details),
            errors: None,
        });
        Ok((StatusCode::CREATED, resp_body).into_response())
    })
    .await?)
}

pub async fn process_multipart(
    mut multipart: Multipart,
) -> Result<Vec<(String, Option<String>, Vec<u8>)>, FileUploadError> {
    // Collect the fields into (name, value): (String, Vec<u8>)
    let mut parts: Vec<(String, Option<String>, Vec<u8>)> = Vec::new();
    for part in multipart.next_field().await? {
        let name = part.name().map(|name| name.to_string()).unwrap_or_default();
        let filename = part.file_name().map(|filename| filename.to_string());
        let value = part.bytes().await?;
        parts.push((name, filename, value.to_vec()))
    }
    Ok(parts)
}

async fn upload_file(
    filename: &str,
    file_data: &[u8],
    upload_folder: &str,
    static_file_dir: &str,
    static_file_address: &str,
    _: Claims,
) -> Result<FileUploadedDetails, FileUploadError> {
    let new_filename = format!(
        "{}/{}-{}",
        upload_folder,
        uuid::Uuid::new_v4().to_string().replace("-", ""),
        filename
    );

    tokio::fs::create_dir_all(upload_folder).await?;
    let mut file = File::create(new_filename.clone()).await?;
    file.write_all(file_data).await?;

    Ok(FileUploadedDetails {
        height: None,
        width: None,
        url: new_filename.replace(static_file_dir, static_file_address),
        path: new_filename,
    })
}

fn get_extension_from_filename(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .map(|s| s.to_string())
}
