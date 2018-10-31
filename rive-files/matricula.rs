// Dúvidas a respeito da matrícula

! local concat = space

! array arrmatricula = (requerimento|matricula|matriculo|matricular|matriculas)

+ [*] siga [*]
- A matrícula na UFMG é online, realizada no Sistema Acadêmico da Graduação
^ (SIGA), sendo de inteira responsabilidade do aluno. Deve ser feita, a cada
^ semestre letivo, nos períodos fixados no Calendário Acadêmico da UFMG.
^ Você pode acessar o SIGA em MinhaUFMG>UFMG>Sistemas>Siga.

+ [*] (como|o que) [*] (@arrmatricula) [*]
- Para acessar o requerimento de matrícula, acessar o Portal MinhaUFMG, consultar o SIGA,
^ preencher o formulário de avaliação do
^ desempenho didático dos docentes das disciplinas/atividades cursadas no
^ semestre anterior e terá acesso ao Requerimento de Matrícula.

+ [*] fases [*] (@arrmatricula) [*]{weight=10}
- A primeira fase é obrigatória, e poderá requerer matrícula em atividades obrigatórias
^ ou optativas. Na segunda fase, poderá requerer novas atividades que nao tenham sido 
^ requeridas por voce na primeira fase. Na terceira fase, poderá requerer atividades
^ de Formação Livre.

+ [*] primeira fase [*] (@arrmatricula) [*]{weight=10}
- A primeira fase é obrigatória para todos os alunos de Graduação,
^ com exceção dos calouros. Poderá requerer matrícula para as atividades
^ obrigatórias, optativas e de Formação Complementar Aberta (se estiver
^ prevista no plano de estudos ou se houver oferta da atividade).
^ É preciso verificar se há inconsistências antes de enviar o requerimento.

+ [*] (optativa|optativas) [*]{weight=10}
- Na segunda fase, poderá requerer a inclusão de novas
^ atividades, obrigatórias e optativas, em turmas com vaga. 
^ Acesse o SIGA>Meu requerimento de matrícula>Requerimento de inclusão de atividades.
^ É preciso verificar se há inconsistências antes de enviar o requerimento.
^ Uma vez enviado, ele não poderá ser reaberto.

+ [*] (optativa|optativas) [*] (@arrmatricula) [*]{weight=10}
- Na segunda fase, poderá requerer a inclusão de novas
^ atividades, obrigatórias e optativas, em turmas com vaga. 
^ Acesse o SIGA>Meu requerimento de matrícula>Requerimento de inclusão de atividades.
^ É preciso verificar se há inconsistências antes de enviar o requerimento.
^ Uma vez enviado, ele não poderá ser reaberto.

+ [*] (@arrmatricula) [*] (optativa|optativas) [*]{weight=10}
- Na segunda fase, poderá requerer a inclusão de novas
^ atividades, obrigatórias e optativas, em turmas com vaga. 
^ Acesse o SIGA>Meu requerimento de matrícula>Requerimento de inclusão de atividades.
^ É preciso verificar se há inconsistências antes de enviar o requerimento.
^ Uma vez enviado, ele não poderá ser reaberto.

+ [*] segunda fase [*] (@arrmatricula) [*]{weight=10}
- Na segunda fase, poderá requerer a inclusão de novas
^ atividades, obrigatórias e optativas, em turmas com vaga. 
^ Acesse o SIGA>Meu requerimento de matrícula>Requerimento de inclusão de atividades.
^ É preciso verificar se há inconsistências antes de enviar o requerimento.
^ Uma vez enviado, ele não poderá ser reaberto.

+ [*] terceira fase [*] (@arrmatricula) [*]{weight=10}
- Na terceira fase,  poderá requerer matrícula para as
^ atividades de Formação Livre.
^ Acesse o SIGA>Meu requerimento de matrícula>Requerimento de formação livre.
^ É preciso verificar se há inconsistências antes de enviar o requerimento.
^ Uma vez enviado, ele não poderá ser reaberto.

+ [*] (eletiva|eletivas|formacao livre) [*]{weight=10}
- Na terceira fase da matrícula,  poderá requerer matrícula para as
^ atividades de Formação Livre.
^ Acesse o SIGA>Meu requerimento de matrícula>Requerimento de formação livre.
^ É preciso verificar se há inconsistências antes de enviar o requerimento.
^ Uma vez enviado, ele não poderá ser reaberto.

