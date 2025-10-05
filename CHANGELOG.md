# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Versionamento Semântico](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Added
- Suporte à nova interface do Vectorizer v0.3.0
- Integração com coleção `task-interactions` no Vectorizer
- Endpoint `/insert_texts` para inserção de dados no Vectorizer

### Changed
- **BREAKING**: Atualizada integração com Vectorizer
  - Porta padrão mudou de `15001` para `15002`
  - Endpoint de health mudou de `/api/v1/health` para `/health`
  - Endpoint de inserção mudou de `/collections/{name}/vectors` para `/insert_texts`
- Melhorada mensagem de conexão com Vectorizer
- Atualizada configuração padrão para usar nova porta do Vectorizer

### Fixed
- Corrigida conexão com Vectorizer após mudanças na interface
- Corrigido problema de inserção de dados no Vectorizer
- Corrigida configuração de coleção padrão para `task-interactions`

### Technical Details
- **Vectorizer Integration**: Atualizada para usar nova API do Vectorizer v0.3.0
- **Port Configuration**: Mudança de porta de 15001 para 15002
- **API Endpoints**: Atualizados para usar novos endpoints do Vectorizer
- **Collection Management**: Criação automática da coleção `task-interactions`

## [0.1.0] - 2025-10-05

### Added
- Sistema de fila de tarefas com suporte a workflows
- Integração com Vectorizer para persistência de contexto
- API REST para gerenciamento de tarefas
- Interface MCP para integração com IDEs
- Dashboard web para monitoramento
- SDKs em Python e TypeScript
- Sistema de autenticação e autorização
- Suporte a projetos e dependências
- Sistema de métricas e monitoramento
- Cache distribuído
- Rate limiting
- Logging estruturado
- Suporte a WebSocket
- CLI para gerenciamento

### Technical Features
- Arquitetura baseada em Rust com Axum
- Banco de dados SQLite com backup automático
- Integração com Vectorizer para busca semântica
- Sistema de workflows com fases de desenvolvimento
- Suporte a múltiplos ambientes (dev, test, prod)
- Containerização com Docker
- Monitoramento com Prometheus
- Documentação completa com exemplos

---

## Como Contribuir

Para contribuir com este projeto:

1. Faça um fork do repositório
2. Crie uma branch para sua feature (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova feature'`)
4. Push para a branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

## Versionamento

Este projeto usa [Versionamento Semântico](https://semver.org/lang/pt-BR/):

- **MAJOR** (1.0.0): Mudanças incompatíveis na API
- **MINOR** (0.1.0): Funcionalidades adicionadas de forma compatível
- **PATCH** (0.0.1): Correções de bugs compatíveis

## Links

- [Documentação Completa](docs/COMPLETE_DOCUMENTATION.md)
- [API Documentation](docs/API_DOCUMENTATION.md)
- [Roadmap de Desenvolvimento](docs/DEVELOPMENT_ROADMAP.md)
- [Guia de Testes](docs/TESTING_EXECUTION_GUIDE.md)
