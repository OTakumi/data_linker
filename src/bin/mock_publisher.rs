use rand::Rng;
use rumqttc::{self, AsyncClient, MqttOptions, QoS};
use serde::Serialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug)]
struct SensorData {
    device_id: String,
    timestamp: u64,
    temperature: f32,
    humidity: f32,
}

struct MQTTConfig {
    host: &'static str,
    port: u16,
    client_id: &'static str,
    topic: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BROKER_CONFIG: MQTTConfig = MQTTConfig {
        host: "localhost",
        port: 1883,
        client_id: "rust_mock_sensor_pub",
        topic: "sensors/living_room/environment",
    };

    let mut mqttoptions = MqttOptions::new(
        BROKER_CONFIG.client_id,
        BROKER_CONFIG.host,
        BROKER_CONFIG.port,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // サーバーからの応答を処理するためにイベントループをバックグラウンドで動かす
    tokio::spawn(async move {
        loop {
            let _ = eventloop.poll().await;
        }
    });

    println!("✅ Publisher is running.");
    println!("   Broker: {}:{}", BROKER_CONFIG.host, BROKER_CONFIG.port);
    println!("   Topic: {}", BROKER_CONFIG.topic);

    let mut rng = rand::thread_rng();

    // 2秒ごとにデータを生成して送信するループ
    loop {
        // センサーデータを模したデータを作成
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let temp: f32 = rng.gen_range(20.0..28.0);
        let hum: f32 = rng.gen_range(40.0..65.0);

        let data = SensorData {
            device_id: "LIVING-ROOM-SENSOR-01".to_string(),
            timestamp: now,
            temperature: temp,
            humidity: hum,
        };

        // structをMessagePackのバイト列にシリアライズ
        let payload_bytes = rmp_serde::to_vec(&data)?;

        println!(
            "-> Publishing: temp={:.1}°C, hum={:.1}%, size={} bytes",
            data.temperature,
            data.humidity,
            payload_bytes.len()
        );

        // バイト列をMQTTで送信
        client
            .publish(
                BROKER_CONFIG.topic,
                QoS::AtLeastOnce,
                false, // retainフラグ
                payload_bytes,
            )
            .await?;

        // 2秒待機
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
