
use std::{sync::Arc, string};
use std::io::{BufReader, Read};

use axum::body::{self, StreamBody, HttpBody};
use axum::http::status;
use axum::response::Response;
use axum::{
    routing::{get, Route},
    Router, Json, middleware::AddExtension, Extension, response::IntoResponse,
    extract::Path,
};
use hyper::{StatusCode, Body};
use song_meta::{SongMeta, Song};
use tokio::io::{AsyncBufReadExt, AsyncBufRead, AsyncRead, BufStream};
use tokio::sync::Mutex;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let state = Arc::new(Mutex::new(Container::new()?));

    // build our application with a single route
    let app = Router::new()
    .route("/", get(|| async {"WELCOME TO THE SongAPI"}))
    .route("/SongList", get(SongList))
    .route("/ArtistList", get(ArtistList))
    .route("/AlbumList", get(AlbumList))
    .route("/SongPathList", get(SongPathList))
    .route("/Song/:artist/:album/:title", get(Song))
    .layer(Extension(state));


    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        ?;


    Ok(())
}

pub struct Container{
    pub get_song_meta: SongMeta,
}

impl Container {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>>{
        let _self = Self{
            //音源が格納されているディレクトリを引数に入れる必要がある。
            get_song_meta: SongMeta::new("")?
        
        };
        Ok(_self)
        
    }
}

type State = Arc<Mutex<Container>>;

pub async fn SongPathList(Extension(state): Extension<State>) -> impl IntoResponse{
    let songmeta = &state.lock().await.get_song_meta;
    return (
        StatusCode::ACCEPTED,
        Json(songmeta.GetSongPathList())
    )
}

pub async fn SongList(Extension(state): Extension<State>) -> impl IntoResponse{
    let songmeta = &state.lock().await.get_song_meta;
    return (
        StatusCode::ACCEPTED,
        Json(songmeta.GetSongList()),
    );
}

pub async fn ArtistList(Extension(state): Extension<State>) -> impl IntoResponse{
    let songmeta = &state.lock().await.get_song_meta;
    return (
        StatusCode::ACCEPTED,
        Json(songmeta.GetSongArtistList()),
    );
}

pub async fn AlbumList(Extension(state): Extension<State>) -> impl IntoResponse{
    let songmeta = &state.lock().await.get_song_meta;
    return (
        StatusCode::ACCEPTED,
        Json(songmeta.GetSongAlbumList()),
    );
}

pub async fn Song(Extension(state): Extension<State>,
                Path((artist, album, title)) : Path<(String,String,String)>)
                -> impl IntoResponse{
    let song = &state.lock().await.get_song_meta.GetSong(&*artist, &*album, &*title);
    let file = tokio::fs::read(&*song.path).await.unwrap(); 

    return Response::builder()
    .header("Content-Disposition", format!("{}{}{}", "attachment; filename=", &*song.title, ".flac"))
    .header("Content-Type", "audio/x-flac")
    .status(StatusCode::OK)
    .body(Body::from(file))
    .unwrap();
    
    
}

 

