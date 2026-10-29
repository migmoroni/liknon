# Avaliacao Do Estado Atual Do Liknon

## Status

`Current-state record`, atualizado apos a implementacao do A02.

## 1. Finalidade

Este documento registra o que o repositorio implementa hoje. Ele separa fatos
observaveis do codigo de arquiteturas propostas nos planos.

A avaliacao considera como fontes de verdade:

- o workspace Cargo e a arvore de fontes atuais;
- os contratos Rust e os schemas versionados;
- o comportamento documentado das aplicacoes existentes;
- os testes, fixtures e workflows de CI;
- a documentacao que descreve funcionalidades implementadas.

Os demais arquivos em `docs/plans/validation-system/` descrevem trabalho
planejado. Mesmo quando usam linguagem normativa, eles nao provam que uma
funcionalidade ja existe.

## 2. Resumo Executivo

O Liknon atual e um workspace Cargo virtual na versao `0.5.0`, com MSRV Rust
1.87. Ele possui quatro pacotes de aplicacao, duas bibliotecas compartilhadas e
tres ferramentas privadas de manutencao:

| Pacote | Estado atual |
| --- | --- |
| `liknon-validator` | Aplicacao de validacao completa, biblioteca interna e binarios `liknon-validator` / `likv` |
| `liknon-manager` | Manager funcional para descoberta, resolucao, instalacao, verificacao e materializacao autenticada de recursos |
| `liknon-tool` | Shell de identidade para o futuro Tool, sem comandos de consulta |
| `liknon-cli` | Pacote agregado e shell do launcher `liknon` / `likx`; ainda nao despacha capacidades |
| `liknon-cli-ui` | Biblioteca publicavel com perfis visuais, estilos semanticos neutros, integracao Clap, ajuda e politica de terminal compartilhadas |
| `liknon-resource-contracts` | Biblioteca publicavel com contratos de recursos, catalogo, confianca, manifest, lock e assinaturas |
| `liknon-cli-distribution` | Ferramenta privada para contratos e candidatos de distribuicao nativa das CLIs |
| `liknon-knowledge-maintenance` | Ferramenta privada que verifica o conteudo autorado no repositorio |
| `liknon-resource-hub-maintenance` | Ferramenta privada para montar e verificar snapshots assinados do Hub |

O Validation CLI e o Manager sao os produtos funcionais implementados. O
Validation CLI valida a
configuracao completa antes de executar processos, resolve grupos, suites e
checks, verifica ferramentas, executa vetores de argumentos sem shell
implicito, controla timeout e cancelamento, detecta mutacoes visiveis ao Git e
produz relatorios humanos ou JSON.

Manager autentica catalogos e payloads, resolve dependencias, mantem o estado
de recursos em `.liknon/resources/` e suporta verificacao offline. Tool e
launcher ainda possuem apenas seus limites estruturais. Hub publico, consulta
offline por Tool, policy, persistencia de workflows e despacho agregado
continuam planejados ou dependentes de ativacao externa.

## 3. Produto De Validacao Implementado

O pacote `apps/liknon-validator` possui uma unica implementacao de validacao. A
semantica de validacao permanece local, enquanto a experiencia generica de
terminal vem de `crates/liknon-cli-ui`:

| Area | Implementacao atual |
| --- | --- |
| Configuracao | `apps/liknon-validator/src/config/` |
| Contratos estruturados | `apps/liknon-validator/src/contracts/` |
| Planejamento e DAG | `apps/liknon-validator/src/planning/` e `dag.rs` |
| Execucao de processos | `apps/liknon-validator/src/execution/` e `process.rs` |
| Evidencia de repositorio | `apps/liknon-validator/src/repository/` |
| Relatorios e apresentacao | `apps/liknon-validator/src/reporting/` |
| CLI e ajuda | `apps/liknon-validator/src/cli/` |
| Adaptacao de papeis e metricas de validacao | `apps/liknon-validator/src/theme/` |
| Perfis, estilos e ajuda genericos | `crates/liknon-cli-ui/` |
| Inicializacao | `apps/liknon-validator/src/initialization.rs` |
| Schemas | `apps/liknon-validator/schemas/` |

