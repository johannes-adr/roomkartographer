use crate::ScanLog;



pub fn convert_to_scans(src: &str) -> Vec<ScanLog>{
    let res: serde_json::Value = serde_json::from_str(src).unwrap();
    let mut ret = vec![];
    for entry in res.as_array().unwrap(){
        if entry.get("tag").unwrap() != "LidarScan"{
           continue;
        }

        let mut pts_transformed = vec![];
        let pts = entry.get("val").unwrap().as_object().unwrap().get("points").unwrap().as_array().unwrap();

        for pt in pts{
            let x = pt.get("x").unwrap().as_f64().unwrap() as f32;
            let y = pt.get("y").unwrap().as_f64().unwrap() as f32;
            pts_transformed.push((x,y))
        }

        ret.push(ScanLog{points: pts_transformed});
    }
    ret
}