# Go AI Service - Deprecated (2025-11-11)

## 废弃原因

**Phase 47: 架构简化**

- **问题**: Go代码11151行，仅作HTTP网关和Python桥接，功能单一
- **决策**: 简化为双端架构（Rust + Python）
- **替代**: Python HTTP服务（tools/pixly_http_server.py）

## 代码统计

- **总行数**: 11151行
- **文件数**: 66个（其中33个活跃）
- **主要功能**: HTTP API网关、Python桥接、消息传递

## 迁移路径

### 原Go服务端点
- `GET /api/v1/health`
- `POST /api/v1/predict`
- `POST /api/v1/predict/video`

### 新Python服务端点
- `GET /api/v1/health` - 健康检查
- `POST /api/v1/predict` - 图像AI预测
- `POST /api/v1/predict/video` - 视频AI预测

**100%API兼容**，无需修改Rust客户端代码。

## 备份说明

- **备份时间**: 2025-11-11 13:20
- **备份位置**: `core/@deprecated/go_ai_service_2025_11_11/`
- **保留期限**: 1-2周观察期
- **回滚**: 如有问题可快速恢复

## 收益

- ✅ 代码量: **-11151行**（-35%）
- ✅ 语言数: 3种 → **2种**
- ✅ 维护成本: **-33%**
- ✅ 部署: 简化（仅需Python可选服务）

---

**废弃日期**: 2025-11-11  
**负责人**: Phase 47 架构简化  
**状态**: 已完成，可回滚
