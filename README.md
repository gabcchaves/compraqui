# Compraqui
Compraqui é um **projeto avaliativo da disciplina de programação para web** do curso de ciência da computação da UFOPA.

## Documento de visão
[Documento de Visão](https://docs.google.com/document/d/1iKREQ6ZBxhCZRAqSQzwqt6LYVD_vGgBPIaH7wz-FJ08/edit?usp=sharing)

## Suma dos requisitos funcionais
- O sistema deve realizar cadastro de usuários, os quais devem forncecer os seguintes dados:
	- Nome
	- Endereço de *e-mail*
	- Senha
- O sistema deve atribuir uma carteira ao usuário no seu cadastro.
- Os usuários devem ter ops seguintes dados:
	- ID
	- Nome
	- Endereço de *e-mail*
	- Senha (criptografada)
	- Carteira
- O sistema deve permitir consulta de perfil do usuário autenticado.
- O sistema deve permitir que o usuário adicione valores à sua carteira na página de consulta do seu perfil.
- O sistema deve realizar autenticação de usuários.
- O sistema deve apresentar lista de produtos disponíveis no banco de dados,mesmo que o usuário não esteja autenticado.
- O sistema deve permitir pesquisa de produtos a partir de nomes informados pelo usuário.
- O sistema deve permitir filtragem de produtos por faixa de preço.
- Os produtos devem ter os seguintes dados:
	- ID
	- Nome
	- Preço
	- Quantidade
- O sistema deve permitir consulta de produtos individualmente, apresentando os seguintes dados:
	- Nome
	- Preço
	- Quantidade
- O sistema deve permitir a compra de produtos, dada a quantidade desejada e a senha na autenticação.
- O sistema só deve efetivar compra de produtos se o usuário estiver autenticado, e se o saldo em sua carteira for suficiente à compra.
