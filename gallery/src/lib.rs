// Android
#[cfg(target_os = "android")]
use {
    crate::android_activity::{MainEvent, PollEvent},
    core::cell::RefCell,
    slint::android::android_activity,
    std::rc::Rc,
    slint::VecModel,
    slint::StandardListViewItem,
    slint::ModelExt,
    slint::Model,

};

slint::include_modules!();


#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: slint::android::AndroidApp) -> Result<(), slint::PlatformError> {
    //android_logger::init_once(android_logger::Config::default().with_max_level(
    //    if cfg!(debug_assertions) { log::LevelFilter::Debug } else { log::LevelFilter::Info },
    //));
    //
    slint::android::init(android_app).unwrap();
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    // Disable gettext on macOS due to https://github.com/Koka/gettext-rs/issues/114
    #[cfg(not(target_os = "android"))]
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));

    let app = App::new().unwrap();

    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

    for r in 1..101 {
        let items = Rc::new(VecModel::default());

        for c in 1..5 {
            items.push(slint::format!("Item {r}.{c}").into());
        }

        row_data.push(items.into());
    }

    app.global::<TableViewPageAdapter>().set_row_data(row_data.clone().into());

    app.global::<TableViewPageAdapter>().on_sort_ascending({
        let app_weak = app.as_weak();
        let row_data = row_data.clone();
        move |index| {
            let row_data = row_data.clone();

            let sort_model = Rc::new(row_data.sort_by(move |r_a, r_b| {
                let c_a = r_a.row_data(index as usize).unwrap();
                let c_b = r_b.row_data(index as usize).unwrap();

                c_a.text.cmp(&c_b.text)
            }));

            app_weak.unwrap().global::<TableViewPageAdapter>().set_row_data(sort_model.into());
        }
    });

    app.global::<TableViewPageAdapter>().on_sort_descending({
        let app_weak = app.as_weak();
        move |index| {
            let row_data = row_data.clone();

            let sort_model = Rc::new(row_data.sort_by(move |r_a, r_b| {
                let c_a = r_a.row_data(index as usize).unwrap();
                let c_b = r_b.row_data(index as usize).unwrap();

                c_b.text.cmp(&c_a.text)
            }));

            app_weak.unwrap().global::<TableViewPageAdapter>().set_row_data(sort_model.into());
        }
    });

    app.run().unwrap();
    Ok(())
}
