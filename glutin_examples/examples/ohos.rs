#![cfg(ohos_platform)]

use openharmony_ability::OpenHarmonyApp;
use openharmony_ability_derive::ability;

use winit::event_loop::EventLoop;
use winit::platform::ohos::EventLoopBuilderExtOpenHarmony;

#[ability]
pub fn openharmony(openharmony_app: OpenHarmonyApp) {
    let event_loop = EventLoop::builder().with_openharmony_app(openharmony_app).build().unwrap();
    // The OHOS backend retains the owned application and returns to Ability.
    glutin_examples::main(event_loop).unwrap();
}
