package mysqluser

import (
	"database/sql"
	"user/entity"
	"user/pkg/richerror"
	"user/repository/mysql"
)

type UserRepo struct {
	conn *mysql.MySQLDB
}

func New(c *mysql.MySQLDB) UserRepo {
	return UserRepo{
		conn: c,
	}
}

type userModel struct {
	id             uint
	username       string
	hashedPassword string
}

func (e *userModel) ToUserEntity() entity.User {
	var entiy entity.User

	entiy.ID = e.id
	entiy.UserName = e.username
	entiy.HashedPassword = e.hashedPassword

	return entiy
}

func (r UserRepo) GetUserByUsername(username string) (entity.User, bool, error) {
	const op = "mysqluser.GetUserByUsername"

	var model userModel

	row := r.conn.Conn().QueryRow(`select id, username, hashed_password from users where username = ?`, username)
	err := row.Scan(&model.id, &model.username, &model.hashedPassword)
	if err != nil {
		if err == sql.ErrNoRows {
			return entity.User{}, false, nil
		}

		return entity.User{}, false, richerror.New(op).
			WithErr(err).WithKind(richerror.KindUnexpected)
	}

	return model.ToUserEntity(), true, nil
}

func (r UserRepo) CreateUser(u entity.User) (entity.User, error) {
	const op = "mysqluser.CreateUser"

	res, err := r.conn.Conn().Exec(`insert into users (username, hashed_password) values (?, ?)`,
		u.UserName, u.HashedPassword)

	if err != nil {
		return entity.User{}, richerror.New(op).
			WithErr(err).WithKind(richerror.KindUnexpected)
	}

	// error is always nil
	id, _ := res.LastInsertId()
	u.ID = uint(id)

	return u, nil
}
