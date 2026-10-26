# Resumo Do Estado-Alvo Do Liknon

## Status

`Approved target`, com decisoes abertas explicitamente identificadas.

## 1. Finalidade E Status

Este documento resume o que se pretende construir a partir do estado atual do
Liknon. Ele registra a direcao de produto ja discutida, separa decisoes firmes
de pontos ainda abertos e serve de base para refinar os planos.

Ele nao afirma que a arquitetura descrita ja existe e nao substitui os planos
de implementacao. Nenhum detalhe marcado como aberto deve aparecer em um plano
subordinado como contrato aprovado.

## 2. Visao De Produto

O estado-alvo e um sistema de validacao composto por tres CLIs de capacidade,
um launcher sem capacidade propria e um ecossistema aberto de recursos:

1. uma CLI executa validacoes declarativas;
2. uma CLI adquire e organiza recursos;
3. uma CLI consulta recursos locais de forma segura e economica;
4. um launcher oferece uma entrada unificada sem incorporar essas capacidades;
5. recursos de conhecimento e apoio a IA evoluem sem exigir recompilar as
   CLIs;
6. humanos e agentes podem usar as interfaces publicas sem depender de um
   workflow oficial;
7. workflows oficiais demonstram e testam a integracao, mas nao recebem
   privilegios embutidos nas aplicacoes.

O objetivo e manter o mecanismo de validacao pequeno e deterministico, ao
mesmo tempo em que conhecimento, skills, workflows, perfis e outros conteudos
podem crescer de forma independente.

## 3. Produtos Planejados

### 3.1 Launcher Unificado

| Campo | Definicao-alvo |
| --- | --- |
| Pacote agregado | `liknon-cli` |
| Comandos | `liknon` e `likx` |
| Responsabilidade | Selecionar uma CLI de capacidade e encaminhar exatamente a invocacao |
| Rede | Nenhuma capacidade propria |
| Mutacao | Nenhuma capacidade propria |

O launcher reconhece os namespaces `validate`, `manager` e `tool`, remove
somente esse primeiro argumento posicional e encaminha o restante para o
executavel correspondente. Ele nao valida, instala, consulta, interpreta nem
repara componentes.

```text
liknon validate validate fast -> liknon-validate validate fast
liknon validate check ...     -> liknon-validate check ...
liknon validate config ...    -> liknon-validate config ...
liknon manager install ...    -> liknon-manager install ...
liknon tool show ...          -> liknon-tool show ...
```

`likx` usa a mesma gramatica. Nao existe namespace padrao, alias de
compatibilidade ou encaminhamento implicito de comandos antigos.

O pacote `liknon-cli` e tambem a rota agregada promovida no Cargo. Ele possui
os oito alvos binarios `liknon`, `likx`, `liknon-validate`, `likv`,
`liknon-manager`, `likm`, `liknon-tool` e `likt`, reutilizando as mesmas
implementacoes compartilhadas dos tres pacotes de aplicacao independentes. Ele
nao depende de o Cargo instalar binarios pertencentes a dependencias.

### 3.2 Validation CLI

| Campo | Definicao-alvo |
| --- | --- |
| Pacote/aplicacao | `liknon-validate` |
| Comandos | `liknon-validate` e `likv` |
| Responsabilidade | Configurar, inspecionar, planejar e executar validacoes |
| Rede | Nenhuma necessidade para o fluxo normal |
| Recursos de IA | Nao incorpora o corpus completo nem orquestra agentes |

Essa aplicacao preserva o motor agnostico ja existente. Ela continua sendo
utilizavel diretamente por humanos, automacao e agentes que conhecam seu
contrato publico.

### 3.3 Manager

| Campo | Definicao-alvo |
| --- | --- |
| Pacote/aplicacao | `liknon-manager` |
| Comandos | `liknon-manager` e `likm` |
| Responsabilidade | Descobrir, obter, verificar, instalar, atualizar, remover e organizar recursos |
| Rede | Permitida somente nas operacoes declaradas |
| Mutacao | Unica CLI autorizada a alterar o estado local de recursos |

