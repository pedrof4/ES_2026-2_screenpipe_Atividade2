# Eixo A — O Pulso da Gestão (MPS.BR — GPR)

**Projeto analisado:** Screenpipe
**Repositório:** https://github.com/screenpipe/screenpipe

---

# 1. Arqueologia de Issues

## Funcionalidade analisada

Foi selecionada a Issue **#3274**, relacionada à captura de conteúdo compartilhado em aplicações de videoconferência, como Zoom, Google Meet, Microsoft Teams, Discord e FaceTime.

A issue descreve uma falha importante no processo de captura de informações pelo Screenpipe. Em determinadas situações, o sistema utilizava apenas os dados obtidos pela Accessibility API do sistema operacional, deixando de executar OCR sobre conteúdos exibidos em vídeo ou compartilhamento de tela. Como consequência, apresentações, slides e documentos compartilhados durante reuniões não eram armazenados nem indexados pelo sistema.

---

## Quem abriu a issue?

A issue foi aberta por um colaborador da comunidade que identificou perda silenciosa de informações durante reuniões online.

O relato apresenta elevado nível de detalhamento técnico, incluindo:

* análise de registros reais do banco de dados;
* identificação da causa raiz;
* exemplos concretos de falha;
* proposta de correção;
* análise de impacto;
* critérios de aceitação.

Esse nível de detalhamento demonstra uma participação ativa da comunidade no processo de evolução do projeto.

---

## Como ocorreu a discussão?

Diferentemente de issues simples que apenas relatam um erro, a Issue #3274 já foi aberta acompanhada de uma investigação técnica aprofundada.

O autor apresentou evidências extraídas diretamente do banco SQLite utilizado pelo sistema. A análise mostrou que a árvore de acessibilidade retornava milhares de caracteres, mas grande parte deles era composta por elementos repetidos da interface do Zoom.

Exemplos encontrados:

| Elemento      | Repetições |
| ------------- | ---------- |
| Mute my audio | 29         |
| Audio options | 29         |
| Stop video    | 29         |
| Video options | 29         |

Apesar da grande quantidade de texto disponível, praticamente não havia conteúdo relevante relacionado aos slides ou documentos compartilhados na reunião.

O algoritmo responsável por decidir entre Accessibility API e OCR interpretava incorretamente esse volume de texto como informação útil e, por isso, o OCR não era executado.

---

## Houve mudança de escopo?

Sim.

Inicialmente o problema parecia estar relacionado apenas ao Zoom.

Durante a investigação foi identificado que o mesmo comportamento poderia ocorrer em diversos outros softwares:

* Google Meet;
* Microsoft Teams;
* Discord;
* FaceTime;
* Webex;
* Figma;
* Excalidraw.

O escopo da mudança evoluiu de uma correção específica para Zoom para uma melhoria geral no mecanismo de decisão entre Accessibility API e OCR.

Essa expansão foi documentada diretamente na própria issue por meio da descrição técnica e da proposta de solução.

---

## Como a mudança foi documentada?

A issue apresenta documentação detalhada da solução proposta.

Foram sugeridas duas alterações principais:

### Alteração 1

Criar uma lista de aplicações que obrigatoriamente utilizem OCR juntamente com Accessibility API.

Exemplos:

* Zoom;
* Meet;
* Teams;
* Discord;
* FaceTime.

### Alteração 2

Melhorar a heurística de decisão utilizada pelo sistema.

A proposta sugere identificar padrões de repetição excessiva de texto. Quando a maior parte do conteúdo for composta por elementos repetidos da interface, o OCR deverá ser executado automaticamente.

Além disso, a issue documenta:

* impactos em CPU;
* impactos em armazenamento;
* impactos em bateria;
* possíveis melhorias futuras;
* critérios de aceitação da solução.

---

## Conclusão da análise

A Issue #3274 demonstra um processo de gerenciamento técnico bem estruturado.

