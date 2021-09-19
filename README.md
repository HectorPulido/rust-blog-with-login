# Crud-Blog with Login made with Rust 

This is a Crud blog made with Rust + Actix + Diesel + HerokuPostgres, the frontend was made with Tera + bootstrap.

## Features

* Show all posts
* Show specific post
* Create a post
* Publish a post
* Login
* Admin permission


### Setup Django

1. cp .env.test .env

2. modify the .env and add your credentials

3. Configure your heroku project, save the name, you will need the Heroku Postgresql addon

4. Build the docker development and run it to make sure everything is ok

```
docker build -t web:latest .
docker run -d --name <herokuname> -e "PORT=8765" -e "DEBUG=0" -p 8007:8765 web:latest
```

5. You can deactivate the container like this
```
docker stop <herokuname>
docker rm <herokuname>
```

6. You can upload your project with this commands
```
docker run -d --name <herokuname> -e "PORT=8765" -e "DEBUG=0" -p 8007:8765 web:latest
heroku login
heroku container:login
heroku container:push web -a <herokuname>
heroku container:release web -a <herokuname>
```

7. Enter to your proyect from this url
```
http://<herokuname>.herokuapp.com/
```

8. You are ready


## Licence
This proyect was totally handcrafted by me, so the licence is MIT, use it as you want.

<div align="center">
<h3 align="center">Let's connect 😋</h3>
</div>
<p align="center">
<a href="https://www.linkedin.com/in/hector-pulido-17547369/" target="blank">
<img align="center" width="30px" alt="Hector's LinkedIn" src="https://www.vectorlogo.zone/logos/linkedin/linkedin-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://twitter.com/Hector_Pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitter" src="https://www.vectorlogo.zone/logos/twitter/twitter-official.svg"/></a> &nbsp; &nbsp;
<a href="https://www.twitch.tv/hector_pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitch" src="https://www.vectorlogo.zone/logos/twitch/twitch-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://www.youtube.com/channel/UCS_iMeH0P0nsIDPvBaJckOw" target="blank">
<img align="center" width="30px" alt="Hector's Youtube" src="https://www.vectorlogo.zone/logos/youtube/youtube-icon.svg"/></a> &nbsp; &nbsp;
