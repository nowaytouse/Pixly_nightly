#!/usr/bin/env python3
"""
测试优化后的AI服务性能
"""
import time
import json
import requests
from pathlib import Path

def test_optimizations():
    print("="*60)
    print("🧪 测试AI服务优化效果")
    print("="*60)
    
    base_url = "http://localhost:50052"
    
    # 1. 测试健康检查
    try:
        r = requests.get(f"{base_url}/api/v1/health", timeout=2)
        health = r.json()
        print(f"✅ 服务状态: {health['status']}")
        print(f"   功能: {health.get('features', {})}")
    except:
        print("❌ 服务未运行")
        return False
    
    # 2. 测试单个预测（会触发缓存和缩放）
    test_images = [
        "/Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/@reference/data/gbrp.png",
        "/Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/@reference/data/blackwhite.png",
        "/Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/@reference/data/four-colors.png"
    ]
    
    print("\n📊 单个预测测试（测试缩放和缓存）:")
    
    # 第一次调用（无缓存）
    for img_path in test_images[:1]:
        if Path(img_path).exists():
            start = time.time()
            r = requests.post(f"{base_url}/api/v1/predict", 
                json={
                    "image_path": img_path,
                    "target_format": "avif",
                    "mode": "balanced"
                },
                timeout=10
            )
            elapsed = time.time() - start
            
            if r.status_code == 200:
                result = r.json()
                print(f"   首次预测: {elapsed:.2f}s")
                if 'cached' in result:
                    print(f"   缓存状态: {'命中' if result['cached'] else '未命中'}")
            
            # 第二次调用（有缓存）
            start = time.time()
            r = requests.post(f"{base_url}/api/v1/predict", 
                json={
                    "image_path": img_path,
                    "target_format": "avif",
                    "mode": "balanced"
                },
                timeout=10
            )
            elapsed = time.time() - start
            
            if r.status_code == 200:
                result = r.json()
                print(f"   缓存预测: {elapsed:.2f}s")
                if 'cached' in result:
                    print(f"   缓存状态: {'✅ 命中' if result['cached'] else '❌ 未命中'}")
            break
    
    # 3. 测试批量并发预测
    print("\n📊 批量并发预测测试:")
    
    batch_request = {
        "images": [
            {
                "image_path": img,
                "target_format": "avif",
                "mode": "balanced"
            }
            for img in test_images if Path(img).exists()
        ],
        "concurrent": True
    }
    
    if batch_request["images"]:
        # 并发处理
        start = time.time()
        r = requests.post(f"{base_url}/api/v1/predict/batch",
            json=batch_request,
            timeout=30
        )
        elapsed = time.time() - start
        
        if r.status_code == 200:
            result = r.json()
            print(f"   并发处理 {result['total']} 个图像: {elapsed:.2f}s")
            print(f"   成功: {result['successful']}, 失败: {result['failed']}")
            print(f"   平均时间: {elapsed/result['total']:.2f}s/图像")
        
        # 串行处理对比
        batch_request["concurrent"] = False
        start = time.time()
        r = requests.post(f"{base_url}/api/v1/predict/batch",
            json=batch_request,
            timeout=30
        )
        elapsed = time.time() - start
        
        if r.status_code == 200:
            result = r.json()
            print(f"\n   串行处理 {result['total']} 个图像: {elapsed:.2f}s")
            print(f"   平均时间: {elapsed/result['total']:.2f}s/图像")
    
    print("\n✅ 优化测试完成!")
    print("="*60)
    return True

if __name__ == '__main__':
    test_optimizations()
