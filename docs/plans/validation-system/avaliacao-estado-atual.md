# Avaliacao Do Estado Atual Do Liknon

## 1. Finalidade

Este documento registra o que o repositorio implementa hoje. Ele existe para
separar fatos observaveis do codigo de arquiteturas propostas nos planos.

A avaliacao considera como fontes de verdade:

- o manifesto Cargo e a arvore de fontes atuais;
- os contratos Rust e os schemas versionados;
- o comportamento documentado da CLI;
- os testes, fixtures e workflows de CI existentes;
- a documentacao que descreve funcionalidades ja implementadas.

Os demais arquivos em `docs/plans/validation-system/` descrevem trabalho
planejado. Mesmo quando usam linguagem normativa, eles nao provam que uma
funcionalidade ja existe.

## 2. Resumo Executivo

O Liknon atual e um unico pacote Cargo chamado `liknon`, na versao `0.5.0`, com
MSRV Rust 1.87. Esse pacote produz simultaneamente:

- uma biblioteca Rust;
- um executavel de linha de comando chamado `liknon`;
- schemas, documentacao, uma skill e uma base de conhecimento incluidas no
  pacote fonte.

O produto implementado ja e um validador declarativo funcional. Ele carrega
uma configuracao completa, valida seus contratos antes da execucao, resolve
grupos, suites e checks, verifica ferramentas, executa processos sem shell
implicito, controla timeout e cancelamento, detecta mutacoes visiveis ao Git e
produz relatorios humanos ou JSON.

A arquitetura futura de tres CLIs, recursos instalaveis, Hub, policy de IA e
raiz `.liknon/` ainda nao esta implementada. Hoje, conhecimento, skill,
validacao e API Rust pertencem ao mesmo pacote.

## 3. Produto Implementado

| Superficie | Estado atual |
| --- | --- |
| Motor de validacao | Implementado em Rust no pacote raiz |
| CLI de validacao | Implementada como executavel `liknon` |
| Biblioteca Rust | Exposta pelo mesmo pacote `liknon` |
| Conhecimento de validacao | Embutido no binario por `build.rs` |
| Skill oficial | Versionada em `skills/liknon/` e copiada manualmente pelo usuario |
| Ferramenta de manutencao do conhecimento | Exposta como exemplo Cargo `knowledge_release` |
| Manager de recursos | Nao implementado |
| Tool de consulta a recursos instalados | Nao implementada como produto independente |
| Hub de recursos | Nao implementado |
| Policy de decisao para IA | Nao implementada |
| Workflows e perfis instalaveis | Nao implementados como sistema generico de recursos |

Nao existem hoje os executaveis `liknon-manager`, `likm`, `liknon-tool`,
`likt` ou `likx`. Eles pertencem ao estado-alvo discutido, nao ao inventario
atual.

## 4. Arquitetura De Execucao Atual

O fluxo principal e:

