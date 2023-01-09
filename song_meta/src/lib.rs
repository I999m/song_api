use core::str;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::string;

use file_format::FileFormat; 

use serde::Serialize;

#[derive(Serialize,Clone)]
pub struct Song{
    pub artist:String,
    pub album:String,
    pub title:String,
    pub path:String
    
}

impl Song {
    pub fn Equals(
        &self,
        artist:&str,
        album:&str,
        title:&str,
    ) -> bool{
        return 
        &self.artist == artist
        && &self.album == album
        && &self.title == title;
    }
}

#[derive(Clone)]
pub struct SongMeta {
    song_path_list:Vec<String>,
    song_list:Vec<Song>,
    song_artist_list:Vec<String>,
    song_album_list:Vec<String>,
}

impl SongMeta {
    //指定ディレクトリ以下に存在する音源データを回帰的にサーチして、パスをsong_path_listに追加
    fn AddSongPath(&mut self, path:&str)-> Result<(), Box<dyn std::error::Error>>{
        let files = std::fs::read_dir(path)?;
        for file_entry in files.into_iter(){

            let filepath = file_entry?.path();

            if !filepath.is_dir() {
                let filepath_str = filepath.into_os_string().into_string().unwrap();
                                
                if  FileFormat::from_file(&filepath_str).unwrap() == FileFormat::FreeLosslessAudioCodec {
                    self.song_path_list.push(filepath_str);
                }
            } else {
                self.AddSongPath(filepath.as_os_str().to_str().unwrap())?
            }

        } 
        Ok(())
    }
    
    //song_path_listに追加されたパスをもとに曲のメタデータをsong_listに追加する
    fn AddSongMeta(&mut self){

        fn get_exit_meta(hash:&HashMap<String, String, RandomState>, key:&str) -> String {
            let mut meta:&str = "empty";

            if hash.contains_key(&key.to_uppercase()) {
                meta = &hash[key];
            } else if hash.contains_key(&key.to_lowercase()) {
                meta = &hash[&key.to_lowercase()];
            }

            return meta.trim().to_string();
        }

        for song_path in &self.song_path_list{
            match flac::metadata::get_vorbis_comment(&song_path) {
                Ok(vorbis_comment) => {

                    // let _artist = vorbis_comment.comments["ARTIST"].to_string();
                    // let _album = vorbis_comment.comments["ALBUM"].to_string();
                    // let _title = vorbis_comment.comments["TITLE"].to_string();
                    

                    let _artist:String = get_exit_meta(&vorbis_comment.comments, "ARTIST");
                    let _album:String = get_exit_meta(&vorbis_comment.comments, "ALBUM");
                    let _title:String = get_exit_meta(&vorbis_comment.comments, "TITLE");

                    //アルバムリスト
                    if !self.song_album_list.contains(&_album) {
                        self.song_album_list.push(_album.clone());
                    }
                    //アーティストリスト
                    if !self.song_artist_list.contains(&_artist) {
                        self.song_artist_list.push(_artist.clone());
                    }     

                    self.song_list.push(Song{
                        artist: _artist,
                        album: _album,
                        title: _title,
                        path: song_path.clone(),
                    });
                }
                Err(error)         => println!("{:?}", error),
              }
        }
    }

    pub fn new(song_directory:&str) -> Result<Self, Box<dyn std::error::Error>>{
        let mut getsongmeta:SongMeta = Self { song_list: Vec::new(), song_path_list:Vec::new(), 
            song_album_list:Vec::new(), song_artist_list:Vec::new() };

        //ディレクトリ内に存在する曲を追加
        getsongmeta.AddSongPath(song_directory)?;
        getsongmeta.AddSongMeta();

        Ok(getsongmeta)
    }


    pub fn GetSongPathList(&self) -> Vec<String>{
        return self.song_path_list.clone();
    }
    pub fn GetSongArtistList(&self) -> Vec<String>{
        return self.song_artist_list.clone(); 
    }
    pub fn GetSongAlbumList(&self) -> Vec<String>{
        return self.song_album_list.clone();
    }
    pub fn GetSongList(&self) -> Vec<Song>{
        return self.song_list.clone();
    }
    pub fn GetSong(&self, artist:&str, album:&str, title:&str) -> Song {
        for song in self.song_list.clone() {
            if(song.Equals(artist, album, title)){
                return song;
            }
        }

        return Song {
            artist: "".to_string(),
            album: "".to_string(),
            title: "".to_string(),
            path: "".to_string(),
        }
    }
}





