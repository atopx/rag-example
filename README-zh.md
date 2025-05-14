# rag-example

一个基于RAG（检索增强生成）示例项目，通过处理PDF文档构建知识库，提供智能问答功能。

## 功能特性
- PDF文档加载与分块处理
- 文本向量化与向量存储（使用Qdrant）
- 动态上下文检索的智能问答

## 依赖要求
- Rust 1.75+
- OpenAI API 密钥（用于嵌入模型和LLM）
- Qdrant 向量数据库（需提供URL和API密钥）

## 安装与运行
1. 克隆仓库：`git clone https://github.com/atopx/rag-example.git`
2. 安装依赖：`cargo build --release`
3. 配置环境变量：
   - `OPENAI_API_KEY`：OpenAI API 密钥
   - `OPENAI_BASE_URL`：OpenAI API 地址（可选，默认https://api.openai.com）
   - `QDRANT_URL`：Qdrant 服务URL（如http://localhost:6333）
   - `QDRANT_API_KEY`：Qdrant API 密钥（如无则留空）
4. 运行程序：`cargo run`

## 许可
本项目采用MIT许可协议，详见[LICENSE](LICENSE)文件。