use tk_spawn::tk_spawn_run;
use tk_join::tk_join_run;

// tokia 的 编排
pub mod tk_spawn;
pub mod tk_join;
pub mod tk_rice;


pub async fn run() {

    tk_spawn_run().await;
    tk_join_run().await;

}
