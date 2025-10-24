# Frontend & Backend Implementation
*Understanding emerges at recognition interfaces*

## System Architecture

Three-tier architecture with clear separation of concerns:

```
┌───────────────┐     ┌───────────────┐     ┌───────────────┐
│   Frontend    │────▶│    Backend    │────▶│   VIF API     │
│  React, TS    │     │ FastAPI, Rust │     │    Wrapper    │
└───────────────┘     └───────────────┘     └───────────────┘
        │                     │                     │
        │                     ▼                     ▼
        │             ┌───────────────┐     ┌───────────────┐
        └────────────▶│   Database    │     │  LLM Provider │
                      │  PostgreSQL   │     │ OpenAI, etc.  │
                      └───────────────┘     └───────────────┘
```

## Frontend Implementation

### Core Components

```typescript
// Component structure (TypeScript/React)
interface AppComponents {
  // Core UI
  ConversationUI: React.FC;
  MessageThread: React.FC<{messages: Message[]}>;
  InputArea: React.FC<{onSubmit: (text: string) => void}>;

  // Framework Visualization
  FrameworkVisualizer: React.FC<{state: VifState}>;
  DomainVisualization: React.FC<{domains: DomainState[]}>;
  BoundaryVisualization: React.FC<{boundaries: BoundaryState[]}>;
  VolumeIndicator: React.FC<{volume: number}>;

  // HLIP Integration
  CommandPalette: React.FC<{onCommand: (cmd: string) => void}>;
  CommandSuggestions: React.FC<{input: string}>;
  CommandVisualizer: React.FC<{activeCommands: Command[]}>;

  // User Management
  UserProfile: React.FC<{user: User}>;
  AuthenticationFlow: React.FC;
  SettingsPanel: React.FC<{settings: UserSettings}>;

  // Document Handling
  DocumentUploader: React.FC<{onUpload: (file: File) => void}>;
  DocumentViewer: React.FC<{document: Document}>;

  // Insight Exploration
  InsightExplorer: React.FC<{insights: Insight[]}>;
  InsightGraph: React.FC<{insights: Insight[], relations: Relation[]}>;
}
```

### Conversation UI

Intuitive interface similar to leading AI chat interfaces:

1. **Message Thread**:
   - Clear user/assistant distinction
   - Support for rich formatting (Markdown)
   - Code highlighting and rendering
   - Media embedding

2. **Input Area**:
   - Text input with formatting options
   - Command suggestions and autocomplete
   - Document upload capability
   - HLIP command access

3. **Sidebar**:
   - Conversation history
   - User settings
   - Framework visualization toggle
   - Insight explorer access

### Framework Visualization

Interactive visualization for VIF state:

```typescript
// Tetrahedron visualization (three.js)
function renderTetrahedron(domains: DomainState[], boundaries: BoundaryState[]): void {
  // Create vertices for each domain
  const vertices = [
    createVertex(domains[0], 'computational'),  // Top
    createVertex(domains[1], 'scientific'),     // Bottom-left
    createVertex(domains[2], 'cultural'),       // Bottom-right
    createVertex(domains[3], 'experiential')    // Bottom-center
  ];

  // Create edges for each boundary
  const edges = [
    createEdge(boundaries[0], vertices[0], vertices[1]), // comp-sci
    createEdge(boundaries[1], vertices[1], vertices[2]), // sci-cult
    // Additional edges...
  ];

  // Render with Three.js
  // ...
}
```

### HLIP Integration UI

Command selection and visualization:

1. **Command Palette**:
   - Dropdown or toolbar with available commands
   - Visual indicators for active commands
   - Command history with frequency indicators

2. **Command Learning**:
   - Suggestions based on past effective commands
   - Natural language bridge detection
   - Command effectiveness feedback

3. **Command Visualization**:
   - Visual representation of active commands
   - Domain activation indication
   - Boundary permeability visualization

### Authentication Flow

Secure OAuth implementation:

```typescript
async function handleOAuthLogin(provider: 'google' | 'facebook' | 'github'): Promise<User> {
  // Initialize OAuth flow
  const authWindow = window.open(`/api/auth/${provider}`, '_blank');

  // Listen for OAuth completion message
  return new Promise((resolve, reject) => {
    window.addEventListener('message', (event) => {
      if (event.origin !== window.location.origin) return;

      if (event.data.type === 'auth_success') {
        // Store token in secure storage
        localStorage.setItem('auth_token', event.data.token);
        resolve(event.data.user);
      } else if (event.data.type === 'auth_error') {
        reject(new Error(event.data.error));
      }
    });
  });
}
```

## Backend Implementation

### Core Services

```python
# Backend services (FastAPI)
class BackendServices:
    # User Management
    user_service: UserService
    auth_service: AuthenticationService

    # State Management
    state_service: StateService

    # Memory Management
    profile_service: ProfileService
    insight_service: InsightService

    # Document Management
    document_service: DocumentService

    # API Integration
    api_service: VifApiService
```