O crate expoe internamente os modulos exigidos pelos consumidores atuais do
workspace, mas essa superficie Rust ainda nao e uma API programatica publica
prometida. O desenho de APIs e bindings pertence ao B05.

## 4. Fluxo De Execucao Atual

```text
.liknon/validation/config.json
             |
             v
carregamento e validacao integral
             |
             v
planejamento imutavel do alvo
             |
             v
preflight de ferramentas e execucao
             |
             v
relatorio estruturado
             |
             +--> apresentacao humana
             `--> JSON versionado
```

O caminho canonico e `.liknon/validation/config.json`. A descoberta caminha
para cima a partir do diretorio atual e escolhe a configuracao mais proxima.
`init` nao faz descoberta: ele valida os bytes indicados e os instala no
destino canonico dentro do workspace explicitamente selecionado.

## 5. Contratos Atuais

### 5.1 Configuracao E Relatorio

- schema de configuracao: versao 6;
- schema de relatorio JSON: versao 4;
- resultado JSON de inicializacao: versao 1;
- composicao declarativa: `tool -> check -> suite -> group`;
- somente suites possuem `workingDirectory`;
- argumentos sao vetores estruturados e nao passam por shell implicito;
- placeholders sao resolvidos pelos parametros da invocacao da suite;
- caminhos de execucao permanecem abaixo de `workspaceRoot`;
- grupos formam um DAG e membros compartilhados executam uma vez por rodada.

### 5.2 Comandos Funcionais

```text
liknon-validator init
liknon-validator config validate
liknon-validator validate
liknon-validator check
liknon-validator list
liknon-validator explain group
liknon-validator explain suite
liknon-validator explain check
liknon-validator schema config
liknon-validator schema report
```

`likv` oferece o mesmo comportamento. Nao existe namespace de conhecimento no
Validation CLI.

### 5.3 Saida Humana E Automacao

- `--format=human` e o formato padrao;
- `--format=json` produz o relatorio estruturado;
- a apresentacao possui os modos `standard` e `low-vision`;
- cor explicita, paletas acessiveis e `NO_COLOR` sao suportados;
- status, tipo de no, hierarquia e erro nao dependem apenas de matiz;
- os codigos de saida distinguem sucesso, falha, bloqueio, erro de uso, erro
  interno e interrupcao.

## 6. Demais Aplicacoes Implementadas

`liknon-manager` / `likm` implementam `init`, `search`, `info`, `install`,
`update`, `remove`, `list`, `verify`, `materialize` e `schema`. O Manager nao
executa recursos nem workflows; ele termina na administracao autenticada e
transacional dos bytes e do estado instalado.

`liknon-tool` / `likt` continuam como shells estruturais e nao fingem possuir
operacoes futuras.

O pacote `liknon-cli` instala:

- os launchers `liknon` e `likx`;
- os seis binarios das tres capacidades.

Os launchers reconhecem os namespaces `validate`, `manager` e `tool`, mas
encerram com diagnostico explicito porque o despacho ainda nao foi
implementado. A validacao deve ser chamada diretamente por
`liknon-validator` ou `likv`.

Validator, Manager, Tool e launcher usam `liknon-cli-ui` para perfis visuais,
ajuda, diagnosticos Clap e politica ANSI. Cada aplicacao mantem localmente sua
gramatica, apresentacao de dominio, contratos de maquina, erros e codigos de
saida.

## 7. Conhecimento E Apoio A IA

A fonte canonica de conhecimento de validacao esta em
`docs/validation/knowledge/`. Ela e conteudo autorado do repositorio, nao e
embutida no Validation CLI e nao possui comando de leitura em runtime.

O pacote privado `liknon-knowledge-maintenance` valida catalogo, fontes,
integridade, receitas, indice gerado e o handoff da skill. Ele depende da
biblioteca de `liknon-validator` apenas para verificar exemplos de configuracao
com o carregador real. Nenhum produto distribuido depende dessa ferramenta.

O conteudo em `skills/liknon/` permanece como entrada nao ativa para planos
posteriores. O sistema atual nao instala skills, nao resolve workflows, nao
aplica policy de decisao e nao administra recursos de IA.

## 8. Empacotamento E Distribuicao

O workspace ainda nao anuncia uma versao publica. `liknon-validator`,
`liknon-manager`, `liknon-cli-ui` e `liknon-resource-contracts` possuem
metadados de pacote publicavel; Tool, launcher agregado e ferramentas de
manutencao permanecem privados. Workflows protegidos preparam candidatos, mas
nenhum canal deve ser anunciado como ativo antes da publicacao e do readback.

Os caminhos funcionais documentados instalam as CLIs implementadas a partir do
clone:

```sh
cargo install --path apps/liknon-validator --locked --force
cargo install --path apps/liknon-manager --locked --force
```

O pacote `liknon-validator` inclui motor, adaptacao CLI, schemas, exemplo e
testes. Ele nao inclui o corpus de conhecimento, skills, planos, avaliacoes,
fixtures externas ou ferramentas editoriais. Sua experiencia generica de
terminal depende da versao exata de `liknon-cli-ui`; o branding ASCII necessario
ao help pertence ao pacote compartilhado.

`liknon-cli-ui` possui workflow proprio de candidato, atestacao, aprovacao,
publicacao e readback. Os workflows de Validator e Manager ensaiam seus
pacotes contra as crates compartilhadas exatas e exigem que essas dependencias
ja estejam publicadas com os mesmos bytes antes de publicar a aplicacao.
Publicacao efetiva em crates.io, binarios nativos em Cloudflare, Hub publico,
distribuicao das outras aplicacoes e suite agregada pertencem a B01-B06
conforme seus respectivos pacotes e canais.

## 9. Evidencia De Qualidade Existente

O repositorio possui cobertura para:

- parsing, descoberta e validacao da configuracao;
- planejamento, DAG e execucao;
- CLI, ajuda, temas e apresentacao;
- schemas e contratos JSON;
- repositorio e integridade Git;
- timeout, cancelamento e arvore de processos;
- inicializacao atomica em `.liknon/validation/`;
- shells e limites dos quatro pacotes de aplicacao;
- manutencao do conhecimento autorado;
- cenarios opt-in de sucesso, falha e skip em fixtures reais.

A CI cobre Linux, macOS e Windows, Rust estavel e Rust 1.87, formatacao,
Clippy, Rustdoc, testes, auditoria de dependencias, politicas de licenca e
origem, inventario de pacote e instalacao isolada. Ela nao publica artefatos.

## 10. Limites Em Relacao Ao Estado-Alvo

| Limite atual | Consequencia |
| --- | --- |
| Tool ainda e shell | Consulta offline e projecao de recursos ainda nao existem |
| Manager nao possui Hub publico | Operacoes remotas oficiais aguardam autoridade e publicacao externas |
| O launcher nao despacha | A suite agregada ainda nao oferece a ergonomia final |
| Conhecimento permanece no repositorio | Nao ha instalacao, catalogo local ou consulta por Tool |
| A skill nao esta ativa | Nao ha um contrato operacional de IA distribuido |
| Policy, persistencia e workflows nao existem | A governanca e a engenharia assistida continuam planejadas |
| Pacotes nao sao publicos | Instalacao atual depende do clone e do toolchain Rust |
| API Rust interna nao e deliberadamente publica | Integracoes e bindings aguardam consumidores e contrato B05 |

## 11. Regras Para Os Planos Seguintes

Os planos posteriores devem classificar trabalho como preservar, mover,
dividir, alterar, retirar ou criar. Uma proposta nunca implementada deve apenas
deixar de aparecer no estado atual; ela nao deve virar migracao ou camada de
compatibilidade.

Como o produto ainda nao teve lancamento publico, substituicoes podem adotar
diretamente o contrato aprovado. Isso nao autoriza manter duas fontes de
verdade nem quebrar os handoffs intermediarios entre planos.
