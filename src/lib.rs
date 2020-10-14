use std::convert::TryInto;
use std::fs;

use napi::{
  register_module, CallContext, Env, JsArrayBuffer, JsNumber, JsObject, JsString, Module, Result,
  Task,
};
use napi_derive::js_function;

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

register_module!(example, init);

struct AsyncTask(u32);

impl Task for AsyncTask {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::from_millis(self.0 as u64));
    Ok(self.0 * 2)
  }

  fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}

fn init(module: &mut Module) -> Result<()> {
  module.create_named_method("sync", sync_fn)?;

  module.create_named_method("sleep", sleep)?;

  module.create_named_method("readfile", read_file)?;

  module.create_named_method("readfileReverse", read_file_reverse)?;
  Ok(())
}

#[js_function(1)]
fn sync_fn(ctx: CallContext) -> Result<JsNumber> {
  let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;

  ctx.env.create_uint32(argument + 100)
}

#[js_function(1)]
fn sleep(ctx: CallContext) -> Result<JsObject> {
  let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let task = AsyncTask(argument);
  ctx.env.spawn(task)
}

#[js_function(1)]
fn read_file(ctx: CallContext) -> Result<JsString> {
  let filename: String = ctx.get::<JsString>(0)?.try_into()?;
  let contents = fs::read_to_string(filename).expect("wrong");
  ctx.env.create_string(contents.as_str())
}

#[js_function(1)]
fn read_file_reverse(ctx: CallContext) -> Result<JsString> {
  let filename: String = ctx.get::<JsString>(0)?.try_into()?;
  let contents = fs::read_to_string(filename).expect("wrong");
  let mut main_vec:Vec<&str> = contents.split("").collect();
  main_vec.reverse();
  let main_vec_str = main_vec.join("");
  ctx.env.create_string(main_vec_str.as_str())
}
