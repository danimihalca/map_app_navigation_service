[ -f /run/secrets/mapbox_token ] && MAPBOX_TOKEN_VALUE=`cat /run/secrets/mapbox_token` || MAPBOX_TOKEN_VALUE=${MAPBOX_TOKEN}

/workspace/map_app_navigation_service_bin $MAPBOX_TOKEN_VALUE
