# Avaliacao Do Estado Atual Do Liknon

## Status

`Current-state record`, atualizado apos a conclusao estrutural do A01.

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
1.87. Ele possui quatro pacotes de aplicacao e uma ferramenta privada de
manutencao:

| Pacote | Estado atual |
| --- | --- |
| `liknon-validate` | Aplicacao de validacao completa, biblioteca interna e binarios `liknon-validate` / `likv` |
| `liknon-manager` | Shell de identidade para o futuro Manager, sem comandos de recursos |
| `liknon-tool` | Shell de identidade para o futuro Tool, sem comandos de consulta |
| `liknon-cli` | Pacote agregado e shell do launcher `liknon` / `likx`; ainda nao despacha capacidades |
| `liknon-knowledge-maintenance` | Ferramenta privada que verifica o conteudo autorado no repositorio |

O produto funcional implementado e o Validation CLI. Ele valida a
configuracao completa antes de executar processos, resolve grupos, suites e
checks, verifica ferramentas, executa vetores de argumentos sem shell
implicito, controla timeout e cancelamento, detecta mutacoes visiveis ao Git e
produz relatorios humanos ou JSON.

Manager, Tool e launcher possuem somente seus limites de pacote, binarios,
ajuda e versao. Hub, aquisicao e consulta de recursos, policy, persistencia,
workflows instalaveis e despacho agregado continuam planejados.

## 3. Produto De Validacao Implementado

O pacote `apps/liknon-validate` possui uma unica implementacao de validacao. A
separacao entre semantica e terminal ocorre por modulos internos, nao por um
segundo pacote:

| Area | Implementacao atual |
| --- | --- |
| Configuracao | `apps/liknon-validate/src/config/` |
| Contratos estruturados | `apps/liknon-validate/src/contracts/` |
| Planejamento e DAG | `apps/liknon-validate/src/planning/` e `dag.rs` |
| Execucao de processos | `apps/liknon-validate/src/execution/` e `process.rs` |
| Evidencia de repositorio | `apps/liknon-validate/src/repository/` |
| Relatorios e apresentacao | `apps/liknon-validate/src/reporting/` |
| CLI e ajuda | `apps/liknon-validate/src/cli/` |
| Temas acessiveis | `apps/liknon-validate/src/theme/` |
| Inicializacao | `apps/liknon-validate/src/initialization.rs` |
| Schemas | `apps/liknon-validate/schemas/` |

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
liknon-validate init
liknon-validate config validate
liknon-validate validate
liknon-validate check
liknon-validate list
liknon-validate explain group
liknon-validate explain suite
liknon-validate explain check
liknon-validate schema config
liknon-validate schema report
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

`liknon-manager` / `likm` e `liknon-tool` / `likt` aceitam somente as
superficies de identidade, ajuda e versao. Eles nao fingem possuir operacoes
futuras.

O pacote `liknon-cli` instala:

- os launchers `liknon` e `likx`;
- os seis binarios das tres capacidades.

Os launchers reconhecem os namespaces `validate`, `manager` e `tool`, mas
encerram com diagnostico explicito porque o despacho ainda nao foi
implementado. A validacao deve ser chamada diretamente por
`liknon-validate` ou `likv`.

## 7. Conhecimento E Apoio A IA

A fonte canonica de conhecimento de validacao esta em
`docs/validation/knowledge/`. Ela e conteudo autorado do repositorio, nao e
embutida no Validation CLI e nao possui comando de leitura em runtime.

O pacote privado `liknon-knowledge-maintenance` valida catalogo, fontes,
integridade, receitas, indice gerado e o handoff da skill. Ele depende da
biblioteca de `liknon-validate` apenas para verificar exemplos de configuracao
com o carregador real. Nenhum produto distribuido depende dessa ferramenta.

O conteudo em `skills/liknon/` permanece como entrada nao ativa para planos
posteriores. O sistema atual nao instala skills, nao resolve workflows, nao
aplica policy de decisao e nao administra recursos de IA.

## 8. Empacotamento E Distribuicao

O workspace ainda nao anuncia uma versao publica. Os quatro pacotes de
aplicacao possuem `publish = false`; a ferramenta de manutencao tambem e
privada.

O caminho funcional documentado instala o Validation CLI a partir do clone:

```sh
cargo install --path apps/liknon-validate --locked --force
```

O pacote `liknon-validate` inclui motor, adaptacao CLI, schemas, branding,
exemplo e testes. Ele nao inclui o corpus de conhecimento, skills, planos,
avaliacoes, fixtures externas ou ferramentas editoriais. Seu build nao depende
de outro pacote Liknon.

Publicacao em crates.io, binarios nativos em Cloudflare, distribuicao das
outras aplicacoes e suite agregada pertencem aos planos B01-B04.

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
| Manager e Tool sao shells | Aquisicao e consulta de recursos ainda nao existem |
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
