## 1. Análise DIP

### Resumo Executivo
O arquivo identifica uma **violação clara do DIP (Dependency Inversion Principle)** no projeto, especialmente no momento de tentar trocar o provedor OpenAI por Ollama/Llama-3.

O ponto central é que a arquitetura atual está muito acoplada ao OpenAI, e as mudanças não se limitam a um único ponto: exigem alterações em pelo menos 12 arquivos críticos.

### Principais achados

- **AI Gateway**: embora exista a interface abstrata `AIProvider`, o roteamento ainda está acoplado ao OpenAI como padrão.
- **Backend**: várias implementações usam diretamente `new OpenAI()` e dependem de `openai` SDK ou compatibilidade OpenAI, em vez de usar abstração genérica.
- **Frontend Tauri**: código da UI e hooks usam OpenAI diretamente, especialmente em arquivos como `timeline/agents.tsx` e `chatgpt-preset.ts`.
- **OAuth ChatGPT**: há código Rust e bindings Tauri específicos para ChatGPT OAuth, o que criaria dependência forte e desnecessária se Ollama for alvo.
- **Configurações de rede e dependências**: há URLs de permissão e dependência `openai` no `package.json` que também precisariam ser alterados.

### Arquivos que precisariam mudar

1. `packages/ai-gateway/src/providers/index.ts`
   - Factory `createProvider()` retorna `OpenAIProvider` como fallback.
   - Precisa alternar entre providers e permitir Ollama.
2. `packages/ai-gateway/src/providers/openrouter.ts`
   - Usa `import OpenAI from 'openai'` e baseURL específica.
   - Precisaria de provider genérico ou `OllamaProvider` novo.
3. `packages/ai-gateway/src/providers/tinfoil.ts`
   - Herdado de `OpenAIProvider`.
   - Mudança no provider rompe herança se não houver abstração.
4. `apps/screenpipe-app-tauri/components/rewind/timeline/agents.tsx`
   - Instancia `OpenAI` e chama `chat.completions.create()` diretamente.
   - Alto acoplamento no front-end.
5. `apps/screenpipe-app-tauri/components/settings/hooks/use-openai-models.ts`
   - Nome e lógica são OpenAI-específicos, mesmo que já haja algum fallback.
6. `apps/screenpipe-app-tauri/lib/utils/chatgpt-preset.ts`
   - Presets hardcoded para ChatGPT e modelos OpenAI.
7. `apps/screenpipe-app-tauri/components/ui/icons.tsx`
   - Define ícone e naming específicos de OpenAI.
8. `apps/screenpipe-app-tauri/components/rewind/ai-presets-selector.tsx`
   - Usa `useOpenAIModels` e ChatGPT OAuth.
9. `apps/screenpipe-app-tauri/components/settings/recording-settings.tsx`
   - Importa hook OpenAI.
10. `crates/screenpipe-core/src/pipes/mod.rs`
    - Várias funções específicas de ChatGPT OAuth (`read_chatgpt_oauth_token()`, `refresh_chatgpt_token()`, etc.).
11. `apps/screenpipe-app-tauri/lib/utils/tauri.ts`
    - Comandos Tauri específicos `chatgptOauthLogin`, `chatgptOauthStatus`, etc.
12. Arquivos de configuração
    - `apps/screenpipe-app-tauri/package.json` com dependência `openai`
    - `apps/screenpipe-app-tauri/src-tauri/gen/schemas/capabilities.json` com permissões OpenAI

### Impacto e severidade

- O projeto exige **mudança em 12+ arquivos**.
- O acoplamento não está apenas no provedor, mas também em UI, OAuth e bindings.
- A violação de DIP é clara: existem módulos de alto nível que dependem de implementações concretas do OpenAI em vez de abstrações.

### Tabela de impacto resumida

| Arquivo | Tipo | Impacto | Severidade |
|---|---|---|---|
| `providers/index.ts` | TS | Roteamento padrão | Médio |
| `providers/openrouter.ts` | TS | Uso direto do SDK | Médio |
| `providers/tinfoil.ts` | TS | Herança OpenAI | Alto |
| `timeline/agents.tsx` | TSX | UI chama OpenAI direto | Crítico |
| `use-openai-models.ts` | TS | Nomenclatura específica | Menor |
| `chatgpt-preset.ts` | TS | Hardcoded de modelos | Alto |
| `pipes/mod.rs` | Rust | OAuth específico | Crítico |
| `tauri.ts` | TS | IPC ChatGPT | Médio |
| `connect-apps.tsx` | TSX | Referências OpenAI | Médio |
| `ai-presets-selector.tsx` | TSX | Lógica específica | Médio |
| `recording-settings.tsx` | TSX | Import direto | Menor |
| `capabilities.json` | JSON | Permissões de rede | Menor |

