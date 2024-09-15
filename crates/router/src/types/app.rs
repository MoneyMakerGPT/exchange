use redis::RedisManager;

pub struct AppState {
  pub redis_connection: RedisManager,
}