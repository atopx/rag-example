use std::env;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Context;
use anyhow::Result;
use qdrant_client::Qdrant;
use qdrant_client::config::CompressionEncoding;
use qdrant_client::config::QdrantConfig;
use qdrant_client::qdrant::CreateCollectionBuilder;
use qdrant_client::qdrant::Distance;
use qdrant_client::qdrant::QueryPointsBuilder;
use qdrant_client::qdrant::VectorParamsBuilder;
use rig::Embed;
use rig::completion::Prompt;
use rig::embeddings::EmbeddingsBuilder;
use rig::loaders::PdfFileLoader;
use rig::providers::openai;
use rig_qdrant::QdrantVectorStore;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;

#[derive(Embed, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Document {
    id: String,
    #[embed]
    content: String,
}

const EMBED_MODEL: &str = "quentinz/bge-large-zh-v1.5:latest";
const LLM_MODEL: &str = "qwen3:4b";
const LOAD_CHUNK_SIZE: usize = 1000;
const VECTOR_COLLECTION: &str = "chessboard";
const VECTOR_DIMENSIONS: u64 = 1024;
fn load_pdf(path: PathBuf) -> Result<Vec<String>> {
    let content_chunks = PdfFileLoader::with_glob(path.to_str().context("Invalid path")?)?
        .read()
        .into_iter()
        .filter_map(|result| {
            result
                .map_err(|e| {
                    eprintln!("Error reading PDF content: {}", e);
                    e
                })
                .ok()
        })
        .flat_map(|content| {
            let mut chunks = Vec::new();
            let mut current = String::new();

            for word in content.split_whitespace() {
                if current.len() + word.len() + 1 > LOAD_CHUNK_SIZE && !current.is_empty() {
                    chunks.push(std::mem::take(&mut current).trim().to_string());
                }
                current.push_str(word);
                current.push(' ');
            }

            if !current.is_empty() {
                chunks.push(current.trim().to_string());
            }

            chunks
        })
        .collect::<Vec<_>>();

    if content_chunks.is_empty() {
        anyhow::bail!("No content found in PDF file: {}", path.display());
    }

    Ok(content_chunks)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_target(false).init();

    // 初始化 OpenAI 客户端
    let client = openai::Client::from_url(env::var("OPENAI_API_KEY")?.as_str(), env::var("OPENAI_BASE_URL")?.as_str());

    // 初始化 Qdrant 客户端
    let qdclient = Qdrant::new(QdrantConfig {
        uri: env::var("QDRANT_URL")?,
        timeout: Duration::from_secs(10),
        connect_timeout: Duration::from_secs(5),
        keep_alive_while_idle: true,
        api_key: Some(env::var("QDRANT_API_KEY")?),
        compression: Some(CompressionEncoding::Gzip),
        check_compatibility: false,
    })?;

    // 初始化 Qdrant collection
    if !qdclient.collection_exists(VECTOR_COLLECTION).await? {
        let builder = CreateCollectionBuilder::new(VECTOR_COLLECTION)
            .vectors_config(VectorParamsBuilder::new(VECTOR_DIMENSIONS, Distance::Cosine));
        qdclient.create_collection(builder).await?;
    }

    // 使用 Rig 的内置 PDF 加载器加载 PDF
    let documents_dir = std::env::current_dir()?.join("documents");

    println!("成功加载并分块 PDF 文档");

    // 创建嵌入模型
    let model = client.embedding_model(EMBED_MODEL);

    // 创建嵌入构建器
    let mut builder = EmbeddingsBuilder::new(model.clone());

    // PDF加载、分块、向量化
    let chessboard_chunks = load_pdf(documents_dir.join("chessboard.pdf")).context("加载 chessboard.pdf 失败")?;
    for (i, chunk) in chessboard_chunks.into_iter().enumerate() {
        builder = builder.document(Document { id: format!("chessboard_{}", i), content: chunk })?;
    }
    let documents = builder.build().await?;

    // 创建向量存储和索引
    let query_params = QueryPointsBuilder::new(VECTOR_COLLECTION).with_payload(true);
    let vector_store = QdrantVectorStore::new(qdclient, model, query_params.build());

    vector_store
        .insert_documents(documents)
        .await
        .map_err(|err| anyhow::anyhow!("Couldn't insert documents: {err}"))?;

    info!("成功创建向量存储和索引");

    // 创建 RAG 代理
    let rag_agent = client
        .agent(LLM_MODEL)
        .preamble("你是一位乐于助人的助手，能够根据提供的多个相关文档内容综合信息回答问题。/no_think")
        .dynamic_context(4, vector_store)
        .temperature(0.15)
        .build();

    let response = rag_agent.prompt("“中国象棋学习助手”的核心竞争力是什么？").await?;
    println!("{}", response);
    Ok(())
}
