// Informações sobre o colegiado

! local concat = space

// Variáveis

! var collocal = sala 2005 do ICEx

! var horacolfis = 14:00 às 20:00
! var horacolcc = 08:00 às 14:00
! var horacolsi = 16:00 às 22:00
! var horacolmatcomp = 08:00 às 14:00
! var horacolca = 08:00 às 14:00
! var horacolest = 08:00 às 14:00
! var horacolquim = 07:30 às 13:30
! var horacolcb = 08:00 às 14:00
! var horacolmat = 15:00 às 21:00

! var telcolfis = 3409-5806
! var telcolcc = 3409-7547
! var telcolmatcomp = 3409-7547
! var telcolca = 3409-5691
! var telcolest = 3409-5691
! var telcolquim = 3409-5799
! var telcolcb = 3409-5806
! var telcolmat = 3409-5987
! var telcolsi = 3409-5380

! var emailcolfis = colgradfis@icex.ufmg.br
! var emailcolcc = colgraddcc@icex.ufmg.br
! var emailcolmatcomp = matcomp@icex.ufmg.br
! var emailcolca = colgradest@icex.ufmg.br
! var emailcolest = colgradest@icex.ufmg.br
! var emailcolquim = colgradqui@icex.ufmg.br
! var emailcolcb = tania@dcc.ufmg.br
! var emailcolmat = colgradmat@icex.ufmg.br
! var emailcolsi = colgradsi@icex.ufmg.br

! var secretcolfis = Leonardo Soares da Silva
! var secretcolcc = Patrícia de Cássia Gomes Pimentel
! var secretcolmatcomp = Patrícia de Cássia Gomes Pimentel
! var secretcolca = Márcia de Sousa Fileto e Vera Lúcia Pereira Andrade Diniz
! var secretcolest = Márcia de Sousa Fileto e Vera Lúcia Pereira Andrade Diniz
! var secretcolquim = Aparecida Maria Ferreira dos Santos
! var secretcolcb = Tânia da Silva
! var secretcolmat = Túlio Marcos Gonçalves
! var secretcolsi = Lydia Helena de Souza

! var coordcolcc = Prof. Renato Antônio Celso Ferreira
! var coordcolmatcomp = Prof. Thiago Ferreira de Noronha
! var coordcolca = Prof. Bernardo Lanza Queiroz
! var coordcolest = Prof. Magda Carvalho Pires
! var coordcolquim = Prof. Amary César Ferreira
! var coordcolfis = Prof. José Guilherme M. A. Moreira
! var coordcolmat = Prof. Seme Gebara Neto
! var coordcolsi = Prof. Clodoveu Augusto Davis Junior

! var salacoordcolcc = 6307
! var salacoordcolmatcomp = 6316
! var salacoordcolca = 3052
! var salacoordcolest = 3045
! var salacoordcolquim = 121
! var salacoordcolfis = 4184
! var salacoordcolmat = 4216
! var salacoordcolsi = 6329

! array arrcolcontatar = telefone|telefones|contato|email|contatar|comunicar|comunico
! array arrsi = sistema|sistemas|sistemas de informacao|sistema de informacao|sistemas da informacao|sistema da informacao
! array arrcolhora = hora|horas|horario|horarios|quando

// Perguntas

+ [*] (colegiado|colegiados) [*]
- O colegiado é um elo entre os alunos, professores e a universidade, de modo a orientar 
^ e coordenar as atividades realizadas pelos alunos dentro do curso. É responsável pelos 
^ processos de matrícula, reopção, dispensa e inclusão de atividades acadêmicas regulares, 
^ transferência, obtenção de novo título ou outras formas de ingresso, aproveitamento de 
^ créditos e trancamentos parciais e totais de matrícula.

+ [*] onde [*] (colegiado|colegiados) [*]
- O colegiado fica na <bot collocal>.

+ [*] quando [*] (colegiado|colegiados) [*]
- Posso te informar os horários de um colegiado específico do ICEx. :)

+ [*] (colegiado|colegiados) [*] quando [*]
@ [*] quando [*] (colegiado|colegiados) [*]

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*]
@ [*] quando [*] (colegiado|colegiados) [*]

+ [*] (colegiado|colegiados) [*] (@arrcolhora) [*]
@ [*] quando [*] (colegiado|colegiados) [*]

+ [*] (sala|salas) [*] (colegiado|colegiados) [*]
@ [*] onde [*] (colegiado|colegiados) [*]

