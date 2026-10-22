# ADR-002: Aislamiento multi-tenant de datos

- **Estado:** Propuesto
- **Fecha:** 2026-10-22

## Contexto

Atlas está diseñado como una API multi-tenant en la que cada usuario pertenece como máximo a una organización.

Los recursos de la aplicación, como proyectos y tareas, pertenecen a una organización. Por lo tanto, es necesario garantizar que los usuarios solo puedan acceder a los recursos de la organización a la que pertenecen y para los que cuentan con los permisos correspondientes.

La aplicación debe prevenir accesos no autorizados entre organizaciones, incluidos aquellos ocasionados por errores en las consultas o por la manipulación de identificadores enviados por el cliente.

## Decisión

Se propone implementar un modelo de aislamiento multi-tenant basado en los siguientes mecanismos:

- Cada recurso deberá estar asociado a su organización propietaria.
- Las consultas de la aplicación deberán limitarse a la organización autorizada para el usuario.
- PostgreSQL Row-Level Security (RLS) se utilizará como una capa adicional de defensa para controlar el acceso a las filas.
- Las relaciones entre recursos deberán contar con restricciones de integridad referencial que impidan asociaciones entre organizaciones incompatibles.
- El contexto de organización utilizado por RLS deberá establecerse a partir de la identidad autenticada y de la autorización validada por la aplicación.
- Las operaciones protegidas deberán utilizar transacciones SQLx para mantener el contexto de RLS durante su ejecución.

## Alternativas consideradas

### 1. Aislamiento únicamente en la aplicación

Consiste en aplicar todos los filtros de organización y las comprobaciones de autorización desde la capa de aplicación.

Se considera insuficiente como única defensa, ya que una consulta que omita accidentalmente un filtro podría permitir el acceso a datos de otra organización.

### 2. Aislamiento mediante PostgreSQL RLS

Consiste en utilizar políticas de seguridad a nivel de fila para restringir los registros visibles o modificables según el contexto de la operación.

Se considera una defensa adicional útil, pero no sustituye la autorización de negocio ni una configuración correcta de los privilegios de la base de datos.

### 3. Aislamiento combinado entre aplicación y base de datos

Combina las comprobaciones de autorización de Atlas con las políticas RLS y las restricciones de integridad de PostgreSQL.

Esta es la alternativa seleccionada para el diseño de Atlas.

## Consecuencias

- Las consultas protegidas deberán respetar el aislamiento por organización.
- Se deberán crear y probar políticas RLS para cada tabla que requiera aislamiento.
- Las transacciones SQLx deberán establecer el contexto de organización de forma segura y con alcance limitado a la transacción.
- Las restricciones de integridad deberán impedir que un recurso se vincule con un proyecto de otra organización.
- Se deberán realizar pruebas de integración que verifiquen la imposibilidad de acceder a datos de otras organizaciones.
- La configuración de PostgreSQL deberá evitar que el rol de aplicación eluda las políticas RLS mediante privilegios excesivos.
- La coordinación entre operaciones protegidas y cambios de estado de las organizaciones deberá diseñarse y validarse por separado.

## Referencias

- Documentación oficial de PostgreSQL sobre Row Security Policies: https://www.postgresql.org/docs/current/ddl-rowsecurity.html
- ADR-001: Usar PostgreSQL como base de datos