O Manager acessa o Hub, apresenta planos antes de mutacoes e registra o estado
desejado e o estado resolvido. Instalar um recurso nao significa executa-lo ou
ativa-lo.

### 3.4 Tool

| Campo | Definicao-alvo |
| --- | --- |
| Pacote/aplicacao | `liknon-tool` |
| Comandos | `liknon-tool` e `likt` |
| Responsabilidade | Catalogar, pesquisar, resolver e retornar recursos ja instalados |
| Rede | Proibida |
| Mutacao | Proibida |

Tool e a interface local de leitura para humanos, agentes, editores e
automacao. Ela verifica o estado instalado e entrega somente o conteudo
solicitado. Ela nao executa instrucoes encontradas nos recursos e nao
orquestra modelos de IA.

### 3.5 Distincao Importante

`liknon-tool` e uma aplicacao de consulta. **AI-Tool** e uma skill oficial de
operacao assistida. Os nomes proximos nao tornam os dois o mesmo produto:

- Tool retorna recursos e metadados;
- AI-Tool descreve como um agente realiza uma operacao delimitada;
- workflows podem selecionar AI-Tool ou outra skill compativel;
- nenhuma CLI deve conter uma condicao especial para o ID de AI-Tool.

## 4. Estrutura Do Repositorio

A direcao estrutural e transformar o pacote raiz em um workspace Cargo com
fronteiras semelhantes a:

```text
apps/
├── liknon-cli/          # launcher e pacote agregado
├── liknon-validate/
├── liknon-manager/
└── liknon-tool/

crates/
├── liknon-core/
└── liknon-workspace/

resources/
├── knowledge/
│   └── official/
├── skills/
│   └── official/
├── workflows/
│   └── official/
├── agent-profiles/
│   └── official/
├── tools/
│   └── official/
├── prompts/
│   └── official/
├── guides/
│   └── official/
├── templates/
│   └── official/
└── collections/
    └── official/
```

O tipo do recurso vem antes da origem. `official` identifica uma origem e nao
concede privilegio de execucao. Outras origens devem poder usar os mesmos
contratos.

`liknon-core` concentra semantica de configuracao, planejamento, execucao e
relatorios estruturados. `liknon-workspace` concentra contratos compartilhados
de workspace, descoberta e caminhos. As tres CLIs de capacidade usam crates
comuns, mas nao invocam umas as outras como subprocessos ocultos. Somente o
launcher pode invocar a CLI selecionada, sem importar sua API para executar a
capacidade.

Os nomes exatos das crates internas e a divisao final de APIs devem ser
confirmados durante a extracao. A fronteira de responsabilidade e mais
importante que preservar uma decomposicao especulativa.

## 5. Workspace Do Consumidor

Todos os produtos convergem para uma unica raiz local:

```text
.liknon/
├── validation/
│   ├── config.json
│   └── reports/
├── resources/
│   ├── manifest.json
│   └── lock.json
└── ai/
    ├── policy.json
    └── persistence/
```

Essa arvore e um mapa de propriedade, nao uma ordem para criar diretorios
vazios. Cada comando cria somente os arquivos e diretorios necessarios para a
funcionalidade explicitamente solicitada.

Responsabilidades:

- `.liknon/validation/config.json` define exclusivamente o que a validacao
  executa;
- `.liknon/validation/reports/` guarda relatorios exatos quando um fluxo
  autorizado decidir persisti-los;
- `.liknon/resources/manifest.json` registra recursos desejados;
- `.liknon/resources/lock.json` registra a resolucao exata e sua proveniencia;
- `.liknon/ai/policy.json` governa decisoes de workflows compativeis;
- `.liknon/ai/persistence/` guarda artefatos declarados por workflows que
  possuem continuidade.

Configuracao de validacao, policy de IA, recursos instalados e persistencia
analitica permanecem contratos diferentes.

