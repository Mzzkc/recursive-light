# Phase 6: Advanced Search (Qdrant + CLIP + Embeddings)

**Phase Duration:** 2-3 weeks
**Phase Status:** ⬜ Not Started
**Started:** [DATE]
**Completed:** [DATE]
**Prerequisites:** Phase 5 (Frontend) complete

---

## 📋 Phase Overview

Implement the revolutionary 3-tier hybrid search system:
- **Meilisearch** (already integrated): 1-5ms exact-match text search
- **Qdrant** (NEW): <500ms semantic search + CLIP image similarity
- **Python Embeddings Service** (NEW): sentence-transformers + CLIP

**Search Types:**
1. **Exact Match:** "Alber's Cat Avatar v2" → Meilisearch only
2. **Semantic:** "cute pink cat girl" → Hybrid (Meilisearch + Qdrant)
3. **Image Search:** Screenshot upload → Qdrant CLIP only

**Goal:** Industry-leading search experience with text, semantic, and visual search capabilities.

---

## ✅ Milestone 6.1: Qdrant Cloud Setup

**Duration:** 2-3 days
**Status:** ⬜ Not Started

### Tasks

#### 6.1.1: Provision Qdrant Cluster
**Steps:**
1. Go to https://cloud.qdrant.io
2. Create account
3. Create cluster:
   - **Name:** naurva-vectors-us
   - **Region:** US East (same as backend)
   - **Plan:** Starter 1GB ($25/mo)
   - **Cluster:** Single node (can scale later)

4. Get credentials:
   - **URL:** https://xxxxx.us-east-1.aws.cloud.qdrant.io:6333
   - **API Key:** (copy from dashboard)

5. Store in `.env`:
```bash
QDRANT_URL=https://xxxxx.us-east-1.aws.cloud.qdrant.io:6333
QDRANT_API_KEY=your-api-key-here
```

**Acceptance Criteria:**
- Qdrant cluster running
- Accessible from backend
- Credentials secured

---

#### 6.1.2: Create Qdrant Collections
**Collections needed:**

**1. Product Text Embeddings**
```rust
use qdrant_client::qdrant::{CreateCollection, Distance, VectorParams};

async fn create_text_collection(client: &QdrantClient) -> Result<()> {
    client.create_collection(&CreateCollection {
        collection_name: "product_text_embeddings".to_string(),
        vectors_config: Some(VectorParams {
            size: 384, // sentence-transformers dimension
            distance: Distance::Cosine,
            ..Default::default()
        }.into()),
        ..Default::default()
    }).await?;
    Ok(())
}
```

**2. Product Image Embeddings**
```rust
async fn create_image_collection(client: &QdrantClient) -> Result<()> {
    client.create_collection(&CreateCollection {
        collection_name: "product_image_embeddings".to_string(),
        vectors_config: Some(VectorParams {
            size: 512, // CLIP ViT-B/32 dimension
            distance: Distance::Cosine,
            ..Default::default()
        }.into()),
        ..Default::default()
    }).await?;
    Ok(())
}
```

**Acceptance Criteria:**
- Both collections created
- Can insert/query vectors
- Distance metric configured

---

## ✅ Milestone 6.2: Python Embeddings Service

**Duration:** 1 week
**Status:** ⬜ Not Started

### Tasks

#### 6.2.1: Python Service Implementation
**Create:** `embeddings/main.py`

**Dependencies:**
```python
# requirements.txt
fastapi==0.104.1
uvicorn[standard]==0.24.0
sentence-transformers==2.2.2
torch==2.1.0
pillow==10.1.0
transformers==4.35.0
```

**FastAPI server:**
```python
from fastapi import FastAPI, UploadFile, File
from sentence_transformers import SentenceTransformer
import torch
from PIL import Image
import io

app = FastAPI()

# Load models (cached in memory)
text_model = SentenceTransformer('all-MiniLM-L6-v2')  # 384-dim
clip_model = SentenceTransformer('clip-ViT-B-32')     # 512-dim

@app.post("/embed/text")
async def embed_text(text: str):
    embedding = text_model.encode(text).tolist()
    return {"embedding": embedding}

@app.post("/embed/image")
async def embed_image(file: UploadFile = File(...)):
    image_data = await file.read()
    image = Image.open(io.BytesIO(image_data))
    embedding = clip_model.encode(image).tolist()
    return {"embedding": embedding}

@app.get("/health")
async def health():
    return {"status": "ok", "models_loaded": True}
```

**Deployment:**
```dockerfile
# embeddings/Dockerfile
FROM python:3.11-slim

WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY main.py .

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
```

