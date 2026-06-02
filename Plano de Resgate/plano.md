#### Refatoração Conceitual: Otimização do Core de Captura

No desenvolvimento inicial do Screenpipe, as rotinas de captura de mídia (áudio e tela) e o processamento subsequente (OCR e Transcrição Whisper) frequentemente sofriam de Acoplamento Justaposto e violação do princípio de responsabilidade única (SRP).
O código que gerenciava a gravação muitas vezes inicializava diretamente as instâncias de OCR ou motores de IA locais, criando dependências rígidas. Se a API mudasse ou se o motor de OCR precisasse ser substituído em tempo de execução (ex: migrar de Apple Vision para Tesseract no Linux), o core de captura precisaria ser reescrito.

Solução: Padrão Abstract Factory com Pipeline Stack 
Para desacoplar a engine de captura da infraestrutura de IA e processamento, propõe-se a abstração dos provedores de processamento através de uma fábrica abstrata, permitindo a injeção dinâmica de dependências baseada no sistema operacional e nas configurações do usuário. 

#### Roadmap de Maturidade: Alinhamento ao Nível G do MPS.BR

### Ação 1: Instituir a Gerência de Requisitos (GRE) via GitHub Templates e RFCs
Criar modelos obrigatórios de Issues divididos em Requisitos Funcionais (ex: novos conectores de pipes) e Não-Funcionais (ex: limites de uso de CPU/RAM).
Processo: Para grandes mudanças na arquitetura (como a integração do MCP da Anthropic), exigir a abertura de uma RFC (Request for Comments) formal no repositório. Nenhuma linha de código complexa é mesclada sem que o requisito de negócio e impacto técnico estejam documentados e revisados.

### Ação 2: Formalizar o Planejamento e Monitoramento (GPR) através de GitHub Projects Estáticos
Implementar um Product Backlog estruturado dentro do GitHub Projects com marcos (Milestones) claros e trimestrais.
Processo: Estabelecer estimativas de esforço (mesmo que por T-shirt sizing: S, M, L, XL) para as issues priorizadas. Criar uma rotina semanal assíncrona onde o status do projeto é medido com base no fechamento dessas metas, gerando um relatório de Burn-up para rastrear se o ritmo de entrega condiz com os planos declarados.

### Ação 3: Estabelecer uma Política de Versionamento e Baseline de Liberação (GPR)
Separar o fluxo de desenvolvimento em canais de distribuição distintos.
Processo: * O branch main e builds diários tornam-se o canal Nightly/Canary (para entusiastas e testes).
Criar um comitê interno mínimo (ou aprovação dupla de mantenedores via CODEOWNERS) para gerar releases estáveis semanais ou quinzenais (canal Stable).
Cada release estável deve vir acompanhada de um Changelog auditável correlacionando quais Issues de requisitos (Ação 1) foram fechadas naquela versão, garantindo a rastreabilidade ponta a ponta.
