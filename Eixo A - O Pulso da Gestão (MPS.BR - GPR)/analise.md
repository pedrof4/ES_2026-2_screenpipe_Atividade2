
# Eixo A — O Pulso da Gestão (MPS.BR – GPR)

**Projeto analisado:** Screenpipe
**Repositório:** https://github.com/screenpipe/screenpipe

## 1. Arqueologia de Issues

### Funcionalidade Analisada

Para esta análise, foi selecionada a **Issue #3274**, relacionada à captura de conteúdo compartilhado em aplicações de videoconferência (Zoom, Google Meet, Microsoft Teams, Discord, FaceTime e Webex).

A escolha dessa issue ocorreu por representar uma funcionalidade crítica para o propósito central do Screenpipe: registrar, indexar e disponibilizar informações contextuais capturadas da tela do usuário. Diferentemente de correções simples de interface, esta issue afeta diretamente a qualidade dos dados armazenados pelo sistema e a eficácia dos mecanismos que dependem dessas informações.

### Evidência

* **Link da Issue:** https://github.com/screenpipe/screenpipe/issues/3274

A issue documenta um problema de perda silenciosa de informações durante reuniões online. Embora o sistema continuasse registrando dados normalmente, conteúdos importantes compartilhados em vídeo, como apresentações, slides e documentos, deixavam de ser armazenados.

Durante a investigação, o autor identificou que o mecanismo responsável por decidir entre utilizar OCR ou Accessibility API estava tomando decisões incorretas. Como algumas aplicações retornavam milhares de caracteres relacionados apenas à interface gráfica, o sistema concluía erroneamente que não era necessário executar OCR, deixando de processar o conteúdo efetivamente exibido ao usuário.

O autor apresentou evidências quantitativas da base de dados, demonstrando a repetição excessiva de elementos da interface que causavam o problema:

| Elemento da Interface (AX label) | Ocorrências em um frame |
| :------------------------------- | :---------------------- |
| Mute my audio                    | 29                      |
| Audio options                    | 29                      |
| Stop video                       | 29                      |
| Participants options             | 29                      |

![Evidência quantitativa do problema](fig1-evidencia-quantitativa-issue3274.png)