**Kubernetes deployment:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: embeddings-service
  namespace: embeddings
spec:
  replicas: 2
  template:
    spec:
      containers:
      - name: embeddings
        image: ghcr.io/eatz-altf4/naurva/embeddings:latest
        resources:
          requests:
            memory: "2Gi"
            cpu: "1000m"
          limits:
            memory: "4Gi"
            cpu: "2000m"
```

**Acceptance Criteria:**
- Python service running in K8s
- Text embeddings working
- Image embeddings working
- <500ms response time

---

## ✅ Milestone 6.3: Hybrid Search Implementation

**Duration:** 4-5 days
**Status:** ⬜ Not Started

### Tasks

#### 6.3.1: Qdrant Rust Client Integration
**Add to `backend/Cargo.toml`:**
```toml
qdrant-client = "1.7"
```

**Create `backend/src/search/qdrant_client.rs`:**
```rust
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{SearchPoints, PointStruct};

#[derive(Clone)]
pub struct QdrantSearchClient {
    client: QdrantClient,
    embeddings_url: String,
}

impl QdrantSearchClient {
    pub async fn new(url: String, api_key: String, embeddings_url: String) -> Result<Self> {
        let client = QdrantClient::from_url(&url)
            .with_api_key(api_key)
            .build()?;

        Ok(QdrantSearchClient { client, embeddings_url })
    }

    pub async fn semantic_search(&self, query: &str, filter_nsfw: bool) -> Result<Vec<String>> {
        // Call embeddings service
        let embedding = self.get_text_embedding(query).await?;

        // Search Qdrant
        let mut search_request = SearchPoints {
            collection_name: "product_text_embeddings".to_string(),
            vector: embedding,
            limit: 20,
            with_payload: Some(true.into()),
            ..Default::default()
        };

        if filter_nsfw {
            search_request.filter = Some(Filter {
                must: vec![Condition {
                    field: "is_nsfw".to_string(),
                    r#match: Some(Match::Boolean(false)),
                }],
            });
        }

        let results = self.client.search_points(&search_request).await?;

        Ok(results.result.into_iter()
            .map(|p| p.payload.get("product_id").unwrap().to_string())
            .collect())
    }

    async fn get_text_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let res = reqwest::Client::new()
            .post(format!("{}/embed/text", self.embeddings_url))
            .json(&serde_json::json!({"text": text}))
            .send()
            .await?;

        let data: EmbeddingResponse = res.json().await?;
        Ok(data.embedding)
    }
}
```

**Acceptance Criteria:**
- Qdrant client connects
- Can perform searches
- Results returned correctly

---

#### 6.3.2: Query Classification
**Determine search strategy based on query:**

```rust
pub enum SearchStrategy {
    ExactOnly,      // "Alber's Cat v2" - specific names
    Semantic,       // "cute pink cat" - descriptive
    Hybrid,         // Mix of both
    ImageOnly,      // Screenshot upload
}

pub fn classify_query(query: &str, has_image: bool) -> SearchStrategy {
    if has_image {
        return SearchStrategy::ImageOnly;
    }

    // Simple heuristic (can improve with ML later)
    let words: Vec<&str> = query.split_whitespace().collect();
    let has_quotes = query.contains('"');
    let is_short = words.len() <= 3;
    let has_version = query.contains("v") || query.contains("version");

    if has_quotes || (is_short && has_version) {
        SearchStrategy::ExactOnly
    } else if words.len() > 5 {
        SearchStrategy::Semantic
    } else {
        SearchStrategy::Hybrid
    }
}
```

**Acceptance Criteria:**
- Classification logic works
- Routes queries appropriately
- Can be tuned/improved

---

#### 6.3.3: Result Merging & Deduplication
**Merge Meilisearch + Qdrant results:**

```rust
pub async fn hybrid_search(
    meili: &MeilisearchClient,
    qdrant: &QdrantSearchClient,
    query: &str,
    filter_nsfw: bool,
) -> Result<Vec<SearchResult>> {
    // Parallel search
    let (meili_results, qdrant_results) = tokio::join!(
        meili.search_products(query, filter_nsfw),
        qdrant.semantic_search(query, filter_nsfw)
    );

    let mut meili_ids: Vec<String> = meili_results?.into_iter()
        .map(|p| p.id.to_string())
        .collect();

    let qdrant_ids: Vec<String> = qdrant_results?;

    // Deduplicate (keep Meilisearch results, add unique Qdrant results)
    let meili_set: HashSet<String> = meili_ids.iter().cloned().collect();
    for id in qdrant_ids {
        if !meili_set.contains(&id) {
            meili_ids.push(id);
        }
    }

    Ok(meili_ids)
}
```

**Acceptance Criteria:**
- No duplicate results
- Meilisearch results prioritized
- Semantic results enrich, not replace

---

#### 6.3.4: Image Similarity Search
**Allow users to upload screenshots:**

**Frontend:**
```svelte
<script>
  let imageFile: File;

  async function searchByImage() {
    const formData = new FormData();
    formData.append('image', imageFile);

    const res = await fetch('/api/v1/search/image', {
      method: 'POST',
      body: formData
    });

    const results = await res.json();
    // Display results
  }
