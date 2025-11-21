#!/usr/bin/env python3
"""
ML模型自动评估系统
定期评估模型性能并生成报告
"""

import json
import sys
from pathlib import Path
from datetime import datetime
import numpy as np

def load_model_results():
    """加载最新的评估结果"""
    models_dir = Path("models")
    eval_files = sorted(models_dir.glob("evaluation_results_*.json"))
    
    if not eval_files:
        return None
    
    latest = eval_files[-1]
    with open(latest) as f:
        return json.load(f)

def analyze_performance(results):
    """分析模型性能"""
    if not results or "models" not in results:
        return None
    
    models = results["models"]
    analysis = {
        "best_quality_model": None,
        "best_effort_model": None,
        "fastest_model": None,
        "recommendations": []
    }
    
    # 找出最佳模型
    best_quality_acc = 0
    best_effort_acc = 0
    fastest_time = float('inf')
    
    for name, metrics in models.items():
        # 质量准确度
        if metrics["accuracy_quality"] > best_quality_acc:
            best_quality_acc = metrics["accuracy_quality"]
            analysis["best_quality_model"] = name
        
        # Effort准确度
        if metrics["accuracy_effort"] > best_effort_acc:
            best_effort_acc = metrics["accuracy_effort"]
            analysis["best_effort_model"] = name
        
        # 推理速度
        if metrics["inference_time_ms"] < fastest_time:
            fastest_time = metrics["inference_time_ms"]
            analysis["fastest_model"] = name
    
    # 生成建议
    if best_quality_acc < 0.5:
        analysis["recommendations"].append({
            "level": "warning",
            "message": f"Quality accuracy is low ({best_quality_acc:.1%}). Consider retraining with more diverse data."
        })
    
    if best_effort_acc < 0.5:
        analysis["recommendations"].append({
            "level": "warning",
            "message": f"Effort accuracy is low ({best_effort_acc:.1%}). Model may need more training samples."
        })
    
    if fastest_time > 50:
        analysis["recommendations"].append({
            "level": "info",
            "message": f"Inference time is {fastest_time:.1f}ms. Consider model optimization."
        })
    
    # 推荐最佳模型
    if analysis["best_quality_model"] == analysis["fastest_model"]:
        analysis["recommendations"].append({
            "level": "success",
            "message": f"Model '{analysis['best_quality_model']}' is both accurate and fast. Recommended for production."
        })
    
    return analysis

def generate_report(results, analysis):
    """生成评估报告"""
    print("=" * 60)
    print("🤖 MLModel性能评估报告")
    print("=" * 60)
    print(f"评估时间: {results['timestamp']}")
    print()
    
    print("📊 Model性能对比:")
    print("-" * 60)
    
    for name, metrics in results["models"].items():
        print(f"\n{name.upper()}:")
        print(f"  Quality准确度: {metrics['accuracy_quality']:.1%}")
        print(f"  Effort准确度:  {metrics['accuracy_effort']:.1%}")
        print(f"  Quality MAE:   {metrics['mae_quality']:.2f}")
        print(f"  Effort MAE:    {metrics['mae_effort']:.2f}")
        print(f"  推理时间:      {metrics['inference_time_ms']:.2f}ms")
        print(f"  Average置信度:    {metrics['avg_confidence']:.1%}")
    
    print("\n" + "=" * 60)
    print("🎯 AnalyzingResult:")
    print("-" * 60)
    
    if analysis:
        print(f"最佳QualityModel: {analysis['best_quality_model']}")
        print(f"最佳EffortModel: {analysis['best_effort_model']}")
        print(f"最快Model: {analysis['fastest_model']}")
        
        if analysis["recommendations"]:
            print("\n💡 建议:")
            for rec in analysis["recommendations"]:
                icon = {"warning": "⚠️", "info": "ℹ️", "success": "✅"}.get(rec["level"], "•")
                print(f"  {icon} {rec['message']}")
    
    print("=" * 60)

def main():
    print("🔍 LoadingModel评估Result...")
    results = load_model_results()
    
    if not results:
        print("❌ 未找到评估Resultfiles")
        print("   请先运行: python3 scripts/test_ml_all_formats.py")
        return 1
    
    print("✅ 评估Resultloaded")
    
    print("\n📈 AnalyzingModel性能...")
    analysis = analyze_performance(results)
    
    print("\n" + "=" * 60)
    generate_report(results, analysis)
    
    # 保存分析结果
    output_file = Path("models") / "performance_analysis.json"
    with open(output_file, 'w') as f:
        json.dump({
            "timestamp": datetime.now().isoformat(),
            "evaluation_results": results,
            "analysis": analysis
        }, f, indent=2)
    
    print(f"\n💾 AnalyzingResultsaved: {output_file}")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
