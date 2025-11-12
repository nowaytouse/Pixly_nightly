# 🤖 PIXLY Go AI Service

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://golang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](../../LICENSE)

AI-powered decision service that handles intelligent quality prediction, format recommendation, and parameter optimization.

---

## 📋 Overview

The Go Service is PIXLY's **AI decision core**. It:
- Runs LightGBM ML models for quality prediction
- Provides PPO reinforcement learning optimization
- Recommends optimal formats based on image analysis
- Validates conversion quality with SSIM/VMAF
- Exposes RESTful HTTP API for the plugin

**Architecture**: Go + LightGBM + PPO + SSIM/VMAF validation

---

## ✨ Features

### AI Capabilities
- 🧠 **LightGBM Quality Prediction** - ML-based quality parameter selection
- 🎯 **PPO Reinforcement Learning** - Continuous optimization of decision accuracy
- 📊 **SSIM Validation** - Structural similarity verification
- 🎬 **VMAF Assessment** - Video quality evaluation
- 🔧 **Format Recommendation** - Intelligent format selection

### Processing Modes
- **Smart Mode**: AI-powered prediction with validation
- **Balanced Mode**: ML + SSIM verification (recommended)
- **Quality Mode**: Deep AI + PPO + multi-metric validation

---

## 🚀 Quick Start

### Prerequisites
```bash
# Install Go 1.21+
brew install go

# Install dependencies
go mod download
```

### Running the Service
```bash
# Development mode
go run main.go

# Production build
go build -o pixly-ai-service
./pixly-ai-service
```

### Default Configuration
- **Port**: `3001`
- **API Base**: `http://localhost:3001`
- **Health Check**: `GET /health`

---

## 📡 API Endpoints

### Health Check
```bash
GET /health
```

### Quality Prediction
```bash
POST /predict-quality
Content-Type: application/json

{
  "imagePath": "/path/to/image.jpg",
  "targetFormat": "jxl",
  "optimizeMode": "balanced"
}
```

**Response**:
```json
{
  "quality": 85,
  "effort": 7,
  "confidence": 0.92,
  "recommendation": {
    "format": "jxl",
    "reason": "Best compression for this image type"
  }
}
```

### Format Recommendation
```bash
POST /recommend-format
Content-Type: application/json

{
  "imagePath": "/path/to/image.jpg",
  "constraints": {
    "maxSize": 1048576,
    "preserveTransparency": true
  }
}
```

### SSIM Validation
```bash
POST /validate-ssim
Content-Type: application/json

{
  "originalPath": "/path/to/original.jpg",
  "convertedPath": "/path/to/converted.jxl",
  "threshold": 0.95
}
```

---

## 🏗️ Architecture

### Components

```
┌─────────────────────────────────────────┐
│         Go AI Service (Port 3001)        │
├─────────────────────────────────────────┤
│  ┌──────────────────────────────────┐   │
│  │   LightGBM Model                 │   │
│  │   - Quality Prediction           │   │
│  │   - Feature Extraction           │   │
│  └──────────────────────────────────┘   │
│  ┌──────────────────────────────────┐   │
│  │   PPO Optimizer                  │   │
│  │   - Reinforcement Learning       │   │
│  │   - Decision Refinement          │   │
│  └──────────────────────────────────┘   │
│  ┌──────────────────────────────────┐   │
│  │   SSIM/VMAF Validator            │   │
│  │   - Quality Verification         │   │
│  │   - Metric Calculation           │   │
│  └──────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

### Data Flow
1. **Plugin** sends prediction request
2. **Go Service** analyzes image features
3. **LightGBM** predicts optimal quality
4. **PPO** refines decision
5. **Validator** verifies result quality
6. Response returned to plugin

---

## 📁 Project Structure

```
core/go/
├── main.go                 # Service entry point
├── handlers/               # HTTP request handlers
│   ├── predict.go         # Quality prediction
│   ├── recommend.go       # Format recommendation
│   └── validate.go        # Quality validation
├── ml/                     # Machine learning
│   ├── lightgbm.go        # LightGBM integration
│   ├── ppo.go             # PPO optimizer
│   └── features.go        # Feature extraction
├── models/                 # Trained ML models
│   ├── quality_predictor.txt
│   └── ppo_weights.bin
├── validators/             # Quality validation
│   ├── ssim.go            # SSIM calculator
│   └── vmaf.go            # VMAF evaluator
└── utils/                  # Utilities
    ├── logger.go
    └── config.go
