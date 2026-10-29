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
| Pacote da suite | `liknon-cli` |
| Comando do launcher | `liknon` |
| Responsabilidade | Selecionar uma CLI de capacidade e encaminhar exatamente a invocacao |
| Rede | Nenhuma capacidade propria |
| Mutacao | Nenhuma capacidade propria |

O launcher reconhece os namespaces `validator`, `manager` e `tool`, remove
somente esse primeiro argumento posicional e encaminha o restante para o
executavel correspondente. Ele nao valida, instala, consulta, interpreta nem
repara componentes.

```text
liknon validator <args...> -> liknon-validator <args...>
liknon v <args...>         -> liknon-validator <args...>
liknon manager <args...>   -> liknon-manager <args...>
liknon m <args...>         -> liknon-manager <args...>
liknon tool <args...>      -> liknon-tool <args...>
liknon t <args...>         -> liknon-tool <args...>
```

`v`, `m` e `t` sao aliases do parser do launcher. Eles nao sao arquivos,
symlinks, alvos binarios Cargo nem artefatos de download. Nao existe namespace
padrao, alias de compatibilidade ou encaminhamento implicito.

O pacote `liknon-cli` possui exatamente quatro alvos binarios: `liknon`,
`liknon-validator`, `liknon-manager` e `liknon-tool`. A instalacao do
pacote instala os quatro por comportamento normal do Cargo, sem hooks de
pos-instalacao ou binarios pertencentes a dependencias.

### 3.2 Validation CLI

| Campo | Definicao-alvo |
| --- | --- |
| Implementacao de capacidade | Validator |
| Comando | `liknon-validator` |
| Responsabilidade | Configurar, inspecionar, planejar e executar validacoes |
| Rede | Nenhuma necessidade para o fluxo normal |
| Recursos de IA | Nao incorpora o corpus completo nem orquestra agentes |

Essa aplicacao preserva o motor agnostico ja existente. Ela continua sendo
utilizavel diretamente por humanos, automacao e agentes que conhecam seu
contrato publico.

### 3.3 Manager

| Campo | Definicao-alvo |
| --- | --- |
| Implementacao de capacidade | Manager |
| Comando | `liknon-manager` |
| Responsabilidade | Descobrir, obter, verificar, instalar, atualizar, remover e organizar recursos |
| Rede | Permitida somente nas operacoes declaradas |
| Mutacao | Unica CLI autorizada a alterar o estado local de recursos |

O Manager acessa o Hub, apresenta planos antes de mutacoes e registra o estado
desejado e o estado resolvido. Instalar um recurso nao significa executa-lo ou
ativa-lo.

### 3.4 Tool

| Campo | Definicao-alvo |
| --- | --- |
| Implementacao de capacidade | Tool |
| Comando | `liknon-tool` |
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
├── liknon-cli/           # pacote publico e quatro alvos binarios
├── liknon-validator/     # implementacao da capacidade de validacao
├── liknon-manager/       # implementacao da capacidade de recursos
└── liknon-tool/          # implementacao da capacidade de consulta

crates/
├── liknon-cli-ui/        # experiencia de terminal compartilhada
└── liknon-resource-contracts/ # contratos do ecossistema de recursos

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

`liknon-validator` concentra sua semantica de configuracao, planejamento,
execucao, relatorios estruturados, schemas e apresentacao especifica de
validacao em modulos internos de um unico pacote. `liknon-cli-ui` concentra
somente paletas, apresentacoes acessiveis, integracao visual com Clap, ajuda
generica e politica de terminal demonstravelmente comuns entre as CLIs. Formatos
JSON, codigos de saida e renderizadores de dominio continuam pertencendo a cada
aplicacao.

As tres CLIs de capacidade nao invocam umas as outras como subprocessos
ocultos. Somente o launcher pode invocar a CLI selecionada, sem importar sua
API para executar a capacidade. Os pacotes internos de implementacao deixam de
possuir alvos binarios publicos quando B04 consolidar a suite.

