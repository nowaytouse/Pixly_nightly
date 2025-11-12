package ai

import (
	"context"
	"fmt"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	pb "pixly/ai/pb"
)

// PredictionMode 预测模式
type PredictionMode string

const (
	ModeBasic    PredictionMode = "basic"
	ModeAdvanced PredictionMode = "advanced"
	ModeHybrid   PredictionMode = "hybrid"
)

// Client AI客户端
type Client struct {
	conn   *grpc.ClientConn
	client pb.AIPredictorClient
	mode   PredictionMode
	
	// 配置
	serverAddr     string
	timeout        time.Duration
	enableBayesian bool
}

// NewClient 创建AI客户端
func NewClient(serverAddr string) (*Client, error) {
	// 连接选项
	opts := []grpc.DialOption{
		grpc.WithTransportCredentials(insecure.NewCredentials()),
		grpc.WithBlock(),
		grpc.WithTimeout(3 * time.Second),
	}
	
	// 连接服务器
	conn, err := grpc.Dial(serverAddr, opts...)
	if err != nil {
		return nil, fmt.Errorf("连接AI服务失败: %w", err)
	}
	
	client := &Client{
		conn:           conn,
		client:         pb.NewAIPredictorClient(conn),
		mode:           ModeHybrid, // 默认混合模式
		serverAddr:     serverAddr,
		timeout:        30 * time.Second,
		enableBayesian: false,
	}
	
	// 健康检查
	if err := client.healthCheck(); err != nil {
		conn.Close()
		return nil, fmt.Errorf("AI服务健康检查失败: %w", err)
	}
	
	logging.Info("✅ AI客户端已连接: %s", serverAddr)
	
	return client, nil
}

// SetMode 设置预测模式
func (c *Client) SetMode(mode PredictionMode) {
	c.mode = mode
	logging.Info("🔄 切换预测模式: %s", mode)
}

// EnableBayesianOptimization 启用/禁用贝叶斯优化
func (c *Client) EnableBayesianOptimization(enable bool) {
	c.enableBayesian = enable
}

// Predict 预测最优参数
func (c *Client) Predict(imageInfo *ImageInfo, tool string) (*PredictResult, error) {
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()
	
	startTime := time.Now()
	
	// 构建请求
	req := &pb.PredictRequest{
		Mode: c.modeToProto(c.mode),
		Image: &pb.ImageInfo{
			FilePath:   imageInfo.FilePath,
			Width:      int32(imageInfo.Width),
			Height:     int32(imageInfo.Height),
			Size:       imageInfo.Size,
			Format:     imageInfo.Format,
			IsAnimated: imageInfo.IsAnimated,
			HasAlpha:   imageInfo.HasAlpha,
		},
		Tool: tool,
		Options: &pb.PredictOptions{
			TargetFormat:    imageInfo.TargetFormat,
			EnableBayesian:  c.enableBayesian,
			MaxIterations:   20,
			EnableSsimCheck: true,
			SsimThreshold:   0.95,
		},
	}
	
	// 调用 gRPC
	resp, err := c.client.Predict(ctx, req)
	if err != nil {
		return nil, fmt.Errorf("AI预测失败: %w", err)
	}
	
	elapsed := time.Since(startTime)
	
	// 转换结果
	result := c.protoToResult(resp)
	result.ClientElapsedMs = elapsed.Milliseconds()
	
	log.Printf("✅ AI预测完成 [%s模式, %.1fms客户端, %.1fms服务端]",
		c.mode, float64(elapsed.Milliseconds()), resp.PredictionTimeMs)
	log.Printf("   推荐参数: quality=%d, effort=%d",
		result.Params.CjxlQuality, result.Params.CjxlEffort)
	
	return result, nil
}

// PredictBatch 批量预测
func (c *Client) PredictBatch(images []*ImageInfo, tool string) ([]*PredictResult, error) {
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout*time.Duration(len(images)))
	defer cancel()
	
	// 构建批量请求
	pbImages := make([]*pb.ImageInfo, len(images))
	for i, img := range images {
		pbImages[i] = &pb.ImageInfo{
			FilePath:   img.FilePath,
			Width:      int32(img.Width),
			Height:     int32(img.Height),
			Size:       img.Size,
			Format:     img.Format,
			IsAnimated: img.IsAnimated,
			HasAlpha:   img.HasAlpha,
		}
	}
	
	req := &pb.PredictBatchRequest{
		Mode:   c.modeToProto(c.mode),
		Images: pbImages,
		Tool:   tool,
		Options: &pb.PredictOptions{
			EnableBayesian:  c.enableBayesian,
			MaxIterations:   10, // 批量时减少迭代
			EnableSsimCheck: false,
		},
	}
	
	// 调用 gRPC
	resp, err := c.client.PredictBatch(ctx, req)
	if err != nil {
		return nil, fmt.Errorf("批量预测失败: %w", err)
	}
	
	// 转换结果
	results := make([]*PredictResult, len(resp.Results))
	for i, r := range resp.Results {
		results[i] = c.protoToResult(r)
	}
	
	log.Printf("✅ 批量预测完成: %d成功, %d失败, 总耗时%.1fms",
		resp.SuccessCount, resp.FailedCount, resp.TotalTimeMs)
	
	return results, nil
}

