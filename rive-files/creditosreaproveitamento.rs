// Reaproveitamento de créditos de disciplinas cursadas

! local concat = space

! array arraproveitar = aproveitamento|reaproveitamento|aproveito|aproveitar|reaproveitar|reaproveito|reaproveitadas|aproveitadas
! array arrdisciplinas = disciplinas|materias|atividades|creditos

+ [*] (@arraproveitar) [*] (@arrdisciplinas) [*]
- Para reaproveitar disciplinas já cursadas em outra Instiruição, você deve preencher o formulário 
^ para Requerimento para Aproveitamento de Estudo e entregar no Colegiado do curso, juntamente com 
^ a ementa da disciplina e o histórico escolar. Para disciplinas obrigatórias, o processo é analisado 
^ pelo professor da disciplina. Se for optativa/eletiva, é analisado pelo coordenador. 
^ http://www.icex.ufmg.br/images/Requerimento_de_aproveitamento_de_estudos.pdf

+ [*] (qual|quais) [*] (@arrdisciplinas) [*] (@arraproveita) [*]
- Se a disciplina tem ementa e carga horária compatíveis com outra disciplina 
^ obrigatória do curso, ela pode ser reaproveitada mediante aprovação do professor 
^ da disciplina.
^ Disciplinas optativas devem ter ementa relacionada a disciplinas da computação 
^ (ou formação complementar escolhida).
^ Já nas eletivas se enquadram qualquer disciplina não relacionada ao curso.

+ [*] (@arraproveitar) [*] (iniciacao|iniciacao cientifica) [*]
- Compareça ao colegiado para preencher o formulário correspondente, e 
^ apresentar um relatório da atividade que deve ser assinado pelo orientador, 
^ que também lhe atribuirá uma nota.
^ Iniciação científica, empresa júnior, estágio  ou eventos podem contabilizar no máximo 8 créditos.
^ Iniciação científica 20hs/sem  4 cr

+ [*] (iniciacao|iniciacao cientifica) [*] (@arraproveitar) [*]
- Compareça ao colegiado para preencher o formulário correspondente, e 
^ apresentar um relatório da atividade que deve ser assinado pelo orientador, 
^ que também lhe atribuirá uma nota.
^ Iniciação científica, empresa júnior, estágio  ou eventos podem contabilizar no máximo 8 créditos.
^ Iniciação científica 20hs/sem  4 cr

+ [*] (@arraproveitar) [*] (estagio) [*]
- Compareça ao colegiado para preencher o formulário correspondente, e 
^ apresentar um relatório da atividade que deve ser assinado pelo orientador, 
^ que também lhe atribuirá uma nota.
^ Iniciação científica, empresa júnior, estágio ou eventos podem contabilizar no máximo 8 créditos.
^ Estágio 20hs/sem  4 cr

+ [*] (estagio) [*] (@arraproveitar) [*]
- Compareça ao colegiado para preencher o formulário correspondente, e 
^ apresentar um relatório da atividade que deve ser assinado pelo orientador, 
^ que também lhe atribuirá uma nota.
^ Iniciação científica, empresa júnior, estágio ou eventos podem contabilizar no máximo 8 créditos.
^ Estágio 20hs/sem  4 cr

+ [*] (empresa junior) [*] (@arraproveitar) [*]
- Compareça ao colegiado para preencher o formulário correspondente, e 
^ apresentar um relatório da atividade. A diretoria deve aprovar o relatório. 
^ A nota é atribuída pelo coordenador, depois de ler seu relatório.
^ Iniciação científica, empresa júnior, estágio ou eventos podem contabilizar no máximo 8 créditos.
^ Empresa Junior 30 hs  1 cr

+ [*] (empresa junior) [*] (@arraproveitar) [*]
- Compareça ao colegiado para preencher o formulário correspondente, e 
^ apresentar um relatório da atividade. A diretoria deve aprovar o relatório. 
^ A nota é atribuída pelo coordenador, depois de ler seu relatório.
^ Iniciação científica, empresa júnior, estágio ou eventos podem contabilizar no máximo 8 créditos.
^ Empresa Junior 30 hs  1 cr

+ [*] (@arraproveitar) [*] (empreendedorismo) [*]
- Compareça ao colegiado para preencher o formulário correspondente, e 
^ apresentar um relatório da atividade com dados sobre seu empreendimento/empresa, 
^ listando o objetivo, produto, clientes, etc. É interessante ressaltar como os 
^ conhecimentos do curso aparecem ou ajudam no seu negócio.
^ Iniciação científica, empresa júnior, estágio ou eventos podem contabilizar no máximo 8 créditos.
^ Empreendedorismo 30 hs  1 cr

+ [*] (empreendedorismo) [*] (@arraproveitar) [*]
- Compareça ao colegiado para preencher o formulário correspondente, e 
^ apresentar um relatório da atividade com dados sobre seu empreendimento/empresa, 
^ listando o objetivo, produto, clientes, etc. É interessante ressaltar como os 
^ conhecimentos do curso aparecem ou ajudam no seu negócio.
^ Iniciação científica, empresa júnior, estágio ou eventos podem contabilizar no máximo 8 créditos.
^ Empreendedorismo 30 hs  1 cr

+ [*] (@arraproveitar) [*] (evento|eventos) [*]
- Compareça ao colegiado para preencher o formulário correspondente e apresentar o 
^ comprovante de participação.
^ Iniciação científica, empresa júnior, estágio ou eventos podem contabilizar no máximo 8 créditos.
^ Participação em eventos cada trabalho/1 cr

+ [*] (evento|eventos) [*] (@arraproveitar) [*]
- Compareça ao colegiado para preencher o formulário correspondente e apresentar o 
^ comprovante de participação.
^ Iniciação científica, empresa júnior, estágio ou eventos podem contabilizar no máximo 8 créditos.
^ Participação em eventos cada trabalho/1 cr