+ [*] (colegiado|colegiados) [*] (sala|salas) [*]
@ [*] onde [*] (colegiado|colegiados) [*]

+ [*] como [*] (chegar|chego) [*] (colegiado|colegiados) [*]
@ [*] onde [*] (colegiado|colegiados) [*]

+ [*] papel [*] coordenador [*]
- O coordenador é responsável por resolver qualquer assunto acadêmico relativo ao curso ou aos alunos de curso.

+ [*] [o que] [*] coordenador [*] faz [*]
@ [*] papel [*] coordenador [*]

+ [*] (conversa|conversar|encontrar) [*] (coordenador|coordenadora) [*]
- Envie um email para o colegiado do seu curso informando sempre seu nome, número de matrícula e assunto. 
^ Assim vocês podem agendar um horário.

+ [*] sou novo [*] ufmg [*]
- No Guia Acadêmico da UFMG você pode encontrar todas as informações necessárias para entender a estrutura da universidade, seus direitos e deveres.
^ De uma atenção especial aos itens que constam na seção "Vida Acadêmica" do Guia. https://www.ufmg.br/online/arquivos/anexos/guia-academico.pdf

+ [*] guia academico [*]
@ [*] sou novo [*] ufmg [*]

+ [*] [sou] calouro [*]
@ [*] sou novo [*] ufmg [*]

// HORARIOS ///////////////////////////////////////////////////////////////////////////////////

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*] ciclo basico [*]
- O colegiado do Ciclo Básico funciona de <bot horacolcb>.

+ [*] (colegiado|colegiados) [*] ciclo basico [*] (@arrcolhora)
- O colegiado do Ciclo Básico funciona de <bot horacolcb>.

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*] computacao [*]
- O colegiado de Ciência da Computação funciona de <bot horacolcc>.

+ [*] (colegiado|colegiados) [*] computacao [*] (@arrcolhora)
- O colegiado de Ciência da Computação funciona de <bot horacolcc>.

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*] aturais [*]
- O colegiado de Ciências Aturais funciona de <bot horacolca>.

+ [*] (colegiado|colegiados) [*] aturais [*] (@arrcolhora) [*]
- O colegiado de Ciências Aturais funciona de <bot horacolca>.

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*] estatistica [*]
- O colegiado de Estatística funciona de <bot horacolest>.

+ [*] (colegiado|colegiados) [*] estatistica [*] (@arrcolhora) [*]
- O colegiado de Estatística funciona de <bot horacolest>.

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*] fisica [*]
- O colegiado de Física funciona de <bot horacolfis>.

+ [*] (colegiado|colegiados) [*] fisica [*] (@arrcolhora) [*]
- O colegiado de Física funciona de <bot horacolfis>.

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*] (@arrsi) [*]
- O colegiado de Sistemas de Informação funciona de <bot horacolsi>.

+ [*] (colegiado|colegiados) [*] (@arrsi) [*] (@arrcolhora) [*]
- O colegiado de Sistemas de Informação funciona de <bot horacolsi>.

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*] quimica [*]
- O colegiado de Química funciona de <bot horacolquim>.

+ [*] (colegiado|colegiados) [*] quimica [*] (@arrcolhora) [*]
- O colegiado de Química funciona de <bot horacolquim>.

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*] matematica [*]
- O colegiado de Matemática funciona de <bot horacolmat>.

+ [*] (colegiado|colegiados) [*] matematica [*] (@arrcolhora) [*]
- O colegiado de Matemática funciona de <bot horacolmat>.

+ [*] (@arrcolhora) [*] (colegiado|colegiados) [*] (matematica computacional|matcomp) [*]
- O colegiado de Matemática Computacional funciona de <bot horacolmatcomp>.

+ [*] (colegiado|colegiados) [*] (matematica computacional|matcomp) [*] (@arrcolhora) [*]
- O colegiado de Matemática Computacional funciona de <bot horacolmatcomp>.

// TELEFONES E EMAIL ////////////////////////////////////////////////////////////////////////

+ [*] (@arrcolcontatar) [*] (colegiado|colegiados) [*] ciclo basico [*]
- Colegiado Ciclo Básico: <bot telcolcb> <bot emailcolcb>

+ [*] (@arrcolcontatar) [*] (colegiado|colegiados) [*] aturais [*]
- Colegiado Ciências Atuariais: <bot telcolca> <bot emailcolca>