</script>

<input type="file" accept="image/*" bind:files={imageFile} />
<button on:click={searchByImage}>Search Similar</button>
```

**Backend endpoint (`api/search.rs`):**
```rust
pub async fn image_search(
    qdrant: web::Data<QdrantSearchClient>,
    mut payload: Multipart,
) -> impl Responder {
    // Extract image
    let mut image_data = Vec::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        while let Some(chunk) = field.next().await {
            image_data.extend_from_slice(&chunk?);
        }
    }

    // Get CLIP embedding
    let embedding = qdrant.get_image_embedding(&image_data).await?;

    // Search Qdrant
    let results = qdrant.search_by_vector(embedding).await?;

    HttpResponse::Ok().json(results)
}
```

**Acceptance Criteria:**
- Image upload works
- CLIP embeddings generated
- Similar products returned
- <2s end-to-end

---

## ✅ Milestone 6.4: Search Optimization

**Duration:** 2-3 days
**Status:** ⬜ Not Started

### Tasks

#### 6.4.1: Caching Layer
**Cache frequent searches:**
```rust
use redis::AsyncCommands;

pub async fn cached_search(
    redis: &redis::Client,
    query: &str,
    search_fn: impl Future<Output = Vec<SearchResult>>,
) -> Result<Vec<SearchResult>> {
    let cache_key = format!("search:{}", query);

    // Check cache
    let cached: Option<String> = redis.get(&cache_key).await?;
    if let Some(data) = cached {
        return Ok(serde_json::from_str(&data)?);
    }

    // Execute search
    let results = search_fn.await;

    // Cache for 5 minutes
    redis.setex(&cache_key, 300, serde_json::to_string(&results)?).await?;

    Ok(results)
}
```

**Acceptance Criteria:**
- Repeated searches instant
- Cache invalidates properly
- No stale results

---

#### 6.4.2: Search Analytics
**Track:**
- Query volume
- Search-to-click rate
- Zero-result queries
- Popular search terms

**Implementation:**
```rust
pub async fn log_search_event(
    pool: &PgPool,
    user_id: Option<Uuid>,
    query: String,
    strategy: SearchStrategy,
    result_count: i32,
) {
    sqlx::query(
        "INSERT INTO search_analytics (user_id, query, strategy, result_count)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(user_id)
    .bind(query)
    .bind(format!("{:?}", strategy))
    .bind(result_count)
    .execute(pool)
    .await
    .ok();
}
```

**Acceptance Criteria:**
- Search events tracked
- Can query analytics
- Privacy compliant

---

#### 6.4.3: Personalized Ranking
**Use purchase history for "Recommended for You":**

```rust
pub async fn personalized_ranking(
    qdrant: &QdrantSearchClient,
    user_id: Uuid,
    results: Vec<SearchResult>,
) -> Vec<SearchResult> {
    // Get user's owned products
    let owned = get_user_purchases(user_id).await;

    // Get similar products to what user owns
    let recommendations = qdrant.find_similar_to_owned(&owned).await;

    // Boost search results that match recommendations
    results.iter()
        .map(|r| {
            let boost = if recommendations.contains(&r.id) { 1.2 } else { 1.0 };
            SearchResult { score: r.score * boost, ..r.clone() }
        })
        .collect()
}
```

**Acceptance Criteria:**
- Personalized results work
- Owned products influence ranking
- 10% weight redistribution

---

## 📊 Phase 6 Success Criteria

**When Phase 6 is complete:**

✅ **3-Tier Search:**
- Meilisearch: 1-5ms exact match
- Qdrant: <500ms semantic search
- CLIP: Image similarity search

✅ **Query Intelligence:**
- Auto-classifies query type
- Routes to optimal strategy
- Merges results intelligently

✅ **Performance:**
- <600ms hybrid search
- Cached repeated queries
- Analytics tracking

---

## 💰 Cost Impact

**New monthly costs:**
- Qdrant Cloud: $25/mo (US region)
- Total added: $25/mo

**Total infrastructure:** ~$182/mo

---

**Last Updated:** October 19, 2025
