/*!
桥接模块
提供与外部系统（Python AI服务等）的桥接功能
*/

pub mod python_bridge;

pub use python_bridge::{
    PythonBridge,
    PythonBridgeConfig,
    get_python_bridge,
    predict_image_params,
    predict_audio_params,
    predict_video_params,
};
