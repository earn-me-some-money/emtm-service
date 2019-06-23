echo "Test Case 1: Cow user withdraw"
curl http://localhost:6789/balance/withdraw\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "withdraw_amount":1}'
echo ""

echo "Test Case 2: Student user withdraw"
curl http://localhost:6789/balance/withdraw\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "withdraw_amount":2}'
echo ""
