# Crud-Blog with Login made with Rust 

docker build -t <appname> .
docker run -d --name <appname> -e "PORT=8765" -p 8007:8765 <appname>

heroku login
docker ps
heroku container:login
heroku container:push web -a <appname>
heroku container:release web -a <appname>