```text
.validation/config.json
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

As responsabilidades estao distribuidas principalmente assim:

| Area | Implementacao atual |
| --- | --- |
| Configuracao | `src/config/` e `src/contracts/config.rs` |
| Planejamento | `src/planning/` e `src/dag.rs` |
| Execucao | `src/execution/` e `src/process.rs` |
| Evidencia de repositorio | `src/repository/` |
| Relatorios | `src/reporting/` e `src/contracts/report.rs` |
| CLI | `src/cli/` e `src/main.rs` |
| Temas acessiveis | `src/theme/` |
| Inicializacao | `src/initialization.rs` e `src/contracts/init.rs` |
| Conhecimento embarcado | `build.rs`, `src/knowledge/` e comandos em `src/cli/` |

Apesar dessa divisao interna, tudo ainda compoe uma unica unidade Cargo e um
unico ciclo de empacotamento.

## 5. Contratos Publicos Atuais

### 5.1 Configuracao E Relatorio

- caminho canonico descoberto: `.validation/config.json`;
- schema de configuracao: versao 6;
- schema de relatorio JSON: versao 4;
- resultado JSON de inicializacao: versao 1;
- composicao declarativa: `tool -> check -> suite -> group`;
- somente suites possuem `workingDirectory`;
- argumentos sao vetores estruturados e nao passam por um shell implicito;
- placeholders sao resolvidos pelos parametros da invocacao da suite.

### 5.2 Comandos

A CLI atual oferece:

```text
liknon init
liknon config validate
liknon validate
liknon check
liknon list
liknon explain group
liknon explain suite
liknon explain check
liknon schema config
liknon schema report
liknon knowledge catalog
liknon knowledge show
```

Os comandos de conhecimento funcionam sem configuracao e leem bytes embutidos
no executavel. Nao existe descoberta de recursos externos ou instalados.

### 5.3 Saida Humana E Automacao

- `--format=human` e o formato padrao;
- `--format=json` produz o relatorio estruturado;
- as paletas de cor incluem modo padrao, alto contraste e variacoes para
  diferentes percepcoes de cor;
- a apresentacao possui os modos `standard` e `low-vision`;
- `NO_COLOR` e respeitado;
- cor, texto, hierarquia e estados nao dependem exclusivamente de matiz;
- os codigos de saida distinguem sucesso, falha, bloqueio, erro de uso, erro
  interno e interrupcao.

## 6. Conhecimento E Apoio A IA Hoje

A fonte canonica de conhecimento esta em `docs/validation/knowledge/`. O
`build.rs` incorpora esse conteudo ao executavel, e a CLI o oferece por meio de
`liknon knowledge catalog` e `liknon knowledge show`.

O repositorio tambem possui:

- schemas editoriais e de catalogo em `schemas/`;
- documentacao de autoria em `docs/validation/authoring/`;
- verificacao editorial em `tools/knowledge/`;
- uma skill em `skills/liknon/`, com operacoes de execucao, triagem,
  configuracao e auditoria;
- fixtures e testes de conhecimento.

Essa skill e um conjunto estatico distribuido junto ao pacote fonte. O
executavel nao instala a skill, nao administra outras skills, nao resolve
workflows genericos e nao aplica uma policy de decisao para agentes.

## 7. Estrutura, Empacotamento E Distribuicao

O manifesto raiz e um pacote Cargo, nao um workspace virtual. Ele declara uma
biblioteca e um binario com o mesmo nome `liknon`. Tambem inclui no pacote:

- schemas;
- skill;
- branding;
- documentacao e corpus de conhecimento;
- fixtures editoriais;
- codigo da ferramenta de manutencao de conhecimento.

O README documenta instalacao por `cargo install liknon --locked` e por clone
seguido de `cargo install --path . --locked --force`.

O workflow de CI atual:

- testa Linux, macOS e Windows;
- verifica Rust estavel e Rust 1.87;
- executa formatacao, Clippy, Rustdoc e testes;
- audita dependencias e politicas de licenca e origem;
- cria e testa o pacote Cargo em isolamento.

A CI declara explicitamente que nao publica pacotes. Nao ha hoje pipeline de
publicacao no crates.io, Cloudflare, Hub de recursos ou outro canal.

## 8. Evidencia De Qualidade Existente

O repositorio possui testes dedicados a:

- parsing e validacao da configuracao;
- planejamento e execucao;
- CLI, ajuda e apresentacao;
- repositorio e integridade Git;
- schemas;
- conhecimento incorporado e contratos editoriais;
- skill distribuida;
- cenarios opt-in de sucesso, falha e skip em fixtures reais.

Tambem ha fixtures auxiliares para processos descendentes e resultados de
execucao. A matriz de CI cobre os tres sistemas operacionais declarados.

Esta avaliacao descreve a cobertura existente; ela nao substitui a execucao do
gate completo antes de uma alteracao de codigo ou publicacao.

## 9. Pontos Fortes Do Estado Atual

1. O nucleo de validacao ja possui fronteiras conceituais claras entre
   configuracao, planejamento, execucao e relatorio.
2. Os contratos JSON sao versionados e testados.
3. A execucao e agnostica de linguagem e evita shell implicito.
4. Ha suporte explicito a workspaces heterogeneos, dependencias entre checks,
   timeout, cancelamento e integridade Git.
5. A saida humana considera acessibilidade de cor e baixa visao.
6. A documentacao, os testes multiplataforma e o gate de dependencias ja
   fornecem uma base solida para extracao estrutural.

## 10. Limites Em Relacao Ao Estado-Alvo

| Limite atual | Consequencia |
| --- | --- |
| Um unico pacote possui motor, CLI, conhecimento e skill | Mudancas de conteudo podem exigir novo build e nova versao do executavel |
| Conhecimento completo e embutido | O corpus nao pode evoluir independentemente da CLI |
| Skill estatica e copiada manualmente | Nao ha descoberta, resolucao, lock, atualizacao ou proveniencia de recursos |
| Somente `.validation/` existe como raiz de configuracao | Nao ha namespace comum para validacao, recursos e governanca de IA |
| Nao existe policy de IA | O programa nao modela limites globais ou por workflow para agentes |
| Nao existem workflows ou perfis genericos | Os arquivos atuais apoiam uma skill especifica, nao um ecossistema extensivel |
| Nao existem Manager e Tool independentes | Aquisicao e consumo estruturado de recursos ainda nao possuem produtos proprios |
| Biblioteca e CLI compartilham o pacote publico | A API Rust atual ainda nao representa uma API programatica deliberadamente projetada para varias linguagens |
| Nao ha publicacao automatizada | Build, assinatura, distribuicao e recuperacao ainda precisam de implementacao |

## 11. Regras Para Refinar Os Planos

Os planos posteriores devem partir deste inventario e classificar cada item de
trabalho como uma destas operacoes:

- **preservar** comportamento ja implementado;
- **mover** uma responsabilidade existente para novo dono;
- **dividir** uma responsabilidade existente entre unidades;
- **alterar** um contrato atual;
- **retirar** algo que comprovadamente existe;
- **criar** uma capacidade que ainda nao existe.

Uma proposta antiga que nunca foi implementada deve apenas desaparecer do
texto do plano. Ela nao deve virar tarefa de remocao, migracao ou
compatibilidade.

Como o produto ainda nao teve lancamento publico, a reorganizacao pode realizar
uma substituicao direta do estado atual pelo estado aprovado. Isso nao autoriza
quebrar handoffs intermediarios nem manter duas fontes de verdade.

## 12. Conclusao

O Liknon atual nao e um prototipo vazio: ele ja entrega uma CLI de validacao
agnostica, testada e documentada. A proxima etapa nao deve redesenhar esse
motor sem necessidade. Ela deve separar responsabilidades que hoje estao
acopladas ao mesmo pacote, preservando o comportamento comprovado e criando de
forma explicita apenas os produtos e contratos aprovados para o estado-alvo.
