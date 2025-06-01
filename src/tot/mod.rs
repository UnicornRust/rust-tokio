use async_abs::timeout_run;
use tk_rice::tk_race_run;
use tk_spawn::tk_spawn_run;
use tk_join::tk_join_run;
use tk_stream::stream_run;
use tk_yield::tk_yield_run;

// tokia 的 编排
pub mod tk_spawn;
pub mod tk_join;
pub mod tk_rice;
pub mod tk_yield;
pub mod async_abs;
pub mod tk_stream;


pub async fn run() {

    // tk_spawn_run().await;
    // tk_join_run().await;
    // tk_race_run().await;
    // tk_yield_run().await;
    // timeout_run().await;
    stream_run().await;
}