// AssessQuality 评估质量
func (c *Client) AssessQuality(originalPath, compressedPath string) (*QualityMetrics, error) {
	ctx, cancel := context.WithTimeout(context.Background(), c.timeout)
	defer cancel()
	
	req := &pb.AssessRequest{
		OriginalPath:   originalPath,
		CompressedPath: compressedPath,
	}
	
	resp, err := c.client.AssessQuality(ctx, req)
	if err != nil {
		return nil, fmt.Errorf("质量评估失败: %w", err)
	}
	
	metrics := &QualityMetrics{
		SSIM:              resp.Ssim,
		PSNR:              resp.Psnr,
		PerceptualQuality: resp.PerceptualQuality,
		CompressionRatio:  resp.CompressionRatio,
		OriginalSize:      resp.OriginalSize,
		CompressedSize:    resp.CompressedSize,
	}
	
	log.Printf("📊 质量评估: SSIM=%.4f, PSNR=%.2fdB, 压缩率=%.1f%%",
		metrics.SSIM, metrics.PSNR, (1-metrics.CompressionRatio)*100)
	
	return metrics, nil
}

// Close 关闭连接
func (c *Client) Close() error {
	if c.conn != nil {
		return c.conn.Close()
	}
	return nil
}

// healthCheck 健康检查
func (c *Client) healthCheck() error {
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	
	req := &pb.HealthCheckRequest{}
	resp, err := c.client.HealthCheck(ctx, req)
	if err != nil {
		return err
	}
	
	if !resp.IsHealthy {
		return fmt.Errorf("AI服务不健康")
	}
	
	log.Printf("✅ AI服务健康: version=%s, models=%v, uptime=%ds",
		resp.Version, resp.ModelsLoaded, resp.UptimeSeconds)
	
	return nil
}

// modeToProto 转换模式
func (c *Client) modeToProto(mode PredictionMode) pb.PredictRequest_Mode {
	switch mode {
	case ModeBasic:
		return pb.PredictRequest_BASIC
	case ModeAdvanced:
		return pb.PredictRequest_ADVANCED
	case ModeHybrid:
		return pb.PredictRequest_HYBRID
	default:
		return pb.PredictRequest_BASIC
	}
}

// protoToResult 转换结果
func (c *Client) protoToResult(resp *pb.PredictResponse) *PredictResult {
	return &PredictResult{
		Params: ToolParams{
			// cjxl
			CjxlQuality:     int(resp.Params.CjxlQuality),
			CjxlEffort:      int(resp.Params.CjxlEffort),
			CjxlDistance:    resp.Params.CjxlDistance,
			CjxlLossless:    resp.Params.CjxlLossless,
			CjxlModular:     resp.Params.CjxlModular,
			CjxlProgressive: resp.Params.CjxlProgressive,
			
			// avifenc
			AvifQuantizer:      int(resp.Params.AvifQuantizer),
			AvifQuantizerAlpha: int(resp.Params.AvifQuantizerAlpha),
			AvifSpeed:          int(resp.Params.AvifSpeed),
			
			// cwebp
			WebpQuality:   int(resp.Params.WebpQuality),
			WebpMethod:    int(resp.Params.WebpMethod),
			WebpLossless:  resp.Params.WebpLossless,
			
			// ffmpeg
			FfmpegCrf:    int(resp.Params.FfmpegCrf),
			FfmpegPreset: resp.Params.FfmpegPreset,
			FfmpegCodec:  resp.Params.FfmpegCodec,
		},
		EstimatedSize:        resp.EstimatedSize,
		EstimatedCompression: resp.EstimatedCompression,
		Confidence:           resp.Confidence,
		SSIMEstimate:         resp.SsimEstimate,
		ServerPredictionMs:   resp.PredictionTimeMs,
		ModeUsed:             resp.ModeUsed,
		Reason:               resp.Reason,
	}
}

// ==================== 数据结构 ====================

type ImageInfo struct {
	FilePath     string
	Width        int
	Height       int
	Size         int64
	Format       string
	IsAnimated   bool
	HasAlpha     bool
	TargetFormat string
}

type ToolParams struct {
	// cjxl
	CjxlQuality     int
	CjxlEffort      int
	CjxlDistance    float32
	CjxlLossless    bool
	CjxlModular     bool
	CjxlProgressive bool
	
	// avifenc
	AvifQuantizer      int
	AvifQuantizerAlpha int
	AvifSpeed          int
	
	// cwebp
	WebpQuality  int
	WebpMethod   int
	WebpLossless bool
	
	// ffmpeg
	FfmpegCrf    int
	FfmpegPreset string
	FfmpegCodec  string
}

type PredictResult struct {
	Params               ToolParams
	EstimatedSize        int64
	EstimatedCompression float32
	Confidence           float32
	SSIMEstimate         float32
	ServerPredictionMs   float32
	ClientElapsedMs      int64
	ModeUsed             string
	Reason               string
}

type QualityMetrics struct {
	SSIM              float32
	PSNR              float32
	PerceptualQuality float32
	CompressionRatio  float32
	OriginalSize      int64
	CompressedSize    int64
}
