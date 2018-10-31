// Créditos de disciplinas

! local concat = space

// Variaveis

! var formcomplcc = http://dcc.ufmg.br/dcc/?q=BCC-formacao-complementar
! var formcomplsi = http://dcc.ufmg.br/dcc/?q=BSI-formacao-complementar

+ [*] (credito|creditos) [*]
- 1 crédito corresponde a 15h da carga horáia. Você poderá ter acesso 
^ à carga horária a ser cursada por você em 
^ MinhaUFMG>UFMG>Sistemas>Siga>Minhas Matrículas>Meu extrato de integralização curricular.

+ [*] formacao complementar [*]
- A partir de um certo período, você pode optar por uma 
^ das formações complementares do currículo do curso. 
^ Por exemplo, se você tem interesse em finanças, você pode optar pela formação 
^ complementar nessa área em específico. Assim, você deve fazer 24 créditos 
^ correspondentes às disciplinas listadas nessa formação escolhida.

+ [*] formacoes complementares [*]
@ [*] formacao complementar [*]

+ [*] como [*] formacao complementar [*]
- Ao escolher sua formação complementar, você deve comparecer pessoalmente ao 
^ colegiado, e preencher e assinar o formulário correspondente.

+ [*] (nao|nunca) [*] formacao complementar [*]
- Se você está no fim do curso e não escolheu nenhuma formação complementar, 
^ você ainda pode desde que você tenha tempo hábil para cursos todos os créditos. 
^ Nesse caso, você deve fazer um plano de quando pretende cursar quais disciplinas 
^ e submeter para análise da coordenadora no colegiado do curso.

+ [*] formacao complementar aberta [*]
- A Formação Complementar Aberta faz parte das Formações complementares e 
^ são propostas pelos próprios alunos. As disciplinas de FCA sempre aparecerão 
^ no seu histórico e na Minha UFMG como formação livre, conforme explicitado no 
^ quadra acima. Porém, na verdade elas são disciplinas OPTATIVAS dentre as 
^ selecionadas nos planos de FCA propostos.

+ [*] formacoes complementares (aberta|abertas) [*]
@ [*] formacao complementar aberta [*]

+ [*] (sugerir|sugiro|propor|proponho|crio|criar) [*] formacao complementar [*] 
- Para propor uma nova formação complementar, denominada Formação 
^ Complementar Aberta, o aluno deve certificar-se de que ela está 
^ relacionada ao curso. A partir daí, deve-se ter um professor orientador 
^ e uma relação das disciplinas relacionadas a formação proposta.
^ Envie um email para o coordenador, que pode te passar um documento 
^ exemplo. A formação complementar só passa a valer depois de aprovada 
^ pelo colegiado.

+ [*] (api|atividades praticas integradas|atividade pratica integrada) [*]
- A API é uma disciplina criada para desenvolvimento de projetos interdisciplinares 
^ entre diferentes departamentos da universidade. Seu resultado é como uma monografia, 
^ mas ela tem como requisitos obrigatórios a participação de dois professores 
^ orientadores, de diferentes departamentos da universidade.

+ [*] tempo [*] integralizacao [*]
- Você terá acesso ao tempo para a integralização do curso em 
^ MinhaUFMG>UFMG>Sistemas>Siga>Meus dados de Registro

+ [*] quantos [*] semestres [*] formar [*]
@ [*] tempo [*] integralizacao [*]

+ [*] (qual|quais) [*] (formacao complementar|formacoes complementares) [*]
- No momento, apenas conheço FC de Ciência da Computação e Sistemas de Informação.
^ Procure saber mais das formações complementares acessando a pagina 
^ do seu curso ou entrando em contato com o colegiado.
^ FC Ciência da Computação: <bot formcomplcc>
^ FC Sistemas de Informação: <bot formcomplsi>
