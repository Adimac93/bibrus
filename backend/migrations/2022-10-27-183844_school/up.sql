create table schools(
    id uuid not null default gen_random_uuid() primary key,
    name varchar not null,
    place varchar not null,
    school_type varchar
);

create table groups(
    id uuid not null default gen_random_uuid() primary key,
    name varchar not null,
    school_id uuid not null,
    foreign key (school_id) references schools(id)
);

create table subjects(
    id uuid not null default gen_random_uuid() primary key,
    name varchar not null,
    school_id uuid not null,
    foreign key (school_id) references schools(id)
);

create table students(
    id uuid not null default gen_random_uuid() primary key,
    first_name varchar not null,
    last_name varchar not null,
    date_of_birth date not null,
    user_id uuid,
    group_id uuid not null,
    school_id uuid not null,
    foreign key (user_id) references users(id),
    foreign key (school_id) references schools(id),
    foreign key (group_id) references groups(id)
);

create table teachers(
    id uuid not null default gen_random_uuid() primary key,
    first_name varchar not null,
    last_name varchar not null,
    user_id uuid unique not null,
    school_id uuid not null,
    foreign key (user_id) references users(id),
    foreign key (school_id) references schools(id)
);

create table classes(
    id uuid not null default gen_random_uuid() primary key,
    subject_id uuid,
    group_id uuid,
    teacher_id uuid not null,
    foreign key (subject_id)  references subjects(id),
    foreign key (group_id) references groups(id),
    foreign key (teacher_id) references teachers(id)
);

create table class_students(
    primary key (class_id,student_id),
    class_id uuid not null,
    student_id uuid not null,
    foreign key (class_id) references classes(id),
    foreign key (student_id) references students(id)
);

create table tasks(
    id uuid not null default gen_random_uuid() primary key,
    name varchar
);

create table grades(
    primary key (student_id, subject_id, task_id),
    value float,
    weight int,
    task_id uuid,
    student_id uuid not null,
    subject_id uuid not null,
    teacher_id uuid not null,
    foreign key (task_id) references tasks(id),
    foreign key (student_id) references students(id),
    foreign key (subject_id) references subjects(id),
    foreign key (teacher_id) references teachers(id)
);