## 6. Ecossistema De Recursos

O Hub e o Manager devem tratar recursos de forma generica. Os tipos iniciais
podem incluir:

- knowledge;
- skills;
- workflows;
- agent profiles;
- tool descriptors;
- prompts;
- guides;
- templates;
- collections.

Cada recurso possui identidade estavel, versao, origem, inventario de arquivos,
compatibilidade, dependencias e digests. Origem oficial, estado de revisao e
assinaturas sao metadados de proveniencia, nao autoridade automatica.

Conhecimento deixa de ser uma capacidade especial embutida na Validation CLI.
Ele passa a ser um tipo de recurso que o Manager pode obter e que Tool pode
consultar progressivamente. O usuario pode selecionar itens, colecoes focadas
ou uma colecao completa.

Conteudo instalado nunca e executado durante instalacao. A ativacao de uma
skill, workflow ou prompt depende de uma escolha posterior do humano ou do host
de IA sob seus contratos de seguranca.

## 7. Arquitetura De IA

### 7.1 Contrato De Entrada

Operacoes governadas por Liknon comecam por um contrato de bootstrap e
conformidade pertencente ao programa. Esse contrato:

- define ordem de leitura e autoridade;
- informa como localizar policy, configuracao, recursos e CLIs;
- permanece abaixo de instrucoes superiores do host, repositorio e humano;
- nao e substituivel por um recurso baixado do Hub;
- nao escolhe modelo, persona ou workflow.

Tool deve conseguir expor os bytes e metadados exatos desse contrato para um
host externo.

### 7.2 Policy

`.liknon/ai/policy.json` pertence exclusivamente ao humano. Uma IA pode ler,
validar, explicar e sugerir alteracoes, mas nao pode criar, editar, formatar,
mover, restaurar ou excluir esse arquivo.

Cada categoria de decisao possui:

- um valor global;
- overrides opcionais por ID canonico de workflow;
- precedencia do override aplicavel sobre o valor global;
- combinacao restritiva quando workflows compostos implicam mais de uma regra.

As categorias escalares usam a ordem `4 > 3 > 2 > 1 > 0`, com significado
proprio documentado para cada categoria. `processContinuation` e ternario:
`stop > human > auto` em ordem de restricao.

A policy limita tanto operacoes assistidas pontuais quanto workflows de
engenharia. Ela nao altera o grafo de validacao e nunca concede a um recurso
autoridade para ampliar seus proprios limites.

### 7.3 Workflows, Perfis E Skills

Workflow, agent profile e skill sao conceitos distintos:

- o **workflow** define o grafo, os nos, transicoes e referencias;
- o **agent profile** define papel e comportamento para um no;
- a **skill** descreve uma capacidade operacional reutilizavel;
- o **tool descriptor** descreve uma operacao de ferramenta;
- conhecimento e prompts fornecem contexto, nao autoridade.

Um workflow pode selecionar perfis diferentes em nos diferentes. Processos
`0 -> 1` e `2 -> 3` pertencem ao workflow oficial de engenharia de validacao;
eles nao sao constantes da plataforma.

### 7.4 Recursos Oficiais Iniciais

O conjunto inicial pretende oferecer pelo menos:

1. um workflow oficial de acao, voltado a uma tarefa delimitada e apoiado por
   AI-Tool;
2. um workflow oficial de engenharia de validacao, com modelagem de dominio,
   desenho de cobertura, triagem e avaliacao de drift;
3. perfis, skills, tool descriptors e conhecimentos referenciados por esses
   workflows.

Esses recursos oficiais testam a integracao real do ecossistema. Aplicacoes e
schemas, entretanto, devem aceitar recursos externos compativeis sem branches
por nome oficial.

## 8. Operacao Humana E Operacao Assistida

O fluxo humano continua completo e independente:

```text
humano -> config.json -> liknon-validate -> relatorio
```

