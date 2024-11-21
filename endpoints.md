1. [x] `POST /boards`: Создание новой доски. 

   - Параметры: `name` (имя доски), `creator_id` (идентификатор создателя доски)
   - Пример запроса:
     ```json
     {
       "name": "My Kanban Board",
       "creator_id": 1
     }
     ```

2. [x] `GET /boards`: Получение списка всех досок.

3. `GET /boards/{id}`: Получение информации о конкретной доске.

   - Параметр: `id` (идентификатор доски)

4. [x] `PUT /boards/{id}`: Обновление информации о конкретной доске.

   - Параметр: `id` (идентификатор доски)
   - Параметры: `name` (имя доски)

5. [x] `DELETE /boards/{id}`: Удаление конкретной доски.

   - Параметр: `id` (идентификатор доски)

6. [x] `POST /boards/{id}/columns`: Создание новой колонки в конкретной доске.

   - Параметр: `id` (идентификатор доски)
   - Параметры: `name` (имя колонки), `position` (позиция колонки)

7. [x] `GET /boards/{id}/columns`: Получение списка всех колонок в конкретной доске.

   - Параметр: `id` (идентификатор доски)

8. [x] `GET /boards/{id}/columns/{columnId}`: Получение информации о конкретной колонке в конкретной доске.

   - Параметры: `id` (идентификатор доски), `columnId` (идентификатор колонки)

9. [x] `PUT /boards/{id}/columns/{columnId}`: Обновление информации о конкретной колонке в конкретной доске.

   - Параметры: `id` (идентификатор доски), `columnId` (идентификатор колонки)
   - Параметры: `name` (имя колонки), `position` (позиция колонки)

10. [x] `DELETE /boards/{id}/columns/{columnId}`: Удаление конкретной колонки из конкретной доски.

    - Параметры: `id` (идентификатор доски), `columnId` (идентификатор колонки)

11. [x] `POST /boards/{id}/columns/{columnId}/cards`: Создание новой карточки в конкретной колонке.

    - Параметры: `id` (идентификатор доски), `columnId` (идентификатор колонки)
    - Параметры: `description` (описание карточки)

12. [x] `GET /boards/{id}/columns/{columnId}/cards`: Получение списка всех карточек в конкретной колонке.

    - Параметры: `id` (идентификатор доски), `columnId` (идентификатор колонки)

13. [x] `GET /boards/{id}/columns/{columnId}/cards/{cardId}`: Получение информации о конкретной карточке в конкретной колонке.

    - Параметры: `id` (идентификатор доски), `columnId` (идентификатор колонки), `cardId` (идентификатор карточки)

14. [x] `PUT /boards/{id}/columns/{columnId}/cards/{cardId}`: Обновление информации о конкретной карточке в конкретной колонке.

    - Параметры: `id` (идентификатор доски), `columnId` (идентификатор колонки), `cardId` (идентификатор карточки)
    - Параметры: `description` (описание карточки)

15. [x] `DELETE /boards/{id}/columns/{columnId}/cards/{cardId}`: Удаление конкретной карточки из конкретной колонки.

    - Параметры: `id` (идентификатор доски), `columnId` (идентификатор колонки), `cardId` (идентификатор карточки)

16. [x] `POST /boards/{id}/collaborators`: Добавление нового участника в конкретную доску.

    - Параметр: `id` (идентификатор доски)
    - Параметры: `user_id` (идентификатор пользователя), `role` (роль пользователя в доске)

17. [x] `GET /boards/{id}/collaborators`: Получение списка всех участников в конкретной доске.

    - Параметр: `id` (идентификатор доски)

18. [x] `DELETE /boards/{id}/collaborators/{collaboratorId}`: Удаление конкретного участника из конкретной доски.

    - Параметры: `id` (идентификатор доски), `collaboratorId` (идентификатор участника)

19. `PUT /boards/{id}/collaborators/{collaboratorId}`: Обновление информации о конкретном участнике в конкретной доске.
    - Параметры: `id` (идентификатор доски), `collaboratorId` (идентификатор участника)
    - Параметры: `role` (роль пользователя в доске)

Эти эндпоинты должны обеспечить основную функциональность для работы с досками, колонками, карточками и участниками. Однако, возможно, вам понадобятся дополнительные эндпоинты для реализации конкретных требований вашего приложения.

Пожалуйста, обратите внимание, что это только примеры эндпоинтов, и вы можете изменить их или добавить новые в соответствии с вашими потребностями.