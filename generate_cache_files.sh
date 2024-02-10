# DIST_DIR=./dist
#
# SERVICE_WORKER_FILE=$DIST_DIR/service-worker.js
# FILES_TO_CACHE=$(find $DIST_DIR -type d -exec echo -n '"{}/",' \; -or -type f -exec echo -n '"{}",' \;)
# FILES_TO_CACHE=$(echo $FILES_TO_CACHE | sed "s|$DIST_DIR/|./|g")
# FILES_TO_CACHE=$(echo $FILES_TO_CACHE | sed 's/,$//g')
# SUBSTITUTION_TAG='$FILES_TO_CACHE'
#
# echo "Generating cache files for service worker"
# echo "Files to cache: $FILES_TO_CACHE"
# echo "Service worker file: $SERVICE_WORKER_FILE"
# echo "Substitution tag: $SUBSTITUTION_TAG"
#
# # Replace the tag with the actual files
# sed -i "s|$SUBSTITUTION_TAG|$FILES_TO_CACHE|g" $SERVICE_WORKER_FILE