O humano pode tambem usar uma IA pontualmente, com documentacao ou skills, sem
adotar um workflow persistente. Quando adota um workflow governado, o host de
IA usa o contrato de bootstrap, Tool, policy e recursos instalados para obter o
contexto minimo necessario.

Nenhuma CLI executa a cognicao do workflow. O host externo seleciona o modelo,
aplica o perfil e conduz os nos. Liknon fornece contratos, recursos, validacao
deterministica, persistencia delimitada e evidencia verificavel.

## 9. Distribuicao

### 9.1 Validation CLI

A direcao ja definida para a Validation CLI inclui:

- codigo fonte versionado no GitHub para clone e compilacao local;
- pacote fonte `liknon-validate` no crates.io;
- binarios nativos verificados e publicados por alvo no dominio do projeto,
  com armazenamento no Cloudflare;
- instalacao direta por script ou download pelo site;
- metadados de release e proveniencia sem usar o GitHub como origem obrigatoria
  dos binarios publicos.

Os detalhes de build, alvo, libc, checksums, assinatura, aprovacao e recuperacao
pertencem ao plano B01.

### 9.2 Launcher E Suite Completa

O pacote agregado `liknon-cli`, os comandos `liknon` e `likx`, a descoberta de
componentes e as suites nativas completas pertencem ao plano B04. A instalacao
agregada e uma alternativa promovida as instalacoes seletivas dos tres
produtos; ela nao transfere ao launcher a publicacao individual de nenhum
componente.

### 9.3 Manager E Tool

Manager e Tool usam as mesmas categorias de canal aprovadas para a Validation
CLI:

- codigo fonte versionado no GitHub para clone e compilacao local;
- pacotes fonte independentes no crates.io;
- binarios nativos independentes publicados no dominio do projeto com
  armazenamento no Cloudflare;
- instalacao direta por script ou download pelo site.

Essa decisao compartilha o modelo de distribuicao, nao os artefatos ou o ciclo
de prontidao. Cada CLI possui identidade de pacote, tags, builds, metadados,
evidencias, credenciais e autorizacao de publicacao proprios. Os contratos
detalhados de release pertencem respectivamente aos planos B02 e B03 e somente
podem ser finalizados depois dos contratos de produto dessas aplicacoes.

### 9.4 APIs Programaticas Futuras

Pacotes importaveis para Rust, Node.js, Ruby, Python e .NET pertencem ao plano
B05 e a uma fase futura. Eles so devem existir quando houver API deliberada e
consumidor real em cada ecossistema. Nao sao copias da CLI nem pacotes
genericos de conteudo.
O nome de pacote `liknon` fica reservado para essas integracoes e nao identifica
o pacote do launcher nem fornece o comando `liknon` por implicacao.

## 10. Decisoes Firmes

1. Existem tres CLIs de capacidade com responsabilidades independentes e um
   launcher sem capacidade propria.
2. Validation CLI nao deve depender do Manager, de Tool ou de recursos de IA.
3. Manager e a unica CLI que acessa o Hub e altera recursos instalados.
4. Tool e offline, somente leitura e nao executa conteudo retornado.
5. O corpus completo e os recursos de IA evoluem fora dos binarios das CLIs.
6. O workspace do consumidor usa uma unica raiz `.liknon/`.
7. Tipos de recurso e origem sao dimensoes distintas; a organizacao canonica
   usa o tipo antes da origem, como `skills/official/`.
8. Workflows, agent profiles, skills e tools possuem contratos separados.
9. As aplicacoes nao reconhecem recursos oficiais por branches embutidos.
10. Policy e humana, obrigatoria para workflows governados e imutavel por IA.
11. Regras globais podem receber overrides por workflow.
12. Os workflows oficiais iniciais sao recursos substituiveis, nao logica da
    plataforma.
13. APIs e bindings de linguagem ficam para uma fase posterior com casos de
    uso concretos.
14. Como ainda nao houve lancamento publico, a mudanca estrutural nao precisa
    preservar aliases, caminhos ou contratos substituidos por compatibilidade.
