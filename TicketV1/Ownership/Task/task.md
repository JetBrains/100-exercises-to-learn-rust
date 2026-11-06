У цьому завданні потрібно переробити accessor methods (`title`, `description`, `status`) struct `Ticket`.
Ваша мета — змінити їхні сигнатури так, щоб вони приймали як аргумент immutable reference на self, тобто `&self`, а не брали ownership над ним.
Усі інструкції детально описано в коментарі `TODO` у коді.