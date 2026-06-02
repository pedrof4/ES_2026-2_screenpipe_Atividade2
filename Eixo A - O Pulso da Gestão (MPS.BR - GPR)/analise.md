

# Eixo A — O Pulso da Gestão (MPS.BR – GPR)

**Projeto analisado:** Screenpipe

**Repositório:** [https://github.com/screenpipe/screenpipe](https://github.com/screenpipe/screenpipe)

## 1. Arqueologia de Issues

### Funcionalidade Analisada

Para esta análise, foi selecionada a **Issue #3274**, relacionada à captura de conteúdo compartilhado em aplicações de videoconferência (Zoom, Google Meet, Microsoft Teams, Discord, FaceTime e Webex).

A escolha dessa issue ocorreu por representar uma funcionalidade crítica para o propósito central do Screenpipe: registrar, indexar e disponibilizar informações contextuais capturadas da tela do usuário. Diferentemente de correções simples de interface, esta issue afeta diretamente a qualidade dos dados armazenados pelo sistema e a eficácia dos mecanismos de sumarização e integração com modelos de Inteligência Artificial.

### Evidência

* **Link da Issue:** [https://github.com/screenpipe/screenpipe/issues/3274](https://github.com/screenpipe/screenpipe/issues/3274)
* **Autor:** [Louis Lambert (louis030195)](https://github.com/louis030195)

A issue documenta um problema de perda silenciosa de dados durante reuniões online. O autor apresenta exemplos extraídos diretamente do banco SQLite do Screenpipe, demonstrando que o mecanismo priorizava informações provenientes da *Accessibility API* (AX) do sistema operacional, impedindo a execução do OCR sobre conteúdos compartilhados em vídeo.

O autor apresentou evidências quantitativas da base de dados, demonstrando a repetição excessiva de elementos da interface que causavam o "falso positivo" no sistema:

| Elemento da Interface (AX label) | Ocorrências em um frame |
| --- | --- |
| Mute my audio | 29 |
| Audio options | 29 |
| Stop video | 29 |
| Participants options | 29 |

*(Print sugerido para o relatório: Trecho da descrição da Issue #3274 mostrando a tabela de repetição e a seção "Why it happens").*

### Investigação Técnica Realizada

O nível de detalhamento da investigação é notável. O autor não apenas reportou um comportamento incorreto, mas rastreou a causa raiz até a linha exata de código (`crates/screenpipe-engine/src/paired_capture.rs:137-165`).

A análise identificou que a heurística do sistema (`a11y_is_thin`) considerava apenas o volume total de texto. Como o Zoom retornava milhares de caracteres repetidos de acessibilidade, o sistema concluía erroneamente que não precisava executar o OCR, perdendo o conteúdo real (slides, apresentações).

### Maturidade na Elicitação de Requisitos

A issue demonstra uma maturidade ímpar na elicitação de requisitos. O autor não se limitou a relatar o erro pontual no Zoom; ele mapeou proativamente todas as aplicações que sofriam da mesma falha arquitetural logo na abertura do chamado, incluindo Google Meet, Microsoft Teams, Discord, FaceTime, Figma e Excalidraw.

Sob a perspectiva do processo GPR do MPS.BR, essa postura demonstra uma preocupação em identificar padrões sistêmicos de falha e propor soluções escaláveis, evitando retrabalho e correções isoladas futuras.

### Classificação de Risco

**Risco: ALTO**

* **Justificativa:** A falha compromete a principal proposta de valor do Screenpipe. Trata-se de uma falha silenciosa: o sistema aparenta funcionar normalmente, mas conteúdos cruciais deixam de ser armazenados e indexados pelo sistema sem qualquer aviso ao usuário.

---

## 2. Gestão de Riscos Ocultos e Dívida Técnica

### Evidência (TODOs e APIs)

A análise da Issue #3274 e do código-fonte revela uma gestão consciente da Dívida Técnica. Na própria issue, o autor incluiu uma seção denominada **"Out of scope (separate work)"**, que funciona como "TODOs" formalmente documentados.

Melhorias como a *desduplicação de labels AX* e o *OCR por região restrita da tela* foram mapeadas como trabalho futuro.

### Análise Crítica da Equipe

O projeto gerencia a dívida técnica de forma estratégica. Ao documentar explicitamente o que **não** será feito na PR atual, a equipe mitiga o risco de inchaço de escopo (*scope creep*), garantindo a entrega do valor principal sem perder de vista as otimizações necessárias.

Além disso, a issue traz uma análise prévia de impacto em hardware (CPU/Bateria), estimando que a mudança custará `~80ms` por frame (cerca de 4% de uso de um núcleo). Esse nível de gestão de risco computacional demonstra um planejamento rigoroso.

**Sobre a instabilidade de APIs de IA:** Embora esta issue específica trate do motor local (OCR), a arquitetura geral do Screenpipe mitiga a volatilidade e os *timeouts* de APIs de terceiros (como OpenAI ou Anthropic) através de padrões de *fallback*. Caso uma API externa sofra instabilidade, o desacoplamento permite que os *Pipes* recorram a modelos locais (ex: Ollama) de forma tolerante a falhas, evidenciando uma gestão madura de dependências externas.

### Classificação de Risco

**Risco: MÉDIO**

* **Justificativa:** O projeto atua proativamente no mapeamento de dívidas técnicas e impactos de hardware, além de possuir mecanismos arquiteturais para falhas de API. Contudo, continua altamente dependente de múltiplas tecnologias externas e atualizações constantes de sistemas operacionais.

---

## 3. Ritmo de Entrega, Dinâmica e Code Review

### Evidência

A análise do histórico (Aba *Insights/Contributors*) demonstra atividade contínua. O mantenedor principal, Louis Lambert, é responsável por uma parcela esmagadora dos commits.

A concentração de milhares de commits em um único mantenedor sugere forte dependência de conhecimento especializado centralizado. Caso esse desenvolvedor se afaste do projeto, o ritmo de evolução e manutenção pode ser significativamente afetado.

No entanto, a garantia da qualidade não é negligenciada. Observando a seção **"Acceptance"** da Issue #3274, nota-se o rigor imposto para o processo de revisão de código.

### Code Review e Critérios de Aceitação

A dinâmica de Code Review é fortalecida por critérios de aceite extremamente rigorosos focados em **Verificação**. Para que o Pull Request desta funcionalidade fosse aprovado, o autor exigiu explicitamente:

1. Criação de um teste unitário (`paired_capture.rs::tests`) simulando o *spam* de AX do Zoom para forçar o OCR.
2. Inclusão de uma entrada manual de regressão no arquivo `TESTING.md`.

### Análise Crítica da Equipe

A exigência de testes automatizados e de regressão como critério de aceitação indica que as revisões de código no projeto não são meramente visuais ("LGTM - Looks Good To Me"). O processo garante estabilidade e previne que futuras atualizações quebrem heurísticas já validadas.

Em relação ao ritmo de entrega, o padrão observado sugere uma cadência constante, sem grandes períodos de "crunch". Contudo, foi identificada uma forte concentração de conhecimento no mantenedor principal. Sob a perspectiva de gestão de projetos (GPR), isso representa um alto **Bus Factor**. Caso este mantenedor reduza sua participação, a continuidade e a agilidade da evolução do sistema podem ser severamente impactadas, sendo mitigado apenas parcialmente pelo engajamento da comunidade open source.

### Classificação de Risco

**Risco: MÉDIO**

* **Justificativa:** Foram observadas Pull Requests contendo discussões técnicas e critérios de validação. Entretanto, a análise não permite afirmar que todas as alterações passam por processos formais e consistentes de Code Review, pois parte das contribuições é integrada com pouca discussão registrada publicamente.
---

## 4. Avaliação Geral do Processo GPR

Sob a perspectiva do MPS.BR (Gerência de Projetos), o Screenpipe apresenta um grau avançado de aderência às práticas recomendadas.

**Pontos Fortes:**

* Processo maduro de elicitação de requisitos, pensando no ecossistema e não em falhas isoladas.
* Comunicação técnica e documentação baseadas em dados concretos de banco de dados e perfis de hardware.
* Gestão transparente de escopo e dívida técnica (uso de escopo negativo/TODOs).
* Rigor em critérios de aceite para Pull Requests.

**Pontos de Atenção:**

* Risco organizacional atrelado ao alto *Bus Factor*.
* Necessidade de monitoramento constante de impactos em CPU/Bateria a cada nova heurística adicionada.

Conclui-se que a dinâmica de trabalho no repositório é altamente profissionalizada, suportando com resiliência a complexidade de um sistema que atua na fronteira entre o baixo nível (sistemas operacionais/captura) e o alto nível (Inteligência Artificial).

À medida que novas funcionalidades são incorporadas rapidamente, torna-se mais difícil manter níveis elevados de rastreabilidade entre requisitos, discussões técnicas e implementações.

Mesmo assim, a análise da Issue #3274 evidencia que mudanças de maior impacto recebem tratamento estruturado, com investigação aprofundada, documentação consistente e avaliação explícita de riscos.

Essas características indicam um grau de maturidade superior ao normalmente encontrado em projetos open source de rápido crescimento.