O problema foi identificado, investigado, documentado e acompanhado por uma proposta de solução baseada em evidências concretas.

Sob a perspectiva do processo GPR do MPS.BR, observa-se:

* boa comunicação técnica;
* análise explícita de riscos;
* documentação de requisitos da mudança;
* definição clara de critérios de validação.

---

# 2. Gestão de Riscos Ocultos

Durante a análise do repositório foram observados diversos pontos relacionados ao gerenciamento de riscos técnicos.

O projeto possui uma arquitetura altamente dependente de componentes externos, incluindo:

* APIs de Inteligência Artificial;
* sistemas operacionais diferentes;
* bibliotecas de OCR;
* mecanismos de captura de áudio e vídeo;
* provedores de LLM.

Essa característica naturalmente introduz riscos relacionados à compatibilidade e à estabilidade.

## Evidências observadas

A própria Issue #3274 demonstra uma preocupação explícita com riscos operacionais.

Antes da implementação da solução foram avaliados:

* impacto no uso de CPU;
* impacto no armazenamento;
* impacto na bateria;
* possibilidade de degradação de desempenho.

Além disso, o projeto frequentemente utiliza mecanismos de abstração para provedores de IA, reduzindo dependências diretas de APIs específicas.

Essa estratégia reduz riscos associados a:

* mudanças de versão;
* descontinuação de serviços;
* alterações de contratos de APIs externas;
* vendor lock-in.

## Avaliação

O projeto demonstra preocupação constante com riscos técnicos, principalmente relacionados a desempenho e integração com tecnologias externas.

A existência de discussões técnicas detalhadas antes da implementação de mudanças indica uma postura preventiva em relação aos riscos identificados.

---

# 3. Ritmo de Entrega

A análise do histórico público do projeto mostra um ritmo de desenvolvimento intenso e contínuo.

O repositório apresenta:

* grande volume de commits;
* múltiplos contribuidores ativos;
* atualização frequente de funcionalidades;
* correções constantes de bugs.

Essa dinâmica sugere um processo de evolução contínua, típico de projetos open source em crescimento acelerado.

## Picos de atividade

Observa-se a existência de períodos de maior concentração de entregas, principalmente quando novas funcionalidades relacionadas à IA são incorporadas ao projeto.

Entretanto, o histórico geral não indica longos períodos de inatividade, sugerindo uma cadência relativamente constante de desenvolvimento.

## Revisão de Código

O projeto utiliza Pull Requests como mecanismo principal de integração de mudanças.

Em diversas contribuições é possível observar:

* discussão técnica;
* justificativas para alterações;
* análise de impacto;
* validação por outros colaboradores.

Por outro lado, como ocorre em muitos projetos open source de rápido crescimento, também existem Pull Requests com pouca discussão formal, principalmente em correções menores.

## Avaliação

Sob a perspectiva do GPR, o projeto apresenta sinais de maturidade operacional:

* fluxo contínuo de entregas;
* utilização de Pull Requests;
* participação ativa da comunidade;
* documentação técnica das mudanças mais relevantes.

O principal desafio observado está relacionado à velocidade de evolução do projeto, que pode dificultar a manutenção de rastreabilidade completa entre requisitos, issues e implementações.

---

# Conclusão Geral

A análise do Screenpipe sob a ótica do processo GPR do MPS.BR evidencia um projeto com forte atividade de desenvolvimento e participação comunitária.

A Issue #3274 demonstra um exemplo concreto de identificação, investigação e documentação de uma mudança relevante. Além disso, a preocupação explícita com desempenho, consumo de recursos e integração com serviços externos evidencia práticas de gestão de riscos compatíveis com projetos de software modernos.

Por fim, o ritmo constante de entregas e o uso de Pull Requests indicam uma dinâmica de desenvolvimento ativa, embora a rápida evolução do projeto represente um desafio para a manutenção de rastreabilidade completa entre todos os artefatos de desenvolvimento.