Uma crate compartilhada so deve surgir quando mais de um produto implementado
demonstrar essa necessidade. Validator e Manager demonstram a fronteira de
experiencia de terminal de `liknon-cli-ui`; Manager e Tool demonstram a
fronteira independente de `liknon-resource-contracts`. Nenhuma das duas crates
deve se transformar num modulo generico de conveniencia. A fronteira de
responsabilidade continua mais importante que criar decomposicoes
especulativas.

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
humano -> config.json -> liknon-validator -> relatorio
```

O humano pode tambem usar uma IA pontualmente, com documentacao ou skills, sem
adotar um workflow persistente. Quando adota um workflow governado, o host de
IA usa o contrato de bootstrap, Tool, policy e recursos instalados para obter o
contexto minimo necessario.

Nenhuma CLI executa a cognicao do workflow. O host externo seleciona o modelo,
aplica o perfil e conduz os nos. Liknon fornece contratos, recursos, validacao
deterministica, persistencia delimitada e evidencia verificavel.

## 9. Distribuicao

### 9.1 Fundacao Compartilhada De Distribuicao

Uma unica entrega CLI usa a fundacao de release mantida no repositorio. Essa
fundacao fornece build por alvo, manifests, proveniencia, empacotamento,
assinatura, instaladores, deteccao de plataforma e libc, verificacao,
publicacao, leitura publica e recuperacao.

B01 implementou primeiro grande parte desses mecanismos usando Validator. B04
os consolida em um unico produto publico: o pacote `liknon-cli` e uma suite
nativa completa por alvo. B01, B02 e B03 continuam donos dos comportamentos,
contratos e testes focados, mas nao publicam pacotes CLI ou downloads nativos
independentes depois do corte de B04.

### 9.2 Validation CLI

Validator permanece uma capacidade e um executavel diretamente utilizavel. B01
continua responsavel por sua semantica e evidencia focada. Seu executavel
`liknon-validator` passa a pertencer ao pacote e ao arquivo nativo da suite
definidos por B04.

### 9.3 Launcher E Suite Completa

O pacote `liknon-cli` e o unico pacote fonte publico da suite CLI. Ele instala
`liknon`, `liknon-validator`, `liknon-manager` e `liknon-tool`. As
implementacoes podem permanecer em crates de apoio, mas somente `liknon-cli`
possui os alvos binarios publicos.

Para distribuicao nativa, B04 compila os quatro executaveis para cada alvo e
gera um unico arquivo autenticado daquele alvo. GNU e MUSL permanecem arquivos
distintos. O site e o instalador selecionam exatamente um arquivo compativel;
nenhum usuario baixa binarios de outros alvos ou pacotes separados por
capacidade.

### 9.4 Manager E Tool

Manager e Tool permanecem executaveis e processos independentes, com contratos
e testes proprios em B02 e B03. Seus executaveis pertencem, contudo, ao mesmo
pacote fonte, versao, tag, arquivo nativo por alvo, manifest, instalador,
ponteiro estavel e ciclo publico da suite B04.

### 9.5 APIs Programaticas Futuras

Pacotes importaveis para Rust, Node.js, Ruby, Python e .NET pertencem ao plano
B05 e a uma fase futura. Eles so devem existir quando houver API deliberada e
consumidor real em cada ecossistema. Nao sao copias da CLI nem pacotes
genericos de conteudo.
O nome de pacote `liknon` fica reservado para essas integracoes e nao identifica
o pacote do launcher nem fornece o comando `liknon` por implicacao.

Uma capacidade inicial candidata e configurar explicitamente a integracao da
Validation CLI com gerenciadores de Git hooks ja presentes no workspace. O
pacote deve planejar e apresentar a diferenca antes de alterar arquivos,
preservar configuracao alheia, ser idempotente e nunca instalar a CLI ou mudar
hooks durante a instalacao do proprio pacote. O hook gerado chama diretamente
a rota nativa selecionada, enquanto a CI continua sendo a autoridade final de
validacao.

### 9.6 Primeira Publicacao Coordenada

B04 entrega um candidato exato da suite CLI. B02 entrega o Hub, E01 entrega o
site e B05 entrega somente integracoes programaticas ja ativadas. O plano B06
coordena a primeira ativacao publica desse conjunto: congela candidatos e
dependencias, confirma contas e raizes de confianca, ensaia a sequencia sem
credenciais de publicacao, verifica a leitura publica e promove ponteiros
estaveis.

B06 nao recompila candidatos aceitos. Um ecossistema de B05 ainda nao
implementado fica explicitamente fora do conjunto, sem pacote vazio ou
placeholder.

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
13. APIs, bindings e integracoes de configuracao por linguagem ficam para uma
    fase posterior com casos de uso concretos.
14. Como ainda nao houve lancamento publico, a mudanca estrutural nao precisa
    preservar aliases, caminhos ou contratos substituidos por compatibilidade.
15. `liknon-cli` e o unico pacote fonte publico da suite CLI e possui quatro
    executaveis: `liknon`, `liknon-validator`, `liknon-manager` e
    `liknon-tool`.
16. `liknon` e o unico executavel do launcher; o nome de pacote `liknon`
    permanece reservado para B05.
17. Toda invocacao de capacidade pelo launcher informa explicitamente
    `validator|v`, `manager|m` ou `tool|t`; nao existe capacidade padrao e
    os nomes curtos nao sao executaveis.
18. A ajuda raiz do launcher resume apenas os tres namespaces, e sua versao
    informa primeiro o launcher e depois os tres componentes resolvidos.
19. B01-B03 preservam propriedade de comportamento e testes; B04 possui uma
    unica identidade publica, versao, tag, pacote, suite nativa, credenciais e
    autoridade de publicacao para as quatro CLIs.
20. Cada alvo suportado possui um unico arquivo nativo com os quatro
    executaveis; nao existem downloads independentes por capacidade.
21. B06 coordena a primeira publicacao publica a partir do candidato exato de
    B04, do Hub de B02, do site de E01, das integracoes B05 selecionadas e da
    evidencia integrada, sem reconstruir artefatos.

## 11. Questoes Ainda Abertas

Antes de transformar todo o estado-alvo em tarefas de implementacao, ainda e
necessario decidir:

1. a gramatica publica completa dos comandos de Manager e Tool;
2. o contrato exato do Hub, catalogo, assinaturas, revogacao e hospedagem;
3. os schemas finais de recursos, manifest, lock, workflows e agent profiles;
4. se uma API compartilhada futura justificara extrair algum modulo para um
   crate proprio, sem antecipar essa separacao na arquitetura atual;
5. qual produto valida ou inicializa policy e qual produto fornece operacoes
   deterministicas de persistencia, sem retirar do humano a propriedade de
   `policy.json` nem transferir cognicao de workflow para uma CLI;
6. o limite inicial entre persistencia fornecida pelas CLIs e persistencia
   conduzida pelo host de IA;
7. o primeiro conjunto publicavel de recursos oficiais e seus testes de
   conformidade;
8. quando cada API programatica futura possui justificativa suficiente para
   iniciar seu proprio contrato;
9. a resolucao confiavel dos executaveis irmaos pelo launcher;
10. os detalhes de instalacao atomica e recuperacao da suite;
11. o formato e o codigo de saida exatos da inspecao de versoes quando
    um componente estiver ausente, invalido ou nao responder; e
12. a matriz inicial exata de alvos publicos e seus modos de autenticidade.

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

- `liknon-validator` executa validacoes diretamente;
- `liknon-manager` obtem e organiza recursos;
- `liknon-tool` consulta recursos locais;
- `liknon` encaminha uma invocacao com namespace explicito para uma dessas
  tres capacidades;
- `v`, `m` e `t` existem somente como aliases de namespace no launcher;
- `liknon-cli` instala os quatro executaveis e cada arquivo nativo por alvo
  entrega a suite completa;
- o Hub distribui conteudo versionado sem rebuild das CLIs;
- humanos podem operar diretamente;
- agentes podem operar diretamente ou por workflows governados;
- workflows oficiais exercitam o sistema sem limitar extensoes externas;
- APIs de linguagem surgem somente quando houver uso programatico real.

Esse estado preserva a qualidade do validador existente e cria uma base para
conteudo e assistencia de IA evoluirem sem transformar a Validation CLI em um
produto monolitico.