+ [*] (eletiva|eletivas|formacao livre) [*] (matricula|matricular|matriculo) [*]{weight=10}
- Na terceira fase da matrícula,  poderá requerer matrícula para as
^ atividades de Formação Livre.
^ Acesse o SIGA>Meu requerimento de matrícula>Requerimento de formação livre.
^ É preciso verificar se há inconsistências antes de enviar o requerimento.
^ Uma vez enviado, ele não poderá ser reaberto.

+ [*] (matricula|matricular|matriculo) [*] (eletiva|eletivas|formacao livre) [*]{weight=10}
- Na terceira fase da matrícula,  poderá requerer matrícula para as
^ atividades de Formação Livre.
^ Acesse o SIGA>Meu requerimento de matrícula>Requerimento de formação livre.
^ É preciso verificar se há inconsistências antes de enviar o requerimento.
^ Uma vez enviado, ele não poderá ser reaberto.

+ [*] (data|periodo|datas|dia|prazo|prazos|quando) [*] (@arrmatricula) [*]{weight=10}
- As datas para o requerimento de matrícula estão disponíveis no calendário
^ acadêmico da UFMG. https://www.ufmg.br/conheca/calendario.shtml

+ [*] disciplina [*] (topico|topicos) [*]{weight=10}
- Ao requerer matrícula em atividade do tipo TÓPICOS, deverá observar se as turmas 
^ de atividades ofertadas, com o mesmo código e nome (Tópicos em ....), diferem de 
^ acordo com o conteúdo (assunto) específico de cada uma das turmas e se o requerimento 
^ de matrícula será feito em uma única turma ou em várias turmas da atividade.

+ [*] (consistencia|inconsistencias|consistente|inconsistente) [*] (@arrmatricula){weight=10}
- Para finalizar o seu requerimento de matrícula, após as alterações, você deverá
^ salvar o requerimento e verificar a consistência do mesmo.
^ Ao acionar o comando [Verificar consistência], o requerimento pode não apresentar 
^ nenhuma mensagem de inconsistência (nesse caso, acionar o comando [Enviar requerimento sem
^ inconsistências]) ou o requerimento pode apresentar inconsistências do tipo “AVISO”. 
^ Nesse caso, as inconsistências deverão ser justificadas e enviadas para análise
^ e aprovação do Colegiado do Curso. Acione o comando [Enviar
^ requerimento com inconsistências].

+ [*] (@arrmatricula) [*] (consistencia|inconsistencias|consistente|inconsistente) [*]{weight=10}
- Para finalizar o seu requerimento de matrícula, após as alterações, você deverá
^ salvar o requerimento e verificar a consistência do mesmo.
^ Ao acionar o comando [Verificar consistência], o requerimento pode não apresentar 
^ nenhuma mensagem de inconsistência (nesse caso, acionar o comando [Enviar requerimento sem
^ inconsistências]) ou o requerimento pode apresentar inconsistências do tipo “AVISO”. 
^ Nesse caso, as inconsistências deverão ser justificadas e enviadas para análise
^ e aprovação do Colegiado do Curso. Acione o comando [Enviar
^ requerimento com inconsistências].

+ [*] resultado [*] (@arrmatricula) [*]{weight=10}
- Consulte no SIGA, ícone [Minhas Matrículas], para
^ verificar em quais atividades/turmas você foi
^ matriculado.

+ [*] como (sei|saber|fico sabendo) [*] (matriculado|matricula|matricula) [*]{weight=10}
- Consulte no SIGA, ícone [Minhas Matrículas], para
^ verificar em quais atividades/turmas você foi
^ matriculado.

+ [*] (materias|disciplinas) [*] posso [*] (cadastrar|enviar|adicionar) [*] (@arrmatricula) [*]{weight=10}
- Fique atento aos limites mínimo e máximo de créditos exigidos pelo seu curso.
^ Após adicionar suas disciplinas no requerimento, verifique se o mesmo está consistente.
^ Caso não esteja, terá que enviar para que seu colegiado aprove.

+ [*] (matricula|matricular) [*] (nao|sem) [*] [houver] [*] (vaga|vagas) [*]{weight=10}
- Preencha o requerimento com as disciplinas que estão sendo ofertadas presentes nas buscas no requerimento. O resultado da matrícula
^ lhe dirá se foi matriculado ou não. Consulte a data do resultado da matrícula em https://www.ufmg.br/conheca/calendario.shtml