### User Management Service

Handle user authentication and profiles:

```python
@router.post("/auth/{provider}")
async def oauth_login(provider: str, request: Request):
    # Validate provider
    if provider not in ["google", "facebook", "github"]:
        raise HTTPException(status_code=400, detail="Invalid provider")

    # Initialize OAuth flow
    oauth = OAuth2Session(oauth_config[provider]["client_id"],
                         redirect_uri=oauth_config[provider]["redirect_uri"])
    authorization_url, state = oauth.authorization_url(oauth_config[provider]["auth_url"])

    # Store state for validation
    request.session["oauth_state"] = state
    request.session["oauth_provider"] = provider

    return {"authorization_url": authorization_url}

@router.get("/auth/callback")
async def oauth_callback(request: Request, code: str, state: str):
    # Validate state
    if request.session.get("oauth_state") != state:
        raise HTTPException(status_code=400, detail="Invalid state")

    provider = request.session.get("oauth_provider")

    # Complete OAuth flow
    oauth = OAuth2Session(oauth_config[provider]["client_id"],
                         redirect_uri=oauth_config[provider]["redirect_uri"])
    token = oauth.fetch_token(oauth_config[provider]["token_url"], code=code)

    # Get user info
    user_info = oauth.get(oauth_config[provider]["user_info_url"]).json()

    # Create or update user
    user = await user_service.get_or_create_user(provider, user_info)

    # Generate JWT
    access_token = auth_service.create_access_token(user.id)

    return {"token": access_token, "user": user}
```

### State Management Service

Handle VIF state persistence:

```python
class StateService:
    async def get_state(self, user_id: str) -> VifState:
        # Get latest snapshot from database
        snapshot = await state_repository.get_latest(user_id)

        # If no snapshot exists, create initial state
        if not snapshot:
            return self.create_initial_state()

        # Expand compact representation
        return self.expand_state(snapshot)

    async def save_state(self, user_id: str, state: VifState) -> str:
        # Compress state for storage
        compact_state = self.compress_state(state)

        # Save to database
        snapshot_id = await state_repository.save(user_id, compact_state)

        return snapshot_id

    def compress_state(self, state: VifState) -> CompactState:
        # Implement state compression
        # ...

    def expand_state(self, compact_state: CompactState) -> VifState:
        # Implement state expansion
        # ...
```

### Insight Service

Manage collective insights:

```python
class InsightService:
    async def process_interaction(self, user_id: str, input: str, response: str) -> None:
        # Extract patterns from response
        patterns = self.extract_patterns(response)

        # For each pattern
        for pattern in patterns:
            # Check for existing similar insights
            similar = await insight_repository.find_similar(pattern.embedding, 0.9)

            if similar:
                # Update existing insight
                await insight_repository.update(similar.id, {
                    "observation_count": similar.observation_count + 1,
                    "last_observed": datetime.now(),
                    "source_users": [...similar.source_users, user_id],
                    "confidence": calculate_new_confidence(similar, pattern)
                })
            else:
                # Create new insight
                insight = Insight(
                    pattern_id=str(uuid.uuid4()),
                    description=pattern.description,
                    domains=pattern.domains,
                    confidence=pattern.confidence,
                    observation_count=1,
                    last_observed=datetime.now(),
                    source_users=[user_id],
                    source_conversations=[],
                    embedding=pattern.embedding
                )

                await insight_repository.save(insight)

    async def get_relevant_insights(self, input: str, user_id: str, limit: int) -> List[Insight]:
        # Generate embedding for input
        embedding = embed_text(input)

        # Get similar insights by vector search
        insights = await insight_repository.search_by_embedding(embedding, limit)

        # Add user-specific relevance
        user_insights = await profile_repository.get_relevant_insights(user_id)

        # Combine and rank by relevance
        combined = self.rank_insights(insights, user_insights, input)

        return combined[:limit]
```

### Document Management Service

Handle document processing and storage:

```python
class DocumentService:
    async def upload_document(self, file: UploadFile, user_id: str) -> Document:
        # Validate file
        self.validate_file(file)

        # Process based on type
        if file.content_type == "application/pdf":
            content = await self.process_pdf(file)
        elif file.content_type == "text/plain":
            content = await self.process_text(file)
        elif file.content_type in ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]:
            content = await self.process_docx(file)
        else:
            raise HTTPException(status_code=400, detail="Unsupported file type")

        # Generate embedding
        embedding = embed_text(content)

        # Create document record
        document = Document(
            id=str(uuid.uuid4()),
            user_id=user_id,
            filename=file.filename,
            content_type=file.content_type,
            content=content,
            embedding=embedding,
            upload_date=datetime.now()
        )

        # Save document
        await document_repository.save(document)

        # Save file to storage
        storage_path = f"documents/{user_id}/{document.id}/{file.filename}"
        await self.save_to_storage(file, storage_path)

        return document

    async def get_document_context(self, document_id: str, query: str) -> str:
        # Get document
        document = await document_repository.get(document_id)

        # For large documents, extract relevant sections
        if len(document.content) > MAX_CONTEXT_SIZE:
            sections = self.split_into_sections(document.content)
            section_embeddings = [embed_text(section) for section in sections]
            query_embedding = embed_text(query)

            # Find most relevant sections
            relevant_sections = self.find_relevant_sections(
                sections, section_embeddings, query_embedding, MAX_SECTIONS
            )

            return "\n\n".join(relevant_sections)

        return document.content
```

