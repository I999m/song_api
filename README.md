
# SongApi

## About
音源を配信するサーバです。<br />
rustで書いてます。<br />
[FlacApiServer](https://github.com/I999m/FlacApiServer)のrust版です。
<br/>
<br />
とりあえず動くけど、未完成です。<br />
気が向いたら完成させます。<br/>

## Support

- [x] sorted song meta
- [x] file response
- [ ] more
### file format
- [x] flac
- [ ] more

## some

rustのコーディング規約を把握してないので、
かなり適当に書いてます。<br />
参考にしないほうがいいです。<br/>

## API

/SongList
- 音源のリストが返ってきます。

/ArtistList
- アーティストのリストが返ってきます。

/AlbumList
- アルバムのリストが返ってきます。

/SongPathList
- 実行している環境における音源のパスが返ってきます。

/Song/{artist}/{album}/{title}
- アーティスト, アルバム, タイトルをそれぞれ正しいものを入れると音源が返ってきます。
- 存在しないものを入力した場合の何も返ってこないはず...
