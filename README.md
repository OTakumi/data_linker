# data_linker

## メモ

- IoTの開発のためのテストプロジェクト
- Publisherを温度センサーを模したMockで実装する
- Subscriberはまずは、取得したデータをそのまま取得する実装にする
- BrokerはMosquittoを使用し、dockerで動作させる
- まずは、ローカルで動作する構成で実装を進める

## 動作検証方法

dockerでMosquittoを起動する

```bash
docker compose up -d mosquitto
```

Rustで実装した、センサーモックを起動する

```bash
cargo run
```

mosquitto subを使用して、データを読み取る
-h：ホスト名
-t：トピック

```bash
mosquitto_sub -h localhost -t sensors/living_room/environment
```