*Figura 1: Evidência quantitativa do problema extraída do banco de dados e detalhamento da causa raiz associada à heurística de captura (Issue #3274).*

### Investigação Técnica Realizada

Um aspecto importante observado foi a qualidade da investigação realizada. O autor não apenas reportou o defeito, mas identificou a causa raiz, apontou os trechos de código envolvidos, apresentou evidências extraídas do banco de dados e propôs alternativas concretas para solução.

A análise identificou que a heurística utilizada pelo sistema considerava principalmente a quantidade total de texto disponível. Como aplicações de videoconferência retornavam grandes volumes de informações relacionadas apenas à interface gráfica, o mecanismo deixava de executar OCR, impedindo a captura de conteúdos relevantes compartilhados durante as reuniões.

### Maturidade na Elicitação de Requisitos e Impacto

A issue demonstra elevado nível de maturidade na elicitação e análise de requisitos. Inicialmente, o problema parecia estar relacionado apenas ao Zoom. Entretanto, durante a investigação, foi identificado que a mesma limitação afetava diversas outras aplicações, incluindo Google Meet, Microsoft Teams, Discord, FaceTime, Webex, Figma e Excalidraw.

Sob a perspectiva do processo GPR do MPS.BR, essa postura demonstra uma preocupação em identificar um padrão arquitetural de falha, em vez de tratar apenas um caso específico. Essa abordagem contribui para soluções mais abrangentes e reduz a probabilidade de retrabalho futuro.

Além disso, a documentação produzida na issue apresenta elevado nível de detalhamento. Foram incluídas justificativas técnicas, avaliação de impacto, critérios de aceitação e sugestões para evolução futura da solução. Essa prática reduz ambiguidades, facilita a comunicação entre os colaboradores e aumenta a rastreabilidade das decisões tomadas ao longo do desenvolvimento.

Essa falha afeta diretamente a principal funcionalidade do sistema e pode causar perda de informações importantes sem que o usuário perceba.

---

## 2. Gestão de Riscos Ocultos

### Evidência 1: Controle de Escopo e Avaliação de Impacto

A análise da Issue #3274 revela uma preocupação explícita com gestão de riscos e controle de escopo. Na própria issue existe uma seção denominada **"Out of scope (separate work)"**, onde são registradas melhorias futuras que não serão implementadas na mesma alteração.

Entre essas melhorias estão mecanismos de otimização de OCR e aprimoramentos relacionados à remoção de informações duplicadas provenientes da Accessibility API.

Essa prática demonstra preocupação em evitar o crescimento excessivo da complexidade durante uma única implementação e contribui para uma evolução mais controlada do sistema.

Além disso, a issue apresenta estimativas de impacto relacionadas ao consumo de CPU, armazenamento e bateria antes mesmo da implementação da solução proposta.

![Análise de Custo e Dívida Técnica](cost.png)

*Figura 2: Avaliação prévia dos impactos computacionais e documentação de melhorias futuras fora do escopo da implementação atual.*

![](ofs.png)

### 2. Gestão de Riscos Ocultos

Durante a inspeção do código-fonte, foram identificadas diversas ocorrências da palavra-chave `HACK`, amplamente utilizada para documentar soluções temporárias, limitações conhecidas ou adaptações necessárias devido ao comportamento não confiável de bibliotecas e sistemas externos.

Um exemplo relevante está relacionado à inicialização do mecanismo de captura do macOS. Os desenvolvedores registram explicitamente comportamentos inconsistentes durante a inicialização da GPU e da API Metal.

Em vez de ignorar o problema, foi implementado um mecanismo de **retry**, reduzindo falhas intermitentes durante a captura. Esse comportamento representa uma forma de mitigação preventiva de riscos operacionais.

### Arquitetura e Dependências Externas

Outro aspecto importante identificado foi a utilização de uma camada de abstração para integração com diferentes provedores de Inteligência Artificial.

Os Pipes podem operar utilizando serviços como OpenAI, Anthropic e Ollama, permitindo substituir provedores sem alterações significativas na lógica principal do sistema.

Essa abordagem reduz a dependência de fornecedores específicos e aumenta a tolerância a falhas externas, minimizando riscos relacionados à indisponibilidade de serviços, mudanças de API ou alterações de custos.

### Análise Crítica da Equipe

A equipe demonstra preocupação constante com riscos técnicos, desempenho e sustentabilidade da arquitetura. A documentação detalhada das limitações conhecidas, o uso de mecanismos de recuperação e a existência de planejamento explícito para melhorias futuras evidenciam uma postura preventiva em relação à evolução do sistema.

Embora existam mecanismos de mitigação, o projeto permanece dependente de APIs externas, sistemas operacionais e tecnologias de terceiros que podem sofrer alterações ao longo do tempo. Ainda assim, a arquitetura apresenta estratégias que reduzem significativamente os impactos dessas dependências.

---

## 3. Ritmo de Entrega, Dinâmica e Code Review

### Evidência

A análise do histórico demonstra atividade contínua, característica comum de projetos open source ativos. O projeto conta com aproximadamente 130 contribuidores e apresenta atividade frequente em Issues, Pull Requests e Commits, demonstrando evolução constante e participação ativa da comunidade.

O mantenedor principal, Louis Lambert, é responsável por uma parcela significativa dos commits do projeto, contribuindo diretamente para sua evolução e manutenção.

### Code Review e Critérios de Aceitação

A dinâmica de Code Review é fortalecida pela existência de critérios formais para alterações de maior impacto.

Observando a seção **"Acceptance"** da Issue #3274, nota-se a exigência explícita de mecanismos de validação antes da aprovação da implementação, incluindo:

1. Criação de testes unitários para validar o novo comportamento.
2. Inclusão de testes de regressão para evitar o reaparecimento do problema em versões futuras.

![Critérios de Aceitação e Verificação](acceptance.png)

*Figura 3: Critérios de aceitação exigindo testes unitários e testes de regressão para validação da implementação.*

### Análise Crítica da Equipe

A exigência de testes como critério de aceitação demonstra preocupação com a qualidade e a estabilidade do sistema. Além disso, a participação ativa da comunidade, evidenciada pelo número de contribuidores e pela frequência de Issues e Pull Requests, indica um processo de desenvolvimento colaborativo.

Também foram observados Pull Requests menores aprovados rapidamente, com pouca ou nenhuma discussão registrada. Esse comportamento é comum em projetos open source com elevado volume de contribuições, onde alterações simples tendem a ser integradas de forma mais direta.

Por outro lado, funcionalidades de maior complexidade costumam apresentar documentação detalhada, critérios claros de validação e maior rigor técnico durante sua análise.

---

## 4. Avaliação Geral do Processo GPR

Sob a perspectiva do MPS.BR (Gerência de Projetos), o Screenpipe apresenta diversas práticas compatíveis com os objetivos do processo GPR.

### Pontos Fortes

* Processo maduro de investigação e documentação de requisitos.
* Rastreabilidade das decisões técnicas e das mudanças propostas.
* Avaliação prévia de impactos relacionados a desempenho e consumo de recursos.
* Controle explícito de escopo por meio da definição de atividades futuras.
* Utilização de critérios formais de aceitação para funcionalidades críticas.
* Preocupação com mitigação de riscos técnicos e operacionais.
* Evolução contínua apoiada por uma comunidade ativa de contribuidores.

### Conclusão

A análise realizada evidencia que o Screenpipe adota práticas compatíveis com os objetivos do processo de Gerência de Projetos do MPS.BR. Foram identificadas evidências de documentação estruturada, rastreabilidade de decisões, avaliação de riscos técnicos, critérios formais de validação e evolução contínua do projeto.

A **Issue #3274** representa um excelente exemplo desse processo, pois demonstra como uma demanda complexa é investigada, documentada, discutida e transformada em uma proposta concreta de melhoria.

Apesar dos riscos inerentes ao uso de tecnologias externas, APIs e componentes dependentes dos sistemas operacionais, o projeto apresenta mecanismos arquitetônicos e práticas de desenvolvimento que contribuem para reduzir esses impactos, demonstrando um nível de maturidade superior ao normalmente encontrado em projetos open source de crescimento acelerado.

**Pontos de Atenção:**
* Risco organizacional atrelado ao alto *Bus Factor*.
* Base de código com instabilidades documentadas (*hacks*) na comunicação direta com APIs de hardware e sistema operacional.

Conclui-se que a dinâmica de trabalho no repositório é altamente profissionalizada. A análise da Issue #3274 evidencia que mudanças de maior impacto recebem tratamento estruturado, com investigação aprofundada e documentação consistente, indicando um grau de maturidade técnica e de gestão superior ao normalmente encontrado em projetos open source de rápido crescimento.
