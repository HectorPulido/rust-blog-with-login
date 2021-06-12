# Crud-Blog with Login made with Rust 

This is a Crud blog made with Rust + Actix + Diesel + HerokuPostgres, the frontend was made with Tera + bootstrap, you can test it here: https://blog-rs.herokuapp.com/


## Features

* Show all posts
* Show specific post
* Create a post
* Publish a post
* Login
* Admin permission

## How to use
1. cp .env.test .env
2. modify the .env to add your credentials
3. docker build -t <appname> .
4. docker run -d --name <appname> -e "PORT=8765" -p 8007:8765 <appname>

## How to Deploy
heroku login
docker ps
heroku container:login
heroku container:push web -a <appname>
heroku container:release web -a <appname>

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

</p>