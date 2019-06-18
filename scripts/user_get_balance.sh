echo "Test Case 1: Cow user get balance"
curl http://localhost:6789/get_balance\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306"}'
echo ""

echo "Test Case 2: Student user get balance"
curl http://localhost:6789/get_balance\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12308"}'
echo ""

echo "Test Case 2: Student user get balance"
curl http://localhost:6789/get_balance\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307"}'
echo ""
