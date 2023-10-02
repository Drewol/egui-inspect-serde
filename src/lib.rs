use serde_json::Value;

pub fn inspect<T: serde::Serialize>(value: &T, ui: &mut egui::Ui) -> Result<(), serde_json::Error> {
    let mut jobj = serde_json::to_value(value)?;
    json_widget(&mut jobj, ui, "");
    Ok(())
}

pub fn inspect_mut<T: serde::Serialize + serde::de::DeserializeOwned>(
    value: &mut T,
    ui: &mut egui::Ui,
) -> Result<(), serde_json::Error> {
    let mut jobj = serde_json::to_value(&value)?;
    json_widget(&mut jobj, ui, "");

    *value = serde_json::from_value(jobj)?;

    Ok(())
}

fn json_widget(jobj: &mut Value, ui: &mut egui::Ui, name: &str) {
    match jobj {
        Value::Array(valvec) => {
            ui.collapsing(name, |ui| {
                for v in valvec {
                    json_widget(v, ui, "");
                }
            });
        }
        Value::Object(children) => {
            ui.collapsing(name, |ui| {
                for (name, v) in children {
                    json_widget(v, ui, name);
                }
            });
        }
        Value::Bool(v) => {
            ui.checkbox(v, name);
        }
        Value::Number(v) => {
            ui.horizontal(|ui| {
                if v.is_i64() {
                    let mut value = v.as_i64().unwrap();
                    ui.label(name);
                    ui.add(egui::widgets::DragValue::new(&mut value));
                    *v = serde_json::Number::from(value);
                } else if v.is_f64() {
                    let mut value = v.as_f64().unwrap();
                    ui.label(name);
                    ui.add(egui::widgets::DragValue::new(&mut value));
                    *v = serde_json::Number::from_f64(value).unwrap();
                }
            });
        }
        Value::String(v) => {
            ui.horizontal(|ui| {
                ui.label(name);
                ui.text_edit_singleline(v);
            });
        }
        Value::Null => {}
    }
}