### Conclusão desse arquivo

Sim, há violação de DIP. Embora exista uma interface `AIProvider`, a arquitetura ainda depende de muitos detalhes concretos do OpenAI. Uma migração para Ollama ou Llama-3 requer abstração real, remoção de SDK direto no front-end e refatoração do OAuth e bindings.

### Categoria de risco

- **Risco geral**: Alto
- **Motivo**: A mudança afeta múltiplas camadas (frontend, backend, OAuth, provider) e exige refatoração de dependências e arquiteturas de roteamento.
- **Impacto**: Alto, porque a falha em abstrair corretamente pode resultar em acoplamento contínuo ao OpenAI e regressões em funcionalidades de chat.

---

## 2. Análise God Object

### Resumo Executivo
O arquivo identifica a struct `SCServer` como um **God Object** típico e mostra que ela acumula ao menos **7 responsabilidades** diferentes.

### Responsabilidades detectadas

1. Inicialização do servidor HTTP e roteamento Axum.
2. Gerenciamento de persistência SQLite via `DatabaseManager`.
3. Controle de loops assíncronos de captura de áudio e vídeo.
4. Gerenciamento de múltiplos caches (`frame_cache`, `search_cache`, `hot_frame_cache`).
5. Autenticação e autorização.
6. Coordenação de sync na nuvem, archive e retention.
7. Orquestração de pipes e agentes.
8. Manipulação de WebSockets e streaming em tempo real.

### Estrutura da struct `SCServer`

A struct possui campos como:

- `db: Arc<DatabaseManager>`
- `addr: SocketAddr`
- `audio_manager: Arc<AudioManager>`
- `sync_handle: Option<Arc<SyncServiceHandle>>`
- `pipe_manager: Option<SharedPipeManager>`
- `vision_metrics`, `audio_metrics`
- `hot_frame_cache`
- `power_manager`
- `pipe_permissions`
- `api_auth`, `api_auth_key`
- `secret_store`

Isso indica um alto grau de acoplamento de várias áreas do sistema.

### AppState também é um God Object

A struct `AppState` complementa o problema com outros campos como:

- `frame_cache`, `frame_image_cache`, `search_cache`
- `cloud_search`, `sync_state`
- `pipe_query_semaphore`, `frame_extraction_semaphore`
- `archive_state`, `retention_state`
- `vault`, `browser_bridge`
- `manual_meeting`

Ao todo, `AppState` acumula cerca de **26 responsabilidades** distintas.

### Violação de SRP no método `create_router()`

O método `create_router()` é descrito como monolítico e faz:

- criação de rotas Axum
- spawn de tasks de métricas e monitoramento
- inicialização de cache
- injeção de `AppState`
- configuração de mais de 60 rotas HTTP

Ele controla:

- analytics de uso de API,
- métricas da pipeline de visão,
- métricas de áudio,
- warm-up do hot frame cache,
- rotas de banco de dados, vault, sync, archive e retenção.

Isso caracteriza a violação do SRP.

### Canais de violação detalhados

#### 1. Inicialização e roteamento HTTP

- `create_router()` cria o `Router` e registra dezenas de rotas.
- Rota `/raw_sql` e `/add` mostram acesso direto ao banco de dados.
- A criação de rotas mistura persistência, sync e UI.

#### 2. Persistência de banco de dados

- `DatabaseManager` é injetado em `AppState` e compartilhado por todas as rotas.
- Isso demonstra dependência indireta do servidor sobre todas as operações de DB.

#### 3. Loops assíncronos de captura

- Tasks que monitoram `vision_metrics` e `audio_metrics` a cada 60 segundos.
- A lógica inclui `frames_captured`, `chunks_sent`, `vad_rejected`, etc.
- Esse comportamento não deveria estar na orquestração HTTP.

#### 4. Gerenciamento de caches

- `frame_image_cache` com `LruCache` de 1000 entradas.
- `search_cache` com TTL de 60 segundos.
- `frame_cache` persistente carregada com `FrameCache::new()`.
- Há também warm-up do cache a partir do DB.

#### 5. Permissões e autenticação

- `pipe_permissions` e `api_auth` em `SCServer`.
- Autorização de pipes e validação de tokens ficam centralizadas.

### Conclusão desse arquivo

`SCServer` e `AppState` são God Objects que violam SRP e causam alta complexidade. O servidor concentra inicialização, persistência, caching, sync, métricas, autenticação e gerenciamento de pipes no mesmo lugar.

### Categoria de risco

- **Risco geral**: Alto
- **Motivo**: A concentração de responsabilidades em um único objeto torna a manutenção perigosa, aumenta o risco de regressões e dificulta a divisão segura de código.
- **Impacto**: Alto, porque qualquer mudança na infraestrutura de servidor ou no fluxo de dados pode afetar diversas áreas do sistema de forma imprevisível.

