#!/bin/sh

WEBHOOK_URL="https://hooks.slack.com/services/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"

MESSAGE_FILE=$(mktemp -t webhooks.XXXXXX)
trap "
rm ${MESSAGE_FILE}
" 0

cd scrayper
go run scrayper.go > ${MESSAGE_FILE}

MESSAGE="I notify the result of getting specimen today.\n"`cat ${MESSAGE_FILE}`

curl -sSX POST --data-urlencode "payload={\"channel\": \"#batch-job-notify\", \"username\": \"rucef.batch\", \"text\": \"${MESSAGE}\"}" ${WEBHOOK_URL} > /dev/null
rm ${MESSAGE_FILE}