```

---

## 🔧 Configuration

### Environment Variables
```bash
# Service configuration
AI_SERVICE_PORT=3001
AI_LOG_LEVEL=info

# Model paths
LIGHTGBM_MODEL_PATH=./models/quality_predictor.txt
PPO_WEIGHTS_PATH=./models/ppo_weights.bin

# Validation thresholds
SSIM_THRESHOLD=0.95
VMAF_THRESHOLD=85
```

### Config File (`config.yaml`)
```yaml
service:
  port: 3001
  timeout: 30s

models:
  lightgbm: "./models/quality_predictor.txt"
  ppo: "./models/ppo_weights.bin"

validation:
  ssim_threshold: 0.95
  vmaf_threshold: 85
  enable_auto_retry: true
```

---

## 🧪 Testing

### Run Tests
```bash
# Unit tests
go test ./...

# With coverage
go test -cover ./...

# Specific package
go test ./ml/
```

### Integration Tests
```bash
# Start service
go run main.go &

# Run integration tests
go test -tags=integration ./tests/

# Stop service
pkill pixly-ai-service
```

---

## 📊 ML Models

### LightGBM Quality Predictor
- **Input Features**: Image dimensions, complexity, format, color space
- **Output**: Quality value (0-100), effort level (1-9)
- **Accuracy**: ~92% on validation set

### PPO Optimizer
- **Training**: Reinforcement learning on conversion results
- **Reward**: File size reduction + SSIM maintenance
- **Updates**: Continuous learning from user conversions

---

## 🐛 Debugging

### Enable Debug Logs
```bash
export AI_LOG_LEVEL=debug
go run main.go
```

### Test Endpoints
```bash
# Health check
curl http://localhost:3001/health

# Predict quality
curl -X POST http://localhost:3001/predict-quality \
  -H "Content-Type: application/json" \
  -d '{"imagePath": "test.jpg", "targetFormat": "jxl"}'
```

---

## 📝 Development

### Adding New Features
1. Create handler in `handlers/`
2. Implement business logic
3. Add tests
4. Update API documentation
5. Register route in `main.go`

### Model Updates
1. Train new model
2. Export to compatible format
3. Place in `models/` directory
4. Update config paths
5. Restart service

---

## 🤝 Integration

### With Rust Service
```go
// Call Rust service for conversion
resp, err := http.Post("http://localhost:3000/convert", ...)
```

### With Plugin (JS)
```javascript
// Call Go service from plugin
const response = await fetch('http://localhost:3001/predict-quality', {
  method: 'POST',
  body: JSON.stringify({
    imagePath: '/path/to/image.jpg',
    targetFormat: 'jxl'
  })
});
```

---

## 📚 Resources

- [LightGBM Documentation](https://lightgbm.readthedocs.io/)
- [Go Documentation](https://golang.org/doc/)
- [PPO Algorithm](https://arxiv.org/abs/1707.06347)
- [SSIM Index](https://en.wikipedia.org/wiki/Structural_similarity)

---

## 📄 License

MIT License - see [LICENSE](../../LICENSE) for details

---

## 🔗 Related Services

- **[Rust Conversion Service](../rust/)** - Handles actual conversions
- **[Plugin (JS)](../plugin/)** - User interface in Eagle

---

**Version**: 1.0.0  
**Last Updated**: 2025-11-09  
**Maintainer**: PIXLY Team
