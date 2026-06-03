# Auditoria Forense de Software e Plano de Resgate Técnico
## Projeto: screenpipe

---

## Vídeo da Auditoria
 Link do vídeo:

 ---

## Equipe

- Gian Glauberty Santos Nascimento – 202300061616 (Eixo A: O Pulso da Gestão)
- Pedro César Figueiredo Carneiro – 202300061732 (Eixo B: Anatomia do Código)
- Pedro Miguel Castro França – 202300061741 (Eixo C: Padrões de Projeto)
- João Marcelo Silva da Conceição – 202300095820 (Montagem de Slide e Revisão)
- Renner do Nascimento Brito – 202300061797 (Direção e edição do vídeo)
- Paulo Gabriel de Oliveira Cardoso – 202000047735 (Plano de Resgate)

---

## Objetivo

Este repositório tem como objetivo apresentar uma auditoria forense de software do projeto Screenpipe, avaliando aspectos de gestão, arquitetura, qualidade e engenharia de software com base em referências do MPS.BR, princípios SOLID, padrões GoF e boas práticas de desenvolvimento moderno.

Além da identificação de problemas e riscos técnicos, o trabalho propõe recomendações e um plano de resgate para evolução da maturidade do projeto.

---

### Repositório Oficial

https://github.com/screenpipe/screenpipe

### Principais Tecnologias

* Rust
* Tauri
* SQLite + FTS5
* Whisper
* Deepgram
* OCR (Apple Vision, Windows OCR, Tesseract)
* GitHub Actions

---

## Metodologia da Auditoria

A auditoria foi conduzida por meio da coleta de evidências diretamente do repositório oficial, incluindo:

* Código-fonte
* Estrutura de pastas
* Issues
* Pull Requests
* Workflows de CI/CD
* Documentação
* Arquitetura observada

A análise foi organizada em três eixos principais.

---

# Eixo A – O Pulso da Gestão (MPS.BR – GPR)

Avaliação da maturidade do gerenciamento do projeto.

### Itens analisados

* Gestão de requisitos
* Evolução do backlog
* Releases
* Histórico de Issues
* Dívida técnica
* Gestão de mudanças

### Objetivo

Identificar sinais de maturidade, riscos de gestão e oportunidades de melhoria nos processos de desenvolvimento.

---

# Eixo B – Anatomia do Código (SOLID & DRY)

Avaliação da qualidade estrutural do software.

### Itens analisados

* Acoplamento entre componentes
* Responsabilidade única
* Repetição de código
* Dependência de provedores externos
* Possíveis violações dos princípios SOLID

### Objetivo

Identificar fragilidades arquiteturais e riscos de manutenção.

---

# Eixo C – Padrões de Projeto (GoF)

Avaliação dos padrões arquiteturais identificados na aplicação.

### Padrões investigados

#### Criacionais

* Singleton
* Factory Method

#### Estruturais

* Adapter
* Facade

#### Comportamentais

* Strategy
* Chain of Responsibility
* Pipeline

### Objetivo

Analisar a utilização de padrões de projeto e sua contribuição para modularidade, extensibilidade e manutenção do sistema.

---

## Evidências Utilizadas

As evidências da auditoria foram obtidas diretamente do projeto:

* Estrutura arquitetural do sistema
* Componentes de captura e processamento
* DatabaseManager
* APIs públicas
* Sistema de plugins (Pipes)
* Issues e Pull Requests
* GitHub Actions
* Histórico de desenvolvimento

---

## Principais Achados

### Pontos Fortes

* Arquitetura modular
* Uso de componentes desacoplados
* Forte utilização de pipelines de processamento
* Integração flexível com IA
* Processo colaborativo ativo

### Pontos de Atenção

* Possível risco de vendor lock-in
* Dependência de provedores externos de IA
* Formalização limitada de alguns contratos arquiteturais
* Rastreabilidade parcialmente manual
* Ausência de processos formais de qualidade alinhados ao MPS.BR

---

### Estrutura do repositório

[Análise Pulso da Gestão](./)

---

## Conclusão

O Screenpipe apresenta uma arquitetura moderna, modular e alinhada a tendências atuais de software baseado em inteligência artificial. Entretanto, a auditoria identificou oportunidades de melhoria relacionadas à formalização arquitetural, redução de acoplamento e mitigação de riscos de dependência tecnológica.

As recomendações propostas no plano de resgate visam aumentar a manutenibilidade, extensibilidade e maturidade do projeto, contribuindo para sua evolução sustentável a longo prazo.
