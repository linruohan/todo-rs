# diesel demo

## 安装diesel-cli工具
cargo install diesel_cli --no-default-features --features sqlite
## 设置环境变量
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
echo DATABASE_URL=db.sqlite > .env
## diesel初始化,生成db.sqlite文件
diesel setup
## 初始化sql模板
diesel migration generate create_todos_table
## 生成schema文件
diesel print-schema >src/schema.rs

## 生成todo table 
修改sql后执行，检查sql语句是否正确
diesel migration redo
执行生成表格动作
diesel migration run

```bash
# Create a todo
curl -X POST -H "Content-Type: application/json" -d '{"title":"Buy groceries","content":"banana,milk"}' http://localhost:5002/todos
# List all todos
curl http://localhost:5002/todos
# Get a specific todo
curl http://localhost:5002/todos/1
# Update a todo
curl -X POST -H "Content-Type: application/json" -d '{"title":"Buy Groceries", "content": "banana"}' http://localhost:5002/todos/1
# Delete a todo
curl -X DELETE http://localhost:5002/todos/1
```