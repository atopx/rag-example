# rag-example

A RAG (Retrieval-Augmented Generation) example project. It builds a knowledge base by processing PDF documents and provides intelligent Q&A functionality.

## Features
- PDF document loading and chunking
- Text vectorization and storage (using Qdrant)
- Intelligent Q&A with dynamic context retrieval

## Dependencies
- Rust 1.75+
- OpenAI API key (for embedding model and LLM)
- Qdrant vector database (requires URL and API key)

## Installation & Run
1. Clone the repository: `git clone https://github.com/atopx/rag-example.git`
2. Install dependencies: `cargo build --release`
3. Configure environment variables:
   - `OPENAI_API_KEY`: OpenAI API key
   - `OPENAI_BASE_URL`: OpenAI API endpoint (optional, default: https://api.openai.com)
   - `QDRANT_URL`: Qdrant service URL (e.g., http://localhost:6333)
   - `QDRANT_API_KEY`: Qdrant API key (leave empty if not required)
4. Run the program: `cargo run`

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.