# ADR-001: Usar PostgreSQL como base de datos

- **Estado:** Aceptado
- **Fecha:** 2026-10-22

## Contexto

Atlas necesita un sistema gestor de bases de datos relacional que permita almacenar la información de usuarios, organizaciones, membresías, roles, permisos y recursos de la aplicación.

Además, se requiere un sistema que permita implementar restricciones de integridad referencial y mecanismos de aislamiento de datos entre organizaciones.

## Decisión

Se decidió utilizar PostgreSQL como sistema gestor de bases de datos para Atlas.

La elección se basa en sus capacidades de modelado relacional, sus restricciones de integridad, la compatibilidad con Row-Level Security (RLS) y sus mecanismos de transacciones y concurrencia.

Estas características se consideran relevantes para implementar el modelo multi-tenant y los requisitos de seguridad de Atlas.

## Alternativas consideradas

**MariaDB**

Se consideró MariaDB como alternativa por sus capacidades relacionales y su uso extendido en aplicaciones backend.

Sin embargo, se eligió PostgreSQL por las características de seguridad a nivel de fila y los mecanismos de integridad y concurrencia que se planea evaluar para Atlas.

## Consecuencias

- Se utilizará PostgreSQL como base de datos principal.
- El diseño del esquema y las migraciones deberán considerar las características específicas de PostgreSQL.
- Las políticas RLS requerirán configuración y pruebas para garantizar el aislamiento entre organizaciones.
- El equipo deberá familiarizarse con las herramientas y particularidades de PostgreSQL.

## Referencias

- Documentación oficial de PostgreSQL: https://www.postgresql.org/docs/