15. Validation CLI, Manager e Tool usam GitHub como canal de fonte, crates.io
    como canal de pacote fonte e Cloudflare/site como canal de binarios
    nativos, com releases e artefatos independentes.
16. `liknon-cli` e o pacote Cargo agregado; `liknon` e `likx` sao seus comandos
    de launcher, enquanto o nome de pacote `liknon` fica reservado para B05.
17. Toda invocacao de capacidade pelo launcher informa explicitamente
    `validate`, `manager` ou `tool`; nao existe capacidade padrao.
18. A ajuda raiz do launcher resume apenas os tres namespaces, e sua versao
    informa primeiro o launcher e depois os tres componentes resolvidos.

## 11. Questoes Ainda Abertas

Antes de transformar todo o estado-alvo em tarefas de implementacao, ainda e
necessario decidir:

1. a gramatica publica completa dos comandos de Manager e Tool;
2. o contrato exato do Hub, catalogo, assinaturas, revogacao e hospedagem;
3. os schemas finais de recursos, manifest, lock, workflows e agent profiles;
4. quais crates internas precisam ser publicas apenas para sustentar builds e
   quais permanecem privadas ao workspace;
5. qual produto valida ou inicializa policy e qual produto fornece operacoes
   deterministicas de persistencia, sem retirar do humano a propriedade de
   `policy.json` nem transferir cognicao de workflow para uma CLI;
6. o limite inicial entre persistencia fornecida pelas CLIs e persistencia
   conduzida pelo host de IA;
7. o primeiro conjunto publicavel de recursos oficiais e seus testes de
   conformidade;
8. quando cada API programatica futura possui justificativa suficiente para
   iniciar seu proprio contrato;
9. a precedencia confiavel de descoberta dos executaveis pelo launcher;
10. a compatibilidade, o versionamento coordenado ou independente e a
    recuperacao dos componentes da suite agregada;
11. o formato e o codigo de saida exatos da inspecao agregada de versoes quando
    um componente estiver ausente, invalido ou nao responder; e
12. a composicao e a propriedade das suites nativas completas por canal.

Esses pontos podem ser resolvidos em fases distintas. Eles nao bloqueiam a
documentacao do estado atual, mas bloqueiam afirmar que todos os planos de
Manager, Tool, Hub e bindings ja estao prontos para implementacao.

## 12. Direcao Para O Refinamento Dos Planos

Depois da aprovacao deste resumo e da avaliacao do estado atual, o refinamento
deve:

1. substituir o indice atual por um roteiro curto, coerente e end-to-end;
2. ligar cada tarefa a um fato atual ou a uma capacidade-alvo aprovada;
3. separar extracao estrutural, aprovacao de contrato, implementacao local e
   publicacao;
4. marcar explicitamente decisoes ainda abertas em vez de inventar contratos;
5. remover dos textos propostas superadas que nunca foram implementadas, sem
   criar falsas tarefas de remocao;
6. manter planos historicos concluidos apenas como evidencia do que realmente
   existe;
7. fazer cada plano subordinado depender deste resumo e do inventario atual,
   sem redefinir a arquitetura global.

## 13. Resultado Esperado

Ao final, Liknon sera um conjunto coeso, mas desacoplado:

- `liknon-validate` ou `likv` executa validacoes diretamente;
- `liknon-manager` ou `likm` obtem e organiza recursos;
- `liknon-tool` ou `likt` consulta recursos locais;
- `liknon` ou `likx` encaminha uma invocacao com namespace explicito para uma
  dessas tres capacidades;
- o Hub distribui conteudo versionado sem rebuild das CLIs;
- humanos podem operar diretamente;
- agentes podem operar diretamente ou por workflows governados;
- workflows oficiais exercitam o sistema sem limitar extensoes externas;
- APIs de linguagem surgem somente quando houver uso programatico real.

Esse estado preserva a qualidade do validador existente e cria uma base para
conteudo e assistencia de IA evoluirem sem transformar a Validation CLI em um
produto monolitico.
