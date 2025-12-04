use std::error::Error;
use dioxus::prelude::*;
use dioxus_use_js::JsError;

use crate::{LatLng, MapOptions, MapPosition, MarkerIcon, PathOptions, PopupOptions, types::Id};

pub const DL_JS: Asset = asset!("/assets/dioxus_leaflet.js");

mod js_api {
    use dioxus::prelude::*;
    use dioxus_use_js::use_js;

    use_js!("js_utils/src/map.ts", "assets/dioxus_leaflet.js"::{update_map, delete_map, /*on_map_click*/});
    use_js!("js_utils/src/marker.ts", "assets/dioxus_leaflet.js"::{update_marker, delete_marker});
    use_js!("js_utils/src/polygon.ts", "assets/dioxus_leaflet.js"::{update_polygon, delete_polygon});
    use_js!("js_utils/src/popup.ts", "assets/dioxus_leaflet.js"::{update_popup});

    #[allow(non_snake_case)]
    pub async fn on_map_click(
        map_id: impl dioxus_use_js::SerdeSerialize,
        mut callback: impl AsyncFnMut(
            Vec<f64>,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
    ) -> Result<(), dioxus_use_js::JsError> {
        const MODULE: Asset = {
            const __ASSET_SOURCE_PATH: &'static str = "/home/svergenz/Projects/dioxus-leaflet/dioxus-leaflet/assets/dioxus_leaflet.js";
            const __ASSET_OPTIONS: manganis::AssetOptions = manganis::AssetOptions::builder()
                .into_asset_options();
            const __ASSET_HASH: &'static str = "2e86b9e87f94a467";
            const __ASSET: manganis::BundledAsset = manganis::macro_helpers::create_bundled_asset(
                __ASSET_SOURCE_PATH,
                __ASSET_OPTIONS,
            );
            const __BUFFER: manganis::macro_helpers::const_serialize::ConstVec<u8> = manganis::macro_helpers::serialize_asset(
                &__ASSET,
            );
            const __BYTES: &[u8] = __BUFFER.as_ref();
            const __LEN: usize = __BYTES.len();
            #[unsafe(export_name = "__MANGANIS__2e86b9e87f94a467")]
            static __LINK_SECTION: [u8; __LEN] = manganis::macro_helpers::copy_bytes(
                __BYTES,
            );
            static __REFERENCE_TO_LINK_SECTION: &'static [u8] = &__LINK_SECTION;
            manganis::Asset::new(|| unsafe {
                std::ptr::read_volatile(&__REFERENCE_TO_LINK_SECTION)
            })
        };
        // let js = ::alloc::__export::must_use({
        //     ::alloc::fmt::format(
        //         format_args!(
        //             "\nconst {{ on_map_click }} = await import(\"{0}\");\nlet map_id = await dioxus.recv();\nconst callback = async (value) => {{ dioxus.send([1, value]); await dioxus.recv(); }};\nlet ___result___;\ntry {{\n\n___result___ = await on_map_click(map_id, callback);\n\n}}\ncatch (e) {{\nconsole.error(\"Executing function `on_map_click` threw an error:\", e);\n___result___ = undefined;\n}}\nif (___result___ === undefined) {{ dioxus.send([0, null]); }}; dioxus.send([0, ___result___]);\n",
        //             MODULE,
        //         ),
        //     )
        // });
        let js = format!(r#"
            const {{ on_map_click }} = await import(\"{0}\");
            let map_id = await dioxus.recv();
            const callback = async (value) => {{
                //console.log('cb called with', value);
                dioxus.send([1, value]);
                //console.log('value sent');
                await dioxus.recv();
                //console.log('recv complete');
            }};
            let ___result___;
            try {{
                ___result___ = await on_map_click(map_id, callback);
            }}
            catch (e) {{
                console.error(\"Executing function `on_map_click` threw an error:\", e);
                ___result___ = undefined;
            }}
            if (___result___ === undefined) {{
                dioxus.send([0, null]);
            }};
            dioxus.send([0, ___result___]);
            "#,
            MODULE,
        );
        let mut eval = dioxus::document::eval(js.as_str());
        eval.send(map_id).map_err(dioxus_use_js::JsError::Eval)?;
        loop {
            let value = eval
                .recv::<dioxus_use_js::SerdeJsonValue>()
                .await
                .map_err(dioxus_use_js::JsError::Eval)?;
            match value {
                dioxus_use_js::SerdeJsonValue::Array(values) => {
                    if values.len() != 2 {
                        {
                            // ::core::panicking::panic_fmt(
                            //     format_args!(
                            //         "internal error: entered unreachable code: {0}",
                            //         format_args!("{0}", dioxus_use_js::__SEND_VALIDATION_MSG),
                            //     ),
                            // );
                            panic!("internal error: entered unreachable code: {0}", dioxus_use_js::__SEND_VALIDATION_MSG);
                        }
                    }
                    let mut iter = values.into_iter();
                    let action_ = match iter.next().unwrap() {
                        dioxus_use_js::SerdeJsonValue::Number(action_) => action_,
                        _ => {
                            // ::core::panicking::panic_fmt(
                            //     format_args!(
                            //         "internal error: entered unreachable code: {0}",
                            //         format_args!("{0}", dioxus_use_js::__INDEX_VALIDATION_MSG),
                            //     ),
                            // );
                            panic!("internal error: entered unreachable code: {0}", dioxus_use_js::__INDEX_VALIDATION_MSG);
                        }
                    };
                    let value = iter.next().unwrap();
                    match action_.as_u64().expect(dioxus_use_js::__INDEX_VALIDATION_MSG) {
                        0 => {
                            return dioxus_use_js::serde_json_from_value(value)
                                .map_err(|e| {
                                    dioxus_use_js::JsError::Eval(
                                        dioxus::document::EvalError::Serialization(e),
                                    )
                                })
                                .and_then(|_: dioxus_use_js::SerdeJsonValue| {
                                    // if #[allow(non_exhaustive_omitted_patterns)]
                                    // match e {
                                    //     dioxus_use_js::SerdeJsonValue::Null => true,
                                    //     _ => false,
                                    // } {
                                    //     Ok(())
                                    // } else {
                                        Err(
                                            dioxus_use_js::JsError::Eval(
                                                dioxus::document::EvalError::Serialization(
                                                    <dioxus_use_js::SerdeJsonError as dioxus_use_js::SerdeDeError>::custom(
                                                        dioxus_use_js::__BAD_VOID_RETURN.to_owned(),
                                                    ),
                                                ),
                                            ),
                                        )
                                    // }
                                });
                        }
                        1u64 => {
                            let value = dioxus_use_js::serde_json_from_value(value)
                                .map_err(|e| {
                                    dioxus_use_js::JsError::Eval(
                                        dioxus::document::EvalError::Serialization(e),
                                    )
                                })?;
                            let value = match callback(value).await {
                                Ok(value) => value,
                                Err(error) => {
                                    return Err(dioxus_use_js::JsError::Callback(error));
                                }
                            };
                            eval.send(dioxus_use_js::SerdeJsonValue::Null)
                                .map_err(dioxus_use_js::JsError::Eval)?;
                        }
                        _ => {
                            // ::core::panicking::panic_fmt(
                            //     format_args!(
                            //         "internal error: entered unreachable code: {0}",
                            //         format_args!("{0}", dioxus_use_js::__BAD_CALL_MSG),
                            //     ),
                            // );
                            panic!("internal error: entered unreachable code: {0}", dioxus_use_js::__BAD_CALL_MSG);
                        }
                    }
                }
                _ => {
                    // ::core::panicking::panic_fmt(
                    //     format_args!(
                    //         "internal error: entered unreachable code: {0}",
                    //         format_args!("{0}", dioxus_use_js::__SEND_VALIDATION_MSG),
                    //     ),
                    // );
                    panic!("internal error: entered unreachable code: {0}", dioxus_use_js::__SEND_VALIDATION_MSG);
                }
            }
        }
    }
}

fn js_to_eval(err: JsError) -> Box<dyn Error + Send + Sync> {
    match err {
        JsError::Eval(err) => err.into(),
        _ => panic!("Unexpected JsError variant"),
    }
}

pub async fn update_map<'a>(
    id: &Id, initial_position: &MapPosition, options: &MapOptions,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::update_map(id, initial_position, options).await.map_err(js_to_eval)
}

pub async fn delete_map<'a>(id: &Id) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::delete_map(id).await.map_err(js_to_eval)
}

pub async fn on_map_click(
    mut test: WriteSignal<bool>,
    map_id: &Id,
    event_handler: EventHandler<LatLng>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::on_map_click(map_id, async |coords| {
        test.set(true);
        //event_handler(LatLng::new(coords[0], coords[1]));
        Ok(())
    }).await.map_err(|e| e.into())
}

pub async fn update_marker(
    marker_id: &Id,
    coordinate: &LatLng,
    icon: &Option<MarkerIcon>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::update_marker(marker_id.parent().unwrap(), marker_id.id(), coordinate, icon).await.map_err(js_to_eval)
}

pub async fn delete_marker(marker_id: &Id) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::delete_marker(marker_id.parent().unwrap(), marker_id.id()).await.map_err(js_to_eval)
}

pub async fn update_polygon(
    polygon_id: &Id,
    coordinates: &Vec<Vec<Vec<LatLng>>>,
    options: &PathOptions,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::update_polygon(polygon_id.parent().unwrap(), polygon_id.id(), coordinates, options).await.map_err(js_to_eval)
}

pub async fn delete_polygon(polygon_id: &Id) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::delete_polygon(polygon_id.parent().unwrap(), polygon_id.id()).await.map_err(js_to_eval)
}

pub async fn update_popup(
    popup_id: &Id,
    options: &PopupOptions,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let marker_id = popup_id.parent().unwrap();
    js_api::update_popup(marker_id, popup_id, options).await.map_err(js_to_eval)
}