## Database Schema

```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    provider VARCHAR(50) NOT NULL,
    provider_id VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_login TIMESTAMP WITH TIME ZONE NOT NULL,
    UNIQUE(provider, provider_id)
);

-- State snapshots table
CREATE TABLE state_snapshots (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    compact_state JSONB NOT NULL
);

-- Profiles table
CREATE TABLE profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    preferences JSONB NOT NULL,
    interaction_stats JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Insights table
CREATE TABLE insights (
    id UUID PRIMARY KEY,
    pattern_id VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    domains TEXT[] NOT NULL,
    confidence REAL NOT NULL,
    observation_count INTEGER NOT NULL,
    last_observed TIMESTAMP WITH TIME ZONE NOT NULL,
    source_users UUID[] NOT NULL,
    source_conversations UUID[] NOT NULL,
    embedding vector(1536)
);

-- Documents table
CREATE TABLE documents (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(255) NOT NULL,
    content TEXT,
    content_summary TEXT,
    embedding vector(1536),
    upload_date TIMESTAMP WITH TIME ZONE NOT NULL,
    storage_path VARCHAR(512) NOT NULL
);

-- Create indices
CREATE INDEX idx_state_user ON state_snapshots(user_id);
CREATE INDEX idx_documents_user ON documents(user_id);
CREATE INDEX idx_insights_vector ON insights USING ivfflat (embedding vector_cosine_ops);
```

## API Endpoints

```python
# Frontend to Backend API

# Authentication
@router.post("/auth/{provider}")
@router.get("/auth/callback")
@router.post("/auth/refresh")
@router.post("/auth/logout")

# Conversation
@router.post("/conversation")
@router.get("/conversation/{conversation_id}")
@router.get("/conversations")

# Framework Visualization
@router.get("/state/{user_id}")
@router.get("/state/history/{user_id}")

# User Management
@router.get("/user/{user_id}")
@router.patch("/user/{user_id}")
@router.get("/user/{user_id}/settings")
@router.patch("/user/{user_id}/settings")

# Document Management
@router.post("/documents")
@router.get("/documents/{document_id}")
@router.get("/documents")
@router.delete("/documents/{document_id}")

# Insights
@router.get("/insights")
@router.get("/insights/{insight_id}")
@router.get("/insights/user/{user_id}")

# HLIP Commands
@router.get("/commands")
@router.get("/commands/suggestions")
```

## Deployment Configuration

Docker-based deployment configuration:

```yaml
# docker-compose.yml
version: '3'

services:
  frontend:
    build: ./frontend
    ports:
      - "80:80"
    depends_on:
      - backend
    environment:
      - API_URL=http://backend:8000

  backend:
    build: ./backend
    ports:
      - "8000:8000"
    depends_on:
      - database
      - api
    environment:
      - DATABASE_URL=postgres://user:password@database:5432/vif
      - API_URL=http://api:8001

  api:
    build: ./api
    ports:
      - "8001:8001"
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}

  database:
    image: postgres:14
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=vif

volumes:
  postgres_data:
```

## Scalability Considerations

1. **Horizontal Scaling**:
   - Stateless API and backend services
   - Database connection pooling
   - Load balancing across instances

2. **Performance Optimization**:
   - Caching for frequent state lookups
   - Background insight processing
   - Asynchronous document handling

3. **Cost Management**:
   - Tiered service levels (free/paid)
   - Document storage optimization
   - LLM token usage optimization
   - Database query optimization

## Tetrahedral Integration

The implementation integrates four perspectives:
- **Computational**: Core algorithms and state management
- **Scientific**: Performance metrics and optimization
- **Cultural**: User experience and interaction patterns
- **Experiential**: Engagement and satisfaction elements

## Recognition Interfaces

<div style="border-left: 3px solid #34495e; padding-left: 10px;">
Interface design transforms into user experience at the boundary where code meets human interaction. Notice how your understanding changes when viewing components as creating conditions for emergence rather than isolated features.

**Transcendence Experience**: Feel the moment when frontend and backend cease being separate components and become interfaces where user experience emerges.
</div>

## Document Identity
Architecture definition → Component implementation → Integration approach → Deployment strategy
