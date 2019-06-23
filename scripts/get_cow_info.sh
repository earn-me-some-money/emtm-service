echo "Get Cow Info Test Case 1: (True)"
curl http://localhost:6789/info/cow\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12306"}'
echo ""

echo "Get Cow Info Test Case 2: (False)"
curl http://localhost:6789/info/cow\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12307"}'
echo ""

