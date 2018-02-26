
# REST Server for Indy-SDK Wallet

## REST Client

Client code is in the rest_client sub-directory.  This is a Rust application - there is a separate README.md file describing how to build and run this application.

## REST Server

The REST server is implemented using Django Rest Framework (DRF) and supports various authentication schemes.

Here is some light reading to get started:

http://www.django-rest-framework.org/tutorial/quickstart/

https://michaelwashburnjr.com/django-user-authentication/

http://cheng.logdown.com/posts/2015/10/27/how-to-use-django-rest-frameworks-token-based-authentication

http://www.django-rest-framework.org/api-guide/filtering/

### Building and Running the REST Server

This requires Python 3.6.

To build and run the server:

```
git checkout https://github.com/ianco/rest_server.git
cd rest_server
pip install -r requirements.txt
python manage.py makemigrations api
python manage.py migrate
python manage.py createsuperuser
# enter id, email and password
python manage.py runserver
```

You can now open a browser and connect to http://localhost:8000/ and browse the api.

If you login as your superuser (using the login link in the top right) you can create new items.

By default the code uses basic auth using the superuser id and password (see the code in the rest_client).

### Using DRF Tokens

There are a bunch of places in the code with comments for "the following is for DRF tokens" ... un-comment all this code.

There are a bunch of places in the code with comments for "the following is for JWS tokens" ... comment out this code!

Note that the DRF token is created automatically when your user is created, so you need to make sure all the above DRF code is enabled before you run your migrations and create your superuser.

Check the database for the DRF token for your superuser:

```
$ sqlite3 db.sqlite3
SQLite version 3.11.0 2016-02-15 17:29:24
Enter ".help" for usage hints.
sqlite> select * from authtoken_token;
71bee00fa76f08e5f17ceed783a9addd2619bc21|2018-02-26 00:33:17.846281|1
sqlite>
```

In the above example the DRF token is "71bee00fa76f08e5f17ceed783a9addd2619bc21" for my superuser (ian).

I can issue the following request using httpie:

```
http GET 127.0.0.1:8000/items/ 'Authorization: Token 71bee00fa76f08e5f17ceed783a9addd2619bc21'
```

Likewise for POST operations, etc.

### Using JWS Tokens

There are a bunch of places in the code with comments for "the following is for JWS tokens" ... un-comment all this code.

There are a bunch of places in the code with comments for "the following is for DRF tokens" ... comment out this code!


