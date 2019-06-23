echo "Test Case 1: Cow user recharge"
curl http://localhost:6789/balance/recharge\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "recharge_amount":0}'
echo ""

echo "Test Case 2: Student user recharge"
curl http://localhost:6789/balance/recharge\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12308", "recharge_amount":6}'
echo ""