+ [*] (@arrcolcontatar) [*] (colegiado|colegiados) [*] computacao [*]
- Colegiado Ciência da Computação: <bot telcolcc> <bot emailcolcc>

+ [*] (@arrcolcontatar) [*] (colegiado|colegiados) [*] (@arrsi) [*]
- Colegiado Sistemas de Informação: <bot telcolsi> <bot emailcolsi>

+ [*] (@arrcolcontatar) [*] (colegiado|colegiados) [*] matematica [*]
- Colegiado Matemática: <bot telcolmat> <bot emailcolmat>

+ [*] (@arrcolcontatar) [*] (colegiado|colegiados) [*] (matematica computacional|matcomp) [*]
- Colegiado Matemática Computacional: <bot telcolmatcomp> <bot emailcolmatcomp>

+ [*] (@arrcolcontatar) [*] (colegiado|colegiados) [*] quimica [*]
- Colegiado Química: <bot telcolquim> <bot emailcolquim>

+ [*] (@arrcolcontatar) [*] (colegiado|colegiados) [*] estatistica [*]
- Colegiado Estatística: <bot telcolest> <bot emailcolest>

+ [*] (@arrcolcontatar) [*] (colegiado|colegiados) [*] fisica [*]
- Colegiado Física: <bot telcolfis> <bot emailcolfis>

// COORDENADORES ///////////////////////////////////////////////////////////////////////////

+ [*] (coordenador|coordenadora) [*] aturais [*]
- <bot coordcolca>, sala <bot salacoordcolca> é responsável pela Coordenação de Ciências Aturais.

+ [*] (coordenador|coordenadora) [*] computacao [*]
- <bot coordcolcc>, sala <bot salacoordcolcc> é responsável pela Coordenação de Ciência da Computação.

+ [*] (coordenador|coordenadora) [*] (@arrsi) [*]
- <bot coordcolsi>, sala <bot salacoordcolsi> é responsável pela Coordenação de Sistemas de Informação.

+ [*] (coordenador|coordenadora) [*] matematica [*]
- <bot coordcolmat>, sala <bot salacoordcolmat> é responsável pela Coordenação de Matemática.

+ [*] (coordenador|coordenadora) [*] (matematica computacional|matcomp) [*]
- <bot coordcolmatcomp>, sala <bot salacoordcolmatcomp> é responsável pela Coordenação de Matemática Computacional.

+ [*] (coordenador|coordenadora) [*] quimica [*]
- <bot coordcolquim>, sala <bot salacoordcolquim> é responsável pela Coordenação de Química.

+ [*] (coordenador|coordenadora) [*] estatistica [*]
- <bot coordcolest>, sala <bot salacoordcolest> é responsável pela Coordenação de Estatística.

+ [*] (coordenador|coordenadora) [*] fisica [*]
- <bot coordcolfis>, sala <bot salacoordcolfis> é responsável pela Coordenação de Física.

// SECRETARIAS ////////////////////////////////////////////////////////////////////////////

+ [*] (secretaria|secretario|secretarias|secretarios) [*] aturais [*]
- <bot secretcolca> se encontra na secretaria do Colegiado de Ciências Aturais.

+ [*] (secretaria|secretario|secretarias|secretarios) [*] computacao [*]
- <bot secretcolcc> se encontra na secretaria do Colegiado de Ciência da Computação.

+ [*] (secretaria|secretario|secretarias|secretarios) [*] (@arrsi) [*]
- <bot secretcolsi> se encontra na secretaria do Colegiado de Sistemas de Informação.

+ [*] (secretaria|secretario|secretarias|secretarios) [*] matematica [*]
- <bot secretcolmat> se encontra na secretaria do Colegiado de Matemática.

+ [*] (secretaria|secretario|secretarias|secretarios) [*] (matematica computacional|matcomp) [*]
- <bot secretcolmatcomp> se encontra na secretaria do Colegiado de Matemática Computacional.

+ [*] (secretaria|secretario|secretarias|secretarios) [*] quimica [*]
- <bot secretcolquim> se encontra na secretaria do Colegiado de Química.

+ [*] (secretaria|secretario|secretarias|secretarios) [*] estatistica [*]
- <bot secretcolest> se encontra na secretaria do Colegiado de Estatística.

+ [*] (secretaria|secretario|secretarias|secretarios) [*] fisica [*]
- <bot secretcolfis> se encontra na secretaria do Colegiado de Física.
