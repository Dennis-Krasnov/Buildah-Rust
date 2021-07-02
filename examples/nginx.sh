#!/usr/bin/env bash

CONTAINER=$(buildah from nginx:1.21)

buildah copy $CONTAINER html /usr/share/nginx/html

buildah commit $CONTAINER nginx_bash

buildah rm $CONTAINER
