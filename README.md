# Atlas

API en Rust orientada a la gestión multi-tenant, con aislamiento de datos por organización y autorización basada en roles y permisos.

## Objetivo

Diseñar una API segura, mantenible y preparada para evolucionar a múltiples instancias.

## Principios de diseño

- Aislamiento estricto de datos entre organizaciones.
- Autorización basada en el estado actual almacenado en PostgreSQL.
- Roles configurables por organización y permisos definidos por Atlas.
- PostgreSQL Row-Level Security (RLS) como defensa adicional.
- Auditoría de cambios relevantes.
- Separación de responsabilidades entre las capas de la aplicación.

## Estado del proyecto

En fase de diseño de arquitectura y definición del MVP.

## Documentación

- Arquitectura: `docs/architecture.md`
- Modelo de datos: `docs/data-model.md`
- Seguridad: `docs/security.md`
- Decisiones arquitectónicas: `docs/decisions/`