---

## 3. DRY violations

### Resumo Executivo
O arquivo listou **15+ violações de DRY** em todo o repositório, com foco em padrões repetidos que afetam manutenção e consistência.

### Categorias principais

1. Retry e backoff
2. Validação de dimensões de monitor
3. Tratamento de erro de mutex (poison)
4. Lógica de foco/visibilidade de janela
5. Detecção e classificação de tipos de erro
6. Funções utilitárias de tempo
7. Validação de schemas e regras de campos

### Violação 1: Exponential backoff e retry

Foram identificadas **5+ implementações** similares de backoff com pequenas variações.

#### Arquivos afetados

- `apps/screenpipe-app-tauri/src-tauri/src/main.rs`
- `crates/screenpipe-audio/src/audio_manager/device_monitor.rs`
- `crates/screenpipe-audio/src/core/device.rs`
- `packages/ai-gateway/src/services/transcription-ab.ts`
- `packages/ai-gateway/src/handlers/realtime-transcription.ts`
- `packages/browser-extension/src/worker.ts`
- `crates/screenpipe-core/src/pipes/preset_fallback.rs`
- `apps/screenpipe-app-tauri/lib/hooks/__tests__/timeline-ui-issues.test.ts`

#### Problemas identificados

- Variações de jitter
- Caps diferentes
- Falta de padrão único
- Testes duplicados

### Violação 2: Validação de dimensões de monitor

Há **4+ verificações idênticas** no mesmo arquivo `crates/screenpipe-screen/src/monitor.rs`.

#### Problema
Uso de `unwrap_or(0)` e comparação `== 0` repetidos em vários métodos de captura.

#### Solução proposta
Extrair método `validate_dimensions(width, height)` para centralizar a verificação.

### Violação 3: Tratamento de mutex poisoning

Há **3+ locais** que usam a mesma lógica de `lock().unwrap_or_else(|e| e.into_inner())`.

#### Arquivos afetados

- `crates/screenpipe-engine/src/cli/db.rs`
- `apps/screenpipe-app-tauri/src-tauri/src/health.rs`
- `crates/screenpipe-a11y/src/tree/enhanced_mode_cache.rs`
- `crates/screenpipe-connect/src/oauth_refresh_scheduler.rs`

#### Problema
Comportamento inconsistente e perda de contexto de erro.

#### Solução proposta
Criar wrapper utilitário compartilhado para lock ignorando poison.

### Categoria de risco da análise DRY

- **Risco geral**: Médio
- **Motivo**: A duplicação é extensa e pode levar a bugs e inconsistências, mas não há indicação de falhas imediatas do sistema.
- **Impacto**: Moderado, pois a correção melhora a manutenção e a consistência, mas não é uma mudança tão crítica quanto os problemas de arquitetura do DIP ou God Object.

### Violação 4: Lógica de foco/visibilidade de janela

Há **6+ implementações** duplicadas em código de janela do Tauri, especialmente em macOS.

#### Arquivos afetados

- `apps/screenpipe-app-tauri/src-tauri/src/commands.rs`
- `apps/screenpipe-app-tauri/src-tauri/src/window/show.rs`
- `apps/screenpipe-app-tauri/src-tauri/src/window/panel.rs`

#### Problema
Repetição de show/hide/activate/restore com comportamento de plataforma.

#### Solução proposta
Extrair helpers como `activate_app()` e `show_and_focus()`.

### Violação 5: Classificação de tipos de erro

Há detecção de erros repetida em:

- `crates/screenpipe-core/src/pipes/mod.rs`
- `crates/screenpipe-core/src/pipes/preset_fallback.rs`

#### Problema
Heurísticas diferentes para rate_limit, auth e network.

#### Solução proposta
Centralizar em `error_classification.rs`.

### Violação 6: Funções utilitárias de tempo

Há **3+ implementações** de `now_unix()` e similares em diferentes crates.

#### Arquivos afetados

- `crates/screenpipe-engine/src/cli/db.rs`
- `crates/screenpipe-connect/src/oauth.rs`
- `crates/screenpipe-core/src/pipes/preset_fallback.rs`
- `crates/screenpipe-engine/src/external_memory_sync.rs`
- `crates/screenpipe-vault/src/manager.rs`

#### Problema
Código duplicado e inconsistências de rounding/erro.

#### Solução proposta
Criar utilitário comum `epoch_now()` em `crates/screenpipe-common/src/time.rs`.

### Conclusão desse arquivo

O repositório apresenta muitas duplicações de lógica crítica, especialmente em backoff, validação, tratamento de erros e comportamento de janela. A consolidação desses padrões traria manutenção mais fácil e maior consistência.
