echo "Test Case 1: Cow user get balance"
curl http://localhost:6789/balance\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12306"}'
echo ""

echo "Test Case 2: Student user get balance"
curl http://localhost:6789/balance\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12308"}'
echo ""

echo "Test Case 2: Student user get balance"
curl http://localhost:6789/balance\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12307"}'
echo ""
