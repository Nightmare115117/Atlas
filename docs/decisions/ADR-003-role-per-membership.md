# ADR-003: Un rol por membresía

- **Estado:** Aceptado
- **Fecha:** 2026-10-22

## Contexto

Atlas utiliza un modelo multi-tenant en el que cada usuario pertenece como máximo a una organización mediante una membresía.

Es necesario definir cómo se asignan los roles a los usuarios para controlar sus permisos dentro de la organización.

Permitir múltiples roles por usuario aumentaría la complejidad de la autorización, ya que sería necesario definir cómo se combinan los permisos y cómo se resuelven posibles conflictos entre roles.

## Decisión

Se decidió que cada membresía tendrá asignado un único rol.

Los roles pertenecerán a una organización y podrán configurarse según las necesidades de esta. Cada rol tendrá un conjunto de permisos que determinará las operaciones que puede realizar el usuario dentro de la organización.

La autorización se basará en el rol asignado a la membresía activa del usuario, junto con el estado de la cuenta y de la organización.

Los permisos disponibles para asignar a los roles corresponderán a las capacidades que Atlas reconoce e implementa.

## Alternativas consideradas

### 1. Permitir múltiples roles por membresía

Un usuario podría tener varios roles simultáneamente, combinando sus permisos.

Se descartó para la versión inicial debido a la complejidad adicional de definir la combinación de permisos, gestionar cambios y mantener un comportamiento de autorización predecible.

### 2. Asignar un único rol por membresía

Cada membresía tendría exactamente un rol, que agruparía los permisos necesarios para las responsabilidades del usuario.

Se eligió esta alternativa porque simplifica el modelo de datos, la autorización y la administración de permisos, manteniendo la posibilidad