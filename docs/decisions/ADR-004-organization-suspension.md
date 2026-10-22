# ADR-004: Suspensión de organizaciones

- **Estado:** Aceptado
- **Fecha:** 2026-10-22

## Contexto

Atlas es una API multi-tenant en la que cada usuario pertenece como máximo a una organización mediante una membresía.

Es necesario disponer de un mecanismo para suspender temporalmente una organización y bloquear el acceso a sus recursos sin eliminar sus datos ni perder sus relaciones con usuarios, proyectos y tareas.

La suspensión debe tener efecto sobre todas las operaciones protegidas, incluidas las lecturas y las modificaciones. También debe ser posible registrar los cambios de estado para mantener un historial auditable.

## Decisión

Se decidió implementar la suspensión de organizaciones mediante un estado propio de la organización.

Cuando una organización se encuentre suspendida:

- Se bloqueará el acceso a sus recursos mediante todas las operaciones protegidas, incluidas las lecturas.
- No se permitirá realizar modificaciones protegidas sobre sus datos.
- Sus usuarios, membresías, roles, proyectos, tareas y demás relaciones se conservarán.
- La reactivación de la organización permitirá recuperar el acceso únicamente cuando las demás condiciones de autorización también se cumplan.
- La suspensión de una organización no modificará automáticamente el estado individual de sus usuarios ni el de sus membresías.

Los cambios de estado deberán generar registros de auditoría que permitan identificar al actor responsable, la fecha del cambio y los estados anterior y nuevo.

El cambio de estado de la organización y su correspondiente evento de auditoría deberán persistirse de forma atómica.

## Alternativas consideradas

### 1. Eliminar la organización y sus datos

Esta alternativa implicaría eliminar la organización y posiblemente sus relaciones y recursos asociados.

Se descartó porque la suspensión debe ser reversible y conservar la información histórica.

### 2. Suspender individualmente a todos los miembros

Esta alternativa consistiría en cambiar el estado de cada membresía cuando se suspendiera una organización.

Se descartó porque mezcla el estado de la organización con el de sus membresías, complica la reactivación y podría reactivar membresías que ya estaban suspendidas por otros motivos.

### 3. Mantener un estado independiente en la organización

Esta alternativa utiliza el estado de la organización como condición de acceso común para sus recursos y miembros.

Se eligió porque permite bloquear y restaurar el acceso sin alterar innecesariamente los estados individuales ni eliminar información.

## Consecuencias

- La organización tendrá un estado persistente que determinará si puede utilizarse.
- La autorización deberá comprobar el estado actual de la organización.
- PostgreSQL Row-Level Security (RLS) formará parte de la defensa para restringir el acceso a los datos.
- Las operaciones protegidas deberán coordinarse con los cambios de estado para evitar que una petición continúe accediendo a recursos después de que la suspensión haya surtido efecto.
- Se deberá diseñar y probar un protocolo de transacciones y bloqueos en PostgreSQL para coordinar las lecturas, las escrituras y la suspensión.
- Las operaciones que interactúen con sistemas externos o procesos en segundo plano requerirán mecanismos adicionales si también deben detenerse.
- Los eventos de auditoría deberán conservarse y protegerse frente a modificaciones o eliminaciones no autorizadas.
- La reactivación no deberá restablecer accesos bloqueados por una cuenta o membresía suspendida.

## Validación pendiente

Aunque la política funcional está aceptada, el mecanismo técnico deberá validarse mediante pruebas de integración que cubran:

- Suspensión mientras existen lecturas protegidas en curso.
- Suspensión mientras existen modificaciones protegidas en curso.
- Rechazo de nuevas operaciones después de confirmar la suspensión.
- Reactivación sin restaurar accesos bloqueados por otros estados.
- Persistencia atómica del cambio de estado y del evento de auditoría.

## Referencias

- ADR-001: Usar PostgreSQL como base de datos
- ADR-002: Aislamiento multi-tenant de datos