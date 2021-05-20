use crate::{
    app::AppState,
    auth_tokens::Claims,
    errors::FileUploadError,
    util::{upload_bad_request_response, upload_error_response},
};
use backend_repo_pg::{
    insertables::NewUploadedImage,
    models::responses::{FileUploadedDetails, FileUploadedResponse},
    uploaded_images::UploadedImageRepo,
};
use bytes::BufMut;
use chrono::Utc;
use futures::{Stream, TryFutureExt, TryStreamExt};
use std::{ffi::OsStr, path::Path};
use tokio::{fs::File, io::AsyncWriteExt};
use warp::{hyper::StatusCode, Buf, Reply};
use warp::{multipart, reject, Filter};

pub async fn editor_js_upload(
    form: multipart::FormData,
    claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    image_upload(form, claims, state).await
}

pub async fn image_upload(
    form: multipart::FormData,
    claims: Claims,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    let form_data = process_multipart(form).await?;

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
        state.static_file_dir,
        Utc::now().timestamp().to_string()
    );

    let upload_details =
        upload_file(filename, file_data, upload_folder, claims, state.clone()).await?;

    let extension = match get_extension_from_filename(filename) {
        Some(value) => value,
        None => {
            return Err(reject::custom(FileUploadError::new(
                "Could not get file extension".to_string(),
            )));
        }
    };

    let new_uploaded_image = NewUploadedImage {
        extension,
        height: None,
        width: None,
        user_id: 1,
        used_where: None,
        url: upload_details.url.clone(),
        path: upload_details.path.clone(),
    };
    let uploaded_images_repository = UploadedImageRepo::new(state.repo.clone());
    match uploaded_images_repository
        .insert_one(new_uploaded_image)
        .await
    {
        Ok(_) => {}
        Err(err) => {
            return Ok(upload_error_response(err));
        }
    };

    let resp_body = warp::reply::json(&FileUploadedResponse {
        success: 1,
        file: Some(upload_details),
        errors: None,
    });
    let resp_with_status = warp::reply::with_status(resp_body, StatusCode::CREATED);
    return Ok(resp_with_status.into_response());
}

pub async fn process_multipart(
    form: multipart::FormData,
) -> Result<Vec<(String, Option<String>, Vec<u8>)>, FileUploadError> {
    // Collect the fields into (name, value): (String, Vec<u8>)
    let parts: Result<Vec<(String, Option<String>, Vec<u8>)>, warp::Error> = form
        .and_then(|part| {
            let name = part.name().to_string();
            let filename = part.filename().map(|filename| filename.to_string());
            let value = part.stream().try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            });
            value.map_ok(move |vec| (name, filename, vec))
        })
        .try_collect()
        .await
        .map_err(|err| err);
    Ok(parts?)
}

async fn upload_file(
    filename: &String,
    file_data: &Vec<u8>,
    upload_folder: String,
    _: Claims,
    state: AppState,
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

    return Ok(FileUploadedDetails {
        height: None,
        width: None,
        url: new_filename.replace(state.static_file_dir.as_str(), &state.static_file_address),
        path: new_filename,
    });
}

fn get_extension_from_filename(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .map(|s| s.to_string())
}
